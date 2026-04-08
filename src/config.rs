use figment::Figment;
use figment::providers::{Format, Json};
use pumpkin_plugin_api::Context;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::error;

thread_local! {
    static CONFIG: RefCell<Option<ConfigManager>> = const { RefCell::new(None) };
}

/// Manages plugin configuration using a registry pattern.
/// Modules register their configs by name, and ConfigManager handles
/// loading from disk with merge semantics for missing fields.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigManager {
    #[serde(flatten)]
    configs: HashMap<String, Value>,
}

impl ConfigManager {
    /// Creates an empty ConfigManager ready for registration.
    pub fn empty() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Returns the global config manager instance.
    pub fn get() -> Option<Self> {
        CONFIG.with(|c| c.borrow().clone())
    }

    /// Gets a config by name, returning defaults if not found or parse fails.
    pub fn get_config<T: DeserializeOwned + Default>(&self, name: &str) -> T {
        self.configs
            .get(name)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default()
    }

    /// Registers a config with default values for a module.
    pub fn register<T: Serialize + Default>(&mut self, name: &str) {
        let config = T::default();
        self.configs
            .insert(name.to_string(), serde_json::to_value(config).unwrap());
    }

    /// Loads config from disk, merging with registered defaults.
    /// Call this after all modules have registered their configs.
    pub fn finalize(&mut self, context: &Context) {
        let path =
            PathBuf::from(context.get_data_folder().trim_start_matches("./")).join("config.json");

        if path.exists() {
            let file_config: HashMap<String, Value> = Figment::new()
                .merge(Json::file(&path))
                .extract()
                .inspect_err(|e| error!("Failed to load config file: {:?}", e))
                .unwrap_or_default();

            for (key, value) in file_config {
                if self.configs.contains_key(&key) {
                    if let Some(existing) = self.configs.get(&key) {
                        let merged = merge_json(existing, &value);
                        self.configs.insert(key, merged);
                    }
                } else {
                    self.configs.insert(key, value);
                }
            }
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }

        fs::write(
            &path,
            serde_json::to_string_pretty(self).unwrap_or_default(),
        )
        .inspect_err(|e| error!("Failed to write config: {}", e))
        .ok();

        CONFIG.set(Some(self.clone()));
    }
}

/// Merge two JSON values, preferring values from `b` when both exist.
/// For objects, recursively merges fields.
fn merge_json(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Object(a_map), Value::Object(b_map)) => {
            let mut result = a_map.clone();
            for (key, b_val) in b_map {
                let a_val = result.get(key);
                let merged = match a_val {
                    Some(a_val) => merge_json(a_val, b_val),
                    None => b_val.clone(),
                };
                result.insert(key.clone(), merged);
            }
            Value::Object(result)
        }
        _ => b.clone(),
    }
}
