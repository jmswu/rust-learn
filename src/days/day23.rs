//! # Day 23 — Iterators
//!
//! Iterators are lazy: nothing happens until a consumer asks for values.
//! Adapters build a pipeline; consumers drive it. The compiler fuses the whole
//! chain into a loop — zero-cost, like well-written STL algorithms.
//!
//! ```text
//! let total: i32 = xs.iter()          // &i32
//!     .filter(|&&x| x % 2 == 0)
//!     .map(|&x| x * x)
//!     .sum();                          // consumer
//! ```
//!
//! Common adapters: `map`, `filter`, `filter_map`, `take`, `skip`, `chain`,
//! `zip`, `enumerate`, `rev`, `flat_map`.
//! Common consumers: `collect`, `sum`, `product`, `fold`, `count`, `find`,
//! `any`, `all`, `max`, `min`, `for_each`.
//!
//! Three ways to iterate a collection:
//! `iter()` yields `&T`, `iter_mut()` yields `&mut T`, `into_iter()` consumes
//! and yields `T`. Returning a pipeline uses `impl Iterator<Item = T>`.
//!
//! **Exercise:** sum the squares of the even numbers, and find the first value
//! greater than a limit.

pub fn sum_of_even_squares(xs: &[i32]) -> i32 {
    todo!("implement me")
}

pub fn first_over(xs: &[i32], limit: i32) -> Option<i32> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_even_squares() {
        assert_eq!(sum_of_even_squares(&[1, 2, 3, 4]), 20); // 4 + 16
    }

    #[test]
    fn handles_negative_even_numbers() {
        assert_eq!(sum_of_even_squares(&[-2, 1, 3]), 4);
    }

    #[test]
    fn no_even_numbers_sums_to_zero() {
        assert_eq!(sum_of_even_squares(&[1, 3, 5]), 0);
    }

    #[test]
    fn finds_the_first_value_over_the_limit() {
        assert_eq!(first_over(&[1, 5, 8], 4), Some(5));
        assert_eq!(first_over(&[1, 2, 3], 10), None);
    }
}
