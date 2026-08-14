//! APS - AUR Pattern Searcher

use clap::Parser;
use std::path::PathBuf;
use std::process;

mod fetch;
mod help;
mod pkgbases;
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

    #[arg(short = 'P', long, default_value = "pkgbases.txt")]
    pkgbases: PathBuf,

    #[arg(short = 'f', long)]
    fetch: bool,

    #[arg(short = 'R', long)]
    refresh_pkgbases: bool,

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

    // Validate and set pkgbases
    let (mut pkgbases, needs_download) = validate::validate_pkgbases(&args.pkgbases)
        .unwrap_or_else(|error| {
            eprintln!("Error: {error:?}");
            process::exit(3);
        });

    // Fetch new changes in the repo and download / refresh pkgbases list if the -f / --fetch option is passed
    if args.fetch {
        println!("Fetching new changes from the remote repository...\n");
        fetch::fetch_repo(&repo).unwrap_or_else(|error| {
            eprintln!("Error: {error:?}");
            process::exit(4);
        });
    }

    // Fetch (or refresh) the list of current AUR pkgbases if the -R / --refresh-pkgbases option is passed
    // or if the provided list of pkgbases is non-existing / empty
    if args.refresh_pkgbases || needs_download {
        println!("Fetching AUR pkgbases list...\n");
        pkgbases = pkgbases::download_pkgbases(&args.pkgbases).unwrap_or_else(|error| {
            eprintln!("Error: {error:?}");
            process::exit(5);
        });
    }

    // Scan repo for matching patterns
    println!("Scanning repository for matching patterns...\n");
    let matches =
        scan::scan_repo(&repo, &args.repo, &pkgbases, &patterns).unwrap_or_else(|error| {
            eprintln!("Error: {error:?}");
            process::exit(6);
        });

    // Print scan results summary
    println!("Results summary:\n");
    results::summary_results(&patterns, &matches);

    // Print scan detailed summary
    println!("\nDetailed results:\n");
    results::detailed_results(&patterns, &matches);
}
