use anyhow::Result;
use std::path::PathBuf;

pub fn get_current_directory() -> Result<PathBuf> {
    Ok(std::env::current_dir()?)
}
