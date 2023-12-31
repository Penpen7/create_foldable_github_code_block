# create_foldable_github_code_block
## Description
This is a basic Rust program designed to take command results as an argument and generate a string suitable for creating a collapsible code block in GitHub Markdown, which can then be copied to your clipboard.

## Usage
```
$ create_foldable_github_code_block ls -l
$ ls -l | create_foldable_github_code_block
```
then paste the output into your issue comment or pull request description.

## Installation
```
$ cargo install --git
```
