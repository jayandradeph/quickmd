/// Detect the current operating system
pub fn detect_os() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}

/// Check if running on Windows
pub fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

/// Check if running on macOS
pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

/// Check if running on Linux
pub fn is_linux() -> bool {
    cfg!(target_os = "linux")
}

/// Get the platform-specific config directory
pub fn config_dir() -> std::path::PathBuf {
    directories::ProjectDirs::from("com", "quickmd", "QuickMD")
        .map(|dirs| dirs.config_dir().to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from(".quickmd"))
}

/// Get the platform-specific data directory
pub fn data_dir() -> std::path::PathBuf {
    directories::ProjectDirs::from("com", "quickmd", "QuickMD")
        .map(|dirs| dirs.data_dir().to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from(".quickmd"))
}
