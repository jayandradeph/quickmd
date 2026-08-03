use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct FileWatcher {
    watcher: Option<Mutex<RecommendedWatcher>>,
    path: Option<PathBuf>,
}

impl FileWatcher {
    pub fn new() -> Self {
        Self {
            watcher: None,
            path: None,
        }
    }

    pub fn start(&mut self, path: &str) -> Result<(), String> {
        let path_buf = PathBuf::from(path);
        self.path = Some(path_buf.clone());

        let watch_dir = if path_buf.is_dir() {
            path_buf.clone()
        } else {
            path_buf
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."))
        };

        let (tx, _rx) = tokio::sync::mpsc::channel::<Event>(32);

        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    let _ = tx.blocking_send(event);
                }
            },
            Config::default(),
        )
        .map_err(|e| format!("Failed to create file watcher: {}", e))?;

        watcher
            .watch(&watch_dir, RecursiveMode::NonRecursive)
            .map_err(|e| format!("Failed to watch directory: {}", e))?;

        self.watcher = Some(Mutex::new(watcher));

        Ok(())
    }

    pub fn stop(&self) {
        // notify 6.x drops the watcher on drop
    }
}
