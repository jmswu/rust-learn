//! # Day 28 — Checkpoint: word frequency
//!
//! This checkpoint combines collections, iterators, sorting, and ownership.
//! The plan:
//!
//! 1. split on whitespace
//! 2. lowercase each word and count it with the `HashMap` entry API
//! 3. collect into a `Vec`, because `HashMap` itself has no order
//! 4. sort by count descending, then alphabetically for stable output
//! 5. `truncate(n)`
//!
//! Reading a file is one call:
//!
//! ```text
//! let text = std::fs::read_to_string("book.txt")?;
//! ```
//!
//! For a real CLI, return `Result` from `main` and print errors to stderr.
//! Sorting comparator: `b.1.cmp(&a.1)` compares counts in reverse;
//! `.then_with(|| a.0.cmp(&b.0))` breaks ties alphabetically.
//!
//! **Exercise:** return the `n` most frequent words, lowercased.

use std::collections::HashMap;

pub fn top_words(text: &str, n: usize) -> Vec<(String, usize)> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owned(pairs: &[(&str, usize)]) -> Vec<(String, usize)> {
        pairs
            .iter()
            .map(|(word, count)| ((*word).to_string(), *count))
            .collect()
    }

    #[test]
    fn returns_the_top_two() {
        let text = "the cat the dog the";
        assert_eq!(
            top_words(text, 2),
            owned(&[("the", 3), ("cat", 1)]) // tie broken alphabetically
        );
    }

    #[test]
    fn n_larger_than_the_vocabulary_returns_everything() {
        let text = "b a b";
        assert_eq!(top_words(text, 99), owned(&[("b", 2), ("a", 1)]));
    }

    #[test]
    fn n_zero_is_empty() {
        assert!(top_words("anything at all", 0).is_empty());
    }

    #[test]
    fn empty_text_is_empty() {
        assert!(top_words("", 10).is_empty());
    }

    #[test]
    fn lowercases_words() {
        assert_eq!(top_words("Rust rust RUST", 1), owned(&[("rust", 3)]));
    }
}
