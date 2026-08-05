//! Validate the require repo, patterns and db paths

// Return an error if the repo path does not exists, is not readable or if any other errors was
// encountered
pub fn validate_repo_path(repo_path: &Path) -> io::Result<()> {
    fs::read_dir(&repo_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            return Err(Error::other(format!("{} - {error}", repo_path.display())));
        } else if error.kind() == ErrorKind::PermissionDenied {
            return Err(Error::other(format!("{} - {error}", repo_path.display())));
        } else {
            return Err(Error::other(format!("{} - {error}", repo_path.display())));
        }
    });

    // Check if patterns path exists and is readable
    // Exit in error otherwise
    File::open(&patterns_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            eprintln!(
                "Error: Patterns file not found\n{} - {error}",
                patterns_path.display()
            );
            process::exit(1);
        } else if error.kind() == ErrorKind::PermissionDenied {
            eprintln!(
                "Error: Patterns file not readable\n{} - {error}",
                patterns_path.display()
            );
            process::exit(1);
        } else {
            eprintln!("Error:\n{} - {error}", patterns_path.display());
            process::exit(1);
        }
    });
