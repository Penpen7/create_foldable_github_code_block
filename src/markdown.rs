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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_markdown() {
        let command = "ls -la";
        let stdout = "total 0
drwxr-xr-x  3 user  staff   96  7  1 00:00 .
drwxr-xr-x  5 user  staff  160  7  1 00:00 ..
drwxr-xr-x  3 user  staff   96  7  1 00:00 .git
";
        let relative_path = "test";
        let commit_hash = "1234567";
        let git_diff = "diff --git a/test b/test
new file mode 100644
index 0000000..e69de29
";
        let date_now = "2020-07-01 00:00:00";
        let markdown = create_markdown(
            command,
            stdout,
            relative_path,
            commit_hash,
            git_diff,
            date_now,
        )
        .unwrap();
        assert!(markdown.contains("ls -la"));
        assert!(markdown.contains("2020-07-01 00:00:00"));
        assert!(markdown.contains("1234567"));
        assert!(markdown.contains("test"));
        assert!(markdown.contains("diff --git"));
    }
}
