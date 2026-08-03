use crate::services::file_association;
use crate::services::startup;
use crate::utils::platform_detect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub os: String,
    pub is_windows: bool,
    pub is_macos: bool,
    pub is_linux: bool,
}

/// Get information about the current platform
#[tauri::command]
pub async fn get_platform_info() -> Result<PlatformInfo, String> {
    Ok(PlatformInfo {
        os: platform_detect::detect_os().to_string(),
        is_windows: platform_detect::is_windows(),
        is_macos: platform_detect::is_macos(),
        is_linux: platform_detect::is_linux(),
    })
}

/// Register file associations for Markdown files
#[tauri::command]
pub async fn register_file_association() -> Result<(), String> {
    file_association::register_associations().await
}

/// Remove file associations for Markdown files
#[tauri::command]
pub async fn remove_file_association() -> Result<(), String> {
    file_association::remove_associations().await
}

/// Enable or disable autostart
#[tauri::command]
pub async fn set_autostart(enabled: bool) -> Result<(), String> {
    startup::set_autostart(enabled).await
}

/// Check if autostart is currently enabled
#[tauri::command]
pub async fn is_autostart_enabled() -> Result<bool, String> {
    startup::is_enabled().await
}
