use std::{fs::{File, OpenOptions}, path::Path, sync::Mutex, io::Write};

use bs58::encode;
use log::info;
use serde::Serialize;
use solana_geyser_plugin_interface::geyser_plugin_interface::{GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, ReplicaTransactionInfoVersions};

use crate::models::{AccountInfo, TransactionInfo};

#[derive(Debug)]
pub struct Heimdall {
    log_file: Mutex<Option<File>>
}

impl Default for Heimdall {
    fn default() -> Self {
        Self {
            log_file: Mutex::new(None)
        }
    }
}

impl GeyserPlugin for Heimdall {
    fn name(&self) -> &'static str {
        "heimdall-plugin"
    }

    fn on_load(&mut self, config_file: &str, _is_reload: bool) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        info!("loading heimdall from config file: {}", config_file);

        let config_str = std::fs::read_to_string(config_file)
            .map_err(|e| GeyserPluginError::ConfigFileReadError { msg: e.to_string() })?;

        let config: serde_json::Value = serde_json::from_str(&config_str)
            .map_err(|e| GeyserPluginError::ConfigFileReadError { msg: e.to_string() })?;

        let log_path = config["log_path"].as_str().ok_or_else(|| {
            GeyserPluginError::ConfigFileReadError { msg: "log_path not found in config".to_string() }
        })?;

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Path::new(log_path))
            .map_err(|e| GeyserPluginError::ConfigFileReadError { msg: format!("Failed to open log file: {}", e), })?;

        *self.log_file.lock().unwrap() = Some(file);
        info!("Plugin loaded successfully. Logging to {}", log_path);
        Ok(())
    }
    
    fn on_unload(&mut self) {
        info!("Unloading heimdall");
    }

    fn update_account(
            &self,
            account: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaAccountInfoVersions,
            slot: u64,
            _is_startup: bool,
        ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        let account_info = match account {
            ReplicaAccountInfoVersions::V0_0_3(account) => account,
            _ => return Ok(())
        };

        let account = AccountInfo {
            pubkey: encode(account_info.pubkey).into_string(),
            lamports: account_info.lamports,
            owner: encode(account_info.owner).into_string(),
            executable: account_info.executable,
            rent_epoch: account_info.rent_epoch,
            slot
        };

        self.write_log_entry("account_update", account)
    }

    fn notify_transaction(
            &self,
            transaction: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoVersions,
            slot: u64,
        ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        let tx_info = match transaction {
            ReplicaTransactionInfoVersions::V0_0_2(tx) => tx,
            _ => return Ok(()),
        };

        let transaction = TransactionInfo {
            signature: tx_info.signature.to_string(),
            is_vote: tx_info.is_vote,
            slot,
            fee: tx_info.transaction_status_meta.fee,
            status: format!("{:?}", tx_info.transaction_status_meta.status),
            log_messages: tx_info.transaction_status_meta.log_messages.clone(),
        };

        self.write_log_entry("transaction_notification", transaction)
    }
}

impl Heimdall {
    fn write_log_entry<T: Serialize>(
        &self,
        entry_type: &str,
        data: T
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        let mut log_file_guard = self.log_file.lock().unwrap();
        
        if let Some(ref mut file) = *log_file_guard {
            let json_data = serde_json::to_string(&data)
                .map_err(|e| GeyserPluginError::SlotStatusUpdateError { msg: e.to_string() })?;

            writeln!(file, r#"{{"type": "{}", "data": {}}}"#, entry_type, json_data)
                .map_err(|e| GeyserPluginError::SlotStatusUpdateError {
                    msg: e.to_string(),
                })?;
                
            file.flush()
                .map_err(|e| GeyserPluginError::SlotStatusUpdateError {
                    msg: e.to_string(),
                })?;
        }
            
        Ok(())
    }
}