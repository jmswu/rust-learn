//! # Day 05 — Control flow
//!
//! `if` is an expression, so it can produce a value. Conditions must be `bool`:
//! there is no “nonzero is true” rule.
//!
//! ```text
//! let parity = if n % 2 == 0 { "even" } else { "odd" };
//! ```
//!
//! Three loops:
//! - `loop { ... }` — infinite; `break value` returns a value
//! - `while condition { ... }` — like C++
//! - `for item in iterator { ... }` — the idiomatic loop
//!
//! Ranges are iterators: `0..n` excludes `n`, `0..=n` includes it. Label loops
//! with `'outer:` and use `break 'outer;` to leave nested loops. There is no
//! C-style `for (i = 0; i < n; i++)`; use `for i in 0..n`.
//!
//! **Exercise:** count the steps for the Collatz conjecture. If `n` is even,
//! halve it; otherwise compute `3n + 1`; stop at 1.

pub fn collatz_steps(mut n: u64) -> u32 {
    let mut steps: u32 = 0;
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_needs_no_steps() {
        assert_eq!(collatz_steps(1), 0);
    }

    #[test]
    fn six_takes_eight_steps() {
        assert_eq!(collatz_steps(6), 8);
    }

    #[test]
    fn twenty_seven_takes_a_while() {
        assert_eq!(collatz_steps(27), 111);
    }
}
