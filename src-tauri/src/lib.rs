mod commands;
mod services;
mod utils;

use commands::file::*;
use commands::settings::*;
use commands::export::*;
use commands::platform::*;
use commands::watcher::*;
use services::settings_manager::SettingsManager;
use services::recent_files::RecentFiles;
use std::sync::Mutex;
use tauri::Emitter;
use tauri::Manager;

/// Core application state shared across Tauri commands
pub struct AppState {
    pub settings: Mutex<SettingsManager>,
    pub recent_files: Mutex<RecentFiles>,
    pub current_file: Mutex<Option<String>>,
    pub pending_file: Mutex<Option<String>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let app_state = AppState {
        settings: Mutex::new(SettingsManager::new()),
        recent_files: Mutex::new(RecentFiles::new()),
        current_file: Mutex::new(None),
        pending_file: Mutex::new(None),
    };

    tauri::Builder::default()
        // Must be registered first: subsequent launches forward their args here
        // and exit, so the running instance handles them (single tray icon).
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            // argv[0] is the executable path; the rest are the real CLI args
            let args: Vec<String> = argv.into_iter().skip(1).collect();
            let start_hidden = args.iter().any(|a| a == "--hidden" || a == "--tray");

            // Store file path from the new launch for frontend to pick up
            for arg in &args {
                if arg.starts_with("--") {
                    continue;
                }
                let path_buf = std::path::PathBuf::from(arg);
                if path_buf.exists() {
                    let path_str = path_buf.to_string_lossy().to_string();
                    log::info!("Single-instance file path queued: {}", path_str);
                    if let Some(state) = app.try_state::<AppState>() {
                        if let Ok(mut pending) = state.pending_file.lock() {
                            *pending = Some(path_str.clone());
                        }
                    }
                    // Tell the already-running frontend to open it
                    let _ = app.emit("open-file-request", path_str);
                }
                break; // Only take the first non-flag argument
            }

            // Bring the existing window back up (it's hiding in the tray),
            // unless the new launch was explicitly hidden (e.g. autostart).
            if !start_hidden {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .manage(WatcherState::default())
        .setup(|app| {
            // Set up system tray (never panics)
            services::tray::create_tray(app.handle());

            // Collect CLI arguments
            let args: Vec<String> = std::env::args().skip(1).collect();
            let start_hidden = args.iter().any(|a| a == "--hidden" || a == "--tray");

            // Store file path from CLI for frontend to pick up
            for arg in &args {
                if arg.starts_with("--") {
                    continue;
                }
                let path_buf = std::path::PathBuf::from(arg);
                if path_buf.exists() {
                    let path_str = path_buf.to_string_lossy().to_string();
                    log::info!("CLI file path queued: {}", path_str);
                    if let Some(state) = app.try_state::<AppState>() {
                        if let Ok(mut pending) = state.pending_file.lock() {
                            *pending = Some(path_str);
                        }
                    }
                }
                break; // Only take the first non-flag argument
            }

            // If launched with --hidden (autostart mode), don't show window
            if start_hidden {
                log::info!("Starting in hidden/tray mode");
                // Window starts hidden by default when we don't show it
            } else if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
            }

            // Intercept close event → hide to tray instead of exiting
            if let Some(window) = app.get_webview_window("main") {
                let app_handle = app.handle().clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // Prevent the app from exiting
                        api.prevent_close();
                        // Hide the window to tray
                        let _ = app_handle.get_webview_window("main").map(|w| w.hide());
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_file,
            read_file_content,
            get_file_info,
            get_pending_file_path,
            get_settings,
            set_setting,
            reset_settings,
            export_html,
            get_temp_path,
            get_platform_info,
            register_file_association,
            remove_file_association,
            set_autostart,
            is_autostart_enabled,
            start_watching,
            stop_watching,
            get_recent_files,
            clear_recent_files,
            remove_recent_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running QuickMD");
}
