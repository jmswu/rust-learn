//! # Day 27 — Testing, formatting, and lints
//!
//! Every day of this course is a test suite. The pattern:
//!
//! ```text
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[test]
//!     fn describes_the_expected_behavior() {
//!         assert_eq!(add(2, 3), 5);
//!     }
//! }
//! ```
//!
//! Useful test attributes:
//! - `#[should_panic]` — the test passes only if it panics
//! - `#[ignore]` — skipped unless you run `cargo test -- --ignored`
//! - `#[test]` on a function with no arguments and no return value
//!
//! Run a subset with `cargo test day27`. A **doc test** is code in a `///`
//! comment; `cargo test --doc` compiles and runs it, which keeps examples
//! honest. Integration tests live in `tests/` and use the crate like an
//! external user.
//!
//! Before committing: `cargo fmt` and `cargo clippy -- -D warnings`. The
//! compiler warnings are not noise; `cargo check` should be clean.
//!
//! **Exercise:** implement `is_prime` and make both the unit tests and the doc
//! test pass.

/// Returns `true` if `n` is prime.
///
/// # Examples
///
/// ```
/// use rust_learn::days::day27::is_prime;
///
/// assert!(is_prime(13));
/// assert!(!is_prime(21));
/// ```
pub fn is_prime(n: u64) -> bool {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_primes() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
    }

    #[test]
    fn composites() {
        assert!(!is_prime(4));
        assert!(!is_prime(21));
        assert!(!is_prime(100));
    }

    #[test]
    fn less_than_two_is_not_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
    }

    #[test]
    fn larger_prime() {
        assert!(is_prime(97));
    }
}
