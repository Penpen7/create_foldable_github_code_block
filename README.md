# create_foldable_github_code_block
## Description
This is a basic Rust program designed to take command results as an argument and generate a string suitable for creating a collapsible code block in GitHub Markdown, which can then be copied to your clipboard.

## Usage
```
$ create_foldable_github_code_block ls -l
$ ls -l | create_foldable_github_code_block
```
then paste the output into your issue comment or pull request description.
For example, the above command will generate the following output:

<details>
<summary>lsの実行結果(create_foldable_github_code_block, 2024-01-03 02:07:28, 35b2a0b)</summary>

# 実行結果
````
Cargo.lock
Cargo.toml
README.md
create_foldable_github_code_block-v0.1.0-aarch64-apple-darwin
src
target

````

# git diff(35b2a0bからの差分)
````diff
diff --git a/src/lib.rs b/src/lib.rs
index 43b3186..cb0d902 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -39,7 +39,13 @@ pub fn run(command: String, code: String) -> anyhow::Result<()> {
             e
         )
     })?;
-    let relative_path = current_directory.strip_prefix(&root_directory).unwrap();
+
+    // root_directoryより1つ上のディレクトリまでのPathBufを取得
+    let root_directory_parent = root_directory.parent().unwrap();
+
+    let relative_path = current_directory
+        .strip_prefix(root_directory_parent)
+        .unwrap();
 
     let date_now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
 
diff --git a/src/main.rs b/src/main.rs
index 82760d6..ef6483c 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -23,7 +23,7 @@ fn main() {
         (stdout, title)
     };
 
-    create_foldable_github_code_block::run(stdout, command).unwrap_or_else(|e| {
+    create_foldable_github_code_block::run(command, stdout).unwrap_or_else(|e| {
         eprintln!("Error: {}", e);
         std::process::exit(1);
     })

````

</details>

## Installation
- By cargo
  - You can install this program by running the following command:
```
$ cargo install --git https://github.com/Penpen7/create_foldable_github_code_block 
```
- By downloading
 - You can install this program by downloading a release, extracting it, and copying the binary to a directory in your PATH.

