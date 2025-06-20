use crate::{Config, Database, Processor};
use log::{error, info, warn};
use rdkafka::{
    config::ClientConfig,
    consumer::{Consumer as KafkaConsumer, StreamConsumer},
    message::Message,
};
use std::time::Duration;
use tokio::time::interval;

pub struct Consumer {
    kafka_consumer: StreamConsumer,
    processor: Processor,
    config: Config,
}

impl Consumer {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let database = Database::new(&config.clickhouse).await?;
        let processor = Processor::new(database, config.batch_size);

        let mut kafka_config = ClientConfig::new();
        for (key, value) in &config.kafka {
            kafka_config.set(key, value);
        }

        let kafka_consumer: StreamConsumer = kafka_config.create()?;

        let topics = vec![
            config.topics.accounts.as_str(),
            config.topics.slots.as_str(),
            config.topics.transactions.as_str(),
        ];
        kafka_consumer.subscribe(&topics)?;

        info!("Subscribed to topics: {:?}", topics);

        Ok(Self {
            kafka_consumer,
            processor,
            config,
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut flush_interval = interval(Duration::from_millis(self.config.flush_interval_ms));

        info!("Starting consumer loop...");

        loop {
            tokio::select! {
                message_result = self.kafka_consumer.recv() => {
                    match message_result {
                        Ok(message) => {
                            if let Some(payload) = message.payload() {
                                let topic = message.topic();
                                if let Err(e) = self.processor.process_message(topic, payload).await {
                                    error!("Failed to process message from topic {}: {:?}", topic, e);
                                }
                            }

                            if let Err(e) = self.kafka_consumer.commit_message(&message, rdkafka::consumer::CommitMode::Async) {
                                warn!("Failed to commit message: {:?}", e);
                            }
                        }
                        Err(e) => {
                            error!("Kafka consumer error: {:?}", e);
                            tokio::time::sleep(Duration::from_millis(1000)).await;
                        }
                    }
                }

                _ = flush_interval.tick() => {
                    if let Err(e) = self.processor.flush_all().await {
                        error!("Failed to flush batches: {:?}", e);
                    }
                }

                _ = tokio::signal::ctrl_c() => {
                    info!("Received shutdown signal");
                    break;
                }
            }
        }

        if let Err(e) = self.processor.flush_all().await {
            error!("Failed to flush final batches: {:?}", e);
        }

        info!("Consumer shutdown complete");
        Ok(())
    }
}
