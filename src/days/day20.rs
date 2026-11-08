//! # Day 20 — Modules and crates
//!
//! A **crate** is one compilation unit with its own `Cargo.toml`. A **module**
//! organizes code inside a crate, like a namespace. `src/lib.rs` is the crate
//! root; `pub mod days;` pulls in `src/days/mod.rs`.
//!
//! Visibility is private by default. `pub` exposes an item to the parent;
//! `pub(crate)` exposes it inside the crate only. Paths:
//!
//! ```text
//! crate::days::day20::math::gcd   // absolute from the crate root
//! super::helper()                 // parent module
//! self::math::gcd                 // current module
//! ```
//!
//! Adding an external crate is one command: `cargo add serde`, then
//! `use serde::Serialize;`. Browse documentation at <https://docs.rs>.
//! `cargo doc --open` builds docs for this crate and its dependencies.
//!
//! **Exercise:** implement `math::gcd` (Euclid's algorithm), used by
//! `simplify`.

pub mod math {
    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        todo!("implement me")
    }
}

pub fn simplify(numerator: u64, denominator: u64) -> (u64, u64) {
    let divisor = math::gcd(numerator, denominator);
    (numerator / divisor, denominator / divisor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greatest_common_divisor() {
        assert_eq!(math::gcd(12, 18), 6);
        assert_eq!(math::gcd(7, 13), 1);
        assert_eq!(math::gcd(0, 5), 5);
        assert_eq!(math::gcd(5, 0), 5);
    }

    #[test]
    fn simplifies_a_fraction() {
        assert_eq!(simplify(6, 8), (3, 4));
        assert_eq!(simplify(10, 5), (2, 1));
    }
}
