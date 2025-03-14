use bevy::prelude::*;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    signer::{EncodableKey, Signer},
};

use super::*;

pub const WALLET: Pubkey = Pubkey::from_str_const("adTJ5xniDsxZqJVRE5WKfx8btNR9wPgv5SUJZiS7fuN");

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

pub fn sign_message(signer: Res<PlayerSigner>) {
    let message = "Bro just sign the message";

    let signature = signer.sign_message(message.as_bytes());
    let is_valid_signature = signature.verify(&signer.pubkey().to_bytes(), message.as_bytes());
    println!("valid signature: {:?}", is_valid_signature);
}
