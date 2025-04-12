grepatch
========

[![grepatch](https://img.shields.io/crates/v/grepatch.svg)](https://crates.io/crates/grepatch)
[![Actions Status](https://github.com/sile/grepatch/workflows/CI/badge.svg)](https://github.com/sile/grepatch/actions)
![License](https://img.shields.io/crates/l/grepatch)


`grepatch` is a global text replacement tool that processes output from `git grep -n` and applies changes from command line operations like `sed` or manual edits via text editors to modify files directly.

Try it!: `$ git grep -n $QUERY | grepatch --edit`

Features
---------

- **Process grep output**: Works seamlessly with output from `git grep -n` and similar tools
- **Batch editing**: Apply text changes to multiple files in a single operation
- **Interactive editing**: Use your preferred text editor to manually review and modify changes before applying them

Installation
------------

```console
$ cargo install grepatch
$ grepatch -h
Usage: grepatch [OPTIONS]

Options:
      --version       Print version
  -h, --help          Print help ('--help' for full help, '-h' for summary)
  -e, --edit          Edit the input patch before applying it to allow manual modifications
      --editor <PATH> Specify which editor to use when '--edit' is enabled [env: EDITOR]
```

Examples
--------

Rename the `FilePatcher` struct to `DiffPatcher` throughout this repository.

```console
$ git clone https://github.com/sile/grepatch
$ cd grepatch/

// Show all occurrences of FilePatcher
$ git grep -n FilePatcher
src/main.rs:41:    let mut patcher = FilePatcher::open(patch.file_path).or_fail()?;
src/main.rs:48:            patcher = FilePatcher::open(patch.file_path).or_fail()?;
src/main.rs:58:struct FilePatcher {
src/main.rs:64:impl FilePatcher {

// Replace "Patch" with "Diff" in all occurrences (dry-run)
$ git grep -n FilePatcher | sed s/Patch/Diff/g
src/main.rs:41:    let mut patcher = FileDiffer::open(patch.file_path).or_fail()?;
src/main.rs:48:            patcher = FileDiffer::open(patch.file_path).or_fail()?;
src/main.rs:58:struct FileDiffer {
src/main.rs:64:impl FileDiffer {

// Apply the changes to the files
$ git grep -n FilePatcher | sed s/Patch/Diff/g | grepatch
src/main.rs: Applied 4 line patches

// Show the changes we made
$ git diff -U0
diff --git a/src/main.rs b/src/main.rs
index bad880a..2133718 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -41 +41 @@ fn run() -> orfail::Result<()> {
-    let mut patcher = FilePatcher::open(patch.file_path).or_fail()?;
+    let mut patcher = FileDiffer::open(patch.file_path).or_fail()?;
@@ -48 +48 @@ fn run() -> orfail::Result<()> {
-            patcher = FilePatcher::open(patch.file_path).or_fail()?;
+            patcher = FileDiffer::open(patch.file_path).or_fail()?;
@@ -58 +58 @@ fn run() -> orfail::Result<()> {
-struct FilePatcher {
+struct FileDiffer {
@@ -64 +64 @@ struct FilePatcher {
-impl FilePatcher {
+impl FileDiffer {
```
