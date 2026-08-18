use rudd::cli::Cli;
use rudd::{DisplayOptions, diff_directories, display_diff, scan_directory};
use std::process;

fn main() {
    let args = Cli::parse_args();

    // Run the comparison
    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(args: Cli) -> rudd::Result<()> {
    // Scan both directories
    let main_files = scan_directory(&args.main_dir, args.md5)?;
    let compare_files = scan_directory(&args.compare_dir, args.md5)?;

    // Compute the diff
    let diff = diff_directories(main_files, compare_files, args.md5);

    // Display results
    let display_options = DisplayOptions {
        verbose: args.verbose,
        diff_only: args.diff_only,
    };

    display_diff(&diff, &args.main_dir, &args.compare_dir, &display_options);

    // Exit with non-zero code if differences were found?
    //  if diff.has_differences() {
    //      process::exit(1);
    //  }

    Ok(())
}
