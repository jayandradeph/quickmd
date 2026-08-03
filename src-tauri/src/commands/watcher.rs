use crate::services::file_watcher::FileWatcher;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

/// Start watching a file for changes
#[tauri::command]
pub async fn start_watching(
    path: String,
    watcher_state: State<'_, WatcherState>,
) -> Result<(), String> {
    let mut watcher_map = watcher_state.watchers.lock().map_err(|e| e.to_string())?;
    if watcher_map.contains_key(&path) {
        return Ok(()); // Already watching
    }

    let watcher = FileWatcher::new();
    watcher_map.insert(path, watcher);
    Ok(())
}

/// Stop watching a file
#[tauri::command]
pub async fn stop_watching(
    path: String,
    watcher_state: State<'_, WatcherState>,
) -> Result<(), String> {
    let mut watcher_map = watcher_state.watchers.lock().map_err(|e| e.to_string())?;
    watcher_map.remove(&path);
    Ok(())
}

pub struct WatcherState {
    pub watchers: Mutex<HashMap<String, FileWatcher>>,
}

impl Default for WatcherState {
    fn default() -> Self {
        Self {
            watchers: Mutex::new(HashMap::new()),
        }
    }
}
