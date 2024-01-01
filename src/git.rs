use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn get_root_directory() -> Result<PathBuf> {
    let root_directory = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?
        .stdout;
    let root_directory = String::from_utf8(root_directory)?;
    Ok(Path::new(&root_directory.trim().to_string()).to_owned())
}

pub fn get_commit_hash() -> Result<String> {
    let commit_hash = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()?
        .stdout;
    let commit_hash = String::from_utf8(commit_hash)?;
    Ok(commit_hash.trim().to_string().chars().take(7).collect())
}

pub fn get_gitdiff() -> Result<String> {
    let git_diff = std::process::Command::new("git")
        .arg("diff")
        .arg("--no-color")
        .output()?
        .stdout;
    Ok(String::from_utf8(git_diff)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_root_directory() {
        let root_directory = get_root_directory().unwrap();
        assert!(root_directory.exists());
    }

    #[test]
    fn test_get_commit_hash() {
        let commit_hash = get_commit_hash().unwrap();
        assert_eq!(commit_hash.len(), 7);
    }

    #[test]
    fn test_get_gitdiff() {
        let git_diff = get_gitdiff().unwrap();
        assert!(git_diff.contains("diff --git"));
    }
}
