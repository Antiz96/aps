//! Scan repository for matching patterns

use anyhow::Context;
use std::str;

// Match fields
pub struct Match {
    pub package: String,
    pub path: String,
    pub line: usize,
    pub pattern: String,
}

pub fn scan_repo(repo: &gix::Repository, patterns: &[String]) -> anyhow::Result<Vec<Match>> {
    // Match vector
    let mut matches = Vec::new();

    // Get all git refs
    for reference in repo
        .references()
        .context("Failed to access Git references")?
        .all()?
    {
        // Convert gix error into anyhow type
        let reference = reference.map_err(anyhow::Error::msg)?;

        // Convert to byte-string to match gix expectation
        let ref_name = reference.name().as_bstr();

        // Skip potential refs that aren't branches / heads (e.g. "refs/tags/...")
        // The AUR repo has one branch per package, no other refs type, so this check is technically
        // not needed but it's cheap and future proof
        if !ref_name.starts_with(b"refs/heads/") {
            continue;
        }

        // Extract package name from the ref (one branch per pkg, so "refs/heads/<pkgname>")
        let package = String::from_utf8_lossy(&ref_name[b"refs/heads/".len()..]).into_owned();

        // Skip branches that have no commits
        // Here again, there's little to no chance that this check is needed but it's cheap and
        // future proof
        let Some(commit_id) = reference.try_id() else {
            continue;
        };

        // Load commit object
        let commit = repo
            .find_commit(commit_id)
            .with_context(|| format!("Failed to find commit for package {package}"))?;

        // Load tree from commit
        let tree = commit
            .tree()
            .with_context(|| format!("Failed to get tree for package {package}"))?;

        // Scan file tree for matching patterns
        scan_tree(repo, &tree, &package, "", patterns, &mut matches)?;
    }

    Ok(matches)
}

fn scan_tree(
    repo: &gix::Repository,
    tree: &gix::Tree,
    package: &str,
    path: &str,
    patterns: &[String],
    matches: &mut Vec<Match>,
) -> anyhow::Result<()> {
    // Iterate over the file tree
    for entry in tree.iter() {
        let entry = entry.context("Failed to read tree entry")?;

        // Extract and convert filename to string
        let entry_name = entry.filename().to_string();

        // Construct file path
        let entry_path = if path.is_empty() {
            entry_name.clone()
        } else {
            format!("{path}/{entry_name}")
        };

        // Determine git object kind (tree / directory or blob / file)
        match entry.mode().kind() {
            // If git object is a tree / directory, then recusirvely browse it (by recalling the
            // scan_tree function) until we reach blobs / files
            gix::object::tree::EntryKind::Tree => {
                let subtree = repo
                    .find_tree(entry.object_id())
                    .with_context(|| format!("Failed to read tree {entry_path}"))?;

                scan_tree(repo, &subtree, package, &entry_path, patterns, matches)?;
            }

            // If git object is a blob / file, load its content
            gix::object::tree::EntryKind::Blob => {
                let blob = repo
                    .find_blob(entry.object_id())
                    .with_context(|| format!("Failed to read file {entry_path}"))?;

                // Ignore files which content isn't UTF-8 (e.g. binary files, which *shouldn't* be present in an AUR repo but hey...)
                let Ok(contents) = str::from_utf8(&blob.data) else {
                    continue;
                };

                // Iterate over lines and test every patterns, record eventual matches
                for (line_number, line) in contents.lines().enumerate() {
                    for pattern in patterns {
                        if line.contains(pattern) {
                            matches.push(Match {
                                package: package.to_string(),
                                path: entry_path.clone(),
                                line: line_number + 1,
                                pattern: pattern.clone(),
                            });
                        }
                    }
                }
            }

            // Ignore other eventual git tree entry types (e.g. submodules, which *shoudln't* be
            // present in AUR repo as well but hey...)
            _ => {}
        }
    }

    Ok(())
}
