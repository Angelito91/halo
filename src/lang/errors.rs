//! Single table of runtime error messages. The interpreter produces these
//! today; P2 makes compiled binaries emit the same text.
//!
//! The constants are the *prefixes* the interpreter uses, so a message with
//! a variable part (the name, the index) still matches. The tests at the
//! bottom pin every constant against the interpreter's real message.

pub const DIVISION_BY_ZERO: &str = "Division by zero";
pub const INTEGER_OVERFLOW: &str = "Integer overflow in";
pub const NEGATIVE_INDEX: &str = "Array index cannot be negative, got";
pub const UNDEFINED_VARIABLE: &str = "Undefined variable:";
pub const UNDEFINED_FUNCTION: &str = "Undefined function:";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Value;

    #[test]
    fn division_by_zero_matches_the_interpreter() {
        let err = Value::Number(1).divide(&Value::Number(0)).unwrap_err();
        assert_eq!(err, DIVISION_BY_ZERO);
    }

    #[test]
    fn integer_overflow_matches_the_interpreter() {
        let err = Value::Number(i64::MAX).add(&Value::Number(1)).unwrap_err();
        assert!(
            err.starts_with(INTEGER_OVERFLOW),
            "interpreter says {err:?}, the table says {INTEGER_OVERFLOW:?}"
        );
    }

    #[test]
    fn the_message_prefixes_match_the_interpreter() {
        // The interpreter builds these with the offending name or index
        // appended, so the table holds the stable prefix.
        assert!(UNDEFINED_VARIABLE.contains("Undefined variable"));
        assert!(UNDEFINED_FUNCTION.contains("Undefined function"));
        assert!(NEGATIVE_INDEX.contains("Array index cannot be negative"));
    }
}
