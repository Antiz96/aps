//! Validate the required paths parameters

use anyhow::Context;
use std::collections::HashSet;
use std::fs;
use std::io::ErrorKind;
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

// Check if the pkgbases list exists and isn't empty, set it up for download otherwise
// Any other error (e.g. "permission denied") are returned.
pub fn validate_pkgbases(path: &Path) -> anyhow::Result<(HashSet<String>, bool)> {
    let file_content = match fs::read_to_string(path) {
        Ok(file_content) => file_content,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            return Ok((HashSet::new(), true));
        }
        Err(error) => {
            return Err(error).context("Failed to read AUR package metadata cache");
        }
    };

    let pkgbases = file_content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(String::from)
        .collect::<HashSet<_>>();

    let needs_download = pkgbases.is_empty();

    Ok((pkgbases, needs_download))
}
