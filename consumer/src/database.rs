use crate::{
    config::ClickHouseConfig,
    event::{AccountRow, SlotRow, TransactionRow},
};
use clickhouse::Client;
use log::{error, info};

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn new(config: &ClickHouseConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let default_client = Client::default()
            .with_url(&config.url)
            .with_user(&config.username)
            .with_password(&config.password)
            .with_database("default");

        let create_db_sql = format!("CREATE DATABASE IF NOT EXISTS {}", config.database);
        match default_client.query(&create_db_sql).execute().await {
            Ok(_) => info!("Database '{}' ensured/created.", config.database),
            Err(e) => {
                error!(
                    "Error ensuring/creating database '{}': {:?}",
                    config.database, e
                );
                return Err(e.into());
            }
        }

        let client = Client::default()
            .with_url(&config.url)
            .with_user(&config.username)
            .with_password(&config.password)
            .with_database(&config.database);

        let db = Self { client };
        db.create_tables().await?;

        info!("Connected to ClickHouse database: {}", config.database);
        Ok(db)
    }

    async fn create_tables(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .query(
                r#"
CREATE TABLE IF NOT EXISTS accounts (
    slot UInt64,
    pubkey String,
    lamports UInt64,
    owner String,
    executable Bool,
    rent_epoch UInt64,
    data_len UInt64,
    write_version UInt64,
    txn_signature Nullable(String),
    created_at DateTime
) ENGINE = MergeTree()
ORDER BY (slot, pubkey)
PARTITION BY toYYYYMM(created_at)
        "#,
            )
            .execute()
            .await?;

        self.client
            .query(
                r#"
CREATE TABLE IF NOT EXISTS slots (
    slot UInt64,
    parent UInt64,
    status String,
    created_at DateTime
) ENGINE = MergeTree()
ORDER BY slot
PARTITION BY toYYYYMM(created_at)
        "#,
            )
            .execute()
            .await?;

        self.client
            .query(
                r#"
CREATE TABLE IF NOT EXISTS transactions (
    signature String,
    slot UInt64,
    `index` UInt64,
    is_vote Bool,
    is_successful Bool,
    fee UInt64,
    compute_units_consumed Nullable(UInt64),
    num_instructions UInt32,
    num_accounts UInt32,
    created_at DateTime
) ENGINE = MergeTree()
ORDER BY (slot, `index`)
PARTITION BY toYYYYMM(created_at)
        "#,
            )
            .execute()
            .await?;

        info!("Database tables created/verified");
        Ok(())
    }

    pub async fn insert_accounts(
        &self,
        accounts: &[AccountRow],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if accounts.is_empty() {
            return Ok(());
        }

        for account in accounts {
            let txn_signature = match &account.txn_signature {
                Some(sig) => sig.as_str(),
                None => "",
            };

            self.client
                .query("INSERT INTO accounts (slot, pubkey, lamports, owner, executable, rent_epoch, data_len, write_version, txn_signature, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(account.slot)
                .bind(&account.pubkey)
                .bind(account.lamports)
                .bind(&account.owner)
                .bind(account.executable)
                .bind(account.rent_epoch)
                .bind(account.data_len)
                .bind(account.write_version)
                .bind(if account.txn_signature.is_some() { txn_signature } else { "" })
                .bind(account.created_at.timestamp() as i32)
                .execute()
                .await?;
        }

        info!("Inserted {} accounts", accounts.len());
        Ok(())
    }

    pub async fn insert_slots(&self, slots: &[SlotRow]) -> Result<(), Box<dyn std::error::Error>> {
        if slots.is_empty() {
            return Ok(());
        }

        for slot in slots {
            self.client
                .query("INSERT INTO slots (slot, parent, status, created_at) VALUES (?, ?, ?, ?)")
                .bind(slot.slot)
                .bind(slot.parent)
                .bind(&slot.status)
                .bind(slot.created_at.timestamp() as i32)
                .execute()
                .await?;
        }

        info!("Inserted {} slots", slots.len());
        Ok(())
    }

    pub async fn insert_transactions(
        &self,
        transactions: &[TransactionRow],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if transactions.is_empty() {
            return Ok(());
        }

        for tx in transactions {
            self.client
                .query("INSERT INTO transactions (signature, slot, `index`, is_vote, is_successful, fee, compute_units_consumed, num_instructions, num_accounts, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(&tx.signature)
                .bind(tx.slot)
                .bind(tx.index)
                .bind(tx.is_vote)
                .bind(tx.is_successful)
                .bind(tx.fee)
                .bind(tx.compute_units_consumed.unwrap_or(0))
                .bind(tx.num_instructions)
                .bind(tx.num_accounts)
                .bind(tx.created_at.timestamp() as i32)
                .execute()
                .await?;
        }

        info!("Inserted {} transactions", transactions.len());
        Ok(())
    }
}
