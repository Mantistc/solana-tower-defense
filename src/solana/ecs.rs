use std::{
    collections::VecDeque,
    future::Future,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::{
    prelude::*,
    tasks::{block_on, poll_once, AsyncComputeTaskPool, Task},
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::{EncodableKey, Signer},
    system_instruction::transfer,
};
use td_program_sdk::{instructions, PLAYER_SEED, PROGRAM_ID};

use crate::VARIABLES;

use super::*;

#[derive(Resource, Debug)]
pub struct Wallet {
    pub keypair: Arc<Keypair>,
    pub balance: u64,
    pub status_delay: Timer,
    pub balance_task: Option<Task<Result<u64, solana_client::client_error::ClientError>>>,
    pub transaction_tasks: VecDeque<Task<Result<Signature, String>>>,
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            keypair: load_keypair_from_file(),
            balance: 0,
            status_delay: Timer::from_seconds(5.0, TimerMode::Repeating),
            balance_task: None,
            transaction_tasks: VecDeque::new(),
        }
    }
}

impl Wallet {
    pub fn add_task<F>(&mut self, future: F)
    where
        F: Future<Output = Result<Signature, String>> + Send + 'static,
    {
        let task = AsyncComputeTaskPool::get().spawn(future);
        self.transaction_tasks.push_back(task);
    }
}

pub fn load_keypair_from_file() -> Arc<Keypair> {
    // if there is no wallet available in the *signer_wallet_path*, a new one will be generated to sign the message
    // but, probably the cfg.toml will not allow the program to compile if there is something missing
    Arc::new(
        Keypair::read_from_file(VARIABLES.signer_wallet_path).unwrap_or_else(|e| {
            error!(
                "failed to load signer wallet from '{}': {:?}",
                VARIABLES.signer_wallet_path, e
            );
            Keypair::new()
        }),
    )
}

pub fn sign_message(wallet: &ResMut<Wallet>) {
    let signature = wallet.keypair.sign_message(MESSAGE.as_bytes());
    let is_valid_signature =
        signature.verify(&wallet.keypair.pubkey().to_bytes(), MESSAGE.as_bytes());
    println!("valid signature: {:?}", is_valid_signature);
}

pub async fn send_sol(signer: Arc<Keypair>, client: Arc<RpcClient>) -> Result<Signature, String> {
    let to_pubkey = Pubkey::from_str_const(&VARIABLES.payment_wallet);
    let lamports = 100_000_000;
    let ix = transfer(&signer.pubkey(), &to_pubkey, lamports);
    build_and_send_tx(signer, client, &[ix])
}

pub async fn initialize_player(
    signer: Arc<Keypair>,
    client: Arc<RpcClient>,
) -> Result<Signature, String> {
    let signer_pubkey = signer.pubkey();
    let seeds = [PLAYER_SEED, signer_pubkey.as_ref()];
    let now = SystemTime::now();
    let last_time_played = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let (player, bump) = Pubkey::find_program_address(&seeds, &PROGRAM_ID);
    let ix = instructions::initialize_player(&player, &signer_pubkey, last_time_played, bump); // need to refac this ix 100%
    build_and_send_tx(signer, client, &[ix])
}

pub fn check_balance(mut wallet: ResMut<Wallet>, client: Res<SolClient>, time: Res<Time>) {
    wallet.status_delay.tick(time.delta());

    if wallet.status_delay.just_finished() && wallet.balance_task.is_none() {
        let pubkey = wallet.keypair.pubkey();
        let client = client.clone();

        let task = AsyncComputeTaskPool::get().spawn(async move { client.get_balance(&pubkey) });

        wallet.balance_task = Some(task);
    }
}

pub fn update_wallet_balance(mut wallet: ResMut<Wallet>) {
    if let Some(mut task) = wallet.balance_task.take() {
        if let Some(result) = block_on(poll_once(&mut task)) {
            match result {
                Ok(balance) => {
                    wallet.balance = balance;
                    info!("wallet balance: {} SOL", wallet.balance);
                }
                Err(e) => {
                    error!("failed to fetch wallet balance: {:?}", e);
                }
            }
        } else {
            wallet.balance_task = Some(task);
        }
    }
}

pub fn process_tx_tasks(mut wallet_tasks: ResMut<Wallet>) {
    if let Some(mut task) = wallet_tasks.transaction_tasks.pop_front() {
        if let Some(result) = block_on(poll_once(&mut task)) {
            match result {
                Ok(signature) => info!("tx sent, signature: {:?}", signature),
                Err(e) => error!("failed to send transaction: {:?}", e),
            }
        } else {
            wallet_tasks.transaction_tasks.push_front(task);
        }
    }
}
