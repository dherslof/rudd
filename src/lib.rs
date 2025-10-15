//! Rudd - Rust Directory Delta
//!
//! A library for comparing directories and identifying file differences.

pub mod cli;
pub mod diff;
pub mod display;
pub mod error;
pub mod scanner;

pub use diff::{DirectoryDiff, diff_directories};
pub use display::{DisplayOptions, display_diff};
pub use error::{Result, RuddError};
pub use scanner::{FileEntry, scan_directory};
