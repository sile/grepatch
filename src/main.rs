use std::{io::BufRead, num::NonZeroUsize, path::Path};

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
    while let Some(line) = lines.next().transpose().or_fail()? {
        let patch = LinePatch::new(&line).or_fail()?;
    }
    Ok(())
}

#[derive(Debug)]
struct FilePatcher {
    //
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
