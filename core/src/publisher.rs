use {
    crate::{
        message_wrapper::EventMessage::{self, Account, Slot, Transaction},
        Config, MessageWrapper, SlotStatusEvent, TransactionEvent, UpdateAccountEvent,
    },
    log::{debug, error, warn},
    prost::Message,
    rdkafka::{
        error::KafkaError,
        producer::{BaseRecord, DefaultProducerContext, Producer, ThreadedProducer},
    },
    std::time::Duration,
};

pub struct Publisher {
    producer: ThreadedProducer<DefaultProducerContext>,
    shutdown_timeout: Duration,
}

impl Publisher {
    pub fn new(producer: ThreadedProducer<DefaultProducerContext>, config: &Config) -> Self {
        Self {
            producer,
            shutdown_timeout: Duration::from_millis(config.shutdown_timeout_ms),
        }
    }

    pub fn update_account(
        &self,
        ev: UpdateAccountEvent,
        wrap_messages: bool,
        topic: &str,
    ) -> Result<(), KafkaError> {
        let temp_key;
        let (key, buf) = if wrap_messages {
            temp_key = self.copy_and_prepend(ev.pubkey.as_slice(), 65u8);
            (&temp_key, Self::encode_with_wrapper(Account(*Box::new(ev))))
        } else {
            (&ev.pubkey, ev.encode_to_vec())
        };

        let record = BaseRecord::<Vec<u8>, _>::to(topic).key(key).payload(&buf);
        
        match self.producer.send(record) {
            Ok(_) => {
                debug!("Successfully sent account update to topic: {}", topic);
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send account update to topic {}: {:?}", topic, e);
                Err(e)
            }
        }
    }

    pub fn update_slot_status(
        &self,
        ev: SlotStatusEvent,
        wrap_messages: bool,
        topic: &str,
    ) -> Result<(), KafkaError> {
        let temp_key;
        let (key, buf) = if wrap_messages {
            temp_key = self.copy_and_prepend(&ev.slot.to_le_bytes(), 83u8);
            (&temp_key, Self::encode_with_wrapper(Slot(*Box::new(ev))))
        } else {
            temp_key = ev.slot.to_le_bytes().to_vec();
            (&temp_key, ev.encode_to_vec())
        };

        let record = BaseRecord::<Vec<u8>, _>::to(topic).key(key).payload(&buf);
        
        match self.producer.send(record) {
            Ok(_) => {
                debug!("Successfully sent slot status update to topic: {}", topic);
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send slot status update to topic {}: {:?}", topic, e);
                Err(e)
            }
        }
    }

    pub fn update_transaction(
        &self,
        ev: TransactionEvent,
        wrap_messages: bool,
        topic: &str,
    ) -> Result<(), KafkaError> {
        let temp_key;
        let (key, buf) = if wrap_messages {
            temp_key = self.copy_and_prepend(ev.signature.as_slice(), 84u8);
            (
                &temp_key,
                Self::encode_with_wrapper(Transaction(*Box::new(ev))),
            )
        } else {
            (&ev.signature, ev.encode_to_vec())
        };

        let record = BaseRecord::<Vec<u8>, _>::to(topic).key(key).payload(&buf);
        
        match self.producer.send(record) {
            Ok(_) => {
                debug!("Successfully sent transaction update to topic: {}", topic);
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send transaction update to topic {}: {:?}", topic, e);
                Err(e)
            }
        }
    }

    fn encode_with_wrapper(message: EventMessage) -> Vec<u8> {
        MessageWrapper {
            event_message: Some(message),
        }
        .encode_to_vec()
    }

    fn copy_and_prepend(&self, data: &[u8], prefix: u8) -> Vec<u8> {
        let mut temp_key = Vec::with_capacity(data.len() + 1);
        temp_key.push(prefix);
        temp_key.extend_from_slice(data);
        temp_key
    }

    pub fn in_flight_count(&self) -> i32 {
        self.producer.in_flight_count()
    }

    pub fn flush(&self, timeout: Duration) -> Result<(), KafkaError> {
        self.producer.flush(timeout)
    }
}

impl Drop for Publisher {
    fn drop(&mut self) {
        debug!("Shutting down Heimdall publisher");
        match self.producer.flush(self.shutdown_timeout) {
            Ok(()) => debug!("Publisher shutdown completed successfully"),
            Err(e) => {
                warn!(
                    "Publisher shutdown completed with errors after {:?}: {:?}",
                    self.shutdown_timeout, e
                );
            }
        }
    }
}