//! # Day 11 — Tuples, arrays, and `Vec`
//!
//! Three sequence types with different jobs:
//!
//! - **Tuple** `(i32, f64, bool)` — a fixed group of mixed values. Access with
//!   `.0`, `.1`, or destructure: `let (min, max, avg) = stats(&xs);`
//! - **Array** `[T; N]` — fixed length, lives on the stack, type includes the
//!   length (`[u8; 4]`). Like `std::array`.
//! - **`Vec<T>`** — growable heap buffer. Like `std::vector`, but moves and
//!   borrows follow Rust ownership rules.
//!
//! Indexing (`xs[0]`) panics out of bounds; `xs.get(0)` returns `Option<&T>`.
//! Indices are `usize`. Iterate with `for &x in xs` (slice) or `for x in &vec`.
//!
//! **Exercise:** return `(min, max, mean)` for a non-empty slice. For an empty
//! slice return `(0, 0, 0.0)`.

pub fn stats(xs: &[i32]) -> (i32, i32, f64) {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_all_three_stats() {
        assert_eq!(stats(&[1, 2, 3]), (1, 3, 2.0));
    }

    #[test]
    fn handles_negative_numbers() {
        assert_eq!(stats(&[-5, -1]), (-5, -1, -3.0));
    }

    #[test]
    fn one_value_is_its_own_min_max_mean() {
        assert_eq!(stats(&[7]), (7, 7, 7.0));
    }

    #[test]
    fn empty_slice_is_zeroed() {
        assert_eq!(stats(&[]), (0, 0, 0.0));
    }
}
