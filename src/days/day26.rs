//! # Day 26 — Threads and shared state
//!
//! `std::thread::spawn` starts an OS thread. The closure must own its data
//! (`move`) or borrow it through a scope (`thread::scope`, which joins
//! automatically). Rust checks two marker traits at compile time:
//!
//! - `Send` — safe to move to another thread
//! - `Sync` — safe to share a reference between threads
//!
//! Data races are therefore compile errors, not debugging sessions. Shared
//! mutable state looks like this:
//!
//! ```text
//! let counter = Arc::new(Mutex::new(0));
//! let clone = Arc::clone(&counter);
//! thread::spawn(move || {
//!     *clone.lock().unwrap() += 1;
//! });
//! ```
//!
//! Channels (`std::sync::mpsc`) are often nicer than a mutex: send values to a
//! receiver instead of sharing memory. `Rc`/`RefCell` are **not** thread-safe;
//! use `Arc`/`Mutex` across threads.
//!
//! **Exercise:** sum a `Vec<i64>` in four scoped threads. An empty input sums
//! to 0.

pub fn parallel_sum(xs: Vec<i64>) -> i64 {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_a_small_vec() {
        assert_eq!(parallel_sum(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn sums_a_large_vec() {
        let xs: Vec<i64> = (1..=1000).collect();
        assert_eq!(parallel_sum(xs), 500_500);
    }

    #[test]
    fn empty_sums_to_zero() {
        assert_eq!(parallel_sum(Vec::new()), 0);
    }

    #[test]
    fn handles_negative_values() {
        assert_eq!(parallel_sum(vec![-5, 2, -3]), -6);
    }
}
