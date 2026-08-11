//! Validate the required paths parameters

use anyhow::Context;
use std::fs::{self, OpenOptions};
use std::path::Path;

// Check if the repo dir exists, is readable and is a git repo
pub fn validate_repo(repo_path: &Path) -> anyhow::Result<gix::Repository> {
    fs::read_dir(repo_path)
        .with_context(|| format!("Failed to access the {} repository", repo_path.display()))?;

    let repo = gix::open(repo_path)?;

    Ok(repo)
}

// Check if the patterns file exists, is readable and isn't empty
pub fn validate_patterns(patterns_path: &Path) -> anyhow::Result<Vec<String>> {
    let file_content = fs::read_to_string(patterns_path).with_context(|| {
        format!(
            "Failed to access the {} patterns file",
            patterns_path.display()
        )
    })?;

    let patterns = file_content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(String::from)
        .collect::<Vec<_>>();

    anyhow::ensure!(
        !patterns.is_empty(),
        "The {} patterns file is empty or contains no valid patterns",
        patterns_path.display()
    );

    Ok(patterns)
}

// Check if the db file exists and is readable
// Try to create it if it doesn't exist
pub fn validate_db(db_path: &Path) -> anyhow::Result<()> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(db_path)
        .with_context(|| {
            format!(
                "Failed to access or create the {} database file",
                db_path.display()
            )
        })?;

    Ok(())
}
