use agave_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;

mod config;
mod event;
mod filter;
mod plugin;
mod publisher;

pub use {
    config::{Config, ConfigFilter, Producer},
    event::*,
    filter::Filter,
    plugin::HeimdallPlugin,
    publisher::Publisher,
};

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns a pointer to the Heimdall Plugin box implementing trait GeyserPlugin.
///
/// The Solana validator and this plugin must be compiled with the same Rust compiler version and Solana core version.
/// Loading this plugin with mismatching versions is undefined behavior and will likely cause memory corruption.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = HeimdallPlugin::new();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}