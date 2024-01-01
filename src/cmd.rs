use anyhow::Result;
use std::path::PathBuf;

pub fn get_current_directory() -> Result<PathBuf> {
    Ok(std::env::current_dir()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_directory() {
        let current_directory = get_current_directory().unwrap();
        assert!(current_directory.exists());
    }
}
