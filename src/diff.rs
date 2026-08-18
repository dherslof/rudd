use crate::scanner::FileEntry;
use std::collections::HashMap;
use std::path::PathBuf;

/// Result of comparing two directories
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryDiff {
    /// Files only in the main directory
    pub only_in_main: Vec<FileEntry>,
    /// Files only in the comparison directory
    pub only_in_compare: Vec<FileEntry>,
    /// Files present in both directories
    pub in_both: Vec<(FileEntry, FileEntry)>,
    /// Files with the same relative path but different content
    pub different_content: Vec<(FileEntry, FileEntry)>,
}

impl DirectoryDiff {
    /// Check if there are any differences between directories
    pub fn has_differences(&self) -> bool {
        !self.only_in_main.is_empty()
            || !self.only_in_compare.is_empty()
            || !self.different_content.is_empty()
    }

    /// Get total number of files across all categories
    pub fn total_files(&self) -> usize {
        self.only_in_main.len()
            + self.only_in_compare.len()
            + self.in_both.len()
            + self.different_content.len()
    }
}

/// Compare two directories and return the differences
pub fn diff_directories(
    main_files: HashMap<PathBuf, FileEntry>,
    compare_files: HashMap<PathBuf, FileEntry>,
    compare_md5: bool,
) -> DirectoryDiff {
    let mut only_in_main = Vec::new();
    let mut only_in_compare = Vec::new();
    let mut in_both = Vec::new();
    let mut different_content = Vec::new();

    // Find files only in main or in both
    for (path, entry) in &main_files {
        if let Some(compare_entry) = compare_files.get(path) {
            if compare_md5
                && entry.md5.is_some()
                && compare_entry.md5.is_some()
                && entry.md5 != compare_entry.md5
            {
                different_content.push((entry.clone(), compare_entry.clone()));
            } else {
                in_both.push((entry.clone(), compare_entry.clone()));
            }
        } else {
            only_in_main.push(entry.clone());
        }
    }

    // Find files only in compare
    for (path, entry) in &compare_files {
        if !main_files.contains_key(path) {
            only_in_compare.push(entry.clone());
        }
    }

    // Sort for consistent output
    only_in_main.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    only_in_compare.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    in_both.sort_by(|a, b| a.0.relative_path.cmp(&b.0.relative_path));
    different_content.sort_by(|a, b| a.0.relative_path.cmp(&b.0.relative_path));

    DirectoryDiff {
        only_in_main,
        only_in_compare,
        in_both,
        different_content,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_file_entry(rel_path: &str, abs_path: &str) -> FileEntry {
        FileEntry {
            relative_path: PathBuf::from(rel_path),
            absolute_path: PathBuf::from(abs_path),
            md5: None,
        }
    }

    #[test]
    fn test_diff_no_common_files() {
        let mut main_files = HashMap::new();
        main_files.insert(
            PathBuf::from("a.txt"),
            create_file_entry("a.txt", "/main/a.txt"),
        );

        let mut compare_files = HashMap::new();
        compare_files.insert(
            PathBuf::from("b.txt"),
            create_file_entry("b.txt", "/compare/b.txt"),
        );

        let diff = diff_directories(main_files, compare_files, false);
        assert_eq!(diff.only_in_main.len(), 1);
        assert_eq!(diff.only_in_compare.len(), 1);
        assert_eq!(diff.in_both.len(), 0);
        assert_eq!(diff.different_content.len(), 0);
        assert!(diff.has_differences());
    }

    #[test]
    fn test_diff_all_common_files() {
        let mut main_files = HashMap::new();
        main_files.insert(
            PathBuf::from("a.txt"),
            create_file_entry("a.txt", "/main/a.txt"),
        );

        let mut compare_files = HashMap::new();
        compare_files.insert(
            PathBuf::from("a.txt"),
            create_file_entry("a.txt", "/compare/a.txt"),
        );

        let diff = diff_directories(main_files, compare_files, false);
        assert_eq!(diff.only_in_main.len(), 0);
        assert_eq!(diff.only_in_compare.len(), 0);
        assert_eq!(diff.in_both.len(), 1);
        assert_eq!(diff.different_content.len(), 0);
        assert!(!diff.has_differences());
    }

    #[test]
    fn test_diff_mixed() {
        let mut main_files = HashMap::new();
        main_files.insert(
            PathBuf::from("common.txt"),
            create_file_entry("common.txt", "/main/common.txt"),
        );
        main_files.insert(
            PathBuf::from("only_main.txt"),
            create_file_entry("only_main.txt", "/main/only_main.txt"),
        );

        let mut compare_files = HashMap::new();
        compare_files.insert(
            PathBuf::from("common.txt"),
            create_file_entry("common.txt", "/compare/common.txt"),
        );
        compare_files.insert(
            PathBuf::from("only_compare.txt"),
            create_file_entry("only_compare.txt", "/compare/only_compare.txt"),
        );

        let diff = diff_directories(main_files, compare_files, false);
        assert_eq!(diff.only_in_main.len(), 1);
        assert_eq!(diff.only_in_compare.len(), 1);
        assert_eq!(diff.in_both.len(), 1);
        assert_eq!(diff.different_content.len(), 0);
        assert!(diff.has_differences());
    }

    #[test]
    fn test_diff_empty_directories() {
        let main_files = HashMap::new();
        let compare_files = HashMap::new();

        let diff = diff_directories(main_files, compare_files, false);
        assert_eq!(diff.total_files(), 0);
        assert!(!diff.has_differences());
    }

    #[test]
    fn test_diff_md5_marks_different_content() {
        let mut main_files = HashMap::new();
        let mut main_entry = create_file_entry("same.txt", "/main/same.txt");
        main_entry.md5 = Some("aaa".to_string());
        main_files.insert(PathBuf::from("same.txt"), main_entry);

        let mut compare_files = HashMap::new();
        let mut compare_entry = create_file_entry("same.txt", "/compare/same.txt");
        compare_entry.md5 = Some("bbb".to_string());
        compare_files.insert(PathBuf::from("same.txt"), compare_entry);

        let diff = diff_directories(main_files, compare_files, true);
        assert_eq!(diff.in_both.len(), 0);
        assert_eq!(diff.different_content.len(), 1);
        assert!(diff.has_differences());
    }

    #[test]
    fn test_diff_md5_disabled_keeps_common_files_together() {
        let mut main_files = HashMap::new();
        let mut main_entry = create_file_entry("same.txt", "/main/same.txt");
        main_entry.md5 = Some("aaa".to_string());
        main_files.insert(PathBuf::from("same.txt"), main_entry);

        let mut compare_files = HashMap::new();
        let mut compare_entry = create_file_entry("same.txt", "/compare/same.txt");
        compare_entry.md5 = Some("bbb".to_string());
        compare_files.insert(PathBuf::from("same.txt"), compare_entry);

        let diff = diff_directories(main_files, compare_files, false);
        assert_eq!(diff.in_both.len(), 1);
        assert_eq!(diff.different_content.len(), 0);
        assert!(!diff.has_differences());
    }
}
