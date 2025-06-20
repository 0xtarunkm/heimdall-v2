use crate::{
    Database,
    event::{AccountRow, MessageWrapper, SlotRow, TransactionRow},
};
use log::{warn};
use prost::Message;

pub struct Processor {
    database: Database,
    account_batch: Vec<AccountRow>,
    slot_batch: Vec<SlotRow>,
    transaction_batch: Vec<TransactionRow>,
    batch_size: usize,
}

impl Processor {
    pub fn new(database: Database, batch_size: usize) -> Self {
        Self {
            database,
            account_batch: Vec::with_capacity(batch_size),
            slot_batch: Vec::with_capacity(batch_size),
            transaction_batch: Vec::with_capacity(batch_size),
            batch_size,
        }
    }

    pub async fn process_message(
        &mut self,
        topic: &str,
        payload: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(wrapper) = MessageWrapper::decode(payload) {
            if let Some(event_message) = wrapper.event_message {
                match event_message {
                    crate::event::message_wrapper::EventMessage::Account(account_event) => {
                        self.account_batch.push(AccountRow::from(account_event));
                    }
                    crate::event::message_wrapper::EventMessage::Slot(slot_event) => {
                        self.slot_batch.push(SlotRow::from(slot_event));
                    }
                    crate::event::message_wrapper::EventMessage::Transaction(tx_event) => {
                        self.transaction_batch.push(TransactionRow::from(tx_event));
                    }
                }
            }
        } else {
            match topic {
                t if t.contains("account") => {
                    if let Ok(account_event) = crate::event::UpdateAccountEvent::decode(payload) {
                        self.account_batch.push(AccountRow::from(account_event));
                    } else {
                        warn!("Failed to decode account message");
                    }
                }
                t if t.contains("slot") => {
                    if let Ok(slot_event) = crate::event::SlotStatusEvent::decode(payload) {
                        self.slot_batch.push(SlotRow::from(slot_event));
                    } else {
                        warn!("Failed to decode slot message");
                    }
                }
                t if t.contains("transaction") => {
                    if let Ok(tx_event) = crate::event::TransactionEvent::decode(payload) {
                        self.transaction_batch.push(TransactionRow::from(tx_event));
                    } else {
                        warn!("Failed to decode transaction message");
                    }
                }
                _ => {
                    warn!("Unknown topic: {}", topic);
                }
            }
        }

        self.flush_if_needed().await?;
        Ok(())
    }

    pub async fn flush_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.account_batch.is_empty() {
            self.database.insert_accounts(&self.account_batch).await?;
            self.account_batch.clear();
        }

        if !self.slot_batch.is_empty() {
            self.database.insert_slots(&self.slot_batch).await?;
            self.slot_batch.clear();
        }

        if !self.transaction_batch.is_empty() {
            self.database
                .insert_transactions(&self.transaction_batch)
                .await?;
            self.transaction_batch.clear();
        }

        Ok(())
    }

    async fn flush_if_needed(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.account_batch.len() >= self.batch_size {
            self.database.insert_accounts(&self.account_batch).await?;
            self.account_batch.clear();
        }

        if self.slot_batch.len() >= self.batch_size {
            self.database.insert_slots(&self.slot_batch).await?;
            self.slot_batch.clear();
        }

        if self.transaction_batch.len() >= self.batch_size {
            self.database
                .insert_transactions(&self.transaction_batch)
                .await?;
            self.transaction_batch.clear();
        }

        Ok(())
    }
}
