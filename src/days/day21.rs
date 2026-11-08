//! # Day 21 — Checkpoint: generic `Stack<T>`
//!
//! This checkpoint combines structs, generics, `Option`, and methods. A
//! `Stack<T>` is a wrapper around `Vec<T>`:
//!
//! ```text
//! impl<T> Stack<T> { ... }                  // methods for every T
//! impl<T: Display> Stack<T> { ... }         // methods only when T: Display
//! ```
//!
//! The `T` on `impl` is a new parameter, not the one from the struct — it is
//! how you say “for any `T`”. Bounds restrict which `T`s the block applies to.
//! `Option<&T>` from `peek` is a borrow into the stack, so it stays valid only
//! while the stack is not mutated (the borrow checker enforces that).
//!
//! **Exercise:** implement the five core methods, then run the tests with
//! `cargo test day21`.

#[derive(Debug, Default)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        todo!("implement me")
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!("implement me")
    }

    pub fn peek(&self) -> Option<&T> {
        todo!("implement me")
    }

    pub fn len(&self) -> usize {
        todo!("implement me")
    }

    pub fn is_empty(&self) -> bool {
        todo!("implement me")
    }
}

impl<T: std::fmt::Display> Stack<T> {
    pub fn print_all(&self) -> String {
        self.items
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_lifo() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
    }

    #[test]
    fn peek_borrows_without_removing() {
        let mut stack = Stack::new();
        stack.push(10);
        assert_eq!(stack.peek(), Some(&10));
        assert_eq!(stack.len(), 1);
        assert!(!stack.is_empty());
    }

    #[test]
    fn empty_stack_returns_none() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert!(stack.is_empty());
    }

    #[test]
    fn works_with_strings_too() {
        let mut stack = Stack::new();
        stack.push("hello".to_string());
        stack.push("world".to_string());
        assert_eq!(stack.pop(), Some("world".to_string()));
    }

    #[test]
    fn conditional_display_impl() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.print_all(), "1, 2");
    }
}
