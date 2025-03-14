use bevy::prelude::*;
use solana_sdk::pubkey::Pubkey;

use super::*;

pub const WALLET: Pubkey = Pubkey::from_str_const("adTJ5xniDsxZqJVRE5WKfx8btNR9wPgv5SUJZiS7fuN");

#[derive(Resource, Deref, DerefMut)]
pub struct WalletBalance(pub u64);

pub fn update_wallet_balance(
    solana_client: Res<SolClient>,
    mut wallet_balance: ResMut<WalletBalance>,
) {
    if let Ok(balance) = solana_client.get_balance(&WALLET) {
        wallet_balance.0 = balance;
        info!("Updated wallet balance: {} SOL", wallet_balance.0);
    }
}
