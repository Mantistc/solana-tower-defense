use std::sync::Arc;

use solana_client::{client_error::ClientError, rpc_client::RpcClient};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};

use super::*;

pub fn build_and_send_tx(
    signer: Arc<Keypair>,
    client: Arc<RpcClient>,
    instructions: &[Instruction],
) -> Result<TaskResult, ClientError> {
    let blockhash = client
        .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())?
        .0;

    let compiled_message = Message::try_compile(&signer.pubkey(), instructions, &[], blockhash)
        .expect("compile msg failed");

    let versioned_msg = VersionedMessage::V0(compiled_message);
    let versioned_tx = VersionedTransaction::try_new(versioned_msg, &[signer])?;

    let signature = client.send_transaction_with_config(&versioned_tx, SEND_CFG)?;

    Ok(TaskResult::Signature(signature))
}
