<!--
The MIT License (MIT)

Copyright (c) 2025 Almaz Ilaletdinov <a.ilaletdinov@yandex.ru>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
OR OTHER DEALINGS IN THE SOFTWARE.
-->
# Lira

Lira is a CLI tool that filters linter errors based on a specified line range.

## Features

- Reads linter errors from stdin
- Filters errors within a given --line-range
- Supports various linters

## Installation

`cargo install --path .`

## Usage

### Basic usage

`linter_command file.py | lira --line-range 10-20`

### Example:

If a linter outputs:

```
file.rs:5: error: Something went wrong
file.rs:15: warning: Consider refactoring
file.rs:25: error: Unexpected behavior
```

Running:

`linter_command | lira --line-range 10-20`

Will output:

`file.rs:15: warning: Consider refactoring`

## Options

```
Usage: lira [OPTIONS]

Options:
  -l, --line-range <LINE_RANGE>  Filter errors within the specified line range
  -h, --help                     Print help
```
