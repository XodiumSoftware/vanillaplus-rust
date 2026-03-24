use crate::modules::module::Module;
use pumpkin_plugin_api::Server;
use serde::{Deserialize, Serialize};

/// Handles MOTD (Message of the Day) mechanics.
#[derive(Default)]
pub struct Motd {
    /// Configuration for this module.
    config: Config,
}

impl Module for Motd {
    fn enabled(&self) -> bool {
        self.config.enabled
    }
}

impl Motd {
    pub fn motd(&self, _server: &mut Server) {
        if !self.enabled() || self.config.motd.is_empty() {
            return;
        }
        todo!("api is not ready yet")
        //server.set_motd(self.config.motd.join("\n"));
    }
}

/// Configuration for the MOTD mechanics module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// Whether this module is active.
    pub enabled: bool,
    /// Lines of the MOTD displayed to players on the server list.
    pub motd: Vec<String>,
}
