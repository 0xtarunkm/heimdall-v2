// Copyright 2024 Heimdall Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use agave_geyser_plugin_interface::geyser_plugin_interface::SlotStatus as PluginSlotStatus;

// Include the generated protobuf code
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

// Helper functions for working with events
impl UpdateAccountEvent {
    /// Create a new account update event
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
    /// Create a new slot status event
    pub fn new(slot: u64, parent: u64, status: SlotStatus) -> Self {
        Self {
            slot,
            parent,
            status: status.into(),
        }
    }
}

impl TransactionEvent {
    /// Create a new transaction event
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