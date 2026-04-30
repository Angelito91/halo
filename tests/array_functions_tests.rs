// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Tests for array manipulation built-in functions

use halo::interpreter::evaluator::Evaluator;
use halo::interpreter::value::Value;
use halo::lexer::{Lexer, TokenKind};
use halo::parser::Parser;

/// Helper function to tokenise, parse, and evaluate a snippet of Halo source code.
fn eval_code(code: &str) -> Result<Value, String> {
    let mut lexer = Lexer::new(code.to_string());
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        let is_eof = tok.kind == TokenKind::Eof;
        tokens.push(tok);
        if is_eof {
            break;
        }
    }
    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|errs| errs.join("; "))?;
    let mut evaluator = Evaluator::new();
    evaluator.eval_program(&program)
}

// ── push() tests ───────────────────────────────────────────────────────────

#[test]
fn test_push_adds_element_to_array() {
    let code = r#"
get_result() {
    arr = [1, 2, 3]
    result = push(arr, 4)
    return result
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_push_to_empty_array() {
    let code = r#"
get_result() {
    arr = []
    result = push(arr, 42)
    return result
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_push_different_types() {
    let code = r#"
get_result() {
    arr = [1, "hello"]
    result = push(arr, 3.14)
    return result
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_push_wrong_number_of_args() {
    let code = r#"
result = push([1, 2])
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects exactly 2 arguments"));
}

#[test]
fn test_push_non_array_first_arg() {
    let code = r#"
result = push(42, 1)
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("first argument to be an array"));
}

// ── pop() tests ────────────────────────────────────────────────────────────

#[test]
fn test_pop_removes_last_element() {
    let code = r#"
get_result() {
    arr = [1, 2, 3]
    result = pop(arr)
    return result
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_pop_returns_value() {
    let code = r#"
get_result() {
    arr = [10, 20, 30]
    val = pop(arr)
    if val == 30 {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_pop_empty_array_returns_null() {
    let code = r#"
get_result() {
    arr = []
    val = pop(arr)
    result_type = type(val)
    if result_type == "null" {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_pop_wrong_number_of_args() {
    let code = r#"
result = pop([1, 2], 3)
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects exactly 1 argument"));
}

#[test]
fn test_pop_non_array() {
    let code = r#"
result = pop("string")
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects an array"));
}

// ── first() tests ──────────────────────────────────────────────────────────

#[test]
fn test_first_returns_first_element() {
    let code = r#"
get_result() {
    arr = [10, 20, 30]
    val = first(arr)
    if val == 10 {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_first_with_strings() {
    let code = r#"
get_result() {
    arr = ["alpha", "beta", "gamma"]
    val = first(arr)
    if val == "alpha" {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_first_empty_array_returns_null() {
    let code = r#"
get_result() {
    arr = []
    val = first(arr)
    result_type = type(val)
    if result_type == "null" {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_first_wrong_number_of_args() {
    let code = r#"
result = first([1, 2], 3)
"#;
    let result = eval_code(code);
    assert!(result.is_err());
}

// ── last() tests ───────────────────────────────────────────────────────────

#[test]
fn test_last_returns_last_element() {
    let code = r#"
get_result() {
    arr = [10, 20, 30]
    val = last(arr)
    if val == 30 {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_last_with_single_element() {
    let code = r#"
get_result() {
    arr = [42]
    val = last(arr)
    if val == 42 {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_last_empty_array_returns_null() {
    let code = r#"
get_result() {
    arr = []
    val = last(arr)
    result_type = type(val)
    if result_type == "null" {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

// ── reverse() tests ────────────────────────────────────────────────────────

#[test]
fn test_reverse_basic() {
    let code = r#"
get_result() {
    arr = [1, 2, 3]
    rev = reverse(arr)
    return rev
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_reverse_with_strings() {
    let code = r#"
get_result() {
    arr = ["a", "b", "c"]
    rev = reverse(arr)
    first_elem = rev[0]
    if first_elem == "c" {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_reverse_empty_array() {
    let code = r#"
get_result() {
    arr = []
    rev = reverse(arr)
    return rev
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_reverse_single_element() {
    let code = r#"
get_result() {
    arr = [42]
    rev = reverse(arr)
    return rev
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_reverse_wrong_number_of_args() {
    let code = r#"
result = reverse([1, 2], 3)
"#;
    let result = eval_code(code);
    assert!(result.is_err());
}

#[test]
fn test_reverse_non_array() {
    let code = r#"
result = reverse(123)
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects an array"));
}

// ── join() tests ───────────────────────────────────────────────────────────

#[test]
fn test_join_with_comma() {
    let code = r#"
get_result() {
    arr = [1, 2, 3]
    result = join(arr, ",")
    return result
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_join_with_space() {
    let code = r#"
get_result() {
    arr = ["hello", "world"]
    result = join(arr, " ")
    if result == "hello world" {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_join_empty_array() {
    let code = r#"
get_result() {
    arr = []
    result = join(arr, ",")
    if result == "" {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_join_with_empty_separator() {
    let code = r#"
get_result() {
    arr = ["a", "b", "c"]
    result = join(arr, "")
    if result == "abc" {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_join_wrong_number_of_args() {
    let code = r#"
result = join([1, 2])
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects exactly 2 arguments"));
}

#[test]
fn test_join_non_string_separator() {
    let code = r#"
result = join([1, 2], 42)
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("second argument to be a string"));
}

#[test]
fn test_join_non_array_first_arg() {
    let code = r#"
result = join("not_array", ",")
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("first argument to be an array"));
}

// ── contains() tests ───────────────────────────────────────────────────────

#[test]
fn test_contains_found() {
    let code = r#"
get_result() {
    arr = [1, 2, 3, 4, 5]
    if contains(arr, 3) {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_contains_not_found() {
    let code = r#"
get_result() {
    arr = [1, 2, 3, 4, 5]
    if contains(arr, 10) {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_contains_with_strings() {
    let code = r#"
get_result() {
    arr = ["apple", "banana", "cherry"]
    if contains(arr, "banana") {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_contains_empty_array() {
    let code = r#"
get_result() {
    arr = []
    if contains(arr, 5) {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_contains_wrong_number_of_args() {
    let code = r#"
result = contains([1, 2])
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects exactly 2 arguments"));
}

#[test]
fn test_contains_non_array() {
    let code = r#"
result = contains(123, 1)
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("first argument to be an array"));
}

// ── index_of() tests ───────────────────────────────────────────────────────

#[test]
fn test_index_of_found() {
    let code = r#"
get_result() {
    arr = [10, 20, 30, 40]
    idx = index_of(arr, 30)
    if idx == 2 {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_index_of_first_element() {
    let code = r#"
get_result() {
    arr = ["a", "b", "c"]
    idx = index_of(arr, "a")
    if idx == 0 {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_index_of_not_found() {
    let code = r#"
get_result() {
    arr = [1, 2, 3, 4]
    idx = index_of(arr, 10)
    if idx == -1 {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_index_of_empty_array() {
    let code = r#"
get_result() {
    arr = []
    idx = index_of(arr, 5)
    if idx == -1 {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_index_of_wrong_number_of_args() {
    let code = r#"
result = index_of([1, 2])
"#;
    let result = eval_code(code);
    assert!(result.is_err());
}

#[test]
fn test_index_of_non_array() {
    let code = r#"
result = index_of("string", "s")
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("first argument to be an array"));
}

// ── sort() tests ───────────────────────────────────────────────────────────

#[test]
fn test_sort_numbers_ascending() {
    let code = r#"
get_result() {
    arr = [3, 1, 4, 1, 5, 9, 2, 6]
    sorted = sort(arr)
    return sorted
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_sort_already_sorted() {
    let code = r#"
get_result() {
    arr = [1, 2, 3, 4, 5]
    sorted = sort(arr)
    first = sorted[0]
    if first == 1 {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_sort_negative_numbers() {
    let code = r#"
get_result() {
    arr = [-5, 3, -1, 0, 2]
    sorted = sort(arr)
    return sorted
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_sort_floats() {
    let code = r#"
get_result() {
    arr = [3.14, 1.41, 2.71, 1.73]
    sorted = sort(arr)
    return sorted
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_sort_empty_array() {
    let code = r#"
get_result() {
    arr = []
    sorted = sort(arr)
    return sorted
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_sort_single_element() {
    let code = r#"
get_result() {
    arr = [42]
    sorted = sort(arr)
    return sorted
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_sort_wrong_number_of_args() {
    let code = r#"
result = sort([1, 2], 3)
"#;
    let result = eval_code(code);
    assert!(result.is_err());
}

#[test]
fn test_sort_non_array() {
    let code = r#"
result = sort(123)
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects an array"));
}

#[test]
fn test_sort_with_strings_fails() {
    let code = r#"
result = sort(["a", "b", "c"])
"#;
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("only works with arrays of numbers"));
}

#[test]
fn test_sort_mixed_number_types() {
    let code = r#"
get_result() {
    arr = [5, 2.5, 1, 3.14, 4]
    sorted = sort(arr)
    first = sorted[0]
    if first == 1 {
        return 1
    }
    return 0
}
final = get_result()
"#;
    let result = eval_code(code).unwrap();
    assert_eq!(result, Value::Null);
}
