use arboard::Clipboard;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
use tinytemplate::TinyTemplate;

#[derive(Serialize, Debug)]
struct Context {
    command: String,
    date: String,
    stdout: String,
    relative_path: String,
    commit_hash: String,
    git_diff: String,
}

const TEMPLATE: &str = r"<details>
<summary>{command}の実行結果({relative_path}, {date}, {commit_hash})</summary>

# 実行結果
````
{stdout}
````

# git diff({commit_hash}からの差分)
````diff
{git_diff}
````

</details>
";

fn get_root_directory() -> PathBuf {
    let root_directory = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .expect("failed to execute process")
        .stdout;
    let root_directory = String::from_utf8(root_directory).unwrap();
    Path::new(&root_directory.trim().to_string()).to_owned()
}

fn get_current_directory() -> PathBuf {
    let current_directory = std::env::current_dir().unwrap();
    Path::new(&current_directory).to_owned()
}

fn get_commit_hash() -> String {
    let commit_hash = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("failed to execute process")
        .stdout;
    let commit_hash = String::from_utf8(commit_hash).unwrap();
    commit_hash.trim().to_string().chars().take(7).collect()
}

fn get_gitdiff() -> String {
    let git_diff = std::process::Command::new("git")
        .arg("diff")
        .arg("--no-color")
        .output()
        .expect("failed to execute process")
        .stdout;
    String::from_utf8(git_diff).unwrap()
}

fn create_markdown(
    command: String,
    stdout: String,
    relative_path: String,
    commit_hash: String,
    git_diff: String,
    date_now: String,
) -> String {
    let mut tt = TinyTemplate::new();
    tt.add_template("template", TEMPLATE).unwrap();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    let context = Context {
        command,
        date: date_now,
        stdout: stdout.clone(),
        relative_path,
        commit_hash,
        git_diff,
    };

    tt.render("template", &context).unwrap()
}

fn set_clipboard(text: &str) {
    Clipboard::new().unwrap().set_text(text).unwrap();
}

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

fn get_now_date() -> String {
    let date_now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    date_now
}

pub fn run(command: String, code: String) {
    let current_directory = get_current_directory();
    let root_directory = get_root_directory();
    let commit_hash = get_commit_hash();
    let git_diff = get_gitdiff();
    let relative_path = Path::new(&current_directory)
        .strip_prefix(&root_directory)
        .unwrap();

    let date_now = get_now_date();

    let rendered = create_markdown(
        command,
        code.clone(),
        relative_path.to_str().unwrap().to_string(),
        commit_hash.clone(),
        git_diff.clone(),
        date_now.clone(),
    );

    set_clipboard(&rendered);

    display_code_summary(&code);
}
