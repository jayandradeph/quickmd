use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Failed to read file: {0}")]
    FileReadError(String),

    #[error("Failed to write file: {0}")]
    FileWriteError(String),

    #[error("Invalid encoding: {0}")]
    InvalidEncoding(String),

    #[error("File is too large ({size} bytes, max {max} bytes)")]
    FileTooLarge { size: u64, max: u64 },

    #[error("Not a supported file type: {0}")]
    UnsupportedFileType(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Platform error: {0}")]
    PlatformError(String),

    #[error("Settings error: {0}")]
    SettingsError(String),

    #[error("Export error: {0}")]
    ExportError(String),
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}
