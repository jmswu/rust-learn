//! # Day 29 — Capstone I: matching lines
//!
//! The capstone is a tiny `grep`:
//!
//! ```text
//! cargo run --bin grep -- [-i] <pattern> <path>
//! ```
//!
//! Today: the search engine. `text.lines()` yields `&str` slices **without**
//! the newline. `filter` keeps matches; `collect` gathers them. The returned
//! `Vec<&'a str>` borrows from the input text, so the lifetime ties the result
//! to `text` — no copies, no dangling pointers.
//!
//! Case-insensitive matching lowercases both sides. That allocates, which is
//! fine for a small tool; a real grep works on bytes and Unicode case folding.
//!
//! **Exercise:** return matching lines in order. `needle` may be empty (matches
//! every line). Set `ignore_case` to compare lowercased.

pub fn filter_lines<'a>(text: &'a str, needle: &str, ignore_case: bool) -> Vec<&'a str> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "Rust\nis fun\nC++ is fast\nrustaceans";

    #[test]
    fn matches_exactly() {
        assert_eq!(filter_lines(TEXT, "Rust", false), vec!["Rust"]);
    }

    #[test]
    fn case_sensitive_by_default() {
        // "rustaceans" contains lowercase "rust"; "Rust" does not.
        assert_eq!(filter_lines(TEXT, "rust", false), vec!["rustaceans"]);
        assert!(filter_lines(TEXT, "RUST", false).is_empty());
    }

    #[test]
    fn can_ignore_case() {
        assert_eq!(filter_lines(TEXT, "rust", true), vec!["Rust", "rustaceans"]);
    }

    #[test]
    fn returns_lines_in_order() {
        assert_eq!(
            filter_lines(TEXT, "is", false),
            vec!["is fun", "C++ is fast"]
        );
    }

    #[test]
    fn empty_needle_matches_every_line() {
        assert_eq!(filter_lines("a\nb", "", false), vec!["a", "b"]);
    }

    #[test]
    fn empty_text_has_no_lines() {
        assert!(filter_lines("", "anything", false).is_empty());
    }
}
