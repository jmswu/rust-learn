//! # Day 13 — `match` and patterns
//!
//! `match` is a compile-time-checked switch that **must cover every case**.
//! Arms can destructure data, use ranges, guards (`if`), alternation (`|`), and
//! bindings (`@`). `match` is an expression.
//!
//! ```text
//! match n {
//!     0 => "zero",
//!     x if x < 0 => "negative",
//!     1..=9 => "one digit",
//!     _ => "many",
//! }
//! ```
//!
//! `_` is the catch-all. For one interesting arm use `if let`; for “return
//! early otherwise” use `let else`:
//!
//! ```text
//! let Some(x) = maybe else { return None; };
//! ```
//!
//! `matches!(value, pattern)` is a shorthand when you only need a `bool`.
//!
//! **Exercise:** implement FizzBuzz and the number classifier.

pub fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => n.to_string(),
    }
}

pub fn describe(n: i32) -> &'static str {
    match n {
        0 => &"zero",
        x if x < 0 => &"negative",
        1..=9 => &"single digit",
        10..=99 => &"double digit",
        _ => &"large",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizz_buzz_rules() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }

    #[test]
    fn describes_small_numbers() {
        assert_eq!(describe(0), "zero");
        assert_eq!(describe(-5), "negative");
        assert_eq!(describe(7), "single digit");
        assert_eq!(describe(42), "double digit");
        assert_eq!(describe(100), "large");
    }
}
