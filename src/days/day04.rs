//! # Day 04 — Functions and expressions
//!
//! The shape is `fn name(arg: Type, ...) -> ReturnType`. A function body is a
//! block, and a block is an **expression**: its value is the last expression,
//! written without a trailing semicolon.
//!
//! ```text
//! fn abs(n: i32) -> i32 {
//!     if n < 0 { -n } else { n }   // this `if` produces the return value
//! }
//! ```
//!
//! A semicolon turns an expression into a statement whose value is `()`. That
//! is why deleting the last semicolon is often the fix for “expected `i32`,
//! found `()`”. `return` exists, but it is mainly for early exits.
//!
//! There is **no function overloading** and no default arguments. Use generics
//! or traits instead (days 17–18). `!` marks a diverging function: `panic!`,
//! `todo!`, and `unreachable!` never return a value.
//!
//! **Exercise:** implement `clamp` and `sign` using `if` expressions.

pub fn clamp(value: i32, lo: i32, hi: i32) -> i32 {
    if value <= lo {
        lo
    }
    else if value >= hi {
        hi
    }
    else {
        value
    }
}

pub fn sign(n: i32) -> &'static str {
    if n == 0 {
        "zero"
    }else if n < 0 {
        "negative"
    } else {
        "positive"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_below_the_range() {
        assert_eq!(clamp(-5, 0, 10), 0);
    }

    #[test]
    fn clamps_above_the_range() {
        assert_eq!(clamp(42, 0, 10), 10);
    }

    #[test]
    fn keeps_values_inside_the_range() {
        assert_eq!(clamp(7, 0, 10), 7);
    }

    #[test]
    fn classifies_numbers() {
        assert_eq!(sign(-3), "negative");
        assert_eq!(sign(0), "zero");
        assert_eq!(sign(3), "positive");
    }
}
