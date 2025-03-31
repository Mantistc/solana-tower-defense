use std::sync::Arc;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::VersionedTransaction,
};

use super::SEND_CFG;

pub fn build_and_send_tx(
    signer: Arc<Keypair>,
    client: Arc<RpcClient>,
    instructions: &[Instruction],
) -> Result<Signature, String> {
    let blockhash = client
        .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
        .map_err(|e| format!("failed to fetch latest blockhash: {:?}", e))?
        .0;

    let compiled_message = Message::try_compile(&signer.pubkey(), instructions, &[], blockhash)
        .map_err(|e| format!("failed to compile message: {:?}", e))?;

    let versioned_msg = VersionedMessage::V0(compiled_message);
    let versioned_tx = VersionedTransaction::try_new(versioned_msg, &[signer])
        .map_err(|e| format!("failed to create transaction: {:?}", e))?;

    let signature = client
        .send_transaction_with_config(&versioned_tx, SEND_CFG)
        .map_err(|e| format!("failed to send transaction: {:?}", e))?;

    Ok(signature)
}
