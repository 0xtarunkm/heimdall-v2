use {
    agave_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPluginError, Result as PluginResult,
    },
    rdkafka::{
        ClientConfig,
        config::FromClientConfig,
        error::KafkaResult,
        producer::{DefaultProducerContext, ThreadedProducer},
    },
    serde::Deserialize,
    std::{collections::HashMap, fs::File, path::Path},
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[allow(dead_code)]
    libpath: String,

    pub kafka: HashMap<String, String>,

    #[serde(default)]
    pub shutdown_timeout_ms: u64,

    pub filters: Vec<ConfigFilter>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            libpath: "".to_owned(),
            kafka: HashMap::new(),
            shutdown_timeout_ms: 30_000,
            filters: vec![],
        }
    }
}

impl Config {
    pub fn read_from<P: AsRef<Path>>(config_path: P) -> PluginResult<Self> {
        let file = File::open(config_path)?;
        let mut this: Self = serde_json::from_reader(file)
            .map_err(|e| GeyserPluginError::ConfigFileReadError { msg: e.to_string() })?;
        this.fill_defaults();
        Ok(this)
    }

    pub fn producer(&self) -> KafkaResult<ThreadedProducer<DefaultProducerContext>> {
        let mut config = ClientConfig::new();
        for (k, v) in self.kafka.iter() {
            config.set(k, v);
        }

        ThreadedProducer::from_config(&config)
    }

    fn set_default(&mut self, k: &'static str, v: &'static str) {
        if !self.kafka.contains_key(k) {
            self.kafka.insert(k.to_owned(), v.to_owned());
        }
    }

    fn fill_defaults(&mut self) {
        self.set_default("request.required.acks", "1");
        self.set_default("message.timeout.ms", "30000");
        self.set_default("compression.type", "lz4");
        self.set_default("partitioner", "murmur2_random");
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct ConfigFilter {
    /// Kafka topic to send account updates to.
    pub update_account_topic: String,
    /// Kafka topic to send slot status updates to.
    pub slot_status_topic: String,
    /// Kafka topic to send transaction to.
    pub transaction_topic: String,
    /// List of programs to ignore.
    pub program_ignores: Vec<String>,
    /// List of accounts to ignore.
    pub account_ignores: Vec<String>,
    /// List of programs to include
    pub program_filters: Vec<String>,
    /// List of accounts to include
    pub account_filters: Vec<String>,
    /// Publish all accounts on startup.
    pub publish_all_accounts: bool,
    /// Publish vote transactions.
    pub include_vote_transactions: bool,
    /// Publish failed transactions.
    pub include_failed_transactions: bool,
    /// Wrap all event message in a single message type.
    pub wrap_messages: bool,
}

impl Default for ConfigFilter {
    fn default() -> Self {
        Self {
            update_account_topic: "".to_owned(),
            slot_status_topic: "".to_owned(),
            transaction_topic: "".to_owned(),
            program_ignores: Vec::new(),
            account_ignores: Vec::new(),
            program_filters: Vec::new(),
            account_filters: Vec::new(),
            publish_all_accounts: false,
            include_vote_transactions: true,
            include_failed_transactions: true,
            wrap_messages: false,
        }
    }
}

pub type Producer = ThreadedProducer<DefaultProducerContext>;
