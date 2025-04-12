use std::{
    fs::File,
    io::BufRead,
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

use orfail::OrFail;

fn main() -> noargs::Result<()> {
    let mut args = noargs::raw_args();

    args.metadata_mut().app_name = env!("CARGO_PKG_NAME");
    args.metadata_mut().app_description = env!("CARGO_PKG_DESCRIPTION");

    if noargs::VERSION_FLAG.take(&mut args).is_present() {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    noargs::HELP_FLAG.take_help(&mut args);

    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    run().map_err(|e| e.message)?;

    Ok(())
}

fn run() -> orfail::Result<()> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let Some(first_line) = lines.next().transpose().or_fail()? else {
        return Ok(());
    };

    let patch = LinePatch::new(&first_line).or_fail()?;
    let mut patcher = FilePatcher::open(patch.file).or_fail()?;

    for line in std::iter::once(Ok(first_line)).chain(lines) {
        let line = line.or_fail()?;
        let patch = LinePatch::new(&line).or_fail()?;
        patcher.apply(&patch).or_fail()?;
    }
    Ok(())
}

#[derive(Debug)]
struct FilePatcher {
    path: PathBuf,
    file: File,
}

impl FilePatcher {
    fn open(path: &Path) -> orfail::Result<Self> {
        let file = File::open(path)
            .or_fail_with(|e| format!("failed to open file {}: {e}", path.display()))?;
        Ok(Self {
            path: path.to_path_buf(),
            file,
        })
    }

    fn apply(&mut self, patch: &LinePatch) -> orfail::Result<()> {
        todo!()
    }
}

// [FORMAT] FILE_PATH:LINE_NUMBER:NEW_LINE_CONTENT
#[derive(Debug)]
struct LinePatch<'a> {
    file: &'a Path,
    line_number: NonZeroUsize,
    content: &'a str,
}

impl<'a> LinePatch<'a> {
    pub fn new(line: &'a str) -> orfail::Result<Self> {
        let (file_path, line) = line
            .split_once(':')
            .or_fail_with(|_| format!("missing FILE_PATH part: line={line:}"))?;
        let (line_number, content) = line
            .split_once(':')
            .or_fail_with(|_| format!("missing LINE_NUMBER part: line={line:}"))?;
        let line_number = line_number
            .parse::<NonZeroUsize>()
            .or_fail_with(|e| format!("invalid line number: line={line:?}, reason={e}"))?;

        Ok(Self {
            file: Path::new(file_path),
            line_number,
            content,
        })
    }
}
