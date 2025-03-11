use clap::{Parser};

#[derive(Parser)]
#[command(name = "lira")]
struct Cli {
    #[arg(short, long, help = "Filter errors within the specified line range")]
    line_range: Option<String>,
}

pub fn run(args: Cli) -> String {
    match args.line_range {
        Some(line_range) => format!("Line range: {}", line_range),
        None => "Empty line range".to_string()
    }
}

fn main() {
    let args = Cli::parse();
    println!("{}", run(args));
}
