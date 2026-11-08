//! # Day 01 — Cargo and your first function
//!
//! **C++:** compiler + CMake + vcpkg/Conan.
//! **Rust:** `cargo` does all three jobs.
//!
//! Useful commands:
//! - `cargo check` — type-check fast, without producing a binary
//! - `cargo run` — build and run `src/main.rs`
//! - `cargo test day01` — build the crate and run only this day's tests
//! - `cargo fmt` / `cargo clippy` — format / lint
//!
//! A *crate* is one compilation unit. This whole course is one crate; every
//! file under `src/days/` is a module (like a `.cpp` file, minus the header).
//!
//! **Exercise:** make `greet` return `Hello, <name>!`.
//!
//! `format!` is Rust's type-checked `printf`: the value of `name` is inserted
//! where `{name}` appears. A mismatch is a compile error, not a runtime bug.

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_a_name() {
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn greets_an_empty_name() {
        assert_eq!(greet(""), "Hello, !");
    }
}
