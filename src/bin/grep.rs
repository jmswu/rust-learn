//! The capstone mini-grep.
//!
//! ```text
//! cargo run --bin grep -- [-i] <pattern> <path>
//! ```
//!
//! Exit codes: 0 = found, 1 = no match, 2 = usage or I/O error.

use std::process::ExitCode;

use rust_learn::days::{day29, day30};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    match run(&args) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(message) => {
            eprintln!("grep: {message}");
            ExitCode::from(2)
        }
    }
}

fn run(args: &[String]) -> Result<bool, String> {
    let args = day30::parse_args(args)?;
    let text =
        std::fs::read_to_string(&args.path).map_err(|error| format!("{}: {error}", args.path))?;

    let lines = day29::filter_lines(&text, &args.pattern, args.ignore_case);

    for line in &lines {
        println!("{line}");
    }

    Ok(!lines.is_empty())
}
