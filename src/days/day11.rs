//! # Day 11 — Tuples, arrays, and `Vec`
//!
//! Three sequence types with different jobs:
//!
//! - **Tuple** `(i32, f64, bool)` — a fixed group of mixed values. Access with
//!   `.0`, `.1`, or destructure: `let (min, max, avg) = stats(&xs);`
//! - **Array** `[T; N]` — fixed-length inline value (on the stack for locals),
//!   type includes the length (`[u8; 4]`). Like `std::array`.
//! - **`Vec<T>`** — growable heap buffer. Like `std::vector`, but moves and
//!   borrows follow Rust ownership rules.
//!
//! Indexing (`xs[0]`) panics out of bounds; `xs.get(0)` returns `Option<&T>`.
//! Indices are `usize`. Iterate with `for &x in xs` (slice) or `for x in &vec`.
//!
//! **Exercise:** return `(min, max, mean)` for a non-empty slice. For an empty
//! slice return `(0, 0, 0.0)`.

pub fn stats(xs: &[i32]) -> (i32, i32, f64) {
    if xs.is_empty() {
        return (0, 0, 0.0);
    }

    let mut min: i32 = xs[0];
    let mut max: i32 = xs[0];
    let mut sum: i64 = 0;
    for &val in xs {
        min = min.min(val);
        max = max.max(val);
        sum += val as i64;
    }
    (min, max, sum as f64 / xs.len() as f64)
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
