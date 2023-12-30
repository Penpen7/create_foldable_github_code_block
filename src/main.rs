use arboard::Clipboard;
use serde::Serialize;
use std::io::Read;
use std::path::Path;
use tinytemplate::TinyTemplate;

#[derive(Serialize, Debug)]
struct Context {
    command: String,
    date: String,
    stdout: String,
    relative_path: String,
}

const TEMPLATE: &str = r"<details>
<summary>{command}の実行結果({relative_path}, {date})</summary>

```
{stdout}
```

</details>
";

fn main() {
    // get command line arguments
    let args: Vec<String> = std::env::args().collect();

    let (stdout, command) = if args.len() < 2 {
        // コマンドライン引数がなければ、stdinから入力を受け取る
        // 入力をとる
        let mut buf = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buf).unwrap();
        (buf, "stdin".to_string())
    } else {
        // コマンド実行
        let output = std::process::Command::new(&args[1])
            .args(&args[2..])
            .output()
            .expect("failed to execute process");
        // 標準出力を取得
        let stdout = String::from_utf8(output.stdout).unwrap();
        let title = args[1..].join(" ");
        (stdout, title)
    };

    let current_directory = {
        let current_directory = std::env::current_dir().unwrap();
        Path::new(&current_directory).to_owned()
    };

    let root_directory = {
        let root_directory = std::process::Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()
            .expect("failed to execute process")
            .stdout;
        let root_directory = String::from_utf8(root_directory).unwrap();
        Path::new(&root_directory.trim().to_string()).to_owned()
        // root_directory.trim().to_string()
    };

    // 相対パスを取得
    let relative_path = Path::new(&current_directory)
        .strip_prefix(&root_directory)
        .unwrap();

    let date_now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut tt = TinyTemplate::new();
    tt.add_template("template", TEMPLATE).unwrap();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    let context = Context {
        command,
        date: date_now,
        stdout: stdout.clone(),
        relative_path: relative_path.to_str().unwrap().to_string(),
    };

    let rendered = tt.render("template", &context).unwrap();

    Clipboard::new()
        .unwrap()
        .set_text(rendered.clone())
        .unwrap();
    // 最初の5行を表示
    let lines: Vec<&str> = stdout.split('\n').collect();
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
