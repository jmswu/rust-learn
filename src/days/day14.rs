//! # Day 14 — Checkpoint: RPN calculator
//!
//! Reverse Polish notation evaluates left to right with a stack:
//!
//! ```text
//! 5 1 2 + 4 * +   =   5 (1 + 2) 4 * +   =   5 3 4 * +   =   5 12 +   =   17
//! ```
//!
//! This checkpoint combines days 6–13: `Vec` as a stack, `match`, `Option`,
//! `parse`, and `?` on `Option`. `stack.pop()?` returns `None` early if the
//! stack is too small. `token.parse().ok()?` turns a parse failure into `None`.
//!
//! **Exercise:** evaluate a slice of tokens. Return `None` for malformed input,
//! division by zero, or leftovers on the stack (for example `1 2`).

pub fn eval(tokens: &[&str]) -> Option<i64> {
    todo!("implement me")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(eval(&["3", "4", "+"]), Some(7));
    }

    #[test]
    fn evaluates_a_longer_expression() {
        assert_eq!(eval(&["5", "1", "2", "+", "4", "*", "+"]), Some(17));
        assert_eq!(eval(&["3", "4", "+", "2", "*"]), Some(14));
    }

    #[test]
    fn subtraction_order_matters() {
        assert_eq!(eval(&["2", "3", "-"]), Some(-1));
    }

    #[test]
    fn rejects_bad_input() {
        assert_eq!(eval(&[]), None);
        assert_eq!(eval(&["1", "2"]), None);
        assert_eq!(eval(&["1", "+"]), None);
        assert_eq!(eval(&["1", "0", "/"]), None);
        assert_eq!(eval(&["one", "two", "+"]), None);
    }

    #[test]
    fn single_number_is_itself() {
        assert_eq!(eval(&["42"]), Some(42));
    }
}
