use crate::services::settings_manager::SettingsManager;
use crate::AppState;
use serde_json::Value;
use tauri::State;

/// Get all settings as a JSON object
#[tauri::command]
pub async fn get_settings(
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let mut settings = state.settings.lock().map_err(|e| e.to_string())?;
    Ok(settings.get_all())
}

/// Set a single setting value
#[tauri::command]
pub async fn set_setting(
    key: String,
    value: Value,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut settings = state.settings.lock().map_err(|e| e.to_string())?;
    settings.set(&key, value);
    settings.save();
    Ok(())
}

/// Reset all settings to defaults
#[tauri::command]
pub async fn reset_settings(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut settings = state.settings.lock().map_err(|e| e.to_string())?;
    settings.reset();
    settings.save();
    Ok(())
}
