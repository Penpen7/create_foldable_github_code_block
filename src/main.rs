use std::io::Read;

fn main() {
    // get command line arguments
    let args: Vec<String> = std::env::args().collect();

    let (stdout, command) = if args.len() < 2 {
        // コマンドライン引数がなければ、stdinから入力を受け取る
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

    create_foldable_github_code_block::run(command, stdout).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    })
}
