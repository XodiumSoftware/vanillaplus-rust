use pumpkin::server::Server;
use serde::{Deserialize, Serialize};

/// Represents a module handling motd mechanics within the system.
pub struct MotdModule {
    pub config: MotdModuleConfig,
}

impl MotdModule {
    fn motd(&self, server: &mut Server) {
        server.basic_config.motd = self.config.motd.join("\n")
    }
}

/// Represents the config of the module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotdModuleConfig {
    pub motd: Vec<String>,
}

impl Default for MotdModuleConfig {
    fn default() -> Self {
        Self {
            motd: vec![
                "<gradient:#CB2D3E:#EF473A><b>Ultimate Private SMP</b></gradient>".to_string(),
                "<gradient:#FFE259:#FFA751><b>➤ WELCOME BACK LADS!</b></gradient>".to_string(),
            ],
        }
    }
}
