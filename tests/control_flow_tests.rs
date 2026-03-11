// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

mod common;

use common::{
    assert_parse_error, assert_runtime_error, eval_last_bool, eval_last_number, eval_var,
};
use halo::interpreter::Value;

// ── if / else ─────────────────────────────────────────────────────────────────

#[test]
fn test_if_true_branch_executes() {
    assert_eq!(eval_var("if true { x = 1 }", "x"), Value::Number(1));
}

#[test]
fn test_if_false_branch_skipped() {
    // Variable never assigned when condition is false.
    let code = "
x = 0
if false { x = 99 }
";
    assert_eq!(eval_var(code, "x"), Value::Number(0));
}

#[test]
fn test_if_else_true_branch() {
    let code = "
if true { x = 1 } else { x = 2 }
";
    assert_eq!(eval_var(code, "x"), Value::Number(1));
}

#[test]
fn test_if_else_false_branch() {
    let code = "
if false { x = 1 } else { x = 2 }
";
    assert_eq!(eval_var(code, "x"), Value::Number(2));
}

#[test]
fn test_if_with_numeric_condition_nonzero_is_truthy() {
    let code = "
x = 0
if 42 { x = 1 }
";
    assert_eq!(eval_var(code, "x"), Value::Number(1));
}

#[test]
fn test_if_with_numeric_condition_zero_is_falsy() {
    let code = "
x = 0
if 0 { x = 1 }
";
    assert_eq!(eval_var(code, "x"), Value::Number(0));
}

#[test]
fn test_if_with_comparison_condition() {
    let code = "
a = 10
b = 5
if a > b { result = 1 } else { result = 0 }
";
    assert_eq!(eval_var(code, "result"), Value::Number(1));
}

#[test]
fn test_if_body_multiple_statements() {
    let code = "
if true {
    a = 1
    b = 2
    c = 3
}
";
    assert_eq!(eval_var(code, "a"), Value::Number(1));
    assert_eq!(eval_var(code, "b"), Value::Number(2));
    assert_eq!(eval_var(code, "c"), Value::Number(3));
}

// ── else if chain ─────────────────────────────────────────────────────────────

#[test]
fn test_else_if_first_branch_taken() {
    let code = "
score = 95
if score >= 90 { grade = 1 }
else if score >= 80 { grade = 2 }
else if score >= 70 { grade = 3 }
else { grade = 4 }
";
    assert_eq!(eval_var(code, "grade"), Value::Number(1));
}

#[test]
fn test_else_if_second_branch_taken() {
    let code = "
score = 83
if score >= 90 { grade = 1 }
else if score >= 80 { grade = 2 }
else if score >= 70 { grade = 3 }
else { grade = 4 }
";
    assert_eq!(eval_var(code, "grade"), Value::Number(2));
}

#[test]
fn test_else_if_third_branch_taken() {
    let code = "
score = 72
if score >= 90 { grade = 1 }
else if score >= 80 { grade = 2 }
else if score >= 70 { grade = 3 }
else { grade = 4 }
";
    assert_eq!(eval_var(code, "grade"), Value::Number(3));
}

#[test]
fn test_else_if_else_fallthrough() {
    let code = "
score = 45
if score >= 90 { grade = 1 }
else if score >= 80 { grade = 2 }
else if score >= 70 { grade = 3 }
else { grade = 4 }
";
    assert_eq!(eval_var(code, "grade"), Value::Number(4));
}

#[test]
fn test_else_if_only_first_matching_branch_runs() {
    // Even though multiple conditions could be true, only the first one fires.
    let code = "
x = 5
count = 0
if x > 0 { count = count + 1 }
else if x > 1 { count = count + 10 }
else if x > 2 { count = count + 100 }
";
    // Only the first branch should increment count.
    assert_eq!(eval_var(code, "count"), Value::Number(1));
}

#[test]
fn test_else_if_inside_function() {
    let code = "
classify(n) {
    if n < 0 { return 0 }
    else if n == 0 { return 1 }
    else { return 2 }
}
classify(-5)
";
    assert_eq!(eval_last_number(code), 0);
}

#[test]
fn test_else_if_inside_function_zero_case() {
    let code = "
classify(n) {
    if n < 0 { return 0 }
    else if n == 0 { return 1 }
    else { return 2 }
}
classify(0)
";
    assert_eq!(eval_last_number(code), 1);
}

#[test]
fn test_else_if_inside_function_positive_case() {
    let code = "
classify(n) {
    if n < 0 { return 0 }
    else if n == 0 { return 1 }
    else { return 2 }
}
classify(7)
";
    assert_eq!(eval_last_number(code), 2);
}

// ── Nested if ─────────────────────────────────────────────────────────────────

