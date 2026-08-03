use crate::utils::platform_detect;

/// Enable or disable autostart on the current platform
pub async fn set_autostart(enabled: bool) -> Result<(), String> {
    if platform_detect::is_windows() {
        set_windows_autostart(enabled)
    } else if platform_detect::is_macos() {
        set_macos_autostart(enabled)
    } else if platform_detect::is_linux() {
        set_linux_autostart(enabled).await
    } else {
        Err("Unsupported platform for autostart".to_string())
    }
}

/// Check if autostart is enabled
pub async fn is_enabled() -> Result<bool, String> {
    if platform_detect::is_windows() {
        check_windows_autostart()
    } else if platform_detect::is_macos() {
        check_macos_autostart()
    } else if platform_detect::is_linux() {
        check_linux_autostart()
    } else {
        Ok(false)
    }
}

#[cfg(target_os = "windows")]
fn set_windows_autostart(enabled: bool) -> Result<(), String> {
    use std::process::Command;
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;

    if enabled {
        Command::new("reg")
            .args([
                "add",
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                "/v", "QuickMD",
                "/d", &format!("\"{}\" --hidden", exe_path.to_string_lossy()),
                "/f",
            ])
            .output()
            .map_err(|e| format!("Failed to set autostart: {}", e))?;
    } else {
        Command::new("reg")
            .args([
                "delete",
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                "/v", "QuickMD",
                "/f",
            ])
            .output()
            .ok(); // Ignore errors if key doesn't exist
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn set_windows_autostart(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn check_windows_autostart() -> Result<bool, String> {
    use std::process::Command;
    let output = Command::new("reg")
        .args([
            "query",
            "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            "/v", "QuickMD",
        ])
        .output()
        .ok();
    Ok(output.map(|o| o.status.success()).unwrap_or(false))
}

#[cfg(not(target_os = "windows"))]
fn check_windows_autostart() -> Result<bool, String> {
    Ok(false)
}

#[cfg(target_os = "macos")]
fn set_macos_autostart(enabled: bool) -> Result<(), String> {
    use std::fs;
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let launch_agents = format!("{}/Library/LaunchAgents", home);
    let plist_path = format!("{}/com.quickmd.app.plist", launch_agents);

    if enabled {
        fs::create_dir_all(&launch_agents).map_err(|e| e.to_string())?;

        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Failed to get exe path: {}", e))?;

        let plist = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.quickmd.app</string>
    <key>Program</key>
    <string>{}</string>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>"#,
            exe_path.to_string_lossy()
        );

        fs::write(&plist_path, plist).map_err(|e| e.to_string())?;
    } else {
        fs::remove_file(&plist_path).ok();
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn set_macos_autostart(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "macos")]
fn check_macos_autostart() -> Result<bool, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    Ok(std::path::Path::new(&format!(
        "{}/Library/LaunchAgents/com.quickmd.app.plist",
        home
    ))
    .exists())
}

#[cfg(not(target_os = "macos"))]
fn check_macos_autostart() -> Result<bool, String> {
    Ok(false)
}

#[cfg(target_os = "linux")]
async fn set_linux_autostart(enabled: bool) -> Result<(), String> {
    use std::fs;
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let autostart_dir = format!("{}/.config/autostart", home);
    let desktop_path = format!("{}/quickmd.desktop", autostart_dir);

    if enabled {
        fs::create_dir_all(&autostart_dir).map_err(|e| e.to_string())?;

        let exe_path = std::env::current_exe()
            .map_err(|e| format!("Failed to get exe path: {}", e))?;

        let desktop_entry = format!(
            r#"[Desktop Entry]
Type=Application
Name=QuickMD
Exec={}
Hidden=false
X-GNOME-Autostart-enabled=true
"#,
            exe_path.to_string_lossy()
        );

        fs::write(&desktop_path, desktop_entry).map_err(|e| e.to_string())?;
    } else {
        fs::remove_file(&desktop_path).ok();
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
async fn set_linux_autostart(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "linux")]
fn check_linux_autostart() -> Result<bool, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    Ok(
        std::path::Path::new(&format!("{}/.config/autostart/quickmd.desktop", home))
            .exists(),
    )
}

#[cfg(not(target_os = "linux"))]
fn check_linux_autostart() -> Result<bool, String> {
    Ok(false)
}
