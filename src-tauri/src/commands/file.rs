use crate::services::recent_files::RecentFiles;
use crate::services::file_watcher::FileWatcher;
use crate::utils::error::AppError;
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub content: String,
    pub size: u64,
    pub modified_at: String,
    pub encoding: String,
}

const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024; // 50MB

/// Retrieve the CLI file path that was stored during setup.
/// Called by the frontend on mount to pick up any "Open With" file.
#[tauri::command]
pub async fn get_pending_file_path(
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    let mut pending = state.pending_file.lock().map_err(|e| e.to_string())?;
    Ok(pending.take())
}

/// Open a file and return its contents with metadata
#[tauri::command]
pub async fn open_file(
    path: String,
    state: State<'_, AppState>,
) -> Result<FileInfo, String> {
    let file_path = PathBuf::from(&path);

    // Check if file exists
    if !file_path.exists() {
        return Err(AppError::FileNotFound(path).into());
    }

    // Check file size
    let metadata = fs::metadata(&file_path).map_err(|e| AppError::FileReadError(e.to_string()))?;
    if metadata.len() > MAX_FILE_SIZE {
        return Err(AppError::FileTooLarge {
            size: metadata.len(),
            max: MAX_FILE_SIZE,
        }
        .into());
    }

    // Read file with encoding detection
    let bytes = fs::read(&file_path).map_err(|e| AppError::FileReadError(e.to_string()))?;
    let (content, encoding) = detect_encoding(&bytes);

    // Update current file in state
    let mut current = state.current_file.lock().map_err(|e| e.to_string())?;
    *current = Some(path.clone());
    drop(current);

    // Add to recent files
    let mut recent = state.recent_files.lock().map_err(|e| e.to_string())?;
    let file_name = file_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    recent.add(&path, &file_name);
    recent.save();

    let modified = metadata
        .modified()
        .map(|t| {
            let dt: chrono::DateTime<chrono::Utc> = t.into();
            dt.to_rfc3339()
        })
        .unwrap_or_else(|_| "unknown".to_string());

    Ok(FileInfo {
        path,
        name: file_name,
        content,
        size: metadata.len(),
        modified_at: modified,
        encoding,
    })
}

/// Read file content directly (without updating recent files)
#[tauri::command]
pub async fn read_file_content(path: String) -> Result<FileInfo, String> {
    let file_path = PathBuf::from(&path);

    if !file_path.exists() {
        return Err(AppError::FileNotFound(path).into());
    }

    let metadata = fs::metadata(&file_path).map_err(|e| AppError::FileReadError(e.to_string()))?;
    if metadata.len() > MAX_FILE_SIZE {
        return Err(AppError::FileTooLarge {
            size: metadata.len(),
            max: MAX_FILE_SIZE,
        }
        .into());
    }

    let bytes = fs::read(&file_path).map_err(|e| AppError::FileReadError(e.to_string()))?;
    let (content, encoding) = detect_encoding(&bytes);

    let file_name = file_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let modified = metadata
        .modified()
        .map(|t| {
            use std::time::UNIX_EPOCH;
            t.duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs().to_string())
                .unwrap_or_else(|_| "unknown".to_string())
        })
        .unwrap_or_else(|_| "unknown".to_string());

    Ok(FileInfo {
        path,
        name: file_name,
        content,
        size: metadata.len(),
        modified_at: modified,
        encoding,
    })
}

/// Get basic file info without reading content
#[tauri::command]
pub async fn get_file_info(path: String) -> Result<FileInfo, String> {
    let file_path = PathBuf::from(&path);

    if !file_path.exists() {
        return Err(AppError::FileNotFound(path).into());
    }

    let metadata = fs::metadata(&file_path).map_err(|e| AppError::FileReadError(e.to_string()))?;

    let file_name = file_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    Ok(FileInfo {
        path,
        name: file_name,
        content: String::new(),
        size: metadata.len(),
        modified_at: "unknown".to_string(),
        encoding: "utf-8".to_string(),
    })
}

/// Get recent files list
#[tauri::command]
pub async fn get_recent_files(
    state: State<'_, AppState>,
) -> Result<Vec<RecentFileEntry>, String> {
    let recent = state.recent_files.lock().map_err(|e| e.to_string())?;
    Ok(recent.get_all())
}

/// Clear all recent files
#[tauri::command]
pub async fn clear_recent_files(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut recent = state.recent_files.lock().map_err(|e| e.to_string())?;
    recent.clear();
    recent.save();
    Ok(())
}

/// Remove a single recent file entry
#[tauri::command]
pub async fn remove_recent_file(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut recent = state.recent_files.lock().map_err(|e| e.to_string())?;
    recent.remove(&path);
    recent.save();
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecentFileEntry {
    pub path: String,
    pub name: String,
    pub opened_at: String,
}

/// Detect text encoding from bytes, falling back to UTF-8 with replacement
fn detect_encoding(bytes: &[u8]) -> (String, String) {
    // Check BOM
    if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
        return (
            String::from_utf8_lossy(&bytes[3..]).to_string(),
            "utf-8-bom".to_string(),
        );
    }
    if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
        let utf16: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();
        return (
            String::from_utf16_lossy(&utf16),
            "utf-16-be".to_string(),
        );
    }
    if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
        let utf16: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        return (
            String::from_utf16_lossy(&utf16),
            "utf-16-le".to_string(),
        );
    }

    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(bytes, bytes.len() < 1024);
    let encoding = detector.guess(None, true);

    match encoding.name() {
        "UTF-8" => match String::from_utf8(bytes.to_vec()) {
            Ok(s) => (s, "utf-8".to_string()),
            Err(_) => (String::from_utf8_lossy(bytes).to_string(), "utf-8-lossy".to_string()),
        },
        name => {
            let (cow, _actual_encoding, _had_errors) = encoding.decode(bytes);
            (cow.into_owned(), name.to_string())
        }
    }
}
