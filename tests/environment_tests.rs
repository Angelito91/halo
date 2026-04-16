// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Tests for environment and variable scope management

use halo::interpreter::environment::Environment;
use halo::interpreter::value::Value;

#[test]
fn test_set_and_get() {
    let mut env = Environment::new();
    env.set("x".to_string(), Value::Number(42));
    assert_eq!(env.get("x"), Some(Value::Number(42)));
}

#[test]
fn test_undefined_variable() {
    let env = Environment::new();
    assert_eq!(env.get("undefined"), None);
}

#[test]
fn test_scope_isolation() {
    let mut env = Environment::new();
    env.set("x".to_string(), Value::Number(10));

    env.push_scope();
    env.set("x".to_string(), Value::Number(20));
    assert_eq!(env.get("x"), Some(Value::Number(20)));

    env.pop_scope();
    assert_eq!(env.get("x"), Some(Value::Number(10)));
}

#[test]
fn test_nested_scopes() {
    let mut env = Environment::new();
    env.set("a".to_string(), Value::Number(1));

    env.push_scope();
    env.set("b".to_string(), Value::Number(2));
    assert_eq!(env.get("a"), Some(Value::Number(1)));
    assert_eq!(env.get("b"), Some(Value::Number(2)));

    env.push_scope();
    env.set("c".to_string(), Value::Number(3));
    assert_eq!(env.get("a"), Some(Value::Number(1)));
    assert_eq!(env.get("b"), Some(Value::Number(2)));
    assert_eq!(env.get("c"), Some(Value::Number(3)));

    env.pop_scope();
    assert_eq!(env.get("c"), None);
    assert_eq!(env.get("b"), Some(Value::Number(2)));
}

#[test]
fn test_update_existing() {
    let mut env = Environment::new();
    env.set("x".to_string(), Value::Number(10));
    env.update("x", Value::Number(20)).unwrap();
    assert_eq!(env.get("x"), Some(Value::Number(20)));
}

#[test]
fn test_update_creates_if_not_exists() {
    let mut env = Environment::new();
    env.update("x", Value::Number(42)).unwrap();
    assert_eq!(env.get("x"), Some(Value::Number(42)));
}

#[test]
fn test_update_inner_scope_does_not_affect_outer() {
    let mut env = Environment::new();
    env.set("x".to_string(), Value::Number(1));

    env.push_scope();
    env.set("x".to_string(), Value::Number(2));
    // update should hit the inner binding
    env.update("x", Value::Number(99)).unwrap();
    assert_eq!(env.get("x"), Some(Value::Number(99)));

    env.pop_scope();
    // outer binding untouched
    assert_eq!(env.get("x"), Some(Value::Number(1)));
}

#[test]
fn test_no_alloc_on_push_pop() {
    // Verifies that push_scope/pop_scope don't leave orphaned entries.
    let mut env = Environment::new();
    env.set("g".to_string(), Value::Number(0));
    let len_before = env.len();

    env.push_scope();
    env.set("local".to_string(), Value::Number(7));
    assert_eq!(env.len(), len_before + 1);

    env.pop_scope();
    assert_eq!(env.len(), len_before);
    assert_eq!(env.get("local"), None);
}

#[test]
fn test_get_ref() {
    let mut env = Environment::new();
    env.set("pi".to_string(), Value::Float(std::f64::consts::PI));
    let r = env.get_ref("pi").unwrap();
    assert!(matches!(r, Value::Float(f) if (*f - std::f64::consts::PI).abs() < 1e-10));
}

#[test]
fn test_get_mut() {
    let mut env = Environment::new();
    env.set("n".to_string(), Value::Number(1));
    if let Some(v) = env.get_mut("n") {
        *v = Value::Number(100);
    }
    assert_eq!(env.get("n"), Some(Value::Number(100)));
}

#[test]
fn test_depth() {
    let mut env = Environment::new();
    assert_eq!(env.depth(), 0);
    env.push_scope();
    assert_eq!(env.depth(), 1);
    env.push_scope();
    assert_eq!(env.depth(), 2);
    env.pop_scope();
    assert_eq!(env.depth(), 1);
    env.pop_scope();
    assert_eq!(env.depth(), 0);
    // Cannot go below 0
    env.pop_scope();
    assert_eq!(env.depth(), 0);
}

#[test]
fn test_set_dedup_in_same_scope() {
    // Setting the same variable twice in the same scope should not
    // create a duplicate entry.
    let mut env = Environment::new();
    env.set("x".to_string(), Value::Number(1));
    let len1 = env.len();
    env.set("x".to_string(), Value::Number(2));
    assert_eq!(env.len(), len1, "duplicate entry created");
    assert_eq!(env.get("x"), Some(Value::Number(2)));
}
