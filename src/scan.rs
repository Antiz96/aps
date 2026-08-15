//! Scan repository for matching patterns

use aho_corasick::AhoCorasick;
use anyhow::Context;
use gix::Repository;
use rayon::prelude::*;
use regex::RegexSet;
use std::cell::RefCell;
use std::collections::HashSet;
use std::str;

// Match fields
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Match {
    pub package: String,
    pub path: String,
    pub line: usize,
    pub pattern: usize,
    pub context: Vec<(usize, String)>,
}

// Match literal patterns with Aho-Corasick and wildcard (`*`) patterns with regex
struct PatternMatcher {
    literal: AhoCorasick,
    literal_pattern_ids: Vec<usize>,
    wildcard: RegexSet,
    wildcard_pattern_ids: Vec<usize>,
}

// Helper to get all branch refs (each pkgbase has its own branch refs)
fn get_branch_refs(
    repo: &gix::Repository,
    pkgbases: &HashSet<String>,
) -> anyhow::Result<Vec<String>> {
    Ok(repo
        .references()
        .context("Failed to access Git references")?
        .local_branches()?
        .filter_map(|reference| {
            let reference = reference.map_or_else(
                |error| {
                    eprintln!("Warning: Failed to access Git reference: {error}");
                    None
                },
                Some,
            )?;

            // Convert to byte-string to match gix expectation
            let ref_name = reference.name().as_bstr();

            // Extract package name from the ref (one branch per pkg, so "refs/heads/<pkgname>")
            let package = &ref_name["refs/heads/".len()..].to_string();

            // Exclude packages that have been deleted (or actually unreferenced) from the AUR
            if !pkgbases.contains(package) {
                return None;
            }

            Some(ref_name.to_string())
        })
        .collect())
}

// Helper to turn wildcards (`*`) into regex pattern (`.*`)
fn wildcard_to_regex(pattern: &str) -> String {
    pattern
        .split('*')
        .map(regex::escape)
        .collect::<Vec<_>>()
        .join(".*")
}

// Helper to build context for the matches
fn build_context(lines: &[&str], line_index: usize) -> Vec<(usize, String)> {
    let start = line_index.saturating_sub(2);
    let end = (line_index + 3).min(lines.len());

    lines[start..end]
        .iter()
        .enumerate()
        .map(|(index, content)| (start + index + 1, (*content).to_string()))
        .collect()
}

pub fn scan_repo(
    repo: &gix::Repository,
    repo_path: &std::path::Path,
    pkgbases: &HashSet<String>,
    patterns: &[String],
) -> anyhow::Result<Vec<Match>> {
    // Split string sets between literal and regex sets
    let literal_patterns: Vec<_> = patterns
        .iter()
        .enumerate()
        .filter(|(_, pattern)| !pattern.contains('*'))
        .collect();

    let wildcard_patterns: Vec<_> = patterns
        .iter()
        .enumerate()
        .filter(|(_, pattern)| pattern.contains('*'))
        .collect();

    let matcher = PatternMatcher {
        literal: AhoCorasick::new(literal_patterns.iter().map(|(_, pattern)| pattern))?,
        literal_pattern_ids: literal_patterns.iter().map(|(index, _)| *index).collect(),
        wildcard: RegexSet::new(
            wildcard_patterns
                .iter()
                .map(|(_, pattern)| wildcard_to_regex(pattern)),
        )?,
        wildcard_pattern_ids: wildcard_patterns.iter().map(|(index, _)| *index).collect(),
    };

    // one Repository instance for every thread
    thread_local! {
        static REPO: RefCell<Option<Repository>> = const {
            RefCell::new(None)
        };
    }

    let names: Vec<String> =
        get_branch_refs(repo, pkgbases).context("Failed to get valid reference names")?;

    let matches = names
        .into_par_iter()
        .filter_map(|ref_name| {
            REPO.with(|repo| {
                let mut repo = repo.borrow_mut();

                if repo.is_none() {
                    // The expectation should be safe: we already validated the repo with
                    // validate::validate_repo at that point and we don't expect it to be
                    // altered in a way that it cannot be opened anymore in the mean time
                    *repo = Some(gix::open(repo_path).expect("Failed to open repository"));
                }

                let repo = repo.as_ref().unwrap();

                // Resolve reference
                // Silencing failure should be safe here, refs were already validated by get_branch_refs()
                let reference = repo.find_reference(&ref_name).ok()?;
                let id = reference.try_id()?;

                // Load commit
                // Skip the branch if it cannot be loaded (but warn about it)
                let commit = repo.find_commit(id).map_or_else(
                    |error| {
                        eprintln!("Warning: Failed to load commit for {ref_name}: {error}");
                        None
                    },
                    Some,
                )?;

                // Load tree from commit
                // Silencing failure should be safe here, the most probable cause is a missing
                // commit (resulting in a tree that cannot be loaded), which we already warned about above if needed
                let tree = commit.tree().ok()?;

                let package = &ref_name["refs/heads/".len()..];

                // Scan file tree for matching patterns
                let mut matches = Vec::new();
                // Silencing failure should be safe here, the most probable cause is a missing
                // commit (resulting in a tree that cannot be loaded), which we already warned about above if needed
                scan_tree(repo, &tree, package, "", &matcher, &mut matches).ok()?;
                Some(matches)
            })
        })
        .flatten()
        .collect();

    Ok(matches)
}

fn scan_tree(
    repo: &gix::Repository,
    tree: &gix::Tree,
    package: &str,
    path: &str,
    matcher: &PatternMatcher,
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

                scan_tree(repo, &subtree, package, &entry_path, matcher, matches)?;
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

                // Vector for the context
                let lines: Vec<&str> = contents.lines().collect();

                // Iterate over lines and test every patterns, record eventual matches
                for (line_index, line) in lines.iter().enumerate() {
                    let line_number = line_index + 1;

                    // Literal string searches
                    for mat in matcher.literal.find_iter(line) {
                        // Build context
                        let context = build_context(&lines, line_index);

                        // Push results fields
                        matches.push(Match {
                            package: package.to_string(),
                            path: entry_path.clone(),
                            line: line_number,
                            pattern: matcher.literal_pattern_ids[mat.pattern().as_usize()],
                            context,
                        });
                    }

                    // Regex string searches (wildcard expansion)
                    for wildcard_pattern in matcher.wildcard.matches(line) {
                        // Build context
                        let context = build_context(&lines, line_index);

                        // Push results fields
                        matches.push(Match {
                            package: package.to_string(),
                            path: entry_path.clone(),
                            line: line_number,
                            pattern: matcher.wildcard_pattern_ids[wildcard_pattern],
                            context,
                        });
                    }
                }
            }

            // Ignore other eventual git tree entry types (e.g. submodules, which *shouldn't* be
            // present in AUR repo as well but hey...)
            _ => {}
        }
    }

    Ok(())
}
