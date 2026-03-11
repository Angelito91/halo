// The Halo Programming Language
// Value representation in runtime
// Version: 0.2.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// ── Performance notes ─────────────────────────────────────────────────────────
//
// `Value` is a small enum (largest variant is `String`, one pointer + two
// usizes on most platforms = 24 bytes).  The design goals here are:
//
//  1. Keep the enum size small so copies are cheap (Number/Float/Bool are
//     all copy-types wrapped in a clone-able enum).
//  2. Provide `is_truthy` as a `&self` method so branch conditions never
//     need to clone the value.
//  3. Arithmetic helpers take `&self`/`&other` references; the only
//     allocating path is `String` concatenation, which is unavoidable.
// ─────────────────────────────────────────────────────────────────────────────

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

impl Value {
    // ── Boolean coercion ──────────────────────────────────────────────────────

    /// Return the truthiness of this value **without cloning**.
    /// This is the hot path for `if` / `while` conditions.
    #[inline(always)]
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(0) => false,
            // Use a bit-pattern comparison to avoid an f64 equality check
            // that might trigger a floating-point unit stall.
            Value::Float(f) => f.to_bits() != 0,
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }

    // ── Fast scalar extractors (no clone) ────────────────────────────────────

    /// Extract the inner i64 if this is a `Number`, otherwise `None`.
    #[allow(dead_code)]
    #[inline]
    pub fn as_number(&self) -> Option<i64> {
        if let Value::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    /// Extract the inner f64 if this is a `Float`, otherwise `None`.
    #[allow(dead_code)]
    #[inline]
    pub fn as_float(&self) -> Option<f64> {
        if let Value::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    /// Extract the inner bool if this is a `Bool`, otherwise `None`.
    #[allow(dead_code)]
    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    // ── Type tag (no allocation) ─────────────────────────────────────────────

    /// Return the type name as a static string slice — no allocation.
    #[inline]
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Null => "null",
        }
    }

    // ── Checked integer arithmetic helpers ───────────────────────────────────
    //
    // These are called from the hot loop in `eval_expr`.  We inline the most
    // common case (Number op Number) so the compiler can see through the
    // enum dispatch and avoid the branch when both operands are integers.

    /// `self + other` — handles all numeric and string combinations.
    #[inline]
    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a
                .checked_add(*b)
                .map(Value::Number)
                .ok_or_else(|| "Integer overflow in addition".to_string()),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Number(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Number(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b.to_string_value()))),
            (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a.to_string_value(), b))),
            _ => Err(format!(
                "Cannot add {} and {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// `self - other`
    #[inline]
    pub fn subtract(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a
                .checked_sub(*b)
                .map(Value::Number)
                .ok_or_else(|| "Integer overflow in subtraction".to_string()),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Number(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Number(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err(format!(
                "Cannot subtract {} from {}",
                other.type_name(),
                self.type_name()
            )),
        }
    }

    /// `self * other`
    #[inline]
    pub fn multiply(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a
                .checked_mul(*b)
                .map(Value::Number)
                .ok_or_else(|| "Integer overflow in multiplication".to_string()),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Number(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Number(b)) => Ok(Value::Float(a * *b as f64)),
            (Value::String(s), Value::Number(n)) => {
                if *n < 0 {
                    Err("Cannot repeat string negative times".to_string())
                } else {
                    Ok(Value::String(s.repeat(*n as usize)))
                }
            }
            (Value::Number(n), Value::String(s)) => {
                if *n < 0 {
                    Err("Cannot repeat string negative times".to_string())
                } else {
                    Ok(Value::String(s.repeat(*n as usize)))
                }
            }
            _ => Err(format!(
                "Cannot multiply {} and {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// `self / other`
    #[inline]
    pub fn divide(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            (Value::Number(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(*a as f64 / b))
                }
            }
            (Value::Float(a), Value::Number(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / *b as f64))
                }
            }
            _ => Err(format!(
                "Cannot divide {} by {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// `self % other`
    #[inline]
    pub fn modulo(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Number(a % b))
                }
            }
            _ => Err("Modulo only works with integers".to_string()),
        }
    }

    // ── Comparison ────────────────────────────────────────────────────────────

    /// Structural equality — returns a plain `bool`, no `Value` allocation.
    #[inline]
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Number(a), Value::Float(b)) => *a as f64 == *b,
            (Value::Float(a), Value::Number(b)) => *a == *b as f64,
            _ => false,
        }
    }

    #[inline]
    pub fn less_than(&self, other: &Value) -> Result<bool, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(a < b),
            (Value::Float(a), Value::Float(b)) => Ok(a < b),
            (Value::Number(a), Value::Float(b)) => Ok((*a as f64) < *b),
            (Value::Float(a), Value::Number(b)) => Ok(*a < (*b as f64)),
            (Value::String(a), Value::String(b)) => Ok(a < b),
            _ => Err(format!(
                "Cannot compare {} < {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    #[inline]
    pub fn greater_than(&self, other: &Value) -> Result<bool, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(a > b),
            (Value::Float(a), Value::Float(b)) => Ok(a > b),
            (Value::Number(a), Value::Float(b)) => Ok((*a as f64) > *b),
            (Value::Float(a), Value::Number(b)) => Ok(*a > (*b as f64)),
            (Value::String(a), Value::String(b)) => Ok(a > b),
            _ => Err(format!(
                "Cannot compare {} > {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    #[inline]
    pub fn less_equal(&self, other: &Value) -> Result<bool, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(a <= b),
            (Value::Float(a), Value::Float(b)) => Ok(a <= b),
            (Value::Number(a), Value::Float(b)) => Ok((*a as f64) <= *b),
            (Value::Float(a), Value::Number(b)) => Ok(*a <= (*b as f64)),
            (Value::String(a), Value::String(b)) => Ok(a <= b),
            _ => Err(format!(
                "Cannot compare {} <= {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    #[inline]
    pub fn greater_equal(&self, other: &Value) -> Result<bool, String> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(a >= b),
            (Value::Float(a), Value::Float(b)) => Ok(a >= b),
            (Value::Number(a), Value::Float(b)) => Ok((*a as f64) >= *b),
            (Value::Float(a), Value::Number(b)) => Ok(*a >= (*b as f64)),
            (Value::String(a), Value::String(b)) => Ok(a >= b),
            _ => Err(format!(
                "Cannot compare {} >= {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    // ── Logical ───────────────────────────────────────────────────────────────
    //
    // These are only used for the non-short-circuit path.  The short-circuit
    // path in eval_expr never calls these methods.

    #[inline]
    pub fn and(&self, other: &Value) -> Value {
        if self.is_truthy() {
            other.clone()
        } else {
            self.clone()
        }
    }

    #[inline]
    pub fn or(&self, other: &Value) -> Value {
        if self.is_truthy() {
            self.clone()
        } else {
            other.clone()
        }
    }

    #[inline]
    pub fn not(&self) -> Value {
        Value::Bool(!self.is_truthy())
    }

    // ── Conversion ────────────────────────────────────────────────────────────

    /// Convert value to number (for type coercion)
    pub fn to_number(&self) -> Result<f64, String> {
        match self {
            Value::Number(n) => Ok(*n as f64),
            Value::Float(f) => Ok(*f),
            Value::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Value::String(s) => s
                .parse::<f64>()
                .map_err(|_| format!("Cannot convert '{}' to number", s)),
            Value::Null => Err("Cannot convert null to number".to_string()),
        }
    }

    /// Convert value to integer
    pub fn to_int(&self) -> Result<i64, String> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::Float(f) => Ok(*f as i64),
            Value::Bool(b) => Ok(if *b { 1 } else { 0 }),
            Value::String(s) => s
                .parse::<i64>()
                .map_err(|_| format!("Cannot convert '{}' to integer", s)),
            Value::Null => Err("Cannot convert null to integer".to_string()),
        }
    }

    /// Convert value to string (allocates a new String).
    pub fn to_string_value(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Float(f) => {
                if f.fract() == 0.0 {
                    format!("{:.1}", f)
                } else {
                    f.to_string()
                }
            }
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Null => "null".to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_truthy() {
        assert!(Value::Number(1).is_truthy());
        assert!(!Value::Number(0).is_truthy());
        assert!(Value::Bool(true).is_truthy());
        assert!(!Value::Bool(false).is_truthy());
        assert!(!Value::Null.is_truthy());
        assert!(Value::String("hello".to_string()).is_truthy());
        assert!(!Value::String("".to_string()).is_truthy());
    }

    #[test]
    fn test_arithmetic() {
        let a = Value::Number(5);
        let b = Value::Number(3);
        assert_eq!(a.add(&b).unwrap(), Value::Number(8));
        assert_eq!(a.subtract(&b).unwrap(), Value::Number(2));
        assert_eq!(a.multiply(&b).unwrap(), Value::Number(15));
        assert_eq!(a.divide(&b).unwrap(), Value::Number(1));
    }

    #[test]
    fn test_string_operations() {
        let s1 = Value::String("Hello".to_string());
        let s2 = Value::String(" World".to_string());
        assert_eq!(
            s1.add(&s2).unwrap(),
            Value::String("Hello World".to_string())
        );
    }

    #[test]
    fn test_comparison() {
        let a = Value::Number(5);
        let b = Value::Number(3);
        assert!(a.greater_than(&b).unwrap());
        assert!(b.less_than(&a).unwrap());
        assert!(a.equals(&Value::Number(5)));
        assert!(!a.equals(&b));
    }

    #[test]
    fn test_logical_operations() {
        let t = Value::Bool(true);
        let f = Value::Bool(false);
        assert_eq!(t.and(&f), f);
        assert_eq!(t.or(&f), t);
        assert_eq!(f.not(), Value::Bool(true));
    }

    #[test]
    fn test_to_int() {
        assert_eq!(Value::Number(42).to_int().unwrap(), 42);
        assert_eq!(Value::Float(3.14).to_int().unwrap(), 3);
        assert_eq!(Value::Bool(true).to_int().unwrap(), 1);
    }

    #[test]
    fn test_to_string_value() {
        assert_eq!(Value::Number(42).to_string_value(), "42");
        assert_eq!(Value::Bool(true).to_string_value(), "true");
    }
}
