// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Tests for value types and operations

use halo::interpreter::value::Value;

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
fn test_arithmetic_addition() {
    let a = Value::Number(5);
    let b = Value::Number(3);
    assert_eq!(a.add(&b).unwrap(), Value::Number(8));
}

#[test]
fn test_arithmetic_subtraction() {
    let a = Value::Number(5);
    let b = Value::Number(3);
    assert_eq!(a.subtract(&b).unwrap(), Value::Number(2));
}

#[test]
fn test_arithmetic_multiplication() {
    let a = Value::Number(5);
    let b = Value::Number(3);
    assert_eq!(a.multiply(&b).unwrap(), Value::Number(15));
}

#[test]
fn test_arithmetic_division() {
    let a = Value::Number(5);
    let b = Value::Number(3);
    assert_eq!(a.divide(&b).unwrap(), Value::Number(1));
}

#[test]
fn test_string_concatenation() {
    let s1 = Value::String("Hello".to_string());
    let s2 = Value::String(" World".to_string());
    assert_eq!(
        s1.add(&s2).unwrap(),
        Value::String("Hello World".to_string())
    );
}

#[test]
fn test_string_repetition() {
    let s = Value::String("Ha".to_string());
    let times = Value::Number(3);
    assert_eq!(
        s.multiply(&times).unwrap(),
        Value::String("HaHaHa".to_string())
    );
}

#[test]
fn test_comparison_greater_than() {
    let a = Value::Number(5);
    let b = Value::Number(3);
    assert!(a.greater_than(&b).unwrap());
}

#[test]
fn test_comparison_less_than() {
    let a = Value::Number(5);
    let b = Value::Number(3);
    assert!(b.less_than(&a).unwrap());
}

#[test]
fn test_comparison_equals() {
    let a = Value::Number(5);
    assert!(a.equals(&Value::Number(5)));
}

#[test]
fn test_comparison_not_equals() {
    let a = Value::Number(5);
    let b = Value::Number(3);
    assert!(!a.equals(&b));
}

#[test]
fn test_logical_and_true_false() {
    let t = Value::Bool(true);
    let f = Value::Bool(false);
    assert_eq!(t.and(&f), f);
}

#[test]
fn test_logical_and_true_true() {
    let t = Value::Bool(true);
    let t2 = Value::Bool(true);
    assert_eq!(t.and(&t2), Value::Bool(true));
}

#[test]
fn test_logical_or_true_false() {
    let t = Value::Bool(true);
    let f = Value::Bool(false);
    assert_eq!(t.or(&f), t);
}

#[test]
fn test_logical_or_false_false() {
    let f = Value::Bool(false);
    let f2 = Value::Bool(false);
    assert_eq!(f.or(&f2), Value::Bool(false));
}

#[test]
fn test_logical_not_true() {
    let t = Value::Bool(true);
    assert_eq!(t.not(), Value::Bool(false));
}

#[test]
fn test_logical_not_false() {
    let f = Value::Bool(false);
    assert_eq!(f.not(), Value::Bool(true));
}

#[test]
fn test_to_int_from_number() {
    assert_eq!(Value::Number(42).to_int().unwrap(), 42);
}

#[test]
fn test_to_int_from_float() {
    assert_eq!(Value::Float(std::f64::consts::PI).to_int().unwrap(), 3);
}

#[test]
fn test_to_int_from_bool_true() {
    assert_eq!(Value::Bool(true).to_int().unwrap(), 1);
}

#[test]
fn test_to_int_from_bool_false() {
    assert_eq!(Value::Bool(false).to_int().unwrap(), 0);
}

#[test]
fn test_to_string_value_number() {
    assert_eq!(Value::Number(42).to_string_value(), "42");
}

#[test]
fn test_to_string_value_float() {
    let s = Value::Float(std::f64::consts::PI).to_string_value();
    assert!(s.starts_with("3.14"));
}

#[test]
fn test_to_string_value_bool() {
    assert_eq!(Value::Bool(true).to_string_value(), "true");
    assert_eq!(Value::Bool(false).to_string_value(), "false");
}

#[test]
fn test_to_string_value_string() {
    assert_eq!(
        Value::String("hello".to_string()).to_string_value(),
        "hello"
    );
}

#[test]
fn test_to_string_value_null() {
    assert_eq!(Value::Null.to_string_value(), "null");
}

#[test]
fn test_type_name_number() {
    assert_eq!(Value::Number(42).type_name(), "number");
}

#[test]
fn test_type_name_float() {
    assert_eq!(Value::Float(std::f64::consts::PI).type_name(), "float");
}

#[test]
fn test_type_name_bool() {
    assert_eq!(Value::Bool(true).type_name(), "bool");
}

#[test]
fn test_type_name_string() {
    assert_eq!(Value::String("hello".to_string()).type_name(), "string");
}

#[test]
fn test_type_name_null() {
    assert_eq!(Value::Null.type_name(), "null");
}

#[test]
fn test_modulo_operation() {
    let a = Value::Number(10);
    let b = Value::Number(3);
    assert_eq!(a.modulo(&b).unwrap(), Value::Number(1));
}

#[test]
fn test_float_arithmetic() {
    let a = Value::Float(5.5);
    let b = Value::Float(2.5);
    assert_eq!(a.add(&b).unwrap(), Value::Float(8.0));
}

#[test]
fn test_mixed_int_float_addition() {
    let a = Value::Number(5);
    let b = Value::Float(2.5);
    let result = a.add(&b).unwrap();
    assert!(matches!(result, Value::Float(f) if (f - 7.5).abs() < 1e-10));
}

#[test]
fn test_equal_different_types() {
    let n = Value::Number(5);
    let s = Value::String("5".to_string());
    assert!(!n.equals(&s));
}

#[test]
fn test_less_equal_true() {
    let a = Value::Number(3);
    let b = Value::Number(5);
    assert!(a.less_equal(&b).unwrap());
}

#[test]
fn test_less_equal_equal() {
    let a = Value::Number(5);
    let b = Value::Number(5);
    assert!(a.less_equal(&b).unwrap());
}

#[test]
fn test_greater_equal_true() {
    let a = Value::Number(5);
    let b = Value::Number(3);
    assert!(a.greater_equal(&b).unwrap());
}

#[test]
fn test_greater_equal_equal() {
    let a = Value::Number(5);
    let b = Value::Number(5);
    assert!(a.greater_equal(&b).unwrap());
}
