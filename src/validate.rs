//! Validate the required paths parameters

use anyhow::Context;
use std::fs;
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
