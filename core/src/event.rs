use agave_geyser_plugin_interface::geyser_plugin_interface::SlotStatus as PluginSlotStatus;

include!(concat!(
    env!("OUT_DIR"),
    "/heimdall.solana.geyser_plugin_kafka.types.rs"
));

impl From<PluginSlotStatus> for SlotStatus {
    fn from(other: PluginSlotStatus) -> Self {
        match other {
            PluginSlotStatus::Processed => SlotStatus::Processed,
            PluginSlotStatus::Rooted => SlotStatus::Rooted,
            PluginSlotStatus::Confirmed => SlotStatus::Confirmed,
            PluginSlotStatus::FirstShredReceived => SlotStatus::FirstShredReceived,
            PluginSlotStatus::Completed => SlotStatus::Completed,
            PluginSlotStatus::CreatedBank => SlotStatus::CreatedBank,
            PluginSlotStatus::Dead(_) => SlotStatus::Dead,
        }
    }
}

impl UpdateAccountEvent {
    pub fn new(
        slot: u64,
        pubkey: Vec<u8>,
        lamports: u64,
        owner: Vec<u8>,
        executable: bool,
        rent_epoch: u64,
        data: Vec<u8>,
        write_version: u64,
        txn_signature: Option<Vec<u8>>,
    ) -> Self {
        Self {
            slot,
            pubkey,
            lamports,
            owner,
            executable,
            rent_epoch,
            data,
            write_version,
            txn_signature,
        }
    }
}

impl SlotStatusEvent {
    pub fn new(slot: u64, parent: u64, status: SlotStatus) -> Self {
        Self {
            slot,
            parent,
            status: status.into(),
        }
    }
}

impl TransactionEvent {
    pub fn new(
        signature: Vec<u8>,
        is_vote: bool,
        transaction: Option<SanitizedTransaction>,
        transaction_status_meta: Option<TransactionStatusMeta>,
        slot: u64,
        index: u64,
    ) -> Self {
        Self {
            signature,
            is_vote,
            transaction,
            transaction_status_meta,
            slot,
            index,
        }
    }
}
