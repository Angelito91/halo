// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

mod common;

use common::{
    assert_runtime_error, eval_bool, eval_float, eval_last_number, eval_number, eval_var,
};
use halo::interpreter::Value;

// ── Integer arithmetic ────────────────────────────────────────────────────────

#[test]
fn test_addition_result() {
    assert_eq!(eval_number("x = 3 + 4", "x"), 7);
}

#[test]
fn test_subtraction_result() {
    assert_eq!(eval_number("x = 10 - 3", "x"), 7);
}

#[test]
fn test_multiplication_result() {
    assert_eq!(eval_number("x = 6 * 7", "x"), 42);
}

#[test]
fn test_integer_division_truncates() {
    // Halo integer division truncates towards zero (matches i64 semantics).
    assert_eq!(eval_number("x = 7 / 2", "x"), 3);
}

#[test]
fn test_integer_division_exact() {
    assert_eq!(eval_number("x = 20 / 4", "x"), 5);
}

#[test]
fn test_modulo_positive() {
    assert_eq!(eval_number("x = 10 % 3", "x"), 1);
}

#[test]
fn test_modulo_exact_divisible() {
    assert_eq!(eval_number("x = 9 % 3", "x"), 0);
}

#[test]
fn test_modulo_dividend_smaller_than_divisor() {
    assert_eq!(eval_number("x = 2 % 5", "x"), 2);
}

#[test]
fn test_unary_negation_integer() {
    assert_eq!(eval_number("x = -7", "x"), -7);
}

#[test]
fn test_unary_negation_of_expression() {
    assert_eq!(eval_number("x = -(3 + 4)", "x"), -7);
}

#[test]
fn test_double_negation() {
    // - -5 == 5
    assert_eq!(eval_number("x = - -5", "x"), 5);
}

#[test]
fn test_zero_operations() {
    assert_eq!(eval_number("x = 0 + 0", "x"), 0);
    assert_eq!(eval_number("x = 0 * 99", "x"), 0);
    assert_eq!(eval_number("x = 0 - 0", "x"), 0);
}

#[test]
fn test_large_integer_multiplication() {
    // 1_000 * 1_000 = 1_000_000 (well within i64 range)
    assert_eq!(eval_number("x = 1000 * 1000", "x"), 1_000_000);
}

#[test]
fn test_negative_result_subtraction() {
    assert_eq!(eval_number("x = 3 - 10", "x"), -7);
}

#[test]
fn test_consecutive_additions() {
    assert_eq!(eval_number("x = 1 + 2 + 3 + 4 + 5", "x"), 15);
}

#[test]
fn test_consecutive_subtractions() {
    assert_eq!(eval_number("x = 20 - 3 - 4 - 5", "x"), 8);
}

// ── Operator precedence ───────────────────────────────────────────────────────

#[test]
fn test_mul_before_add() {
    // 2 + 3 * 4 = 2 + 12 = 14
    assert_eq!(eval_number("x = 2 + 3 * 4", "x"), 14);
}

#[test]
fn test_mul_before_sub() {
    // 10 - 2 * 3 = 10 - 6 = 4
    assert_eq!(eval_number("x = 10 - 2 * 3", "x"), 4);
}

#[test]
fn test_div_before_add() {
    // 1 + 10 / 2 = 1 + 5 = 6
    assert_eq!(eval_number("x = 1 + 10 / 2", "x"), 6);
}

#[test]
fn test_parentheses_override_precedence() {
    assert_eq!(eval_number("x = (2 + 3) * 4", "x"), 20);
}

#[test]
fn test_nested_parentheses() {
    // ((2 + 3) * (4 - 1)) = 5 * 3 = 15
    assert_eq!(eval_number("x = (2 + 3) * (4 - 1)", "x"), 15);
}

#[test]
fn test_complex_precedence_chain() {
    // 2 + 3 * 4 - 6 / 2 = 2 + 12 - 3 = 11
    assert_eq!(eval_number("x = 2 + 3 * 4 - 6 / 2", "x"), 11);
}

#[test]
fn test_modulo_precedence_with_add() {
    // 10 + 7 % 3 = 10 + 1 = 11
    assert_eq!(eval_number("x = 10 + 7 % 3", "x"), 11);
}

// ── Floating-point arithmetic ─────────────────────────────────────────────────

#[test]
fn test_float_addition() {
    let result = eval_float("x = 1.5 + 2.5", "x");
    assert!((result - 4.0).abs() < 1e-10, "expected 4.0, got {result}");
}

#[test]
fn test_float_subtraction() {
    let result = eval_float("x = 5.0 - 1.5", "x");
    assert!((result - 3.5).abs() < 1e-10, "expected 3.5, got {result}");
}

#[test]
fn test_float_multiplication() {
    let result = eval_float("x = 2.5 * 4.0", "x");
    assert!((result - 10.0).abs() < 1e-10, "expected 10.0, got {result}");
}

#[test]
fn test_float_division() {
    let result = eval_float("x = 7.5 / 2.5", "x");
    assert!((result - 3.0).abs() < 1e-10, "expected 3.0, got {result}");
}

#[test]
fn test_float_unary_negation() {
    let result = eval_float("x = -3.14", "x");
    assert!((result - (-3.14)).abs() < 1e-10);
}

// ── Mixed integer / float promotion ──────────────────────────────────────────

#[test]
fn test_int_plus_float_promotes_to_float() {
    let result = eval_float("x = 5 + 0.5", "x");
    assert!((result - 5.5).abs() < 1e-10, "expected 5.5, got {result}");
}

#[test]
fn test_float_plus_int_promotes_to_float() {
    let result = eval_float("x = 0.5 + 5", "x");
    assert!((result - 5.5).abs() < 1e-10);
}

