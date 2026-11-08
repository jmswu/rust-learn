//! # Day 22 — Closures
//!
//! A closure is an anonymous function that captures its environment. C++
//! lambdas have the same idea, but Rust tracks **how** the closure captures:
//!
//! - `Fn` — borrows immutably; can be called many times
//! - `FnMut` — borrows mutably; can modify captured state
//! - `FnOnce` — consumes captured values; callable once
//!
//! The compiler infers the weakest requirement automatically. `move |x| ...`
//! forces ownership into the closure — necessary for threads and returned
//! closures that outlive the current stack frame.
//!
//! ```text
//! let n = 3;
//! let add = |x| x + n;          // borrows n
//! let make = move |x| x + n;    // owns a copy of n
//! ```
//!
//! Closures are values; store them in a variable, pass them to `sort_by_key`,
//! or return them as `impl Fn(i32) -> i32`. Each closure has its own unique
//! anonymous type.
//!
//! **Exercise:** sort by absolute value, and apply a closure to every element.
//! `make_adder` is prewritten as an example of returning a closure.

pub fn sort_by_abs(xs: &mut [i32]) {
    todo!("implement me")
}

/// Prewritten example: the returned closure owns a copy of `n`.
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

pub fn apply_to_all(xs: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_by_magnitude() {
        let mut xs = vec![3, -1, 2, -5];
        sort_by_abs(&mut xs);
        assert_eq!(xs, vec![-1, 2, 3, -5]);
    }

    #[test]
    fn returns_a_closure_that_owns_its_state() {
        let add_five = make_adder(5);
        assert_eq!(add_five(2), 7);
        assert_eq!(add_five(-10), -5);
    }

    #[test]
    fn two_adders_are_independent() {
        let add_one = make_adder(1);
        let add_ten = make_adder(10);
        assert_eq!(add_one(1), 2);
        assert_eq!(add_ten(1), 11);
    }

    #[test]
    fn applies_a_closure_to_every_item() {
        assert_eq!(apply_to_all(&[1, 2, 3], |x| x * 2), vec![2, 4, 6]);
    }

    #[test]
    fn works_with_a_returned_closure() {
        assert_eq!(apply_to_all(&[1, 2], make_adder(10)), vec![11, 12]);
    }
}
