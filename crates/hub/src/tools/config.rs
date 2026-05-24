//! Key-value configuration store for the hub.
//!
//! [`Config`] stores `(String, String)` pairs and provides
//! typed accessors (`f64`, `usize`, `bool`) as well as
//! loading from environment variables.

use std::collections::HashMap;

/// Key-value configuration container.
#[derive(Debug, Clone)]
pub struct Config {
    values: HashMap<String, String>,
}

impl Config {
    /// Creates an empty configuration.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// Sets the value associated with `key`.
    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }

    /// Returns the value associated with `key`, or `None`.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    /// Returns the value for `key`, or `default` if absent.
    pub fn get_or(&self, key: &str, default: &str) -> String {
        self.values
            .get(key)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    /// Parses the value as `f64`, or `None` if absent or invalid.
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.values.get(key).and_then(|v| v.parse().ok())
    }

    /// Parses the value as `usize`, or `None` if absent or invalid.
    pub fn get_usize(&self, key: &str) -> Option<usize> {
        self.values.get(key).and_then(|v| v.parse().ok())
    }

    /// Parses the value as `bool`, or `None` if absent or invalid.
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.values.get(key).and_then(|v| v.parse().ok())
    }

    /// Iterates over all keys in the configuration.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.values.keys().map(|s| s.as_str())
    }

    /// Number of entries in the configuration.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` if the configuration is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Merges entries from `other` into this configuration.
    pub fn merge(&mut self, other: &Config) {
        for (k, v) in &other.values {
            self.values.insert(k.clone(), v.clone());
        }
    }

    /// Builds a configuration from environment variables starting with `prefix`.
    pub fn from_env_prefix(prefix: &str) -> Self {
        let mut cfg = Self::new();
        for (key, value) in std::env::vars() {
            if let Some(stripped) = key.strip_prefix(prefix) {
                cfg.set(&stripped.to_lowercase(), &value);
            }
        }
        cfg
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
