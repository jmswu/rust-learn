//! # Day 09 — Slices and strings
//!
//! `&str` is a borrowed, UTF-8 string slice (`const char*` + length, but
//! guaranteed valid UTF-8). `String` owns a growable heap buffer
//! (`std::string`). String literals are `&'static str`.
//!
//! Prefer `&str` in parameters: it accepts both `String` references and
//! literals. Return `String` only when you allocate something new.
//!
//! ```text
//! fn takes(s: &str) {}          // borrows, no copy
//! takes("literal");
//! takes(&owned_string);
//! ```
//!
//! `s[0..2]` indexes **bytes**, not characters, and panics if the range splits
//! a UTF-8 code point. Use `chars()`, `bytes()`, or `char_indices()` for text.
//! `&[T]` is the same fat-pointer idea for any array or `Vec<T>`.
//!
//! **Exercise:** return the first whitespace-delimited word. No space means the
//! whole string; a leading space means the empty slice.

pub fn first_word(s: &str) -> &str {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_first_word() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn no_space_is_the_whole_string() {
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn leading_space_is_empty() {
        assert_eq!(first_word(" hello"), "");
    }

    #[test]
    fn empty_is_empty() {
        assert_eq!(first_word(""), "");
    }
}