#[test]
fn test_int_minus_float() {
    let result = eval_float("x = 10 - 0.5", "x");
    assert!((result - 9.5).abs() < 1e-10);
}

#[test]
fn test_int_times_float() {
    let result = eval_float("x = 3 * 1.5", "x");
    assert!((result - 4.5).abs() < 1e-10);
}

#[test]
fn test_int_divided_by_float() {
    let result = eval_float("x = 10 / 4.0", "x");
    assert!((result - 2.5).abs() < 1e-10);
}

// ── Division-by-zero errors ───────────────────────────────────────────────────

#[test]
fn test_integer_division_by_zero_errors() {
    assert_runtime_error("x = 5 / 0", "Division by zero");
}

#[test]
fn test_float_division_by_zero_errors() {
    assert_runtime_error("x = 5.0 / 0.0", "Division by zero");
}

#[test]
fn test_modulo_by_zero_errors() {
    assert_runtime_error("x = 10 % 0", "Division by zero");
}

// ── Overflow protection ───────────────────────────────────────────────────────

#[test]
fn test_integer_overflow_addition_errors() {
    // i64::MAX + 1 must produce a checked-arithmetic error, not wrap.
    assert_runtime_error("x = 9223372036854775807 + 1", "overflow");
}

#[test]
fn test_integer_overflow_multiplication_errors() {
    assert_runtime_error("x = 9223372036854775807 * 2", "overflow");
}

// ── Arithmetic via variables ──────────────────────────────────────────────────

#[test]
fn test_addition_stored_in_variable() {
    assert_eq!(eval_var("x = 10 + 5", "x"), Value::Number(15));
}

#[test]
fn test_variable_arithmetic_chain() {
    let code = "
a = 10
b = 3
c = a * b - 1
";
    assert_eq!(eval_var(code, "c"), Value::Number(29));
}

#[test]
fn test_accumulator_pattern() {
    let code = "
total = 0
total = total + 10
total = total + 20
total = total + 12
";
    assert_eq!(eval_var(code, "total"), Value::Number(42));
}

// ── Arithmetic inside functions ───────────────────────────────────────────────

#[test]
fn test_function_add_returns_correct_value() {
    let code = "
add(a, b) {
    return a + b
}
add(17, 25)
";
    assert_eq!(eval_last_number(code), 42);
}

#[test]
fn test_function_mul_returns_correct_value() {
    let code = "
mul(a, b) {
    return a * b
}
mul(6, 7)
";
    assert_eq!(eval_last_number(code), 42);
}

#[test]
fn test_function_power_by_squaring() {
    // Computes base^exp iteratively using only multiplication.
    let code = "
power(base, exp) {
    result = 1
    i = 0
    while i < exp {
        result = result * base
        i = i + 1
    }
    return result
}
power(2, 10)
";
    assert_eq!(eval_last_number(code), 1024);
}

#[test]
fn test_function_sum_1_to_n() {
    // Gauss formula: sum(1..100) = 5050
    let code = "
sum_to(n) {
    total = 0
    i = 1
    while i <= n {
        total = total + i
        i = i + 1
    }
    return total
}
sum_to(100)
";
    assert_eq!(eval_last_number(code), 5050);
}

#[test]
fn test_factorial_5_equals_120() {
    let code = "
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
factorial(5)
";
    assert_eq!(eval_last_number(code), 120);
}

#[test]
fn test_factorial_0_equals_1() {
    let code = "
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
factorial(0)
";
    assert_eq!(eval_last_number(code), 1);
}

#[test]
fn test_factorial_10_equals_3628800() {
    let code = "
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
factorial(10)
";
    assert_eq!(eval_last_number(code), 3_628_800);
}

// ── String × integer repetition ───────────────────────────────────────────────

#[test]
fn test_string_repeated_by_integer() {
    use common::eval_string;
    let result = eval_string(r#"x = "ab" * 3"#, "x");
    assert_eq!(result, "ababab");
}

#[test]
fn test_integer_times_string() {
    use common::eval_string;
    let result = eval_string(r#"x = 3 * "ab""#, "x");
    assert_eq!(result, "ababab");
}

#[test]
fn test_string_repeated_zero_times_is_empty() {
    use common::eval_string;
    let result = eval_string(r#"x = "hello" * 0"#, "x");
    assert_eq!(result, "");
}

#[test]
fn test_string_repetition_negative_errors() {
    assert_runtime_error(r#"x = "x" * -1"#, "negative");
}

// ── Comparison operators return bools ────────────────────────────────────────

#[test]
fn test_equality_true() {
    assert!(eval_bool("x = 5 == 5", "x"));
}

#[test]
fn test_equality_false() {
    assert!(!eval_bool("x = 5 == 6", "x"));
}

#[test]
fn test_not_equal_true() {
    assert!(eval_bool("x = 4 != 5", "x"));
}

#[test]
fn test_less_than_true() {
    assert!(eval_bool("x = 3 < 10", "x"));
}

#[test]
fn test_less_than_false() {
    assert!(!eval_bool("x = 10 < 3", "x"));
}

#[test]
fn test_greater_than_true() {
    assert!(eval_bool("x = 10 > 3", "x"));
}

#[test]
fn test_less_equal_equal_case() {
    assert!(eval_bool("x = 5 <= 5", "x"));
}

#[test]
fn test_less_equal_less_case() {
    assert!(eval_bool("x = 4 <= 5", "x"));
}

#[test]
fn test_greater_equal_equal_case() {
    assert!(eval_bool("x = 5 >= 5", "x"));
}

#[test]
fn test_mixed_int_float_equality() {
    // 5 (int) == 5.0 (float) must be true.
    assert!(eval_bool("x = 5 == 5.0", "x"));
}
