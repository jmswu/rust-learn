//! # Day 25 — Smart pointers: `Box`, `Rc`, `RefCell`, `Arc`
//!
//! The C++ mapping is close but not one-to-one:
//!
//! | C++ | Rust | Notes |
//! |---|---|---|
//! | `std::unique_ptr<T>` | `Box<T>` | one owner, heap allocation, automatic drop |
//! | `std::shared_ptr<T>` | `Rc<T>` / `Arc<T>` | reference-counted; `Arc` is atomic/thread-safe |
//! | `std::shared_ptr` + mutex | `Arc<Mutex<T>>` | shared mutable state across threads |
//! | `mutable` / `const_cast` | `RefCell<T>` | runtime-checked interior mutability, single thread |
//!
//! `Box<T>` is also how recursive types get a known size:
//!
//! ```text
//! enum List { Cons(i32, Box<List>), Nil }
//! ```
//!
//! Without `Box`, the compiler rejects the enum: it would contain itself and
//! have infinite size. **Compiler gym:** remove the `Box` in `scratch.rs` and
//! read error E0072.
//!
//! `Rc` is not thread-safe; `Arc` is. `RefCell` enforces the borrow rules at
//! runtime (and panics on violation), which is useful when the compiler cannot
//! prove them. Prefer plain references until you actually need these.
//!
//! **Exercise:** recursively sum a boxed list.

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// Builds a list from a slice; the first element becomes the head.
pub fn from_slice(xs: &[i32]) -> List {
    let mut list = List::Nil;
    for &x in xs.iter().rev() {
        list = List::Cons(x, Box::new(list));
    }
    list
}

pub fn sum(list: &List) -> i32 {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_a_list() {
        assert_eq!(sum(&from_slice(&[1, 2, 3])), 6);
    }

    #[test]
    fn single_element() {
        assert_eq!(sum(&from_slice(&[42])), 42);
    }

    #[test]
    fn empty_list_is_zero() {
        assert_eq!(sum(&from_slice(&[])), 0);
    }

    #[test]
    fn structure_is_a_linked_list() {
        assert!(matches!(from_slice(&[1]), List::Cons(1, _)));
    }
}
