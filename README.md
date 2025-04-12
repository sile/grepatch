grepatch
========

[![grepatch](https://img.shields.io/crates/v/grepatch.svg)](https://crates.io/crates/grepatch)
[![Actions Status](https://github.com/sile/grepatch/workflows/CI/badge.svg)](https://github.com/sile/grepatch/actions)
![License](https://img.shields.io/crates/l/grepatch)

`grepatch` is a command-line patch tool that processes grep output to apply text replacements to files.

Try it!
- With manual edits: `$ git grep -n $QUERY | grepatch --edit`
- With `sed`: `$ git grep -n $QUERY | sed s/$FROM/$TO/g | grepatch`

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
A command-line patch tool that processes grep output to apply text replacements to files

Usage: grepatch [OPTIONS]

Options:
      --version       Print version
  -h, --help          Print help ('--help' for full help, '-h' for summary)
  -e, --edit          Edit the input patch before applying it to allow manual modifications
      --editor <PATH> Specify which editor to use when '--edit' is enabled [env: EDITOR]
```
