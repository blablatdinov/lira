// The MIT License (MIT)
//
// Copyright (c) 2025 Almaz Ilaletdinov <a.ilaletdinov@yandex.ru>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
// OR OTHER DEALINGS IN THE SOFTWARE.

use assert_cmd::Command;

#[test]
fn test_without_arguments() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    cmd.write_stdin("").assert().success().stdout("\n");
}

#[test]
fn test_with_line_range_argument() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    cmd.write_stdin("")
        .arg("--line-range")
        .arg("1-5")
        .assert()
        .success()
        .stdout("\n");
}

#[test]
fn test_with_multiple_lines_in_stdin() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    cmd.write_stdin("Line 1\nLine 2\nLine 3\n")
        .assert()
        .success()
        .stdout("Line 1\nLine 2\nLine 3\n");
}

#[test]
fn test_with_line_range_and_multiple_lines() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    cmd.write_stdin("Line 1\nLine 2\nLine 3\n")
        .arg("--line-range")
        .arg("2-3")
        .assert()
        .success()
        .stdout("\n");
}

#[test]
fn test_with_empty_stdin_and_line_range() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    cmd.write_stdin("")
        .arg("--line-range")
        .arg("10-20")
        .assert()
        .success()
        .stdout("\n");
}

#[test]
fn test_with_single_line_in_stdin() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    cmd.write_stdin("Single line\n")
        .assert()
        .success()
        .stdout("Single line\n");
}

#[test]
fn test_with_single_line_and_line_range() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    cmd.write_stdin("Single line\n")
        .arg("--line-range")
        .arg("1-1")
        .assert()
        .success()
        .stdout("\n");
}

#[test]
fn test_filter_linter_errors() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    let input = "file.rs:5: error: Something went wrong\nfile.rs:15: warning: Consider refactoring\nfile.rs:25: error: Unexpected behavior\n";
    cmd.write_stdin(input)
        .arg("--line-range")
        .arg("10-20")
        .assert()
        .success()
        .stdout("file.rs:15: warning: Consider refactoring\n");
}

#[test]
fn test_filter_linter_errors_no_range() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    let input = "file.rs:5: error: Something went wrong\nfile.rs:15: warning: Consider refactoring\nfile.rs:25: error: Unexpected behavior\n";
    cmd.write_stdin(input).assert().success().stdout(input);
}

#[test]
fn test_filter_linter_errors_edge_cases() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    let input = "file.rs:10: error: Edge case 1\nfile.rs:20: warning: Edge case 2\nfile.rs:30: error: Edge case 3\n";
    cmd.write_stdin(input)
        .arg("--line-range")
        .arg("10-20")
        .assert()
        .success()
        .stdout("file.rs:10: error: Edge case 1\nfile.rs:20: warning: Edge case 2\n");
}

#[test]
fn test_invalid_line_range_format() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    cmd.write_stdin("file.rs:5: error: Something went wrong\n")
        .arg("--line-range")
        .arg("invalid")
        .assert()
        .failure()
        .stderr("Error: Invalid line range format: invalid. Expected format: start-end\n");
}

#[test]
fn test_invalid_line_range_numbers() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    cmd.write_stdin("file.rs:5: error: Something went wrong\n")
        .arg("--line-range")
        .arg("abc-def")
        .assert()
        .failure()
        .stderr("Error: Invalid start line number: abc\n");
}

#[test]
fn test_line_range_start_greater_than_end() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    cmd.write_stdin("file.rs:5: error: Something went wrong\n")
        .arg("--line-range")
        .arg("20-10")
        .assert()
        .failure()
        .stderr("Error: Start line (20) cannot be greater than end line (10)\n");
}

#[test]
fn test_mixed_linter_and_non_linter_output() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    let input = "Running linter...\nfile.rs:5: error: Something went wrong\nfile.rs:15: warning: Consider refactoring\nLinter finished.\n";
    cmd.write_stdin(input)
        .arg("--line-range")
        .arg("10-20")
        .assert()
        .success()
        .stdout("file.rs:15: warning: Consider refactoring\n");
}

#[test]
fn test_different_file_formats() {
    let mut cmd = Command::cargo_bin("lira").unwrap();
    let input = "src/main.rs:5: error: Something went wrong\nlib.rs:15: warning: Consider refactoring\ntest.py:25: error: Unexpected behavior\n";
    cmd.write_stdin(input)
        .arg("--line-range")
        .arg("10-20")
        .assert()
        .success()
        .stdout("lib.rs:15: warning: Consider refactoring\n");
}
