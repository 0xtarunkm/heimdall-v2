use serde::Serialize;

#[derive(Debug, Serialize)]
struct AccountInfo {
    pubkey: String,
    lamports: u64,
    owner: String,
    executable: bool,
    rent_epoch: u64,
    slot: u64
}

#[derive(Debug, Serialize)]
struct TransactionInfo {
    signature: String,
    is_vote: bool,
    slot: u64,
    fee: u64,
    status: String,
    log_messages: Option<Vec<String>>
}

#[derive(Debug, Serialize)]
struct BlockInfo {
    slot: u64,
    blockhash: String,
    parent_slot: u64,
    block_time: Option<i64>,
    block_height: Option<u64>
}