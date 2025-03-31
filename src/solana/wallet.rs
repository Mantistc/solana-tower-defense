use std::{fmt::Debug, sync::Arc};

use bevy::prelude::*;
use solana_sdk::{
    signature::Keypair,
    signer::{EncodableKey, Signer},
};

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
