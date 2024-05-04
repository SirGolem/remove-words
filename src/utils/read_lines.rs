use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

/// Reads a file and returns an iterator over its lines
///
/// # Arguments
///
/// * `path` - Path to the file
///
/// # Errors
///
/// Returns an error if opening the file fails
pub fn read_lines(path: &Path) -> Result<Lines<BufReader<File>>> {
    let file = File::open(path).with_context(|| "Failed to open file")?;
    Ok(BufReader::new(file).lines())
}
