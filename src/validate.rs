//! Validate the required paths parameters

use std::fs::{self, File};
use std::io::{self, Error, ErrorKind};
use std::path::Path;

// Check if the repo dir exists and is readable
// Return an error otherwise
pub fn validate_repo(repo_path: &Path) -> io::Result<()> {
    fs::read_dir(repo_path)
        .map_err(|error| Error::new(error.kind(), format!("{} - {error}", repo_path.display())))?;

    Ok(())
}

// Check if the patterns file exists and is readable
// Return an error otherwise
pub fn validate_patterns(patterns_path: &Path) -> io::Result<()> {
    File::open(patterns_path).map_err(|error| {
        Error::new(
            error.kind(),
            format!("{} - {error}", patterns_path.display()),
        )
    })?;

    Ok(())
}

// Check if the db file exists and is readable
// Try to create it if it doesn't exist
// Return an error otherwise
pub fn validate_db(db_path: &Path) -> io::Result<()> {
    File::open(db_path)
        .or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(db_path)
            } else {
                Err(error)
            }
        })
        .map_err(|error| Error::new(error.kind(), format!("{} - {error}", db_path.display())))?;

    Ok(())
}
