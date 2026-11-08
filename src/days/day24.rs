//! # Day 24 — Collections
//!
//! `std::collections` has the containers you expect:
//!
//! - `Vec<T>` — array list (already used)
//! - `HashMap<K, V>` — hash table, unordered, default hasher is DoS-resistant
//! - `BTreeMap<K, V>` — sorted by key
//! - `HashSet<T>` / `BTreeSet<T>`
//! - `VecDeque<T>` — ring buffer, efficient push/pop at both ends
//! - `BinaryHeap<T>` — priority queue
//!
//! `HashMap` is not in the prelude, so import it explicitly. The **entry API**
//! does “look up or insert” without a second hash:
//!
//! ```text
//! *counts.entry(word).or_insert(0) += 1;
//! ```
//!
//! Keys own their data (`String`, not `&str`) if the map outlives the input.
//! Lookups can still borrow: `map.get("rust")` works for a `HashMap<String, _>`.
//!
//! **Exercise:** count words, case-insensitively, separated by whitespace.

use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_each_word() {
        let counts = word_count("the cat the dog");
        assert_eq!(counts.get("the"), Some(&2));
        assert_eq!(counts.get("cat"), Some(&1));
        assert_eq!(counts.get("dog"), Some(&1));
        assert_eq!(counts.len(), 3);
    }

    #[test]
    fn is_case_insensitive() {
        let counts = word_count("Rust rust RUST");
        assert_eq!(counts.get("rust"), Some(&3));
        assert_eq!(counts.len(), 1);
    }

    #[test]
    fn empty_text_has_no_words() {
        assert!(word_count("").is_empty());
    }

    #[test]
    fn collapses_whitespace() {
        let counts = word_count("  a\tb\n a  ");
        assert_eq!(counts.get("a"), Some(&2));
    }
}
