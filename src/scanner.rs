use crate::error::{Result, RuddError};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Represents a file in a directory with its relative path
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    /// Relative path from the root directory
    pub relative_path: PathBuf,
    /// Absolute path to the file
    pub absolute_path: PathBuf,
    /// Optional MD5 checksum of the file contents
    pub md5: Option<String>,
}

/// Scans a directory and returns a map of relative paths to FileEntry
pub fn scan_directory(dir: &Path, calculate_md5: bool) -> Result<HashMap<PathBuf, FileEntry>> {
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
                    RuddError::IoError(std::io::Error::other("Failed to compute relative path"))
                })?
                .to_path_buf();

            let md5 = if calculate_md5 {
                Some(compute_md5(&absolute_path)?)
            } else {
                None
            };

            files.insert(
                relative_path.clone(),
                FileEntry {
                    relative_path,
                    absolute_path,
                    md5,
                },
            );
        }
    }

    Ok(files)
}

fn compute_md5(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut context = md5::Context::new();
    let mut buffer = [0_u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        context.consume(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", context.compute()))
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

        let files = scan_directory(temp_dir.path(), false).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.contains_key(Path::new("file1.txt")));
        assert!(files.contains_key(Path::new("subdir/file2.txt")));
    }

    #[test]
    fn test_scan_directory_not_found() {
        let result = scan_directory(Path::new("/nonexistent/directory"), false);
        assert!(matches!(result, Err(RuddError::DirectoryNotFound(_))));
    }

    #[test]
    fn test_scan_directory_empty() {
        let temp_dir = TempDir::new().unwrap();
        let files = scan_directory(temp_dir.path(), false).unwrap();
        assert_eq!(files.len(), 0);
    }

    #[test]
    fn test_scan_directory_md5_hashes_files() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("file.txt");
        fs::write(&file_path, "hello world").unwrap();

        let files = scan_directory(temp_dir.path(), true).unwrap();
        let entry = files.get(Path::new("file.txt")).unwrap();
        assert_eq!(
            entry.md5.as_deref(),
            Some("5eb63bbbe01eeed093cb22bb8f5acdc3")
        );
    }

    #[test]
    fn test_scan_directory_without_md5_leaves_hash_empty() {
        let temp_dir = TempDir::new().unwrap();
        create_test_file(temp_dir.path(), "file.txt").unwrap();

        let files = scan_directory(temp_dir.path(), false).unwrap();
        let entry = files.get(Path::new("file.txt")).unwrap();
        assert_eq!(entry.md5, None);
    }
}
