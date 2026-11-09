//! # Day 03 — Types, casts, and overflow
//!
//! Integer types are explicit: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`,
//! and the unsigned `u8` … `usize`. `usize` is the index/collection length type.
//! `char` is a 4-byte Unicode scalar value, *not* a C `char`.
//!
//! Rust has no implicit numeric conversions. Use `as` when you really mean it,
//! but remember it truncates:
//!
//! ```text
//! let big: i32 = 300;
//! let small = big as u8;   // 44, no warning
//! ```
//!
//! Integer overflow **panics in debug builds** and wraps in release builds.
//! When either behavior is unacceptable, say what you want:
//! `wrapping_add`, `checked_add` (returns `Option`), `saturating_add`.
//!
//! **Exercise:** implement the two conversions below.

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

pub fn wrapping_add(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn freezes_at_32() {
        assert!(approx(celsius_to_fahrenheit(0.0), 32.0));
    }

    #[test]
    fn boils_at_212() {
        assert!(approx(celsius_to_fahrenheit(100.0), 212.0));
    }

    #[test]
    fn minus_forty_is_the_same() {
        assert!(approx(celsius_to_fahrenheit(-40.0), -40.0));
    }

    #[test]
    fn wraps_instead_of_panicking() {
        assert_eq!(wrapping_add(255, 1), 0);
        assert_eq!(wrapping_add(250, 10), 4);
    }
}
