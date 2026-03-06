// The Halo Programming Language
// Value representation in runtime
// Version: 0.1.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

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
    /// Convert value to boolean for conditionals
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Number(0) => false,
            Value::Float(f) if *f == 0.0 => false,
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }

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

    /// Convert value to string
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

    // ============ Arithmetic Operations ============

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

    // ============ Comparison Operations ============

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

    // ============ Logical Operations ============

    pub fn and(&self, other: &Value) -> Value {
        if self.is_truthy() {
            other.clone()
        } else {
            self.clone()
        }
    }

    pub fn or(&self, other: &Value) -> Value {
        if self.is_truthy() {
            self.clone()
        } else {
            other.clone()
        }
    }

    pub fn not(&self) -> Value {
        Value::Bool(!self.is_truthy())
    }

    // ============ Utility Methods ============

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Null => "null",
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
