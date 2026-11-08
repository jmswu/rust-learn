//! # Day 18 — Generics
//!
//! Generics are like C++ templates, with one important difference: the generic
//! function is type-checked **once, at its definition**, against its bounds.
//! Callers cannot instantiate it in a way that only fails inside the body.
//!
//! ```text
//! fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> { ... }
//! ```
//!
//! `T: PartialOrd + Copy` is the bound. `PartialOrd` gives `<`/`>`, `Copy`
//! lets us return a value out of a reference. Bounds can also be written with a
//! `where` clause when they get long.
//!
//! Rust compiles a separate specialized copy per concrete type
//! (**monomorphization**), just like templates — so generics are zero-cost.
//! Generic structs and enums are everywhere: `Vec<T>`, `Option<T>`,
//! `HashMap<K, V>`.
//!
//! **Exercise:** return the largest item, or `None` for an empty slice.

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_largest_integer() {
        assert_eq!(largest(&[1, 9, 3, 4]), Some(9));
    }

    #[test]
    fn finds_largest_float() {
        assert_eq!(largest(&[1.5, 0.2, 3.7]), Some(3.7));
    }

    #[test]
    fn finds_largest_char() {
        assert_eq!(largest(&['a', 'z', 'm']), Some('z'));
    }

    #[test]
    fn empty_slice_has_no_largest() {
        let empty: &[i32] = &[];
        assert_eq!(largest(empty), None);
    }
}
