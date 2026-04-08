use crate::locator::Config as LocatorConfig;
use crate::motd::Config as MotdConfig;
use crate::player::Config as PlayerConfig;
use crate::tablist::Config as TablistConfig;
use figment::providers::{Format, Json, Serialized};
use figment::{Error, Figment};
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::PathBuf;
use tracing::{error, info};

thread_local! {
    static CONFIG_MANAGER: RefCell<Option<ConfigManager>> = const { RefCell::new(None) };
}

/// Manages plugin configuration, loading from and saving to disk.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigManager {
    /// Configuration for the player mechanics module.
    pub player_module: PlayerConfig,
    /// Configuration for the tablist module.
    pub tablist_module: TablistConfig,
    /// Configuration for the MOTD module.
    pub motd_module: MotdConfig,
    /// Configuration for the locator module.
    pub locator_module: LocatorConfig,
}

impl ConfigManager {
    /// Returns the global config manager instance.
    pub fn get() -> Option<Self> {
        CONFIG_MANAGER.with(|cm| cm.borrow().clone())
    }

    /// Creates a new [`ConfigManager`], loading from disk if it exists,
    /// otherwise writing defaults first. Merges file config with defaults
    /// to ensure any missing fields are populated.
    pub fn new(context: &Context) -> Self {
        let path = Self::path(context);

        let config = Self::load(&path).unwrap_or_else(|err| {
            error!("Failed to load config: {:?}. Using defaults.", err);
            Self::default()
        });

        if let Err(err) = Self::write(&config, &path) {
            error!("Failed to write config: {}", err);
        }

        CONFIG_MANAGER.set(Some(config.clone()));
        config
    }

    /// Loads configuration from disk, merging with defaults.
    /// Missing fields in the file will use default values.
    fn load(path: &PathBuf) -> Result<Self, Box<Error>> {
        if !path.exists() {
            info!("Config file not found at {:?}, using defaults", path);
            return Ok(Self::default());
        }

        Figment::new()
            .merge(Serialized::defaults(Self::default()))
            .merge(Json::file(path))
            .extract()
            .map_err(Box::new)
    }

    /// Serializes and writes the configuration to disk, creating the directory if needed.
    fn write(config: &Self, path: &PathBuf) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(config)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Returns the path to the configuration file within the plugin's data folder.
    fn path(context: &Context) -> PathBuf {
        PathBuf::from(context.get_data_folder().trim_start_matches("./")).join("config.json")
    }
}
