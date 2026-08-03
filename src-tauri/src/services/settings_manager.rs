use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

use crate::utils::platform_detect;

/// Default settings for QuickMD
const DEFAULT_SETTINGS: &str = r#"{
    "theme": "system",
    "zoomLevel": 1.0,
    "fontSize": 16,
    "fontFamily": "system",
    "codeFontFamily": "'JetBrains Mono', 'Fira Code', 'Consolas', monospace",
    "lineSpacing": 1.6,
    "wordWrap": true,
    "autoReload": true,
    "showToolbar": true,
    "showStatusBar": true,
    "recentFilesMax": 15,
    "confirmBeforeClose": false,
    "defaultViewMode": "rendered",
    "startupBehavior": "welcome",
    "enableMermaid": true,
    "enableKatex": true,
    "enableAutostart": false,
    "enableTrayIcon": true,
    "enableAnimations": true,
    "hardwareAcceleration": true,
    "checkForUpdates": true,
    "telemetry": false,
    "isDefaultApp": false
}"#;

pub struct SettingsManager {
    settings: Value,
    config_path: PathBuf,
}

impl SettingsManager {
    pub fn new() -> Self {
        let config_path = Self::get_config_path();
        // Ensure config directory exists on initialization
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let settings = Self::load_or_default(&config_path);
        Self { settings, config_path }
    }

    fn get_config_path() -> PathBuf {
        let config_dir = platform_detect::config_dir();
        let _ = fs::create_dir_all(&config_dir);
        config_dir.join("settings.json")
    }

    fn load_or_default(path: &PathBuf) -> Value {
        if path.exists() {
            fs::read_to_string(path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_else(|| {
                    serde_json::from_str(DEFAULT_SETTINGS).unwrap_or(json!({}))
                })
        } else {
            // Write defaults to disk immediately so they exist
            let defaults: Value = serde_json::from_str(DEFAULT_SETTINGS).unwrap_or(json!({}));
            if let Ok(json) = serde_json::to_string_pretty(&defaults) {
                let _ = fs::write(path, json);
            }
            defaults
        }
    }

    /// Get all settings — returns the in-memory value WITHOUT reloading from disk.
    /// Reloading from disk on every read was the bug: it overwrites unsaved state.
    pub fn get_all(&self) -> Value {
        self.settings.clone()
    }

    /// Get a single setting by key
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.settings.get(key)
    }

    /// Set a single setting value
    pub fn set(&mut self, key: &str, value: Value) {
        if let Some(obj) = self.settings.as_object_mut() {
            obj.insert(key.to_string(), value);
        }
    }

    /// Reset all settings to defaults
    pub fn reset(&mut self) {
        self.settings = serde_json::from_str(DEFAULT_SETTINGS).unwrap_or(json!({}));
    }

    /// Save settings to disk with proper error logging
    pub fn save(&self) {
        match serde_json::to_string_pretty(&self.settings) {
            Ok(json) => {
                if let Some(parent) = self.config_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                match fs::write(&self.config_path, &json) {
                    Ok(_) => log::info!("Settings saved to {:?}", self.config_path),
                    Err(e) => log::error!("Failed to write settings to {:?}: {}", self.config_path, e),
                }
            }
            Err(e) => log::error!("Failed to serialize settings: {}", e),
        }
    }
}
