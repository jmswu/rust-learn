//! # Day 12 — Enums and `Option`
//!
//! Rust enums are sum types: each variant can carry different data. Think
//! `std::variant` with a tag, except `match` is exhaustive and checked.
//!
//! ```text
//! enum Message {
//!     Quit,                         // no data
//!     Move { x: i32, y: i32 },      // named fields
//!     Write(String),                // one value
//!     ChangeColor(u8, u8, u8),      // several values
//! }
//! ```
//!
//! `Option<T>` is just an enum:
//!
//! ```text
//! enum Option<T> { Some(T), None }
//! ```
//!
//! It replaces null pointers. `Option<&T>` and `Option<Box<T>>` are the same
//! size as the pointer thanks to niche optimization, so “no null” costs nothing.
//! Use `match`, `if let`, `?`, `map`, or `unwrap_or` to get the value out.
//!
//! **Exercise:** compute the area of each shape and find the first even number
//! (returning `None` if there is not one).

pub enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

pub fn area(shape: &Shape) -> f64 {
    todo!("implement me")
}

pub fn find_first_even(xs: &[i32]) -> Option<i32> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn circle_area() {
        assert!(approx(area(&Shape::Circle(1.0)), std::f64::consts::PI));
    }

    #[test]
    fn rectangle_area() {
        let shape = Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert!(approx(area(&shape), 12.0));
    }

    #[test]
    fn triangle_area() {
        let shape = Shape::Triangle {
            base: 10.0,
            height: 3.0,
        };
        assert!(approx(area(&shape), 15.0));
    }

    #[test]
    fn finds_the_first_even_number() {
        assert_eq!(find_first_even(&[1, 3, 4, 6]), Some(4));
    }

    #[test]
    fn no_even_number_means_none() {
        assert_eq!(find_first_even(&[1, 3, 5]), None);
        assert_eq!(find_first_even(&[]), None);
    }
}
