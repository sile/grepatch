use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    num::NonZeroUsize,
    path::{Path, PathBuf},
    process::{Command, Stdio},
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

    let edit = noargs::flag("edit")
        .short('e')
        .doc("Edit the input patch before applying it to allow manual modifications")
        .take(&mut args)
        .is_present();
    let editor: PathBuf = noargs::opt("editor")
        .ty("PATH")
        .env("EDITOR")
        .doc("Specify which editor to use when '--edit' is enabled")
        .take(&mut args)
        .then(|a| a.value().parse())
        .or_else(|e| if edit { Err(e) } else { Ok(PathBuf::new()) })?;

    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    if edit {
        edit_and_run(editor).map_err(|e| e.message)?;
    } else {
        let stdin = std::io::stdin();
        run(stdin.lock()).map_err(|e| e.message)?;
    }

    Ok(())
}

fn edit_and_run(editor: PathBuf) -> orfail::Result<()> {
    let mut temp = tempfile::NamedTempFile::new().or_fail()?;
    std::io::copy(&mut std::io::stdin().lock(), &mut temp)
        .or_fail_with(|e| format!("failed to read input: {e}"))?;

    let tty_stdin = File::open("/dev/tty").map(Stdio::from);
    let status = Command::new(editor)
        .arg(temp.path())
        .stdin(tty_stdin.unwrap_or(Stdio::inherit()))
        .status()
        .or_fail_with(|e| format!("failed to execute editor: {e}"))?;
    status
        .success()
        .or_fail_with(|_| "editor aborted".to_owned())?;

    temp.seek(SeekFrom::Start(0)).or_fail()?;
    run(BufReader::new(temp)).or_fail()?;
    Ok(())
}

fn run<R: BufRead>(input: R) -> orfail::Result<()> {
    let mut lines = input.lines();
    let Some(first_line) = lines.next().transpose().or_fail()? else {
        return Ok(());
    };

    let patch = LinePatch::new(&first_line).or_fail()?;
    let mut patcher = FilePatcher::open(patch.file_path).or_fail()?;

    for line in std::iter::once(Ok(first_line)).chain(lines) {
        let line = line.or_fail()?;
        let patch = LinePatch::new(&line).or_fail()?;
        if patcher.path != patch.file_path {
            patcher.finish().or_fail()?;
            patcher = FilePatcher::open(patch.file_path).or_fail()?;
        }
        patcher.apply(&patch).or_fail()?;
    }
    patcher.finish().or_fail()?;

    Ok(())
}

#[derive(Debug)]
struct FilePatcher {
    path: PathBuf,
    lines: Vec<String>,
    applied_count: usize,
}

impl FilePatcher {
    fn open(path: &Path) -> orfail::Result<Self> {
        let mut this = Self {
            path: path.to_path_buf(),
            lines: Vec::new(),
            applied_count: 0,
        };
        this.read_lines().or_fail()?;
        Ok(this)
    }

    fn read_lines(&mut self) -> orfail::Result<()> {
        let file = File::open(&self.path)
            .or_fail_with(|e| format!("failed to open file {}: {e}", self.path.display()))?;
        let mut reader = BufReader::new(file);
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).or_fail()?;
            if bytes_read == 0 {
                break;
            }
            self.lines.push(line);
        }
        Ok(())
    }

    fn apply(&mut self, patch: &LinePatch) -> orfail::Result<()> {
        let line = self
            .lines
            .get_mut(patch.line_number.get() - 1)
            .or_fail_with(|_| {
                format!(
                    "too large line number: file={}, number={}",
                    patch.file_path.display(),
                    patch.line_number
                )
            })?;
        if line.trim_end_matches(['\r', '\n']) == patch.content.trim_end_matches(['\r', '\n']) {
            return Ok(());
        }

        let newlines = &line[line.trim_end_matches(['\r', '\n']).len()..];
        *line = format!("{}{newlines}", patch.content.trim_end_matches(['\r', '\n']));
        self.applied_count += 1;
        Ok(())
    }

    fn finish(self) -> orfail::Result<()> {
        let content = self.lines.join("");
        std::fs::write(&self.path, content)
            .or_fail_with(|e| format!("failed to write file {}: {e}", self.path.display()))?;
        println!(
            "{}: Applied {} line patches",
            self.path.display(),
            self.applied_count,
        );
        Ok(())
    }
}

// [FORMAT] FILE_PATH:LINE_NUMBER:NEW_LINE_CONTENT
#[derive(Debug)]
struct LinePatch<'a> {
    file_path: &'a Path,
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
            file_path: Path::new(file_path),
            line_number,
            content,
        })
    }
}
