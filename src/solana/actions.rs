use std::sync::Arc;

use bevy::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction::transfer,
};
use td_program_sdk::{instructions, seeds::PLAYER_SEED, PROGRAM_ID};

use crate::VARIABLES;

use super::*;

pub async fn send_sol(signer: Arc<Keypair>, client: Arc<RpcClient>) -> ActionResult {
    let to_pubkey = Pubkey::from_str_const(&VARIABLES.payment_wallet);
    let lamports = 100_000_000;
    let ix = transfer(&signer.pubkey(), &to_pubkey, lamports);
    build_and_send_tx(signer, client, &[ix])
}

pub async fn initialize_player(signer: Arc<Keypair>, client: Arc<RpcClient>) -> ActionResult {
    let signer_pubkey = signer.pubkey();
    let seeds = [PLAYER_SEED, signer_pubkey.as_ref()];
    let (player, bump) = Pubkey::find_program_address(&seeds, &PROGRAM_ID);
    let ix = instructions::initialize_player(&player, &signer_pubkey, bump);
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
