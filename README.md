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

```console
$ git grep -n LinePatch src/
src/main.rs:40:    let patch = LinePatch::new(&first_line).or_fail()?;
src/main.rs:45:        let patch = LinePatch::new(&line).or_fail()?;
src/main.rs:90:    fn apply(&mut self, patch: &LinePatch) -> orfail::Result<()> {
src/main.rs:126:struct LinePatch<'a> {
src/main.rs:132:impl<'a> LinePatch<'a> {

$ git grep -n LinePatch src/ | sed s/Patch/Diff/g | grepatch
src/main.rs: Applied 5 line patches

$ git diff
diff --git a/src/main.rs b/src/main.rs
index bad880a..6b1efdd 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -37,12 +37,12 @@ fn run() -> orfail::Result<()> {
         return Ok(());
     };
 
-    let patch = LinePatch::new(&first_line).or_fail()?;
+    let patch = LineDiff::new(&first_line).or_fail()?;
     let mut patcher = FilePatcher::open(patch.file_path).or_fail()?;
 
     for line in std::iter::once(Ok(first_line)).chain(lines) {
         let line = line.or_fail()?;
-        let patch = LinePatch::new(&line).or_fail()?;
+        let patch = LineDiff::new(&line).or_fail()?;
         if patcher.path != patch.file_path {
             patcher.finish().or_fail()?;
             patcher = FilePatcher::open(patch.file_path).or_fail()?;
@@ -87,7 +87,7 @@ impl FilePatcher {
         Ok(())
     }
 
-    fn apply(&mut self, patch: &LinePatch) -> orfail::Result<()> {
+    fn apply(&mut self, patch: &LineDiff) -> orfail::Result<()> {
         let line = self
             .lines
             .get_mut(patch.line_number.get() - 1)
@@ -123,13 +123,13 @@ impl FilePatcher {
 
 // [FORMAT] FILE_PATH:LINE_NUMBER:NEW_LINE_CONTENT
 #[derive(Debug)]
-struct LinePatch<'a> {
+struct LineDiff<'a> {
     file_path: &'a Path,
     line_number: NonZeroUsize,
     content: &'a str,
 }
 
-impl<'a> LinePatch<'a> {
+impl<'a> LineDiff<'a> {
     pub fn new(line: &'a str) -> orfail::Result<Self> {
         let (file_path, line) = line
             .split_once(':')
```
