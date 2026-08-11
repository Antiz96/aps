use anyhow::{Context, Result};
use std::str;

pub struct Match {
    pub package: String,
    pub path: String,
    pub line: usize,
    pub pattern: String,
}

pub fn scan_repo(repo: &gix::Repository, patterns: &[String]) -> Result<Vec<Match>> {
    let mut matches = Vec::new();

    for reference in repo
        .references()
        .context("Failed to access Git references")?
        .all()?
    {
        let reference = reference.map_err(anyhow::Error::msg)?;

        let name = reference.name().as_bstr();

        if !name.starts_with(b"refs/heads/") {
            continue;
        }
        
        let package = String::from_utf8_lossy(&name[b"refs/heads/".len()..]).into_owned();

        let Some(commit_id) = reference.try_id() else {
            continue;
        };

        let commit = repo
            .find_commit(commit_id)
            .with_context(|| format!("Failed to find commit for package {package}"))?;

        let tree = commit
            .tree()
            .with_context(|| format!("Failed to get tree for package {package}"))?;

        scan_tree(
            repo,
            &tree,
            &package,
            "",
            patterns,
            &mut matches,
        )?;
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
) -> Result<()> {
    for entry in tree.iter() {
        let entry = entry.context("Failed to read tree entry")?;

        let entry_name = entry.filename().to_string();

        let entry_path = if path.is_empty() {
            entry_name.clone()
        } else {
            format!("{path}/{entry_name}")
        };

        match entry.mode().kind() {
            gix::object::tree::EntryKind::Tree => {
                let subtree = repo
                    .find_tree(entry.object_id())
                    .with_context(|| format!("Failed to read tree {entry_path}"))?;

                scan_tree(
                    repo,
                    &subtree,
                    package,
                    &entry_path,
                    patterns,
                    matches,
                )?;
            }

            gix::object::tree::EntryKind::Blob => {
                let blob = repo
                    .find_blob(entry.object_id())
                    .with_context(|| format!("Failed to read file {entry_path}"))?;

                let Ok(contents) = str::from_utf8(&blob.data) else {
                    // Ignore binary files for now.
                    continue;
                };

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

            _ => {
                // Ignore other Git tree entry types.
            }
        }
    }

    Ok(())
}
