use crate::locator::Config as LocatorConfig;
use crate::motd::Config as MotdConfig;
use crate::player::Config as PlayerConfig;
use crate::tablist::Config as TablistConfig;
use pumpkin_plugin_api::Context;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::PathBuf;
use tracing::error;

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
}

impl ConfigManager {
    /// Creates a new [`ConfigManager`], loading from disk if it exists,
    /// otherwise writing defaults first. Merges loaded config with defaults
    /// to ensure any missing fields are populated.
    pub fn new(context: &Context) -> Self {
        let mut manager = Self::default();
        manager.read_and_update(context);
        manager.write(context);
        CONFIG_MANAGER.set(Some(manager.clone()));
        manager
    }

    /// Reads configuration from disk and merges it with defaults.
    /// Any missing fields in the file will use default values.
    fn read_and_update(&mut self, context: &Context) {
        let path = Self::path(context);

        match fs::read_to_string(&path) {
            Ok(content) => {
                // Try to parse as generic JSON Value first
                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(file_value) => {
                        // Convert default config to JSON Value
                        match serde_json::to_value(&self) {
                            Ok(default_value) => {
                                // Merge file values into defaults (file takes precedence)
                                if let Some(merged) = Self::merge_values(default_value, file_value)
                                {
                                    // Convert back to ConfigManager
                                    match serde_json::from_value::<ConfigManager>(merged) {
                                        Ok(merged_config) => {
                                            *self = merged_config;
                                        }
                                        Err(err) => {
                                            error!("Failed to parse merged config: {}", err);
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                error!("Failed to serialize default config: {}", err);
                            }
                        }
                    }
                    Err(err) => {
                        error!("Failed to parse config file: {}", err);
                    }
                }
            }
            Err(err) => {
                // File doesn't exist yet, use defaults
                tracing::info!("Config file not found, using defaults: {}", err);
            }
        }
    }

    /// Recursively merges two JSON values, with `b` taking precedence over `a`.
    /// Returns `None` if values are incompatible types.
    fn merge_values(a: serde_json::Value, b: serde_json::Value) -> Option<serde_json::Value> {
        match (a, b) {
            // Both are objects: merge their fields
            (serde_json::Value::Object(mut a_map), serde_json::Value::Object(b_map)) => {
                for (key, b_val) in b_map {
                    match a_map.remove(&key) {
                        Some(a_val) => {
                            // Key exists in both: recursively merge
                            if let Some(merged) = Self::merge_values(a_val, b_val.clone()) {
                                a_map.insert(key, merged);
                            } else {
                                // Incompatible types: use b's value
                                a_map.insert(key, b_val);
                            }
                        }
                        None => {
                            // Key only in b: insert it
                            a_map.insert(key, b_val);
                        }
                    }
                }
                // a_map now contains merged result
                Some(serde_json::Value::Object(a_map))
            }
            // Both are arrays: use b's value (or could merge elements)
            (serde_json::Value::Array(_), serde_json::Value::Array(b_arr)) => {
                Some(serde_json::Value::Array(b_arr))
            }
            // b is not null: use b's value
            (_, b_val) if !b_val.is_null() => Some(b_val),
            // b is null: use a's value
            (a_val, _) => Some(a_val),
        }
    }

    /// Serializes and writes the configuration to disk, creating the directory if needed.
    fn write(&self, context: &Context) {
        let path = Self::path(context);

        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
        {
            Ok(mut file) => match serde_json::to_string_pretty(self) {
                Ok(json) => {
                    if let Err(err) = file.write_all(json.as_bytes()) {
                        error!("{}", err);
                    }
                }
                Err(err) => error!("{}", err),
            },
            Err(err) => error!("{}", err),
        }
    }

    /// Returns the path to the configuration file within the plugin's data folder.
    fn path(context: &Context) -> PathBuf {
        PathBuf::from(context.get_data_folder().trim_start_matches("./")).join("config.json")
    }
}
