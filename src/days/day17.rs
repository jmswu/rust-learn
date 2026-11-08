//! # Day 17 — Traits
//!
//! A trait is shared behavior — like an interface, or a C++20 concept plus
//! virtual functions. Types opt in with `impl Trait for Type`. There is no
//! inheritance; behavior is composed from traits.
//!
//! ```text
//! trait Greet {
//!     fn name(&self) -> String;                 // required
//!     fn greet(&self) -> String {               // provided (default)
//!         format!("Hello, {}", self.name())
//!     }
//! }
//! ```
//!
//! Traits can be used as bounds (`fn f<T: Summary>(x: T)`) for static dispatch,
//! or as `dyn Summary` for a vtable — day 17 is mostly the static side.
//! `impl Trait` in argument position is shorthand for a generic bound; in return
//! position it means “some concrete type implementing this”.
//!
//! Trait methods must be in scope to call them. The standard traits you will
//! implement most often are `Debug`, `Display`, `Clone`, `PartialEq`, and
//! `Iterator`. `#[derive(...)]` writes the boring ones for you.
//!
//! **Exercise:** implement `Summary` for `Article` so `summarize` reads
//! `"<title> by <author>"`.

pub struct Article {
    pub title: String,
    pub author: String,
}

pub trait Summary {
    fn summarize(&self) -> String;

    fn short(&self) -> String {
        self.summarize().chars().take(20).collect()
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        todo!("implement me")
    }
}

pub fn headline(item: &impl Summary) -> String {
    item.short()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn article() -> Article {
        Article {
            title: "Rust in 30 Days".to_string(),
            author: "Ana".to_string(),
        }
    }

    #[test]
    fn implements_the_required_method() {
        assert_eq!(article().summarize(), "Rust in 30 Days by Ana");
    }

    #[test]
    fn inherits_the_default_method() {
        assert_eq!(article().short(), "Rust in 30 Days by A");
    }

    #[test]
    fn works_through_a_trait_bound() {
        assert_eq!(headline(&article()), "Rust in 30 Days by A");
    }
}
