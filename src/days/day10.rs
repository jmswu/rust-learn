//! # Day 10 — Structs and methods
//!
//! A struct is a named product type. Unlike a C++ class, data and methods live
//! in separate blocks: the `struct` declares fields, `impl` adds behavior.
//! There is no inheritance; compose types instead.
//!
//! ```text
//! struct Point { x: f64, y: f64 }
//!
//! impl Point {
//!     fn new(x: f64, y: f64) -> Self { Self { x, y } }
//!     fn length(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
//!     fn shift(&mut self, dx: f64) { self.x += dx; }
//! }
//! ```
//!
//! - `&self` — read-only method (like `const` member function)
//! - `&mut self` — mutating method
//! - `self` — consumes the value
//! - `Self` — the type being implemented
//!
//! `new` is just a naming convention, not a constructor. `#[derive(...)]`
//! generates common impls: `Debug`, `Clone`, `Copy`, `PartialEq`.
//!
//! **Exercise:** implement `area`, `scale`, and `is_square` for `Rectangle`.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> f64 {
        todo!("implement me")
    }

    pub fn scale(&mut self, factor: f64) {
        todo!("implement me")
    }

    pub fn is_square(&self) -> bool {
        todo!("implement me")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_is_width_times_height() {
        assert_eq!(Rectangle::new(3.0, 4.0).area(), 12.0);
    }

    #[test]
    fn scale_mutates_in_place() {
        let mut rect = Rectangle::new(3.0, 4.0);
        rect.scale(2.0);
        assert_eq!(rect, Rectangle::new(6.0, 8.0));
    }

    #[test]
    fn recognises_squares() {
        assert!(Rectangle::new(2.0, 2.0).is_square());
        assert!(!Rectangle::new(2.0, 3.0).is_square());
    }
}
