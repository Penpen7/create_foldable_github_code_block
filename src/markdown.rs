use anyhow::{Ok, Result};
use serde::Serialize;
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

pub fn create_markdown(
    command: &str,
    stdout: &str,
    relative_path: &str,
    commit_hash: &str,
    git_diff: &str,
    date_now: &str,
) -> Result<String> {
    let mut tt = TinyTemplate::new();
    tt.add_template("template", TEMPLATE).unwrap();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    let context = Context {
        command: command.to_owned(),
        date: date_now.to_owned(),
        stdout: stdout.to_owned(),
        relative_path: relative_path.to_owned(),
        commit_hash: commit_hash.to_owned(),
        git_diff: git_diff.to_owned(),
    };

    Ok(tt.render("template", &context)?)
}
