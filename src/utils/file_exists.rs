use anyhow::{anyhow, Result};
use std::{fs, io::ErrorKind, path::Path};

/// Checks that a file exists
///
/// # Arguments
///
/// * `path` - Path to the file
///
/// # Errors
///
/// Returns an error if metadata could not be obtained
pub fn file_exists(path: &Path) -> Result<bool> {
    let metadata_result = fs::metadata(path);
    match metadata_result {
        Ok(metadata) => Ok(metadata.is_file()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Ok(false),
            _ => Err(anyhow!(error)),
        },
    }
}
