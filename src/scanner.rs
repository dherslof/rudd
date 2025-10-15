use crate::error::{Result, RuddError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Represents a file in a directory with its relative path
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    /// Relative path from the root directory
    pub relative_path: PathBuf,
    /// Absolute path to the file
    pub absolute_path: PathBuf,
}

/// Scans a directory and returns a map of relative paths to FileEntry
pub fn scan_directory(dir: &Path) -> Result<HashMap<PathBuf, FileEntry>> {
    if !dir.exists() {
        return Err(RuddError::DirectoryNotFound(dir.to_path_buf()));
    }

    if !dir.is_dir() {
        return Err(RuddError::NotADirectory(dir.to_path_buf()));
    }

    let mut files = HashMap::new();
    let canonical_dir = dir.canonicalize()?;

    for entry in WalkDir::new(&canonical_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let absolute_path = entry.path().to_path_buf();
            let relative_path = absolute_path
                .strip_prefix(&canonical_dir)
                .map_err(|_| {
                    RuddError::IoError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to compute relative path",
                    ))
                })?
                .to_path_buf();

            files.insert(
                relative_path.clone(),
                FileEntry {
                    relative_path,
                    absolute_path,
                },
            );
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_file(dir: &Path, relative_path: &str) -> std::io::Result<()> {
        let file_path = dir.join(relative_path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(file_path, "test content")?;
        Ok(())
    }

    #[test]
    fn test_scan_directory_basic() {
        let temp_dir = TempDir::new().unwrap();
        create_test_file(temp_dir.path(), "file1.txt").unwrap();
        create_test_file(temp_dir.path(), "subdir/file2.txt").unwrap();

        let files = scan_directory(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.contains_key(Path::new("file1.txt")));
        assert!(files.contains_key(Path::new("subdir/file2.txt")));
    }

    #[test]
    fn test_scan_directory_not_found() {
        let result = scan_directory(Path::new("/nonexistent/directory"));
        assert!(matches!(result, Err(RuddError::DirectoryNotFound(_))));
    }

    #[test]
    fn test_scan_directory_empty() {
        let temp_dir = TempDir::new().unwrap();
        let files = scan_directory(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 0);
    }
}
