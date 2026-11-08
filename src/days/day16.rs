//! # Day 16 — Panic vs `Result`
//!
//! Two kinds of failure:
//!
//! - **Bug / unrecoverable** — `panic!`, out-of-bounds index, `unwrap` on
//!   `None`. Unwinds the stack (or aborts), like an uncaught exception. Use it
//!   when continuing would be wrong.
//! - **Expected failure** — file missing, bad user input, network down. Model
//!   it with `Result<T, E>` and let the caller decide.
//!
//! `main` can return a `Result`:
//!
//! ```text
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let text = std::fs::read_to_string("data.txt")?;
//!     println!("{text}");
//!     Ok(())
//! }
//! ```
//!
//! For applications, the `anyhow` crate gives you one ergonomic error type;
//! for libraries, `thiserror` generates custom error enums. The standard
//! library alone is enough for this course.
//!
//! **Exercise:** return the average, or an explanatory error for an empty slice.

pub fn average(xs: &[i64]) -> Result<f64, String> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn averages_numbers() {
        assert_eq!(average(&[2, 4]), Ok(3.0));
    }

    #[test]
    fn averages_one_number() {
        assert_eq!(average(&[7]), Ok(7.0));
    }

    #[test]
    fn empty_is_an_error_not_a_panic() {
        let error = average(&[]).unwrap_err();
        assert!(error.contains("empty"), "unexpected error: {error}");
    }
}
