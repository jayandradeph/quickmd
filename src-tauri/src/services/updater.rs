use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

/// Check for updates using the Tauri updater plugin
pub async fn check_for_updates(app: &AppHandle) -> Result<bool, String> {
    match app.updater() {
        Ok(updater) => match updater.check().await {
            Ok(Some(_update)) => {
                log::info!("Update available");
                Ok(true)
            }
            Ok(None) => {
                log::info!("No updates available");
                Ok(false)
            }
            Err(e) => {
                log::warn!("Failed to check for updates: {}", e);
                Err(format!("Update check failed: {}", e))
            }
        },
        Err(e) => {
            log::warn!("Updater not available: {}", e);
            Err(format!("Updater not available: {}", e))
        }
    }
}
