//! APS - Aur Pattern Searcher

use clap::Parser;
use std::path::PathBuf;
use std::process;

mod help;
mod validate;
mod version;

// Argument definition
#[derive(Parser)]
#[command(disable_help_flag = true, disable_version_flag = true)]
struct Args {
    #[arg(short = 'r', long, default_value = "aur.git")]
    repo: PathBuf,

    #[arg(short = 'p', long, default_value = "patterns.txt")]
    patterns: PathBuf,

    #[arg(short = 'd', long, default_value = "aps.db")]
    database: PathBuf,

    #[arg(short = 'l', long)]
    log: Option<PathBuf>,

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

    // Set repo path and validate it
    let repo_path = args.repo;
    validate::validate_repo(&repo_path).unwrap_or_else(|error| {
        eprintln!("Error: {error:?}");
        process::exit(1);
    });

    // Set patterns path and validate it
    let patterns_path = args.patterns;
    validate::validate_patterns(&patterns_path).unwrap_or_else(|error| {
        eprintln!("Error: {error:?}");
        process::exit(2);
    });

    // Set db path and validate it
    let db_path = args.database;
    validate::validate_db(&db_path).unwrap_or_else(|error| {
        eprintln!("Error: {error:?}");
        process::exit(3);
    });
}
