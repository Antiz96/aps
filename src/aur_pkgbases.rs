//! Get AUR pkgbases
//! https://aur.archlinux.org/packages-meta-v1.json.gz

use anyhow::Context;
use flate2::read::GzDecoder;
use reqwest::blocking::get;
use serde::Deserialize;
use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Debug, Deserialize)]
struct PackageMeta {
    #[serde(rename = "PackageBase")]
    package_base: String,
}

fn save_cache(pkgbases: &[String]) -> io::Result<()> {
    let cache_file_path = "pkgbases.txt";
    std::fs::write(cache_file_path, pkgbases.join("\n"))
}

fn load_cache() -> io::Result<HashSet<String>> {
    let cache_file_path = "pkgbases.txt";
    let content = std::fs::read_to_string(cache_file_path)?;
    Ok(content.lines().map(|line| line.to_string()).collect())
}

/// Download the current AUR package metadata and return the set of
/// currently existing pkgbases
pub fn download_pkgbases() -> anyhow::Result<HashSet<String>> {
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

    let pkgdata: Vec<PackageMeta> =
        serde_json::from_str(&json_parser).context("Failed to parse AUR package metadata")?;

    // Extract pkgbases
    let pkgbases: Vec<String> = pkgdata
        .into_iter()
        .map(|pkgbase| pkgbase.package_base)
        .collect();

    save_cache(&pkgbases).context("Failed to save AUR package metadata cache")?;

    Ok(pkgbases.into_iter().collect())
}

/// Get the cached AUR pkgbases, or download them if the cache is not available
pub fn get_cached_pkgbases() -> anyhow::Result<HashSet<String>> {
    match load_cache() {
        Ok(pkgbases) => Ok(pkgbases),
        Err(_) => download_pkgbases(),
    }
}
