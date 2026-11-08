//! # Day 19 — Lifetimes
//!
//! A lifetime is the region of code for which a reference is valid. The
//! compiler usually infers it (**elision**); annotations only describe how
//! input and output references relate.
//!
//! ```text
//! fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
//! ```
//!
//! This says: the returned reference lives no longer than **both** inputs. It
//! does not extend anything; it is a constraint the compiler checks.
//!
//! Elision rules cover the common cases:
//! - one input reference → the output gets that lifetime
//! - `&self` method → the output gets the lifetime of `self`
//! - otherwise, annotate
//!
//! A struct holding a reference needs a lifetime parameter:
//!
//! ```text
//! struct Excerpt<'a> { part: &'a str }
//! ```
//!
//! `'static` means “lives for the whole program” (string literals). Most Rust
//! code has no explicit lifetimes; when the compiler asks, it is usually
//! pointing at a real dangling-reference risk.
//!
//! **Exercise:** return the longer string; ties return `x`.

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    todo!("implement me")
}

#[derive(Debug)]
pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn announce(&self) -> &str {
        self.part
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_the_longer_string() {
        assert_eq!(longest("abc", "de"), "abc");
        assert_eq!(longest("ab", "cde"), "cde");
    }

    #[test]
    fn ties_return_the_first() {
        assert_eq!(longest("ab", "cd"), "ab");
    }

    #[test]
    fn struct_can_borrow_from_its_owner() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let excerpt = Excerpt {
            part: first_sentence,
        };
        assert_eq!(excerpt.announce(), "Call me Ishmael");
    }
}
