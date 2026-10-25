// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Language-contract tests for the Minilux-style API:
//! functions only (no dot methods) and explicit index mutation.

mod common;

use common::{assert_parse_error, assert_runtime_error, eval_code, eval_var};
use halo::interpreter::value::Value;

fn numbers(values: &[i64]) -> Value {
    Value::Array(values.iter().copied().map(Value::Number).collect())
}

// ── Index assignment ───────────────────────────────────────────────────────

#[test]
fn index_assignment_updates_the_variable() {
    let code = r#"
arr = [1, 2, 3]
arr[1] = 99
"#;
    assert_eq!(eval_var(code, "arr"), numbers(&[1, 99, 3]));
}

#[test]
fn index_assignment_evaluates_to_the_assigned_value() {
    let code = r#"
arr = [1, 2, 3]
result = arr[0] = 42
"#;
    assert_eq!(eval_var(code, "result"), Value::Number(42));
    assert_eq!(eval_var(code, "arr"), numbers(&[42, 2, 3]));
}

#[test]
fn nested_index_assignment_updates_the_inner_array() {
    let code = r#"
matrix = [[1, 2], [3, 4]]
matrix[1][0] = 42
"#;
    assert_eq!(
        eval_var(code, "matrix"),
        Value::Array(vec![numbers(&[1, 2]), numbers(&[42, 4])])
    );
}

#[test]
fn index_assignment_does_not_affect_copies() {
    let code = r#"
a = [1, 2]
b = a
a[0] = 99
"#;
    assert_eq!(eval_var(code, "a"), numbers(&[99, 2]));
    assert_eq!(eval_var(code, "b"), numbers(&[1, 2]));
}

#[test]
fn index_assignment_rejects_negative_indexes() {
    assert_runtime_error(
        "arr = [1, 2]\narr[-1] = 0",
        "Array index cannot be negative",
    );
}

#[test]
fn index_assignment_rejects_out_of_bounds_indexes() {
    assert_runtime_error(
        "arr = [1, 2]\narr[5] = 0",
        "Array index 5 out of bounds (length 2)",
    );
}

#[test]
fn index_assignment_rejects_non_array_targets() {
    assert_runtime_error("x = 1\nx[0] = 2", "Cannot index a value of type 'number'");
}

#[test]
fn index_assignment_rejects_slices() {
    assert_parse_error("arr = [1, 2]\narr[0:1] = 9", "Cannot assign to a slice");
}

// ── No dot-method syntax ───────────────────────────────────────────────────

#[test]
fn dot_method_calls_are_rejected_with_a_helpful_message() {
    assert_parse_error(
        r#"text = "halo"
text.upper()"#,
        "Method calls are not supported; use function syntax",
    );
}

#[test]
fn function_syntax_remains_available() {
    let code = r#"
text = "  halo  "
result = upper(trim(text))
"#;
    assert_eq!(eval_var(code, "result"), Value::String("HALO".to_string()));
}

#[test]
fn pop_and_last_have_distinct_jobs() {
    let code = r#"
arr = [1, 2, 3]
tail = last(arr)
shorter = pop(arr)
"#;
    assert_eq!(eval_var(code, "tail"), Value::Number(3));
    assert_eq!(eval_var(code, "shorter"), numbers(&[1, 2]));
    assert_eq!(eval_var(code, "arr"), numbers(&[1, 2, 3]));
}

// Keep `eval_code` referenced so this crate compiles cleanly if the helper
// set changes; it also documents that a whole program still evaluates.
#[test]
fn whole_program_still_evaluates() {
    let result = eval_code("print(1)").unwrap();
    assert_eq!(result, Value::Null);
}
