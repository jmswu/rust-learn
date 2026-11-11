//! # Day 06 — Ownership, part 1: move vs copy
//!
//! Every value in Rust has exactly **one owner**. When the owner goes out of
//! scope, the value is dropped — deterministic destructors, like C++ RAII, but
//! without writing a destructor.
//!
//! Assigning or passing a heap-owning value **moves** it:
//!
//! ```text
//! let a = String::from("hi");
//! let b = a;              // ownership moves to b
//! println!("{a}");        // error[E0382]: borrow of moved value: `a`
//! ```
//!
//! C++ also “moves” on copy-from-temporary, but a moved-from object remains
//! valid (just unspecified). Rust makes use-after-move a compile error.
//!
//! Small plain values implement `Copy` and are copied instead: all integers,
//! `bool`, `char`, floats, and tuples of `Copy` types. `String`, `Vec`, `Box`,
//! and `HashMap` are not `Copy`.
//!
//! **Compiler gym** (`src/bin/scratch.rs`): try the three lines above and read
//! the full error. Then fix it with `let b = a.clone();` or by borrowing.
//!
//! **Exercise:** take ownership of a `Vec<String>` and join it with spaces.

pub fn join_words(words: Vec<String>) -> String {
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_two_words() {
        let words = vec!["hello".to_string(), "rust".to_string()];
        assert_eq!(join_words(words), "hello rust");
    }

    #[test]
    fn joins_one_word() {
        assert_eq!(join_words(vec!["alone".to_string()]), "alone");
    }

    #[test]
    fn empty_vec_gives_empty_string() {
        assert_eq!(join_words(Vec::new()), "");
    }
}
