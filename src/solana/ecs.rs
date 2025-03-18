use std::sync::Arc;

use bevy::prelude::*;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    signature::Keypair,
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
}

pub fn load_keypair_from_file() -> Arc<Keypair> {
    // if there is no wallet available in the *igner_wallet_path*, a new one will be generated to sign the message
    Arc::new(Keypair::read_from_file(VARIABLES.signer_wallet_path).unwrap_or(Keypair::new()))
}

pub fn get_wallet_balance(solana_client: Res<SolClient>, mut wallet: ResMut<Wallet>) {
    if let Ok(balance) = solana_client.get_balance(&wallet.keypair.pubkey()) {
        wallet.balance = balance;
        info!("Updated wallet balance: {} SOL", wallet.balance);
    }
}

pub fn sign_message(wallet: &Res<Wallet>) {
    let signature = wallet.keypair.sign_message(MESSAGE.as_bytes());
    let is_valid_signature =
        signature.verify(&wallet.keypair.pubkey().to_bytes(), MESSAGE.as_bytes());
    println!("valid signature: {:?}", is_valid_signature);
}

pub fn send_sol(wallet: Res<Wallet>, client: Res<SolClient>) {
    let to_pubkey = Pubkey::from_str_const(&VARIABLES.payment_wallet);

    let lamports = 10_000_000;
    let ix = transfer(&wallet.keypair.pubkey(), &to_pubkey, lamports);

    let blockhash = match client.get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
    {
        Ok((hash, _)) => hash,
        Err(e) => {
            error!("failed to fetch latest blockhash: {:?}", e);
            return;
        }
    };

    let compiled_message =
        match Message::try_compile(&wallet.keypair.pubkey(), &[ix], &[], blockhash) {
            Ok(msg) => msg,
            Err(e) => {
                error!("failed to compile message: {:?}", e);
                return;
            }
        };

    let versioned_msg = VersionedMessage::V0(compiled_message);

    let versioned_tx = match VersionedTransaction::try_new(versioned_msg, &[wallet.keypair.clone()])
    {
        Ok(tx) => tx,
        Err(e) => {
            error!("failed to create transaction: {:?}", e);
            return;
        }
    };

    match client
        .0
        .send_transaction_with_config(&versioned_tx, SEND_CFG)
    {
        Ok(signature) => info!("payment success: {:?}", signature),
        Err(e) => error!("failed to send transaction: {:?}", e),
    };
}