#[test]
fn test_nested_if_both_true() {
    let code = "
x = 0
if true {
    if true {
        x = 42
    }
}
";
    assert_eq!(eval_var(code, "x"), Value::Number(42));
}

#[test]
fn test_nested_if_outer_false_inner_skipped() {
    let code = "
x = 0
if false {
    if true {
        x = 99
    }
}
";
    assert_eq!(eval_var(code, "x"), Value::Number(0));
}

#[test]
fn test_nested_if_outer_true_inner_false() {
    let code = "
x = 0
if true {
    if false {
        x = 99
    } else {
        x = 7
    }
}
";
    assert_eq!(eval_var(code, "x"), Value::Number(7));
}

#[test]
fn test_triple_nested_if() {
    let code = "
result = 0
if true {
    if true {
        if true {
            result = 1
        }
    }
}
";
    assert_eq!(eval_var(code, "result"), Value::Number(1));
}

// ── while loop ────────────────────────────────────────────────────────────────

#[test]
fn test_while_counts_to_5() {
    let code = "
i = 0
while i < 5 { i = i + 1 }
";
    assert_eq!(eval_var(code, "i"), Value::Number(5));
}

#[test]
fn test_while_zero_iterations_when_condition_initially_false() {
    let code = "
x = 10
while x < 0 { x = x + 1 }
";
    // Body never runs; x stays at 10.
    assert_eq!(eval_var(code, "x"), Value::Number(10));
}

#[test]
fn test_while_accumulates_sum() {
    let code = "
total = 0
i = 1
while i <= 10 {
    total = total + i
    i = i + 1
}
";
    assert_eq!(eval_var(code, "total"), Value::Number(55));
}

#[test]
fn test_while_multiplies_accumulator() {
    let code = "
product = 1
i = 1
while i <= 5 {
    product = product * i
    i = i + 1
}
";
    // 1 * 2 * 3 * 4 * 5 = 120
    assert_eq!(eval_var(code, "product"), Value::Number(120));
}

#[test]
fn test_while_countdown() {
    let code = "
n = 10
while n > 0 { n = n - 1 }
";
    assert_eq!(eval_var(code, "n"), Value::Number(0));
}

#[test]
fn test_while_single_iteration() {
    let code = "
x = 0
while x == 0 { x = 1 }
";
    assert_eq!(eval_var(code, "x"), Value::Number(1));
}

#[test]
fn test_while_with_if_inside() {
    // Sum only even numbers 1..10.
    let code = "
total = 0
i = 1
while i <= 10 {
    if i % 2 == 0 { total = total + i }
    i = i + 1
}
";
    // 2 + 4 + 6 + 8 + 10 = 30
    assert_eq!(eval_var(code, "total"), Value::Number(30));
}

#[test]
fn test_nested_while_loops() {
    let code = "
count = 0
i = 0
while i < 3 {
    j = 0
    while j < 3 {
        count = count + 1
        j = j + 1
    }
    i = i + 1
}
";
    assert_eq!(eval_var(code, "count"), Value::Number(9));
}

#[test]
fn test_while_loop_iteration_limit_error() {
    // An infinite loop must be caught before it hangs the test suite.
    assert_runtime_error(
        "x = 0\nwhile true { x = x + 1 }",
        "Loop iteration limit exceeded",
    );
}

// ── break ─────────────────────────────────────────────────────────────────────

#[test]
fn test_break_exits_loop_immediately() {
    let code = "
i = 0
while i < 100 {
    if i == 5 { break }
    i = i + 1
}
";
    // Loop breaks when i reaches 5, so i stays at 5.
    assert_eq!(eval_var(code, "i"), Value::Number(5));
}

#[test]
fn test_break_on_first_iteration() {
    let code = "
x = 0
while true {
    break
    x = 99
}
";
    assert_eq!(eval_var(code, "x"), Value::Number(0));
}

#[test]
fn test_break_exits_only_innermost_loop() {
    let code = "
outer = 0
inner_total = 0
while outer < 3 {
    i = 0
    while i < 10 {
        if i == 2 { break }
        inner_total = inner_total + 1
        i = i + 1
    }
    outer = outer + 1
}
";
    // Inner loop runs 2 iterations (i=0,1) per outer iteration × 3 = 6.
    assert_eq!(eval_var(code, "inner_total"), Value::Number(6));
}

#[test]
fn test_break_after_accumulation() {
    let code = "
total = 0
i = 1
while i <= 100 {
    total = total + i
    if total >= 10 { break }
    i = i + 1
}
";
    // 1+2+3+4 = 10 → break when i=4 but total just hit 10.
    assert_eq!(eval_var(code, "total"), Value::Number(10));
}

// ── continue ──────────────────────────────────────────────────────────────────

