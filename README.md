grepatch
========

Try it!: `$ git grep -n $QUERY | sed s/$FROM/$TO/g | grepatch`

`grepatch` is a global text replacement tool that processes output from `git grep -n` and applies changes from command line operations like `sed` or or manual edits via text editors to modify files directly.

Features
---------

- Process output from `git grep` or similar tools
- Apply text substitutions to multiple files in one operation
- Selectively modify specific lines in files

Installation
------------

```console
$ cargo install grepatch
```

Examples
--------

Change repository name.

```console
// TODO: comment
$ git clone https://github.com/sile/grepatch
$ cd grepatch/

// TODO: comment
$ git grep -n FilePatcher
src/main.rs:41:    let mut patcher = FilePatcher::open(patch.file_path).or_fail()?;
src/main.rs:48:            patcher = FilePatcher::open(patch.file_path).or_fail()?;
src/main.rs:58:struct FilePatcher {
src/main.rs:64:impl FilePatcher {

// TODO: comment
$ git grep -n FilePatcher | sed s/Patch/Diff/g
src/main.rs:41:    let mut patcher = FileDiffer::open(patch.file_path).or_fail()?;
src/main.rs:48:            patcher = FileDiffer::open(patch.file_path).or_fail()?;
src/main.rs:58:struct FileDiffer {
src/main.rs:64:impl FileDiffer {

// TODO: comment
$ git grep -n FilePatcher | sed s/Patch/Diff/g | grepatch
src/main.rs: Applied 4 line patches

// TODO: comment
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
