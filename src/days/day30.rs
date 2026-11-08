//! # Day 30 — Capstone II: argument parsing and next steps
//!
//! Finish the CLI by parsing arguments without panicking:
//!
//! ```text
//! grep [-i] <pattern> <path>
//! ```
//!
//! `std::env::args()` includes the program name at index 0, so skip it.
//! Return `Result<Args, String>`; `main` prints the error to stderr and exits
//! with code 2. `ExitCode::SUCCESS` is 0, `ExitCode::from(1)` means “no match”,
//! and 2 means “usage or I/O error” — the same convention as real grep.
//!
//! **Exercise:** accept `-i`, reject unknown flags, and require exactly two
//! positional arguments.
//!
//! # Where to go next
//!
//! - Read *The Rust Programming Language* (<https://doc.rust-lang.org/book/>),
//!   especially chapters 4, 10, and 13 again now that you have context.
//! - Do `rustlings` for more bite-sized compiler exercises.
//! - Build something: a JSON tool with `serde`, a web service with `axum`, or
//!   a CLI with `clap`.
//! - Learn `async`/`await` + `tokio` when you need many concurrent I/O tasks.
//! - For C/C++ interop: `unsafe`, `extern "C"`, and `bindgen`.
//! - Keep `cargo fmt` and `cargo clippy` in your loop.

#[derive(Debug, PartialEq)]
pub struct Args {
    pub pattern: String,
    pub path: String,
    pub ignore_case: bool,
}

pub fn parse_args(args: &[String]) -> Result<Args, String> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| item.to_string()).collect()
    }

    #[test]
    fn parses_two_positionals() {
        assert_eq!(
            parse_args(&args(&["grep", "needle", "file.txt"])),
            Ok(Args {
                pattern: "needle".to_string(),
                path: "file.txt".to_string(),
                ignore_case: false,
            })
        );
    }

    #[test]
    fn parses_the_i_flag() {
        assert_eq!(
            parse_args(&args(&["grep", "-i", "Rust", "notes.md"])),
            Ok(Args {
                pattern: "Rust".to_string(),
                path: "notes.md".to_string(),
                ignore_case: true,
            })
        );
    }

    #[test]
    fn rejects_unknown_flags() {
        let error = parse_args(&args(&["grep", "-x", "a", "b"])).unwrap_err();
        assert!(error.contains("unknown flag: -x"), "unexpected: {error}");
    }

    #[test]
    fn rejects_missing_positionals() {
        let error = parse_args(&args(&["grep", "only-pattern"])).unwrap_err();
        assert!(error.contains("usage"), "unexpected: {error}");
    }

    #[test]
    fn rejects_extra_positionals() {
        let error = parse_args(&args(&["grep", "a", "b", "c"])).unwrap_err();
        assert!(error.contains("usage"), "unexpected: {error}");
    }
}
