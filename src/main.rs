//! APS - AUR Pattern Searcher

use clap::Parser;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process;

mod aur_pkgbases;
mod fetch;
mod help;
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

    // Fetch new changes in the repo if the -f / --fetch option is passed
    if args.fetch {
        println!("Fetching new changes from the remote repository...\n");
        fetch::fetch_repo(&repo).unwrap_or_else(|error| {
            eprintln!("Error: {error:?}");
            process::exit(3);
        });
    }

    // Get the current list of AUR pkgbases
    println!("Fetching AUR pkgbases list...\n");
    let pkgbases = aur_pkgbases::current_pkgbases().unwrap_or_else(|error| {
        eprintln!("Error: {error:?}");
        process::exit(4);
    });

    // Scan repo for matching patterns
    println!("Scanning repository for matching patterns...\n");
    let matches = scan::scan_repo(&repo, &pkgbases, &patterns).unwrap_or_else(|error| {
        eprintln!("Error: {error:?}");
        process::exit(5);
    });

    // Group matches by pattern and package
    let mut grouped_matches: BTreeMap<&str, BTreeMap<&str, Vec<&scan::Match>>> = BTreeMap::new();
    for matched in &matches {
        grouped_matches
            .entry(&matched.pattern)
            .or_default()
            .entry(&matched.package)
            .or_default()
            .push(matched);
    }

    // Print results summary
    println!("Results summary:\n");
    for (pattern, packages) in &grouped_matches {
        let count: usize = packages.values().map(|matches| matches.len()).sum();

        println!("{pattern}: {count} occurrence(s) found");
    }

    // Print detailed results grouped by pattern and package
    println!("\nDetailled results:\n");
    for (pattern, packages) in grouped_matches {
        for (package, matches) in packages {
            println!("{pattern}:");
            println!("  https://aur.archlinux.org/packages/{package}");

            for matched in matches {
                println!("    {}:{}", matched.path, matched.line);
            }
            println!();
        }
    }
}
