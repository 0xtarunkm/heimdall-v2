use serde::Deserialize;
use std::{collections::HashMap, fs::File, path::Path};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub kafka: HashMap<String, String>,
    pub clickhouse: ClickHouseConfig,
    pub topics: TopicConfig,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    #[serde(default = "default_flush_interval")]
    pub flush_interval_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct ClickHouseConfig {
    pub url: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct TopicConfig {
    pub accounts: String,
    pub slots: String,
    pub transactions: String,
}

impl Config {
    pub fn read_from<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let config: Self = serde_json::from_reader(file)?;
        Ok(config)
    }
}

fn default_batch_size() -> usize {
    1000
}

fn default_flush_interval() -> u64 {
    5000
}
