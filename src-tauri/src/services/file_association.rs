use crate::utils::platform_detect;

/// Register file associations for Markdown files on the current platform
pub async fn register_associations() -> Result<(), String> {
    if platform_detect::is_windows() {
        register_windows().await
    } else if platform_detect::is_linux() {
        register_linux().await
    } else if platform_detect::is_macos() {
        log::info!("macOS file associations are configured via Info.plist");
        Ok(())
    } else {
        Err("Unsupported platform for file associations".to_string())
    }
}

/// Remove file associations
pub async fn remove_associations() -> Result<(), String> {
    if platform_detect::is_windows() {
        remove_windows().await
    } else if platform_detect::is_linux() {
        remove_linux().await
    } else {
        Ok(())
    }
}

#[cfg(target_os = "windows")]
async fn register_windows() -> Result<(), String> {
    use std::process::Command;

    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;
    let exe_str = exe_path.to_string_lossy();

    let extensions = ["md", "markdown", "mdown"];

    // 1. Register QuickMD as a capability-bearing application
    let _ = Command::new("reg")
        .args([
            "add",
            "HKCU\\Software\\RegisteredApplications",
            "/v", "QuickMD",
            "/d", "SOFTWARE\\Classes\\QuickMD.md\\Capabilities",
            "/f",
        ])
        .output();

    // 2. Create the ProgID tree with proper capability registration
    let progid = "QuickMD.md";
    let _ = Command::new("reg")
        .args(["add", &format!("HKCU\\Software\\Classes\\{progid}"), "/ve", "/d", "Markdown Document", "/f"])
        .output();
    let _ = Command::new("reg")
        .args(["add", &format!("HKCU\\Software\\Classes\\{progid}\\DefaultIcon"), "/ve", "/d", &format!("{},0", exe_str), "/f"])
        .output();
    let _ = Command::new("reg")
        .args(["add", &format!("HKCU\\Software\\Classes\\{progid}\\shell\\open"), "/ve", "/d", "Open with QuickMD", "/f"])
        .output();
    let _ = Command::new("reg")
        .args(["add", &format!("HKCU\\Software\\Classes\\{progid}\\shell\\open\\command"), "/ve", "/d", &format!("\"{}\" \"%1\"", exe_str), "/f"])
        .output();

    // Register as a capability handler for .md files
    let _ = Command::new("reg")
        .args(["add", &format!("HKCU\\Software\\Classes\\{progid}\\Capabilities"), "/v", "ApplicationName", "/d", "QuickMD", "/f"])
        .output();
    let _ = Command::new("reg")
        .args(["add", &format!("HKCU\\Software\\Classes\\{progid}\\Capabilities\\FileAssociations"), "/v", ".md", "/d", progid, "/f"])
        .output();
    let _ = Command::new("reg")
        .args(["add", &format!("HKCU\\Software\\Classes\\{progid}\\Capabilities\\FileAssociations"), "/v", ".markdown", "/d", progid, "/f"])
        .output();
    let _ = Command::new("reg")
        .args(["add", &format!("HKCU\\Software\\Classes\\{progid}\\Capabilities\\FileAssociations"), "/v", ".mdown", "/d", progid, "/f"])
        .output();

    // 3. Associate each extension with the ProgID via OpenWithProgids
    for ext in &extensions {
        // Set the user default progid for this extension
        let _ = Command::new("reg")
            .args(["add", &format!("HKCU\\Software\\Classes\\.{ext}"), "/ve", "/d", progid, "/f"])
            .output();

        // Add to OpenWithProgids so it shows in "Open With" menu
        let _ = Command::new("reg")
            .args(["add", &format!("HKCU\\Software\\Classes\\.{ext}\\OpenWithProgids"), "/v", progid, "/d", "", "/f"])
            .output();
    }

    log::info!("Windows file associations registered for .md, .markdown, .mdown");
    Ok(())
}

#[cfg(not(target_os = "windows"))]
async fn register_windows() -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
async fn remove_windows() -> Result<(), String> {
    use std::process::Command;

    let progid = "QuickMD.md";
    let extensions = ["md", "markdown", "mdown"];

    // Remove extension associations
    for ext in &extensions {
        let _ = Command::new("reg")
            .args(["delete", &format!("HKCU\\Software\\Classes\\.{ext}\\OpenWithProgids"), "/v", progid, "/f"])
            .output();
        // Only delete the ext key if we set it — safer to leave it
    }

    // Remove the ProgID
    let _ = Command::new("reg")
        .args(["delete", &format!("HKCU\\Software\\Classes\\{progid}"), "/f"])
        .output();

    // Remove from RegisteredApplications
    let _ = Command::new("reg")
        .args(["delete", "HKCU\\Software\\RegisteredApplications", "/v", "QuickMD", "/f"])
        .output();

    log::info!("Windows file associations removed");
    Ok(())
}

#[cfg(not(target_os = "windows"))]
async fn remove_windows() -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "linux")]
async fn register_linux() -> Result<(), String> {
    use std::fs;
    use std::process::Command;

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let local_share = format!("{}/.local/share", home);
    let applications_dir = format!("{}/applications", local_share);
    fs::create_dir_all(&applications_dir).map_err(|e| e.to_string())?;

    let desktop_entry = format!(
        r#"[Desktop Entry]
Type=Application
Name=QuickMD
Comment=A fast Markdown viewer
Exec={} %f
Icon=quickmd
Terminal=false
Categories=Utility;Viewer;
MimeType=text/markdown;text/x-markdown;
StartupNotify=false
"#,
        std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "quickmd".to_string())
    );

    let desktop_path = format!("{}/quickmd.desktop", applications_dir);
    fs::write(&desktop_path, desktop_entry).map_err(|e| e.to_string())?;

    let _ = Command::new("xdg-mime")
        .args(["default", "quickmd.desktop", "text/markdown"])
        .output();
    let _ = Command::new("update-desktop-database")
        .arg(&applications_dir)
        .output();

    log::info!("Linux desktop entry and MIME associations registered");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
async fn register_linux() -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "linux")]
async fn remove_linux() -> Result<(), String> {
    use std::fs;
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let desktop_path = format!("{}/.local/share/applications/quickmd.desktop", home);
    fs::remove_file(&desktop_path).ok();
    Ok(())
}

#[cfg(not(target_os = "linux"))]
async fn remove_linux() -> Result<(), String> {
    Ok(())
}
