use crate::{LocatorConfig, MotdConfig, PlayerConfig, TablistConfig};
use figment::Figment;
use figment::providers::{Format, Json, Serialized};
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

thread_local! {
    static CONFIG: RefCell<Option<ConfigManager>> = const { RefCell::new(None) };
}

/// Manages plugin configuration, loading from and saving to disk.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigManager {
    pub player_module: PlayerConfig,
    pub tablist_module: TablistConfig,
    pub motd_module: MotdConfig,
    pub locator_module: LocatorConfig,
}

impl ConfigManager {
    /// Returns the global config manager instance.
    pub fn get() -> Option<Self> {
        CONFIG.with(|c| c.borrow().clone())
    }

    /// Creates a new [`ConfigManager`], loading from disk if it exists,
    /// merging with defaults for any missing fields.
    pub fn new(context: &Context) -> Self {
        let path =
            PathBuf::from(context.get_data_folder().trim_start_matches("./")).join("config.json");
        let config = if path.exists() {
            Figment::new()
                .merge(Serialized::defaults(Self::default()))
                .merge(Json::file(&path))
                .extract()
                .inspect_err(|e| error!("Failed to load config: {:?}. Using defaults.", e))
                .unwrap_or_default()
        } else {
            info!("Config file not found, using defaults");
            Self::default()
        };

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }

        fs::write(
            &path,
            serde_json::to_string_pretty(&config).unwrap_or_default(),
        )
        .inspect_err(|e| error!("Failed to write config: {}", e))
        .ok();

        CONFIG.set(Some(config.clone()));
        config
    }
}
