//! # Day 02 — Variables, mutability, shadowing
//!
//! `let` is immutable by default — closer to C++ `const` than to a normal
//! local. `let mut` opts into mutation.
//!
//! ```text
//! let x = 5;        // immutable binding
//! let x = x + 1;    // shadowing: a new binding, may change type
//! let mut y = 5;
//! y += 1;           // reassignment: requires `mut`
//! ```
//!
//! Shadowing is not reassignment: the old binding still exists and is simply
//! hidden. It is idiomatic when a value is transformed step by step.
//!
//! Type inference works like `auto`, but function signatures are always
//! explicit. `const` is a compile-time constant; `static` is a global with a
//! fixed address.
//!
//! **Exercise:** implement `running_total` with a mutable accumulator.

pub fn running_total(values: &[i32]) -> i32 {
    let mut total = 0;
    for &value in values {
        total = total + value;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_positive_numbers() {
        assert_eq!(running_total(&[1, 2, 3]), 6);
    }

    #[test]
    fn empty_slice_is_zero() {
        assert_eq!(running_total(&[]), 0);
    }

    #[test]
    fn handles_negative_numbers() {
        assert_eq!(running_total(&[-1, 1, -2]), -2);
    }
}
