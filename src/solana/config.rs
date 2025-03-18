use std::{collections::VecDeque, sync::Arc};

use bevy::prelude::*;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::commitment_config::CommitmentLevel;

use crate::VARIABLES;

use super::*;

pub struct SolanaPlugin;

impl Plugin for SolanaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SolClient(setup_solana_client()))
            .insert_resource(Wallet {
                keypair: load_keypair_from_file(),
                balance: 0,
                status_delay: Timer::from_seconds(5.0, TimerMode::Repeating),
                balance_task: None,
                transaction_tasks: VecDeque::new(),
            })
            .add_systems(
                Update,
                (check_balance, update_wallet_balance, process_tx_tasks),
            );
    }
}

pub const MESSAGE: &str = "Sign this message to start the game, anon.";

#[derive(Resource, Deref, DerefMut)]
pub struct SolClient(pub Arc<RpcClient>);

pub const SEND_CFG: RpcSendTransactionConfig = RpcSendTransactionConfig {
    skip_preflight: true,
    preflight_commitment: Some(CommitmentLevel::Confirmed),
    encoding: None,
    max_retries: Some(3),
    min_context_slot: None,
};

pub fn setup_solana_client() -> Arc<RpcClient> {
    let rpc_url = VARIABLES.sol_rpc;
    Arc::new(RpcClient::new(rpc_url.to_string()))
}
