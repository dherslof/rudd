use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "rudd",
    version,
    about = "Rust Directory Delta - Visualize delta between directories",
    long_about = "A (hopefully) fast CLI tool to compare two directories and show which files are unique, \
                  common, or different between them."
)]
pub struct Cli {
    /// Main directory (baseline for comparison)
    #[arg(value_name = "MAIN_DIR")]
    pub main_dir: PathBuf,

    /// Directory to compare against main
    #[arg(value_name = "COMPARE_DIR")]
    pub compare_dir: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Show only differences (hide common files)
    #[arg(short = 'd', long)]
    pub diff_only: bool,

    // Placeholders for future features
    /// Compare file contents using MD5 checksums
    #[arg(long)]
    pub md5: bool,
    // Perform detailed content diff, not sure how to do this yet (or if it's wanted...)
    //  #[arg(long)]
    //  pub content_diff: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
