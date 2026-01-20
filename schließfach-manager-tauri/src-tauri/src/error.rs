//! Custom error types for the Schließfach-Manager application.

use thiserror::Error;

/// Application error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Operation error: {0}")]
    Operation(String),
}

/// Result type alias for the application
pub type Result<T> = std::result::Result<T, AppError>;

// Implement conversion to String for Tauri commands
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

// Allow converting string errors
impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Operation(s.to_string())
    }
}
