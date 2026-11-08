//! # Day 07 — Checkpoint: functions, loops, and `Vec`
//!
//! You have enough Rust to build a small program. This checkpoint combines
//! days 1–6: functions, `mut`, expressions, `while`, `Vec`, and ownership.
//!
//! A real CLI would start like this (try it in `src/bin/scratch.rs`):
//!
//! ```text
//! use std::env;
//!
//! fn main() {
//!     let args: Vec<String> = env::args().collect();
//!     let n: u64 = args[1].parse().expect("usage: collatz <n>");
//!     println!("{n} reaches 1 in {} steps", collatz_steps(n));
//! }
//! ```
//!
//! `env::args()` is an iterator of `String`; `.collect()` gathers it into a
//! `Vec<String>`. Indexing with `args[1]` panics if missing — fine for a
//! throwaway script, not for production code (days 15–16).
//!
//! **Exercise:** return the full Collatz sequence, starting at `n` and ending
//! at 1. `n = 1` produces `[1]`.

pub fn collatz_sequence(n: u64) -> Vec<u64> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_is_already_done() {
        assert_eq!(collatz_sequence(1), vec![1]);
    }

    #[test]
    fn six_sequence() {
        assert_eq!(collatz_sequence(6), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn twenty_seven_sequence_length() {
        assert_eq!(collatz_sequence(27).len(), 112);
        assert_eq!(*collatz_sequence(27).last().unwrap(), 1);
    }
}
