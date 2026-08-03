use crate::commands::file::RecentFileEntry;
use crate::utils::platform_detect;
use serde::{Deserialize, Serialize};
use std::fs;

const MAX_RECENT_FILES: usize = 50;

#[derive(Debug, Serialize, Deserialize)]
struct RecentFilesData {
    files: Vec<RecentFileEntry>,
}

pub struct RecentFiles {
    data: RecentFilesData,
    storage_path: std::path::PathBuf,
}

impl RecentFiles {
    pub fn new() -> Self {
        let storage_path = platform_detect::data_dir().join("recent_files.json");
        let data = Self::load(&storage_path);

        Self {
            data,
            storage_path,
        }
    }

    fn load(path: &std::path::PathBuf) -> RecentFilesData {
        if path.exists() {
            fs::read_to_string(path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(RecentFilesData { files: Vec::new() })
        } else {
            RecentFilesData { files: Vec::new() }
        }
    }

    pub fn add(&mut self, path: &str, name: &str) {
        // Remove existing entry if present (for deduplication)
        self.data.files.retain(|f| f.path != path);

        // Add to front
        self.data.files.insert(
            0,
            RecentFileEntry {
                path: path.to_string(),
                name: name.to_string(),
                opened_at: chrono::Utc::now().to_rfc3339(),
            },
        );

        // Trim to max
        self.data.files.truncate(MAX_RECENT_FILES);
    }

    pub fn remove(&mut self, path: &str) {
        self.data.files.retain(|f| f.path != path);
    }

    pub fn get_all(&self) -> Vec<RecentFileEntry> {
        self.data.files.clone()
    }

    pub fn clear(&mut self) {
        self.data.files.clear();
    }

    pub fn save(&self) {
        if let Some(parent) = self.storage_path.parent() {
            fs::create_dir_all(parent).ok();
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.data) {
            fs::write(&self.storage_path, json).ok();
        }
    }
}
