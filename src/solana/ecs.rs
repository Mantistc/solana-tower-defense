use std::{
    collections::VecDeque,
    fmt::Debug,
    future::Future,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::{
    prelude::*,
    tasks::{block_on, poll_once, AsyncComputeTaskPool, Task},
};
use solana_client::{client_error::ClientError, rpc_client::RpcClient};
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
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            keypair: load_keypair_from_file(),
            balance: 0,
        }
    }
}

#[derive(Debug)]
pub enum TaskResult {
    Balance(u64),
    Signature(Signature),
}

pub type ActionResult = Result<TaskResult, ClientError>;

#[derive(Resource, Debug)]
pub struct Tasks {
    pub status_delay: Timer,
    pub pending_tasks: VecDeque<Task<ActionResult>>,
}

impl Default for Tasks {
    fn default() -> Self {
        Self {
            status_delay: Timer::from_seconds(5.0, TimerMode::Repeating),
            pending_tasks: VecDeque::new(),
        }
    }
}

impl Tasks {
    pub fn add_task<F>(&mut self, future: F)
    where
        F: Future<Output = ActionResult> + Send + 'static,
    {
        let task = AsyncComputeTaskPool::get().spawn(future);
        self.pending_tasks.push_back(task);
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

pub async fn send_sol(signer: Arc<Keypair>, client: Arc<RpcClient>) -> ActionResult {
    let to_pubkey = Pubkey::from_str_const(&VARIABLES.payment_wallet);
    let lamports = 100_000_000;
    let ix = transfer(&signer.pubkey(), &to_pubkey, lamports);
    build_and_send_tx(signer, client, &[ix])
}

pub async fn initialize_player(signer: Arc<Keypair>, client: Arc<RpcClient>) -> ActionResult {
    let signer_pubkey = signer.pubkey();
    let seeds = [PLAYER_SEED, signer_pubkey.as_ref()];
    let now = SystemTime::now();
    let last_time_played = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let (player, bump) = Pubkey::find_program_address(&seeds, &PROGRAM_ID);
    let ix = instructions::initialize_player(&player, &signer_pubkey, last_time_played, bump); // need to refac this ix 100%
    build_and_send_tx(signer, client, &[ix])
}

pub fn check_balance(
    wallet: ResMut<Wallet>,
    mut tasks: ResMut<Tasks>,
    client: Res<SolClient>,
    time: Res<Time>,
) {
    tasks.status_delay.tick(time.delta());

    if tasks.status_delay.just_finished() {
        let pubkey = wallet.keypair.pubkey();
        let client = client.clone();

        tasks.add_task(async move { client.get_balance(&pubkey).map(TaskResult::Balance) });
    }
}

pub fn process_tx_tasks(mut tasks: ResMut<Tasks>, mut wallet: ResMut<Wallet>) {
    if let Some(mut task) = tasks.pending_tasks.pop_front() {
        if let Some(result) = block_on(poll_once(&mut task)) {
            match result {
                Ok(tx_result) => match tx_result {
                    TaskResult::Balance(balance) => {
                        wallet.balance = balance;
                        info!("wallet balance updated: {} SOL", balance);
                    }
                    TaskResult::Signature(sig) => {
                        info!("transaction sent, signature: {:?}", sig);
                    }
                },
                Err(err) => {
                    error!("task failed: {:?}", err);
                }
            }
        } else {
            tasks.pending_tasks.push_front(task);
        }
    }
}
