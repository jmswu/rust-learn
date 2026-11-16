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
//! A *crate* is one compilation unit; a *package* (`Cargo.toml`) can hold
//! multiple crates (this repo has one library crate plus binary targets).
//! Modules like `days::day01` organize code inside a crate (declared via `mod`).
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
