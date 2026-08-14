//! Get AUR pkgbases
//! https://aur.archlinux.org/packages-meta-v1.json.gz

use anyhow::Context;
use flate2::read::GzDecoder;
use reqwest::blocking::get;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::io::{ErrorKind, Read};

#[derive(Debug, Deserialize)]
struct PackageMeta {
    #[serde(rename = "PackageBase")]
    package_base: String,
}

// Download the current AUR package metadata and extract the list of pkgbases
pub fn download_pkgbases() -> anyhow::Result<()> {
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

    // Write pkgbases list to disk
    fs::write("pkgbases.txt", pkgbases.join("\n"))
        .context("Failed to save AUR package metadata cache")?;

    Ok(())
}

// Get the cached AUR pkgbases, or download them first if the cache is not available
pub fn load_pkgbases() -> anyhow::Result<HashSet<String>> {
    let content = match fs::read_to_string("pkgbases.txt") {
        Ok(content) => content,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            download_pkgbases()?;
            fs::read_to_string("pkgbases.txt")
                .context("Failed to read AUR package metadata cache")?
        }
        Err(error) => {
            return Err(error).context("Failed to read AUR package metadata cache");
        }
    };

    Ok(content.lines().map(String::from).collect())
}
