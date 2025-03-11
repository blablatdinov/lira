# Onlivi

Onlivi is a CLI tool that filters linter errors based on a specified line range.

## Features

- Reads linter errors from stdin
- Filters errors within a given --line-range
- Supports various linters

## Installation

`cargo install --path .`

## Usage

### Basic usage

`linter_command file.py | onlivi --line-range 10-20`

### Example:

If a linter outputs:

```
file.rs:5: error: Something went wrong
file.rs:15: warning: Consider refactoring
file.rs:25: error: Unexpected behavior
```

Running:

`linter_command | onlivi --line-range 10-20`

Will output:

`file.rs:15: warning: Consider refactoring`

## Options

```
Usage: onlivi [OPTIONS]

Options:
  -l, --line-range <LINE_RANGE>  Filter errors within the specified line range
  -h, --help                     Print help
```
