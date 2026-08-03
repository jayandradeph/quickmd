use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Runtime,
};

/// Create a simple 32x32 RGBA icon (blue square with white "M" letter shape).
fn make_tray_icon() -> tauri::image::Image<'static> {
    let width = 32u32;
    let height = 32u32;
    let mut rgba = Vec::with_capacity((width * height * 4) as usize);

    // QuickMD brand blue: rgb(9, 105, 218)
    let bg_r = 9u8;
    let bg_g = 105u8;
    let bg_b = 218u8;

    for y in 0..height {
        for x in 0..width {
            // Simple "M" letter shape in white
            let is_white =
                // Left vertical bar
                (x >= 4 && x <= 7 && y >= 6 && y <= 25) ||
                // Right vertical bar
                (x >= 24 && x <= 27 && y >= 6 && y <= 25) ||
                // Left diagonal (up)
                (x >= 8 && x <= 15 && y >= 10 && y <= 18 && (x - 8) <= (y - 10) && (15 - x) >= (y - 10)) ||
                // Right diagonal (down)
                (x >= 16 && x <= 23 && y >= 10 && y <= 18 && (x - 16) >= (18 - y) && (23 - x) <= (18 - y));

            if is_white {
                rgba.extend_from_slice(&[255, 255, 255, 255]); // RGBA white
            } else {
                rgba.extend_from_slice(&[bg_r, bg_g, bg_b, 255]); // RGBA blue
            }
        }
    }

    tauri::image::Image::new_owned(rgba, width, height)
}

/// Create and configure the system tray icon.
/// Never panics — silently logs warnings if any step fails.
pub fn create_tray<R: Runtime>(app_handle: &AppHandle<R>) {
    let icon = make_tray_icon();

    let open_item = match MenuItemBuilder::with_id("open", "Open Markdown File").build(app_handle) {
        Ok(item) => item,
        Err(e) => { log::warn!("Tray menu item failed: {}", e); return; }
    };
    let settings_item = match MenuItemBuilder::with_id("settings", "Settings").build(app_handle) {
        Ok(item) => item,
        Err(e) => { log::warn!("Tray menu item failed: {}", e); return; }
    };
    let about_item = match MenuItemBuilder::with_id("about", "About QuickMD").build(app_handle) {
        Ok(item) => item,
        Err(e) => { log::warn!("Tray menu item failed: {}", e); return; }
    };
    let separator = match PredefinedMenuItem::separator(app_handle) {
        Ok(item) => item,
        Err(e) => { log::warn!("Tray separator failed: {}", e); return; }
    };
    let quit_item = match MenuItemBuilder::with_id("quit", "Exit").build(app_handle) {
        Ok(item) => item,
        Err(e) => { log::warn!("Tray menu item failed: {}", e); return; }
    };

    let menu = match MenuBuilder::new(app_handle)
        .item(&open_item)
        .item(&settings_item)
        .item(&separator)
        .item(&about_item)
        .item(&quit_item)
        .build()
    {
        Ok(m) => m,
        Err(e) => { log::warn!("Tray menu build failed: {}", e); return; }
    };

    if let Err(e) = TrayIconBuilder::new()
        .icon(icon)
        .tooltip("QuickMD")
        .show_menu_on_left_click(false)
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "open" => {
                    let _ = app.emit("tray-open-file", ());
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "settings" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                    let _ = app.emit("tray-open-settings", ());
                }
                "about" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                    let _ = app.emit("tray-open-about", ());
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app_handle)
    {
        log::warn!("Failed to build tray icon: {}", e);
    } else {
        log::info!("System tray icon created successfully");
    }
}
