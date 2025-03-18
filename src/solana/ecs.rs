use std::{collections::VecDeque, sync::Arc};

use bevy::{
    prelude::*,
    tasks::{block_on, poll_once, AsyncComputeTaskPool, Task},
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::{EncodableKey, Signer},
    system_instruction::transfer,
    transaction::VersionedTransaction,
};

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

pub fn load_keypair_from_file() -> Arc<Keypair> {
    // if there is no wallet available in the *signer_wallet_path*, a new one will be generated to sign the message
    // but, probably the cfg.toml will not allow the program to compile if there is something missing
    Arc::new(Keypair::read_from_file(VARIABLES.signer_wallet_path).unwrap_or(Keypair::new()))
}

pub fn sign_message(wallet: &Res<Wallet>) {
    let signature = wallet.keypair.sign_message(MESSAGE.as_bytes());
    let is_valid_signature =
        signature.verify(&wallet.keypair.pubkey().to_bytes(), MESSAGE.as_bytes());
    println!("valid signature: {:?}", is_valid_signature);
}

pub fn send_sol(signer: Arc<Keypair>, client: Arc<RpcClient>) -> Result<Signature, String> {
    let to_pubkey = Pubkey::from_str_const(&VARIABLES.payment_wallet);
    let lamports = 100_000_000;
    let ix = transfer(&signer.pubkey(), &to_pubkey, lamports);

    let blockhash = client
        .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
        .map_err(|e| format!("failed to fetch latest blockhash: {:?}", e))?
        .0;

    let compiled_message = Message::try_compile(&signer.pubkey(), &[ix], &[], blockhash)
        .map_err(|e| format!("failed to compile message: {:?}", e))?;

    let versioned_msg = VersionedMessage::V0(compiled_message);
    let versioned_tx = VersionedTransaction::try_new(versioned_msg, &[signer])
        .map_err(|e| format!("failed to create transaction: {:?}", e))?;

    let signature = client
        .send_transaction_with_config(&versioned_tx, SEND_CFG)
        .map_err(|e| format!("failed to send transaction: {:?}", e))?;

    Ok(signature)
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
                Ok(signature) => info!("sol sent, tx signature: {:?}", signature),
                Err(e) => error!("failed to send transaction: {:?}", e),
            }
        } else {
            wallet_tasks.transaction_tasks.push_front(task);
        }
    }
}
