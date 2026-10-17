//! Single table of runtime error messages. The interpreter produces these
//! today; P2 makes compiled binaries emit the same text.

pub const DIVISION_BY_ZERO: &str = "Division by zero";
pub const INTEGER_OVERFLOW: &str = "Integer overflow in";
pub const NEGATIVE_INDEX: &str = "Array index cannot be negative, got";
pub const UNDEFINED_VARIABLE: &str = "Undefined variable:";
pub const UNDEFINED_FUNCTION: &str = "Undefined function:";
