use crate::diff::DirectoryDiff;
use colored::*;
use std::path::Path;

/// Options for displaying diff results
pub struct DisplayOptions {
    pub verbose: bool,
    pub diff_only: bool,
}

/// Display the directory diff in a formatted way
pub fn display_diff(
    diff: &DirectoryDiff,
    main_dir: &Path,
    compare_dir: &Path,
    options: &DisplayOptions,
) {
    println!("\n{}", "=".repeat(80).bright_blue());
    println!(
        "{} {} {}",
        "Directory Comparison:".bright_cyan().bold(),
        main_dir.display().to_string().yellow(),
        format!("vs {}", compare_dir.display()).yellow()
    );
    println!("{}\n", "=".repeat(80).bright_blue());

    // Summary
    print_summary(diff);

    // Only in main directory
    if !diff.only_in_main.is_empty() {
        println!(
            "\n{}",
            format!(
                "━ Only in {} ({} files)",
                main_dir.display(),
                diff.only_in_main.len()
            )
            .red()
            .bold()
        );
        for entry in &diff.only_in_main {
            println!(
                "  {} {}",
                "✗".red(),
                entry.relative_path.display().to_string().red()
            );
            if options.verbose {
                println!("    {}", entry.absolute_path.display().to_string().dimmed());
            }
        }
    }

    // Only in compare directory
    if !diff.only_in_compare.is_empty() {
        println!(
            "\n{}",
            format!(
                "━ Only in {} ({} files)",
                compare_dir.display(),
                diff.only_in_compare.len()
            )
            .green()
            .bold()
        );
        for entry in &diff.only_in_compare {
            println!(
                "  {} {}",
                "+".green(),
                entry.relative_path.display().to_string().green()
            );
            if options.verbose {
                println!("    {}", entry.absolute_path.display().to_string().dimmed());
            }
        }
    }

    // Common files (if not in diff-only mode)
    if !options.diff_only && !diff.in_both.is_empty() {
        println!(
            "\n{}",
            format!("━ Common files ({} files)", diff.in_both.len())
                .white()
                .bold()
        );
        for (main_entry, _) in &diff.in_both {
            println!(
                "  {} {}",
                "=".white(),
                main_entry.relative_path.display().to_string().white()
            );
            if options.verbose {
                println!(
                    "    {}",
                    main_entry.absolute_path.display().to_string().dimmed()
                );
            }
        }
    }

    // Final status
    println!("\n{}", "=".repeat(80).bright_blue());
    if diff.has_differences() {
        println!("{}", "✗ Directories differ".red().bold());
    } else {
        println!("{}", "✓ Directories are identical".green().bold());
    }
    println!("{}\n", "=".repeat(80).bright_blue());
}

fn print_summary(diff: &DirectoryDiff) {
    println!("{}", "Summary:".bright_white().bold());

    let status = if diff.has_differences() {
        "DIFFERENT".red().bold()
    } else {
        "IDENTICAL".green().bold()
    };

    println!("  Status: {}", status);
    println!(
        "  Total files compared: {}",
        diff.total_files().to_string().cyan()
    );

    if diff.only_in_main.is_empty() {
        println!("  Only in main: {}", "0".green());
    } else {
        println!(
            "  Only in main: {}",
            diff.only_in_main.len().to_string().red()
        );
    }

    if diff.only_in_compare.is_empty() {
        println!("  Only in compare: {}", "0".green());
    } else {
        println!(
            "  Only in compare: {}",
            diff.only_in_compare.len().to_string().green()
        );
    }

    println!("  Common files: {}", diff.in_both.len().to_string().white());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::FileEntry;
    use std::path::PathBuf;

    fn create_file_entry(rel_path: &str, abs_path: &str) -> FileEntry {
        FileEntry {
            relative_path: PathBuf::from(rel_path),
            absolute_path: PathBuf::from(abs_path),
        }
    }

    #[test]
    fn test_display_options() {
        let options = DisplayOptions {
            verbose: true,
            diff_only: false,
        };
        assert!(options.verbose);
        assert!(!options.diff_only);
    }

    #[test]
    fn test_display_diff_no_panic() {
        let diff = DirectoryDiff {
            only_in_main: vec![create_file_entry("test.txt", "/main/test.txt")],
            only_in_compare: vec![],
            in_both: vec![],
        };

        let options = DisplayOptions {
            verbose: false,
            diff_only: false,
        };

        // Just ensure it doesn't panic
        display_diff(&diff, Path::new("/main"), Path::new("/compare"), &options);
    }
}
