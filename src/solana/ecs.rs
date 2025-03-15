use bevy::prelude::*;
use solana_sdk::{
    signature::Keypair,
    signer::{EncodableKey, Signer},
};

use super::*;

#[derive(Resource, Deref, DerefMut)]
pub struct WalletBalance(pub u64);

#[derive(Resource, Deref, DerefMut)]
pub struct PlayerSigner(pub Keypair);

pub fn load_keypair_from_file() -> Keypair {
    Keypair::read_from_file(WALLET_PATH).unwrap_or(Keypair::new())
}

pub fn get_wallet_balance(
    solana_client: Res<SolClient>,
    mut wallet_balance: ResMut<WalletBalance>,
    signer: Res<PlayerSigner>,
) {
    if let Ok(balance) = solana_client.get_balance(&signer.pubkey()) {
        wallet_balance.0 = balance;
        info!("Updated wallet balance: {} SOL", wallet_balance.0);
    }
}

pub fn sign_message(signer: &Res<PlayerSigner>) {
    let signature = signer.sign_message(MESSAGE.as_bytes());
    let is_valid_signature = signature.verify(&signer.pubkey().to_bytes(), MESSAGE.as_bytes());
    println!("valid signature: {:?}", is_valid_signature);
}
