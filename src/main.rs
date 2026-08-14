//! APS - AUR Pattern Searcher

use clap::Parser;
use std::path::PathBuf;
use std::process;

mod aur_pkgbases;
mod fetch;
mod help;
mod results;
mod scan;
mod validate;
mod version;

// Arguments definition
#[derive(Parser)]
#[command(disable_help_flag = true, disable_version_flag = true)]
struct Args {
    #[arg(short = 'r', long, default_value = "aur.git")]
    repo: PathBuf,

    #[arg(short = 'p', long, default_value = "patterns.txt")]
    patterns: PathBuf,

    #[arg(short = 'd', long, default_value = "aps.db")]
    database: PathBuf,

    #[arg(short = 'f', long)]
    fetch: bool,

    #[arg(short = 'h', long)]
    help: bool,

    #[arg(short = 'V', long)]
    version: bool,
}

fn main() {
    // Parse arguments
    let args = Args::parse();

    // Show help message if the -h / --help arg is passed
    if args.help {
        help::show_help();
        return;
    }

    // Show name and version if the -V / --version arg is passed
    if args.version {
        version::show_version();
        return;
    }

    // Validate and set repo
    let repo = validate::validate_repo(&args.repo).unwrap_or_else(|error| {
        eprintln!("Error: {error:?}");
        process::exit(1);
    });

    // Validate and set patterns
    let patterns = validate::validate_patterns(&args.patterns).unwrap_or_else(|error| {
        eprintln!("Error: {error:?}");
        process::exit(2);
    });

    // Fetch new changes in the repo and download / refresh pkgbases list if the -f / --fetch option is passed
    if args.fetch {
        println!("Fetching new changes from the remote repository...\n");
        fetch::fetch_repo(&repo).unwrap_or_else(|error| {
            eprintln!("Error: {error:?}");
            process::exit(3);
        });

        println!("Fetching AUR pkgbases list...\n");
        aur_pkgbases::download_pkgbases().unwrap_or_else(|error| {
            eprintln!("Error: {error:?}");
            process::exit(4);
        });
    }

    // Load list of AUR pkgbases
    println!("Loading AUR pkgbases list...\n");
    let pkgbases = aur_pkgbases::load_pkgbases().unwrap_or_else(|error| {
        eprintln!("Error: {error:?}");
        process::exit(5);
    });

    // Scan repo for matching patterns
    println!("Scanning repository for matching patterns...\n");
    let matches =
        scan::scan_repo(&repo, &args.repo, &pkgbases, &patterns).unwrap_or_else(|error| {
            eprintln!("Error: {error:?}");
            process::exit(5);
        });

    // Print scan results summary
    println!("Results summary:\n");
    results::summary_results(&patterns, &matches);

    // Print scan detailed summary
    println!("\nDetailed results:\n");
    results::detailed_results(&patterns, &matches);
}
