include!(concat!(
    env!("OUT_DIR"),
    "/heimdall.solana.geyser_plugin_kafka.types.rs"
));

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct AccountRow {
    pub slot: u64,
    pub pubkey: String,
    pub lamports: u64,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
    pub data_len: u64,
    pub write_version: u64,
    pub txn_signature: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SlotRow {
    pub slot: u64,
    pub parent: u64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TransactionRow {
    pub signature: String,
    pub slot: u64,
    pub index: u64,
    pub is_vote: bool,
    pub is_successful: bool,
    pub fee: u64,
    pub compute_units_consumed: Option<u64>,
    pub num_instructions: u32,
    pub num_accounts: u32,
    pub created_at: DateTime<Utc>,
}

impl From<UpdateAccountEvent> for AccountRow {
    fn from(event: UpdateAccountEvent) -> Self {
        Self {
            slot: event.slot,
            pubkey: bs58::encode(&event.pubkey).into_string(),
            lamports: event.lamports,
            owner: bs58::encode(&event.owner).into_string(),
            executable: event.executable,
            rent_epoch: event.rent_epoch,
            data_len: event.data.len() as u64,
            write_version: event.write_version,
            txn_signature: event
                .txn_signature
                .map(|sig| bs58::encode(&sig).into_string()),
            created_at: Utc::now(),
        }
    }
}

impl From<SlotStatusEvent> for SlotRow {
    fn from(event: SlotStatusEvent) -> Self {
        let status_str = match SlotStatus::from_i32(event.status) {
            Some(SlotStatus::Processed) => "Processed".to_string(),
            Some(SlotStatus::Rooted) => "Rooted".to_string(),
            Some(SlotStatus::Confirmed) => "Confirmed".to_string(),
            Some(SlotStatus::FirstShredReceived) => "FirstShredReceived".to_string(),
            Some(SlotStatus::Completed) => "Completed".to_string(),
            Some(SlotStatus::CreatedBank) => "CreatedBank".to_string(),
            Some(SlotStatus::Dead) => "Dead".to_string(),
            None => "Unknown".to_string(),
        };

        Self {
            slot: event.slot,
            parent: event.parent,
            status: status_str,
            created_at: Utc::now(),
        }
    }
}

impl From<TransactionEvent> for TransactionRow {
    fn from(event: TransactionEvent) -> Self {
        let (is_successful, fee, num_instructions, num_accounts) =
            if let Some(meta) = &event.transaction_status_meta {
                (
                    !meta.is_status_err,
                    meta.fee,
                    meta.inner_instructions.len() as u32,
                    0u32,
                )
            } else {
                (true, 0, 0, 0)
            };

        Self {
            signature: bs58::encode(&event.signature).into_string(),
            slot: event.slot,
            index: event.index,
            is_vote: event.is_vote,
            is_successful,
            fee,
            compute_units_consumed: None,
            num_instructions,
            num_accounts,
            created_at: Utc::now(),
        }
    }
}
