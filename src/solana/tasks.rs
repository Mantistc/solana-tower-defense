use std::{collections::VecDeque, future::Future};

use bevy::{
    prelude::*,
    tasks::{block_on, poll_once, AsyncComputeTaskPool, Task},
};
use solana_client::client_error::ClientError;
use solana_sdk::signature::Signature;
use td_program_sdk::states::Player;

use super::{PlayerInfo, Wallet};

#[derive(Debug)]
pub enum TaskResult {
    Balance(u64),
    Signature(Signature),
    PlayerData(Player),
}

pub type ActionResult = Result<TaskResult, ClientError>;

#[derive(Resource, Debug)]
pub struct Tasks {
    pub status_delay: Timer,
    pub pending_tasks: VecDeque<Task<ActionResult>>,
}

impl Default for Tasks {
    fn default() -> Self {
        Self {
            status_delay: Timer::from_seconds(5.0, TimerMode::Repeating),
            pending_tasks: VecDeque::new(),
        }
    }
}

impl Tasks {
    pub fn add_task<F>(&mut self, future: F)
    where
        F: Future<Output = ActionResult> + Send + 'static,
    {
        let task = AsyncComputeTaskPool::get().spawn(future);
        self.pending_tasks.push_back(task);
    }
}

pub fn process_tx_tasks(
    mut tasks: ResMut<Tasks>,
    mut wallet: ResMut<Wallet>,
    mut player_data: ResMut<PlayerInfo>,
) {
    if let Some(mut task) = tasks.pending_tasks.pop_front() {
        if let Some(result) = block_on(poll_once(&mut task)) {
            match result {
                Ok(tx_result) => match tx_result {
                    TaskResult::Balance(balance) => {
                        wallet.balance = balance;
                        info!("wallet balance updated: {} SOL", balance);
                    }
                    TaskResult::Signature(sig) => {
                        info!("transaction sent, signature: {:?}", sig);
                    }
                    TaskResult::PlayerData(player) => {
                        player_data.data = player;
                        let last_time_played =
                            u64::from_le_bytes(player.last_played.try_into().unwrap_or_default());
                        info!(
                            "last time played: {:?}, wave_reached: {}",
                            last_time_played, player.wave_reached
                        );
                    }
                },
                Err(err) => {
                    error!("task failed: {:?}", err);
                }
            }
        } else {
            tasks.pending_tasks.push_front(task);
        }
    }
}
