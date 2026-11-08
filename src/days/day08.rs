//! # Day 08 — Borrowing
//!
//! **C++ reflex:** `T&` is a pointer you promise not to misuse.
//! **Rust:** the compiler enforces the promise, plus one extra rule.
//!
//! - `&T` — a shared borrow: read-only, many may exist.
//! - `&mut T` — an exclusive borrow: read/write, exactly one may exist.
//! - You can have **many `&T` XOR one `&mut T`**, never both at the same time.
//!
//! References are never null and never dangle; the compiler proves the pointee
//! outlives the reference. The old C++ habit of “I know it is still alive” is
//! not needed: if it is not provable, it does not compile.
//!
//! **Compiler gym** (`src/bin/scratch.rs`) — this must fail:
//!
//! ```text
//! let mut v = vec![1, 2, 3];
//! let first = &v[0];
//! v.push(4);              // error: cannot borrow `v` as mutable ...
//! println!("{first}");
//! ```
//!
//! Fix it by moving `println!` before `push`, or by copying the value.
//!
//! **Exercise:** append `src` to `dst` and return the new length.

// `&mut String` is deliberate: the point is growing an owned buffer with
// `push_str`, which a `&mut str` cannot do.
#[allow(clippy::ptr_arg)]
pub fn append_len(dst: &mut String, src: &str) -> usize {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_and_reports_length() {
        let mut s = String::from("hello");
        assert_eq!(append_len(&mut s, " world"), 11);
        assert_eq!(s, "hello world");
    }

    #[test]
    fn appending_empty_keeps_length() {
        let mut s = String::from("abc");
        assert_eq!(append_len(&mut s, ""), 3);
    }

    #[test]
    fn the_caller_keeps_ownership() {
        let mut s = String::from("a");
        append_len(&mut s, "b");
        assert_eq!(s, "ab"); // `s` was borrowed, not moved
    }
}