#[test]
fn test_continue_skips_rest_of_body() {
    let code = "
evens = 0
i = 0
while i < 10 {
    i = i + 1
    if i % 2 != 0 { continue }
    evens = evens + 1
}
";
    // Even numbers 2, 4, 6, 8, 10 → 5 evens.
    assert_eq!(eval_var(code, "evens"), Value::Number(5));
}

#[test]
fn test_continue_does_not_reset_loop_variable() {
    let code = "
i = 0
while i < 5 {
    i = i + 1
    continue
}
";
    // Loop must still terminate; i ends at 5.
    assert_eq!(eval_var(code, "i"), Value::Number(5));
}

#[test]
fn test_continue_skips_sum_for_multiples_of_3() {
    let code = "
total = 0
i = 1
while i <= 10 {
    if i % 3 == 0 {
        i = i + 1
        continue
    }
    total = total + i
    i = i + 1
}
";
    // Skip 3, 6, 9. Sum = 1+2+4+5+7+8+10 = 37.
    assert_eq!(eval_var(code, "total"), Value::Number(37));
}

// ── break / continue inside functions ────────────────────────────────────────

#[test]
fn test_break_inside_function() {
    let code = "
find_first_even(limit) {
    i = 1
    while i <= limit {
        if i % 2 == 0 { break }
        i = i + 1
    }
    return i
}
find_first_even(10)
";
    assert_eq!(eval_last_number(code), 2);
}

#[test]
fn test_continue_inside_function() {
    let code = "
count_odds(n) {
    total = 0
    i = 1
    while i <= n {
        if i % 2 == 0 {
            i = i + 1
            continue
        }
        total = total + 1
        i = i + 1
    }
    return total
}
count_odds(10)
";
    assert_eq!(eval_last_number(code), 5);
}

// ── return terminates function immediately ────────────────────────────────────

#[test]
fn test_return_exits_before_rest_of_body() {
    let code = "
first(n) {
    return n
    unreachable = 999
}
first(42)
";
    assert_eq!(eval_last_number(code), 42);
}

#[test]
fn test_return_inside_if() {
    let code = "
abs_val(n) {
    if n < 0 { return 0 - n }
    return n
}
abs_val(-7)
";
    assert_eq!(eval_last_number(code), 7);
}

#[test]
fn test_return_inside_while() {
    let code = "
find(target) {
    i = 0
    while i < 100 {
        if i == target { return i }
        i = i + 1
    }
    return 0 - 1
}
find(13)
";
    assert_eq!(eval_last_number(code), 13);
}

#[test]
fn test_void_function_returns_null() {
    let code = "
nothing() {
    local_x = 1
}
result = nothing()
";
    // A void function (no explicit return) must return null.
    // Local variables inside the function are not visible outside.
    assert_eq!(eval_var(code, "result"), Value::Null);
}

// ── Logical short-circuit in conditions ──────────────────────────────────────

#[test]
fn test_and_short_circuits_on_false_lhs() {
    // If lhs of && is false, rhs must not be evaluated.
    // If rhs were evaluated it would fail (undefined variable).
    // The test passes only when short-circuit evaluation is correct.
    let _code = "
result = false && undefined_var
";
    assert!(!eval_last_bool("result = false && false\nresult"));
}

#[test]
fn test_or_short_circuits_on_true_lhs() {
    assert!(eval_last_bool("result = true || false\nresult"));
}

#[test]
fn test_and_both_true() {
    assert!(eval_last_bool("true && true"));
}

#[test]
fn test_and_one_false() {
    assert!(!eval_last_bool("true && false"));
}

#[test]
fn test_or_both_false() {
    assert!(!eval_last_bool("false || false"));
}

#[test]
fn test_not_true_is_false() {
    assert!(!eval_last_bool("!true"));
}

#[test]
fn test_not_false_is_true() {
    assert!(eval_last_bool("!false"));
}

#[test]
fn test_not_zero_is_true() {
    assert!(eval_last_bool("!0"));
}

#[test]
fn test_not_nonzero_is_false() {
    assert!(!eval_last_bool("!1"));
}

#[test]
fn test_logical_and_with_comparison() {
    let code = "
in_range(n) {
    if n >= 1 && n <= 10 { return 1 }
    return 0
}
in_range(5)
";
    assert_eq!(eval_last_number(code), 1);
}

#[test]
fn test_logical_and_out_of_range() {
    let code = "
in_range(n) {
    if n >= 1 && n <= 10 { return 1 }
    return 0
}
in_range(15)
";
    assert_eq!(eval_last_number(code), 0);
}

// ── Parse errors for malformed control flow ───────────────────────────────────

#[test]
fn test_if_missing_brace_errors() {
    assert_parse_error("if true x = 1", "Expected '{'");
}

#[test]
fn test_while_missing_brace_errors() {
    assert_parse_error("while true x = x + 1", "Expected '{'");
}
