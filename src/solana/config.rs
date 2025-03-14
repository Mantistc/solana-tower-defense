use bevy::prelude::*;
use solana_client::rpc_client::RpcClient;

use super::*;

#[derive(Resource, Deref, DerefMut)]
pub struct SolClient(pub RpcClient);

pub fn setup_solana_client() -> RpcClient {
    let rpc_url = "https://api.devnet.solana.com";
    RpcClient::new(rpc_url.to_string())
}

pub struct SolanaPlugin;

impl Plugin for SolanaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SolClient(setup_solana_client()))
            .insert_resource(PlayerSigner(load_keypair_from_file()))
            .insert_resource(WalletBalance(0))
            .add_systems(Startup, (get_wallet_balance, sign_message));
    }
}

pub const WALLET_PATH: &str = "keypair/wallet.json";
pub const MESSAGE: &str = "Sign this message to start the game, anon.";
