//! Format and print scan results

use std::collections::BTreeMap;

use crate::scan;

// Helper to generate the matches grouping
fn group_matches<'a>(
    patterns: &'a [String],
    matches: &'a [scan::Match],
) -> BTreeMap<&'a str, BTreeMap<&'a str, Vec<&'a scan::Match>>> {
    // Group matches by pattern and package
    let mut grouped_matches: BTreeMap<&str, BTreeMap<&str, Vec<&scan::Match>>> = BTreeMap::new();

    // Add every pattern, even if it has no matches (to also show patterns with no occurrence found)
    for pattern in patterns {
        grouped_matches.entry(pattern).or_default();
    }

    // Add actual matches
    for matched in matches {
        grouped_matches
            .entry(patterns[matched.pattern].as_str())
            .or_default()
            .entry(&matched.package)
            .or_default()
            .push(matched);
    }

    grouped_matches
}

// Print results summary containing the number of occurrence(s) found for each pattern
pub fn summary_results(patterns: &[String], matches: &[scan::Match]) {
    // Populate grouped matches
    let grouped_matches = group_matches(patterns, matches);

    // Calculate longest pattern width (for column formatting)
    let pattern_width = grouped_matches
        .keys()
        .map(|pattern| pattern.len())
        .max()
        .unwrap_or(0);

    // Print results summary
    for (pattern, packages) in &grouped_matches {
        let count: usize = packages.values().map(|matches| matches.len()).sum();

        println!("{pattern:<pattern_width$} : {count} occurrence(s) found");
    }
}

// Print detailed results grouped by pattern and package, including context
pub fn detailed_results(patterns: &[String], matches: &[scan::Match]) {
    // Sort matches and deduplicate them
    // Multiple patterns can match the same line, but the detailed output only needs to show
    // each matching line and its context once
    let mut matches = matches.to_vec();

    matches.sort();

    matches.dedup_by(|a, b| a.package == b.package && a.path == b.path && a.line == b.line);

    // Populate grouped matches
    let grouped_matches = group_matches(patterns, &matches);

    // Print a message if there are no results
    if grouped_matches.is_empty() {
        println!("No results found.");
        return;
    }

    // Print the detailed results if there are some
    for (pattern, packages) in grouped_matches {
        for (package, matches) in packages {
            println!("{pattern}:");
            println!("  https://aur.archlinux.org/packages/{package}");

            for matched in matches {
                println!("    {}:{}", matched.path, matched.line);

                for (line, content) in &matched.context {
                    println!("      {line}: {content}");
                }
            }
            println!();
        }
    }
}
