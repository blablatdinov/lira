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

use clap::Parser;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(name = "lira")]
struct Cli {
    #[arg(short, long, help = "Filter errors within the specified line range")]
    line_range: Option<String>,
}

#[derive(Debug)]
struct LineRange {
    start: usize,
    end: usize,
}

impl LineRange {
    fn parse(range_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid line range format: {}. Expected format: start-end",
                range_str
            ));
        }
        let start = parts[0]
            .parse::<usize>()
            .map_err(|_| format!("Invalid start line number: {}", parts[0]))?;
        let end = parts[1]
            .parse::<usize>()
            .map_err(|_| format!("Invalid end line number: {}", parts[1]))?;
        if start > end {
            return Err(format!(
                "Start line ({}) cannot be greater than end line ({})",
                start, end
            ));
        }
        Ok(LineRange { start, end })
    }
    fn contains(&self, line: usize) -> bool {
        line >= self.start && line <= self.end
    }
}

#[derive(Debug)]
struct LinterError {
    line: usize,
}

impl LinterError {
    fn parse(line: &str) -> Option<Self> {
        let trimmed = line.trim();
        if trimmed.starts_with("-->") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let file_line = parts[1];
                let colon_parts: Vec<&str> = file_line.split(':').collect();
                if colon_parts.len() >= 2 {
                    let line_num = colon_parts[1].parse::<usize>().ok()?;

                    return Some(LinterError { line: line_num });
                }
            }
        }
        let colon_parts: Vec<&str> = trimmed.split(':').collect();
        if colon_parts.len() < 3 {
            return None;
        }
        let line_num = colon_parts[1].trim().parse::<usize>().ok()?;
        Some(LinterError { line: line_num })
    }
}

fn filter_errors(input: &str, line_range: Option<LineRange>) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut filtered_lines = Vec::new();
    for line in lines {
        if let Some(error) = LinterError::parse(line) {
            if let Some(ref range) = line_range {
                if range.contains(error.line) {
                    filtered_lines.push(line);
                }
            } else {
                filtered_lines.push(line);
            }
        } else if line_range.is_none() {
            filtered_lines.push(line);
        }
    }
    filtered_lines.join("\n")
}

fn main() {
    let args = Cli::parse();
    let line_range = if let Some(ref range_str) = args.line_range {
        match LineRange::parse(range_str) {
            Ok(range) => Some(range),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        None
    };
    let stdin = io::stdin();
    let handle = stdin.lock();
    let input: String = handle
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("\n");
    let result = filter_errors(&input, line_range);
    println!("{}", result);
}
