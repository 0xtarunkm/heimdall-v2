use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;

use crate::heimdall::Heimdall;

pub mod heimdall;
pub mod models;

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the Heimdall plugin pointer as trait GeyserPlugin.
/// The caller is responsible for the lifetime of the returned pointer.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = Heimdall::default();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}