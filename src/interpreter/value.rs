// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
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

use crate::interpreter::{errors, format};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
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
            Value::Array(arr) => !arr.is_empty(),
            _ => true,
        }
    }

    // ── Fast scalar extractors (no clone) ────────────────────────────────────

    // ── Type tag (no allocation) ─────────────────────────────────────────────

    /// Return the type name as a static string slice — no allocation.
    #[inline]
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Array(_) => "array",
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
                .ok_or_else(|| format!("{} addition", errors::INTEGER_OVERFLOW)),
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
                .ok_or_else(|| format!("{} subtraction", errors::INTEGER_OVERFLOW)),
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
                .ok_or_else(|| format!("{} multiplication", errors::INTEGER_OVERFLOW)),
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
                    Err(errors::DIVISION_BY_ZERO.to_string())
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err(errors::DIVISION_BY_ZERO.to_string())
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            (Value::Number(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err(errors::DIVISION_BY_ZERO.to_string())
                } else {
                    Ok(Value::Float(*a as f64 / b))
                }
            }
            (Value::Float(a), Value::Number(b)) => {
                if *b == 0 {
                    Err(errors::DIVISION_BY_ZERO.to_string())
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
                    Err(errors::DIVISION_BY_ZERO.to_string())
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
            Value::Array(_) => Err("Cannot convert array to number".to_string()),
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
            Value::Array(_) => Err("Cannot convert array to integer".to_string()),
            Value::Null => Err("Cannot convert null to integer".to_string()),
        }
    }

    /// Convert value to string (allocates a new String).
    pub fn to_string_value(&self) -> String {
        match self {
            Value::Number(n) => format::format_number(*n),
            Value::Float(f) => format::format_float(*f),
            Value::Bool(b) => format::format_bool(*b).to_string(),
            Value::String(s) => s.clone(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string_value()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Null => format::format_null().to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_value())
    }
}
