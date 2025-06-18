use agave_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;

#[derive(Default, Debug)]
pub struct Heimdall {}

impl GeyserPlugin for Heimdall {
    fn name(&self) -> &'static str {
        "Heimdall"
    }
}

impl Heimdall {
    pub fn new() -> Self {
        Default::default()
    }
}