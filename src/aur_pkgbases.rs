//! Get AUR pkgbases
//! https://aur.archlinux.org/packages-meta-v1.json.gz

use anyhow::Context;
use flate2::read::GzDecoder;
use reqwest::blocking::get;
use serde::Deserialize;
use std::collections::HashSet;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct PackageMeta {
    #[serde(rename = "PackageBase")]
    package_base: String,
}

// Download the current AUR package metadata and return the set of
// currently existing pkgbases
pub fn current_pkgbases() -> anyhow::Result<HashSet<String>> {
    // AUR packages metadata URL
    let url = String::from("https://aur.archlinux.org/packages-meta-v1.json.gz");

    // Download AUR packages metadata json file
    let response = get(&url)
        .context("Failed to download AUR package metadata")?
        .error_for_status()
        .context("AUR package metadata request failed")?;

    // Store compressed file content
    let compressed_content = response
        .bytes()
        .context("Failed to read AUR package metadata")?;

    // Decompresse and parse the json content
    let mut decoder = GzDecoder::new(&compressed_content[..]);
    let mut json_parser = String::new();

    decoder
        .read_to_string(&mut json_parser)
        .context("Failed to decompress AUR package metadata")?;

    let pkgbases: Vec<PackageMeta> =
        serde_json::from_str(&json_parser).context("Failed to parse AUR package metadata")?;

    // Extract pkgbases
    Ok(pkgbases
        .into_iter()
        .map(|pkgbase| pkgbase.package_base)
        .collect())
}
