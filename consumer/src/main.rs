use consumer::{Config, Consumer};
use log::{error, info};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "config/consumer.json".to_string());

    info!("Starting Heimdall Consumer with config: {}", config_path);

    let config = Config::read_from(&config_path)?;
    let mut consumer = Consumer::new(config).await?;

    if let Err(e) = consumer.run().await {
        error!("Consumer error: {:?}", e);
        return Err(e);
    }

    Ok(())
}
