//! # Day 15 — `Result` and the `?` operator
//!
//! Recoverable errors use `Result<T, E>`, another ordinary enum:
//!
//! ```text
//! enum Result<T, E> { Ok(T), Err(E) }
//! ```
//!
//! There are no exceptions. A function that can fail says so in its signature,
//! and the caller must handle it (or propagate it). `?` means “if this is `Err`,
//! return it early; otherwise unwrap the `Ok` value”. `?` also converts the
//! error via `From`, which is how one error type can absorb many causes.
//!
//! ```text
//! let n: i32 = text.parse()?;      // ParseIntError -> the function's error type
//! ```
//!
//! `unwrap()` and `expect("message")` are fine in tests and prototypes; they
//! panic on `Err`. `map_err` converts an error; `ok_or` turns `Option` into
//! `Result`.
//!
//! **Exercise:** parse `"3,4"` into `(3, 4)`. Trim whitespace and return a
//! descriptive `String` error for a missing comma or a bad number.

pub fn parse_pair(input: &str) -> Result<(i32, i32), String> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_pair() {
        assert_eq!(parse_pair("3,4"), Ok((3, 4)));
    }

    #[test]
    fn trims_whitespace_and_handles_negatives() {
        assert_eq!(parse_pair("-1, 2"), Ok((-1, 2)));
    }

    #[test]
    fn reports_a_missing_comma() {
        let error = parse_pair("3").unwrap_err();
        assert!(error.contains("comma"), "unexpected error: {error}");
    }

    #[test]
    fn reports_which_number_is_bad() {
        let left = parse_pair("a,4").unwrap_err();
        assert!(left.contains("left"), "unexpected error: {left}");

        let right = parse_pair("3,b").unwrap_err();
        assert!(right.contains("right"), "unexpected error: {right}");
    }
}
