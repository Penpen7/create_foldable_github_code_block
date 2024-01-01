mod clipboard;
mod cmd;
mod git;
mod markdown;

fn display_code_summary(code: &str) {
    let lines: Vec<&str> = code.split('\n').collect();
    for line in lines.iter().take(5) {
        println!("{}", line);
    }
    // 5行以上あれば、...を表示
    if lines.len() > 5 {
        println!("...");
    }
    // 最後の5行を表示
    for line in lines.iter().rev().take(5).rev() {
        println!("{}", line);
    }
}

pub fn run(command: String, code: String) -> anyhow::Result<()> {
    let current_directory = cmd::get_current_directory()
        .map_err(|e| anyhow::anyhow!("Failed to get current directory. {}", e))?;
    let root_directory = git::get_root_directory().map_err(|e| {
        anyhow::anyhow!(
            "Failed to get root directory. Please check if you are in a git repository or install git {}",
            e
        )
    })?;
    let commit_hash = git::get_commit_hash().map_err(|e| {
        anyhow::anyhow!(
            "Failed to get commit hash. Please check if you are in a git repository or install git. {}",
            e
        )
    })?;
    let git_diff = git::get_gitdiff().map_err(|e| {
        anyhow::anyhow!(
            "Failed to get git diff. Please check if you are in a git repository or install git.  {}",
            e
        )
    })?;
    let relative_path = current_directory.strip_prefix(&root_directory).unwrap();

    let date_now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let rendered = markdown::create_markdown(
        &command,
        &code,
        relative_path.to_str().ok_or(anyhow::anyhow!(""))?,
        &commit_hash,
        &git_diff,
        &date_now,
    )?;

    clipboard::set_clipboard(&rendered)?;

    display_code_summary(&code);

    Ok(())
}
