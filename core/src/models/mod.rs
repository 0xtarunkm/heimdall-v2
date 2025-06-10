use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccountInfo {
    pub pubkey: String,
    pub lamports: u64,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
    pub slot: u64,
}

#[derive(Debug, Serialize)]
pub struct TransactionInfo {
    pub signature: String,
    pub is_vote: bool,
    pub slot: u64,
    pub fee: u64,
    pub status: String,
    pub log_messages: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct BlockInfo {
    pub slot: u64,
    pub blockhash: String,
    pub parent_slot: u64,
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
}