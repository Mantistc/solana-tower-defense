use std::sync::Arc;

use bevy::prelude::*;
use solana_client::{
    client_error::{ClientError, ClientErrorKind},
    rpc_client::RpcClient,
    rpc_request::RpcRequest,
};
use solana_sdk::{
    pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction::transfer,
};
use td_program_sdk::{instructions, seeds::PLAYER_SEED, states::Player, PROGRAM_ID};

use crate::VARIABLES;

use super::*;

#[derive(Resource, Debug, Clone)]
pub struct PlayerInfo {
    pub data: Player,
    pub address: Pubkey,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        PlayerInfo {
            data: Player {
                wave_reached: 0,
                last_played: [0; 8],
                authority: [0; 32],
            },
            address: Pubkey::new_unique(),
        }
    }
}

impl PlayerInfo {
    pub fn set_address(&mut self, signer_pubkey: &Pubkey) -> (Pubkey, u8) {
        let seeds = [PLAYER_SEED, signer_pubkey.as_ref()];
        let (player, bump) = Pubkey::find_program_address(&seeds, &PROGRAM_ID);
        self.address = player;
        (player, bump)
    }
}

pub async fn get_unpacked_player_info(player: Pubkey, client: Arc<RpcClient>) -> ActionResult {
    let acc_data = client.get_account_data(&player)?;
    Player::unpack(acc_data.as_slice())
        .map(TaskResult::PlayerData)
        .map_err(|e| ClientError {
            request: Some(RpcRequest::GetAccountInfo),
            kind: ClientErrorKind::Custom(format!("ProgramError: {:?}", e)),
        })
}
pub async fn send_sol(signer: Arc<Keypair>, client: Arc<RpcClient>) -> ActionResult {
    let to_pubkey = Pubkey::from_str_const(&VARIABLES.payment_wallet);
    let lamports = 100_000_000;
    let ix = transfer(&signer.pubkey(), &to_pubkey, lamports);
    build_and_send_tx(signer, client, &[ix])
}

pub async fn initialize_player(
    signer: Arc<Keypair>,
    client: Arc<RpcClient>,
    player: Pubkey,
    bump: u8,
) -> ActionResult {
    let signer_pubkey = signer.pubkey();
    let ix = instructions::initialize_player(&player, &signer_pubkey, bump);
    build_and_send_tx(signer, client, &[ix])
}

pub async fn update_player_values(
    signer: Arc<Keypair>,
    client: Arc<RpcClient>,
    wave_count: u8,
    last_time_played: u64,
    player: Pubkey,
) -> ActionResult {
    let signer_pubkey = signer.pubkey();
    let ix = instructions::update_player_game_values(
        &player,
        &signer_pubkey,
        last_time_played,
        wave_count,
    );
    build_and_send_tx(signer, client, &[ix])
}

pub fn update_onchain_values(
    wallet: ResMut<Wallet>,
    mut tasks: ResMut<Tasks>,
    client: Res<SolClient>,
    time: Res<Time>,
    player_info: Res<PlayerInfo>,
) {
    tasks.status_delay.tick(time.delta());

    if tasks.status_delay.just_finished() {
        let pubkey = wallet.keypair.pubkey();
        let client_rpc = client.clone();
        tasks.add_task(async move {
            client_rpc
                .clone()
                .get_balance(&pubkey)
                .map(TaskResult::Balance)
        });
        tasks.add_task(get_unpacked_player_info(
            player_info.address,
            client.clone(),
        ));
    }
}
