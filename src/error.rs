use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuddError {
    #[error("Directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("Path is not a directory: {0}")]
    NotADirectory(PathBuf),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to access path: {path}")]
    AccessDenied { path: PathBuf },
}

pub type Result<T> = std::result::Result<T, RuddError>;
