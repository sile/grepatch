grepatch
========

[![grepatch](https://img.shields.io/crates/v/grepatch.svg)](https://crates.io/crates/grepatch)
[![Actions Status](https://github.com/sile/grepatch/workflows/CI/badge.svg)](https://github.com/sile/grepatch/actions)
![License](https://img.shields.io/crates/l/grepatch)

`grepatch` is a command-line patch tool that processes grep output to apply text replacements to files.

Quick Start
-----------

```console
// Replace text using sed
$ git grep -n "old_text" | sed 's/old_text/new_text/g' | grepatch

// Edit matches manually in your preferred editor
$ git grep -n "function_name" | grepatch --edit
```

Overview
---------

`grepatch` transforms the grep-formatted output (with line numbers) into file modifications. It's perfect for making consistent changes across multiple files in a codebase.

Features
---------

- **Process grep output**: Works seamlessly with the output format from `grep -n`, `git grep -n` and similar tools
- **Batch editing**: Apply text changes to multiple files in a single operation
- **Interactive editing**: Use your preferred text editor to manually review and modify changes before applying them
- **Minimal dependencies**: Simple and focused tool that integrates with standard Unix tools

Installation
------------

```console
$ cargo install grepatch
$ grepatch -h
A command-line patch tool that processes grep output to apply text replacements to files

Usage: grepatch [OPTIONS]

Options:
      --version       Print version
  -h, --help          Print help ('--help' for full help, '-h' for summary)
  -e, --edit          Edit the input patch before applying it to allow manual modifications
      --editor <PATH> Specify which editor to use when '--edit' is enabled [env: EDITOR]
```

How It Works
------------

1. Takes input in the format: `file_path:line_number:content`
2. Identifies the files and line numbers to modify
3. Replaces the original lines with the new content
4. Writes the changes back to the files

