// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

mod common;

use common::{
    assert_runtime_error, eval_last_bool, eval_last_float, eval_last_number, eval_last_string,
    eval_var,
};
use halo::interpreter::Value;

// ── String literals ───────────────────────────────────────────────────────────

#[test]
fn test_string_literal_stored_in_variable() {
    assert_eq!(
        eval_var(r#"s = "hello""#, "s"),
        Value::String("hello".to_string())
    );
}

#[test]
fn test_empty_string_literal() {
    assert_eq!(eval_var(r#"s = """#, "s"), Value::String(String::new()));
}

#[test]
fn test_string_with_spaces() {
    assert_eq!(
        eval_var(r#"s = "hello world""#, "s"),
        Value::String("hello world".to_string())
    );
}

#[test]
fn test_string_with_digits() {
    assert_eq!(
        eval_var(r#"s = "abc123""#, "s"),
        Value::String("abc123".to_string())
    );
}

#[test]
fn test_string_with_special_characters() {
    assert_eq!(
        eval_var(r#"s = "!@#$%^&*""#, "s"),
        Value::String("!@#$%^&*".to_string())
    );
}

// ── Escape sequences ──────────────────────────────────────────────────────────

#[test]
fn test_escape_newline() {
    let s = eval_last_string(r#""\n""#);
    assert_eq!(s, "\n");
}

#[test]
fn test_escape_tab() {
    let s = eval_last_string(r#""\t""#);
    assert_eq!(s, "\t");
}

#[test]
fn test_escape_carriage_return() {
    let s = eval_last_string(r#""\r""#);
    assert_eq!(s, "\r");
}

#[test]
fn test_escape_backslash() {
    let s = eval_last_string(r#""\\""#);
    assert_eq!(s, "\\");
}

#[test]
fn test_escape_double_quote() {
    let s = eval_last_string(r#""\"""#);
    assert_eq!(s, "\"");
}

#[test]
fn test_escape_sequences_combined() {
    // "a\tb\nc" → a<TAB>b<NEWLINE>c
    let s = eval_last_string(r#""a\tb\nc""#);
    assert_eq!(s, "a\tb\nc");
}

// ── String concatenation ──────────────────────────────────────────────────────

#[test]
fn test_string_concat_two_literals() {
    let s = eval_last_string(r#""hello" + " world""#);
    assert_eq!(s, "hello world");
}

#[test]
fn test_string_concat_empty_left() {
    let s = eval_last_string(r#""" + "hello""#);
    assert_eq!(s, "hello");
}

#[test]
fn test_string_concat_empty_right() {
    let s = eval_last_string(r#""hello" + """#);
    assert_eq!(s, "hello");
}

#[test]
fn test_string_concat_three_parts() {
    let s = eval_last_string(r#""foo" + "bar" + "baz""#);
    assert_eq!(s, "foobarbaz");
}

#[test]
fn test_string_concat_via_variables() {
    let code = r#"
a = "hello"
b = " world"
a + b
"#;
    assert_eq!(eval_last_string(code), "hello world");
}

#[test]
fn test_string_concat_with_number_rhs() {
    // string + number coerces the number to a string representation.
    let s = eval_last_string(r#""value: " + 42"#);
    assert_eq!(s, "value: 42");
}

#[test]
fn test_string_concat_with_number_lhs() {
    let s = eval_last_string(r#"42 + " items""#);
    assert_eq!(s, "42 items");
}

#[test]
fn test_string_concat_with_bool() {
    let s = eval_last_string(r#""result: " + true"#);
    assert_eq!(s, "result: true");
}

#[test]
fn test_string_concat_inside_function() {
    let code = r#"
greet(name) {
    return "Hello, " + name + "!"
}
greet("Halo")
"#;
    assert_eq!(eval_last_string(code), "Hello, Halo!");
}

// ── String repetition (string * integer) ─────────────────────────────────────

#[test]
fn test_string_repeat_three_times() {
    let s = eval_last_string(r#""ab" * 3"#);
    assert_eq!(s, "ababab");
}

#[test]
fn test_string_repeat_once() {
    let s = eval_last_string(r#""xyz" * 1"#);
    assert_eq!(s, "xyz");
}

#[test]
fn test_string_repeat_zero_times_gives_empty() {
    let s = eval_last_string(r#""hello" * 0"#);
    assert_eq!(s, "");
}

#[test]
fn test_integer_times_string() {
    let s = eval_last_string(r#"3 * "ab""#);
    assert_eq!(s, "ababab");
}

#[test]
fn test_string_repeat_negative_errors() {
    assert_runtime_error(r#""x" * -1"#, "negative");
}

// ── String comparison ─────────────────────────────────────────────────────────

#[test]
fn test_string_equality_same() {
    assert!(eval_last_bool(r#""hello" == "hello""#));
}

#[test]
fn test_string_equality_different() {
    assert!(!eval_last_bool(r#""hello" == "world""#));
}

#[test]
fn test_string_not_equal() {
    assert!(eval_last_bool(r#""a" != "b""#));
}

#[test]
fn test_string_less_than_lexicographic() {
    // "apple" < "banana" lexicographically.
    assert!(eval_last_bool(r#""apple" < "banana""#));
}

#[test]
fn test_string_greater_than_lexicographic() {
    assert!(eval_last_bool(r#""banana" > "apple""#));
}

#[test]
fn test_string_less_equal_equal_case() {
    assert!(eval_last_bool(r#""abc" <= "abc""#));
}

#[test]
fn test_string_greater_equal_greater_case() {
    assert!(eval_last_bool(r#""z" >= "a""#));
}

#[test]
fn test_empty_string_less_than_nonempty() {
    assert!(eval_last_bool(r#""" < "a""#));
}

// ── String length via len() ───────────────────────────────────────────────────

#[test]
fn test_len_of_ascii_string() {
    assert_eq!(eval_last_number(r#"len("hello")"#), 5);
}

#[test]
fn test_len_of_empty_string() {
    assert_eq!(eval_last_number(r#"len("")"#), 0);
}

#[test]
fn test_len_of_single_char() {
    assert_eq!(eval_last_number(r#"len("x")"#), 1);
}

#[test]
fn test_len_of_string_with_spaces() {
    assert_eq!(eval_last_number(r#"len("hello world")"#), 11);
}

#[test]
fn test_len_of_number_digits() {
    // len(12345) → length of "12345" = 5
    assert_eq!(eval_last_number("len(12345)"), 5);
}

#[test]
fn test_len_of_negative_number() {
    // len(-99) → length of "-99" = 3
    assert_eq!(eval_last_number("len(-99)"), 3);
}

// ── str() — convert to string ─────────────────────────────────────────────────

#[test]
fn test_str_of_integer() {
    assert_eq!(eval_last_string("str(42)"), "42");
}

#[test]
fn test_str_of_zero() {
    assert_eq!(eval_last_string("str(0)"), "0");
}

#[test]
fn test_str_of_negative_integer() {
    assert_eq!(eval_last_string("str(-7)"), "-7");
}

#[test]
fn test_str_of_true() {
    assert_eq!(eval_last_string("str(true)"), "true");
}

#[test]
fn test_str_of_false() {
    assert_eq!(eval_last_string("str(false)"), "false");
}

#[test]
fn test_str_of_float_with_fractional_part() {
    assert_eq!(eval_last_string("str(3.14)"), "3.14");
}

#[test]
fn test_str_of_float_whole_number() {
    // Whole-number floats display with one decimal place: 2.0 → "2.0".
    let s = eval_last_string("str(2.0)");
    assert_eq!(s, "2.0");
}

#[test]
fn test_str_of_string_is_identity() {
    let s = eval_last_string(r#"str("hello")"#);
    assert_eq!(s, "hello");
}

#[test]
fn test_str_wrong_arity_errors() {
    assert_runtime_error("str(1, 2)", "expects exactly 1");
}

// ── int() — convert to integer ────────────────────────────────────────────────

#[test]
fn test_int_of_integer_is_identity() {
    assert_eq!(eval_last_number("int(42)"), 42);
}

#[test]
fn test_int_of_float_truncates() {
    assert_eq!(eval_last_number("int(3.9)"), 3);
}

#[test]
fn test_int_of_negative_float_truncates_toward_zero() {
    assert_eq!(eval_last_number("int(-2.7)"), -2);
}

#[test]
fn test_int_of_true_is_1() {
    assert_eq!(eval_last_number("int(true)"), 1);
}

#[test]
fn test_int_of_false_is_0() {
    assert_eq!(eval_last_number("int(false)"), 0);
}

#[test]
fn test_int_of_numeric_string() {
    assert_eq!(eval_last_number(r#"int("99")"#), 99);
}

#[test]
fn test_int_of_negative_string() {
    assert_eq!(eval_last_number(r#"int("-5")"#), -5);
}

#[test]
fn test_int_of_non_numeric_string_errors() {
    assert_runtime_error(r#"int("abc")"#, "Cannot convert");
}

#[test]
fn test_int_wrong_arity_errors() {
    assert_runtime_error("int(1, 2)", "expects exactly 1");
}

// ── float() — convert to float ───────────────────────────────────────────────

#[test]
fn test_float_of_integer() {
    let f = eval_last_float("float(7)");
    assert!((f - 7.0).abs() < 1e-10, "expected 7.0, got {f}");
}

#[test]
fn test_float_of_float_is_identity() {
    let f = eval_last_float("float(3.14)");
    assert!((f - 3.14).abs() < 1e-10);
}

#[test]
fn test_float_of_true_is_1() {
    let f = eval_last_float("float(true)");
    assert!((f - 1.0).abs() < 1e-10);
}

#[test]
fn test_float_of_false_is_0() {
    let f = eval_last_float("float(false)");
    assert!((f - 0.0).abs() < 1e-10);
}

#[test]
fn test_float_of_numeric_string() {
    let f = eval_last_float(r#"float("2.5")"#);
    assert!((f - 2.5).abs() < 1e-10);
}

#[test]
fn test_float_of_integer_string() {
    let f = eval_last_float(r#"float("10")"#);
    assert!((f - 10.0).abs() < 1e-10);
}

#[test]
fn test_float_of_non_numeric_string_errors() {
    assert_runtime_error(r#"float("xyz")"#, "Cannot convert");
}

#[test]
fn test_float_wrong_arity_errors() {
    assert_runtime_error("float(1, 2)", "expects exactly 1");
}

// ── type() — return type name as string ──────────────────────────────────────

#[test]
fn test_type_of_integer_is_number() {
    assert_eq!(eval_last_string("type(42)"), "number");
}

#[test]
fn test_type_of_zero_is_number() {
    assert_eq!(eval_last_string("type(0)"), "number");
}

#[test]
fn test_type_of_negative_integer_is_number() {
    assert_eq!(eval_last_string("type(-1)"), "number");
}

#[test]
fn test_type_of_float_is_float() {
    assert_eq!(eval_last_string("type(3.14)"), "float");
}

#[test]
fn test_type_of_true_is_bool() {
    assert_eq!(eval_last_string("type(true)"), "bool");
}

#[test]
fn test_type_of_false_is_bool() {
    assert_eq!(eval_last_string("type(false)"), "bool");
}

#[test]
fn test_type_of_string_is_string() {
    assert_eq!(eval_last_string(r#"type("hello")"#), "string");
}

#[test]
fn test_type_of_empty_string_is_string() {
    assert_eq!(eval_last_string(r#"type("")"#), "string");
}

#[test]
fn test_type_wrong_arity_errors() {
    assert_runtime_error("type(1, 2)", "expects exactly 1");
}

// ── abs() — absolute value ────────────────────────────────────────────────────

#[test]
fn test_abs_positive_integer_unchanged() {
    assert_eq!(eval_last_number("abs(7)"), 7);
}

#[test]
fn test_abs_negative_integer() {
    assert_eq!(eval_last_number("abs(-7)"), 7);
}

#[test]
fn test_abs_zero() {
    assert_eq!(eval_last_number("abs(0)"), 0);
}

#[test]
fn test_abs_positive_float_unchanged() {
    let f = eval_last_float("abs(3.14)");
    assert!((f - 3.14).abs() < 1e-10);
}

#[test]
fn test_abs_negative_float() {
    let f = eval_last_float("abs(-3.14)");
    assert!((f - 3.14).abs() < 1e-10);
}

#[test]
fn test_abs_of_bool_errors() {
    assert_runtime_error("abs(true)", "does not support type");
}

#[test]
fn test_abs_wrong_arity_errors() {
    assert_runtime_error("abs(1, 2)", "expects exactly 1");
}

// ── Truthiness of strings ────────────────────────────────────────────────────

#[test]
fn test_nonempty_string_is_truthy() {
    let code = r#"
result = 0
if "hello" { result = 1 }
"#;
    assert_eq!(eval_var(code, "result"), Value::Number(1));
}

#[test]
fn test_empty_string_is_falsy() {
    let code = r#"
result = 0
if "" { result = 1 }
"#;
    assert_eq!(eval_var(code, "result"), Value::Number(0));
}

// ── Round-trip conversions ────────────────────────────────────────────────────

#[test]
fn test_int_then_str_round_trip() {
    // int("42") → 42, str(42) → "42"
    assert_eq!(eval_last_string(r#"str(int("42"))"#), "42");
}

#[test]
fn test_float_then_str_round_trip() {
    // float → str → back to float
    let f = eval_last_float(r#"float(str(2.5))"#);
    assert!((f - 2.5).abs() < 1e-10);
}

#[test]
fn test_str_then_int_preserves_value() {
    assert_eq!(eval_last_number("int(str(99))"), 99);
}

// ── Type coercion in arithmetic ───────────────────────────────────────────────

#[test]
fn test_string_number_concat_in_function() {
    let code = r#"
format_result(label, value) {
    return label + value
}
format_result("answer: ", 42)
"#;
    assert_eq!(eval_last_string(code), "answer: 42");
}

#[test]
fn test_len_of_concatenated_string() {
    // len("abc" + "de") = len("abcde") = 5
    assert_eq!(eval_last_number(r#"len("abc" + "de")"#), 5);
}

#[test]
fn test_type_after_str_conversion_is_string() {
    assert_eq!(eval_last_string("type(str(99))"), "string");
}

#[test]
fn test_type_after_int_conversion_is_number() {
    assert_eq!(eval_last_string(r#"type(int("5"))"#), "number");
}

#[test]
fn test_type_after_float_conversion_is_float() {
    assert_eq!(eval_last_string("type(float(1))"), "float");
}
