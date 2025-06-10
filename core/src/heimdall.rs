use std::{fs::File, sync::Mutex};

use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;

#[derive(Debug)]
struct Heimdall {
    log_file: Mutex<File>
}

impl GeyserPlugin for Heimdall {
    fn name(&self) -> &'static str {
        "heimdall-plugin"
    }
    
}