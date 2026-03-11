// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

mod common;

use common::{assert_runtime_error, eval_last_number, eval_var};
use halo::interpreter::Value;

// ── Variable declaration and reassignment ────────────────────────────────────

#[test]
fn test_variable_initial_value() {
    assert_eq!(eval_var("x = 42", "x"), Value::Number(42));
}

#[test]
fn test_variable_reassignment_updates_value() {
    let code = "
x = 10
x = 20
";
    assert_eq!(eval_var(code, "x"), Value::Number(20));
}

#[test]
fn test_variable_reassignment_multiple_times() {
    let code = "
x = 1
x = 2
x = 3
x = 4
";
    assert_eq!(eval_var(code, "x"), Value::Number(4));
}

#[test]
fn test_variable_self_increment() {
    let code = "
x = 5
x = x + 1
";
    assert_eq!(eval_var(code, "x"), Value::Number(6));
}

#[test]
fn test_variable_self_decrement() {
    let code = "
x = 10
x = x - 3
";
    assert_eq!(eval_var(code, "x"), Value::Number(7));
}

#[test]
fn test_variable_self_multiply() {
    let code = "
x = 4
x = x * x
";
    assert_eq!(eval_var(code, "x"), Value::Number(16));
}

#[test]
fn test_variable_self_divide() {
    let code = "
x = 20
x = x / 4
";
    assert_eq!(eval_var(code, "x"), Value::Number(5));
}

#[test]
fn test_multiple_independent_variables() {
    let code = "
a = 1
b = 2
c = 3
";
    assert_eq!(eval_var(code, "a"), Value::Number(1));
    assert_eq!(eval_var(code, "b"), Value::Number(2));
    assert_eq!(eval_var(code, "c"), Value::Number(3));
}

#[test]
fn test_variable_used_in_another_variable_init() {
    let code = "
a = 7
b = a * 6
";
    assert_eq!(eval_var(code, "b"), Value::Number(42));
}

#[test]
fn test_variables_do_not_interfere_with_each_other() {
    let code = "
x = 10
y = 20
x = x + 1
";
    // Only x changes; y stays untouched.
    assert_eq!(eval_var(code, "y"), Value::Number(20));
}

#[test]
fn test_undefined_variable_produces_error() {
    assert_runtime_error("x = undefined_var", "Undefined variable");
}

#[test]
fn test_variable_used_before_assignment_errors() {
    assert_runtime_error("y = x + 1", "Undefined variable");
}

// ── Global variables ─────────────────────────────────────────────────────────

#[test]
fn test_global_variable_readable_in_function() {
    let code = "
base = 100
add_base(n) {
    return n + base
}
add_base(5)
";
    assert_eq!(eval_last_number(code), 105);
}

#[test]
fn test_multiple_globals_readable_in_function() {
    let code = "
a = 10
b = 32
sum_globals() {
    return a + b
}
sum_globals()
";
    assert_eq!(eval_last_number(code), 42);
}

#[test]
fn test_global_used_as_loop_limit() {
    let code = "
limit = 5
total = 0
i = 1
while i <= limit {
    total = total + i
    i = i + 1
}
";
    // 1+2+3+4+5 = 15
    assert_eq!(eval_var(code, "total"), Value::Number(15));
}

// ── Function scope isolation ─────────────────────────────────────────────────

#[test]
fn test_local_variable_invisible_outside_function() {
    // A variable declared inside a function must not be visible at top level
    // after the call returns.
    let _code = "
set_local() {
    secret = 99
}
set_local()
";
    // Reading `secret` after the call must fail.
    assert_runtime_error(
        "set_local() { secret = 99 }\nset_local()\nx = secret",
        "Undefined variable",
    );
}

#[test]
fn test_function_local_does_not_overwrite_global() {
    let code = "
x = 5
set_x() {
    x = 100
}
x = 5
set_x()
";
    // The local `x` inside set_x must not change the outer `x`.
    assert_eq!(eval_var(code, "x"), Value::Number(5));
}

#[test]
fn test_two_calls_to_same_function_do_not_share_locals() {
    let code = "
bump(n) {
    local = n + 1
    return local
}
a = bump(10)
b = bump(20)
";
    assert_eq!(eval_var(code, "a"), Value::Number(11));
    assert_eq!(eval_var(code, "b"), Value::Number(21));
}

#[test]
fn test_function_param_is_independent_copy() {
    // Mutating a parameter inside a function must not affect the caller's variable.
    let code = "
double_in_place(x) {
    x = x * 2
    return x
}
original = 7
result = double_in_place(original)
";
    // `original` must still be 7.
    assert_eq!(eval_var(code, "original"), Value::Number(7));
    assert_eq!(eval_var(code, "result"), Value::Number(14));
}

#[test]
fn test_function_local_shadowing_global_reads_local() {
    // Inside the function, `x` refers to the local binding, not the global.
    let code = "
x = 1
shadow() {
    x = 99
    return x
}
shadow()
";
    assert_eq!(eval_last_number(code), 99);
}

#[test]
fn test_function_local_shadowing_does_not_mutate_global() {
    let code = "
x = 1
shadow() {
    x = 99
}
x = 1
shadow()
";
    assert_eq!(eval_var(code, "x"), Value::Number(1));
}

// ── Parameter shadowing ──────────────────────────────────────────────────────

#[test]
fn test_param_name_same_as_global_reads_param_value() {
    let code = "
value = 999
get(value) {
    return value
}
get(42)
";
    assert_eq!(eval_last_number(code), 42);
}

#[test]
fn test_param_mutation_does_not_propagate_to_global() {
    let code = "
n = 10
mutate(n) {
    n = n + 1
    return n
}
result = mutate(n)
";
    assert_eq!(eval_var(code, "n"), Value::Number(10));
    assert_eq!(eval_var(code, "result"), Value::Number(11));
}

#[test]
fn test_param_used_as_loop_counter() {
    let code = "
count_down(n) {
    total = 0
    while n > 0 {
        total = total + n
        n = n - 1
    }
    return total
}
outer_n = 5
count_down(outer_n)
";
    // outer_n must not change; return value = 5+4+3+2+1 = 15.
    assert_eq!(eval_last_number(code), 15);
    assert_eq!(eval_var(code, "outer_n"), Value::Number(5));
}

// ── If-block scoping ─────────────────────────────────────────────────────────

#[test]
fn test_variable_assigned_in_if_body_visible_after() {
    // Halo does not introduce a new scope for if/while bodies (only function
    // calls create new scopes).  A variable set inside `if` is visible after.
    let code = "
if true { result = 42 }
";
    assert_eq!(eval_var(code, "result"), Value::Number(42));
}

#[test]
fn test_variable_assigned_in_else_body_visible_after() {
    let code = "
if false { result = 1 } else { result = 2 }
";
    assert_eq!(eval_var(code, "result"), Value::Number(2));
}

#[test]
fn test_variable_set_in_both_branches_has_correct_value() {
    let code = "
flag = 1
if flag == 1 { outcome = 10 } else { outcome = 20 }
";
    assert_eq!(eval_var(code, "outcome"), Value::Number(10));
}

#[test]
fn test_counter_incremented_inside_if() {
    let code = "
count = 0
if true { count = count + 1 }
if true { count = count + 1 }
if false { count = count + 100 }
";
    assert_eq!(eval_var(code, "count"), Value::Number(2));
}

// ── While-loop scoping ───────────────────────────────────────────────────────

#[test]
fn test_variable_assigned_in_while_body_visible_after() {
    let code = "
last = 0
i = 1
while i <= 5 {
    last = i
    i = i + 1
}
";
    assert_eq!(eval_var(code, "last"), Value::Number(5));
}

#[test]
fn test_loop_counter_visible_after_loop() {
    let code = "
i = 0
while i < 10 { i = i + 1 }
";
    assert_eq!(eval_var(code, "i"), Value::Number(10));
}

#[test]
fn test_multiple_vars_accumulated_in_loop() {
    let code = "
sum_odd = 0
sum_even = 0
i = 1
while i <= 10 {
    if i % 2 == 0 { sum_even = sum_even + i }
    else { sum_odd = sum_odd + i }
    i = i + 1
}
";
    // Odd  1+3+5+7+9  = 25
    // Even 2+4+6+8+10 = 30
    assert_eq!(eval_var(code, "sum_odd"), Value::Number(25));
    assert_eq!(eval_var(code, "sum_even"), Value::Number(30));
}

// ── Nested function scopes ───────────────────────────────────────────────────

#[test]
fn test_recursive_function_frame_isolation() {
    // Each recursive call must have its own `n`; results must be independent.
    let code = "
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
a = factorial(5)
b = factorial(3)
";
    assert_eq!(eval_var(code, "a"), Value::Number(120));
    assert_eq!(eval_var(code, "b"), Value::Number(6));
}

#[test]
fn test_mutually_recursive_frames_do_not_share_locals() {
    let code = "
is_even(n) {
    if n == 0 { return 1 }
    return is_odd(n - 1)
}
is_odd(n) {
    if n == 0 { return 0 }
    return is_even(n - 1)
}
e = is_even(6)
o = is_odd(6)
";
    assert_eq!(eval_var(code, "e"), Value::Number(1));
    assert_eq!(eval_var(code, "o"), Value::Number(0));
}

#[test]
fn test_function_called_multiple_times_sees_fresh_locals() {
    let code = "
add_one(n) {
    result = n + 1
    return result
}
x = add_one(10)
y = add_one(20)
z = add_one(30)
";
    assert_eq!(eval_var(code, "x"), Value::Number(11));
    assert_eq!(eval_var(code, "y"), Value::Number(21));
    assert_eq!(eval_var(code, "z"), Value::Number(31));
}

// ── Call-before-definition ───────────────────────────────────────────────────

#[test]
fn test_global_var_set_before_function_definition() {
    let code = "
result = triple(7)
triple(n) { return n * 3 }
";
    assert_eq!(eval_var(code, "result"), Value::Number(21));
}

#[test]
fn test_function_defined_after_call_in_another_function() {
    // Both `caller` and `callee` are defined after the top-level call.
    let code = "
caller() { return callee(6) }
callee(n) { return n * 7 }
caller()
";
    assert_eq!(eval_last_number(code), 42);
}

// ── Variable type changes ────────────────────────────────────────────────────

#[test]
fn test_variable_can_change_from_number_to_bool() {
    // Halo is dynamically typed; a variable can hold any value type.
    let code = "
x = 42
x = true
";
    assert_eq!(eval_var(code, "x"), Value::Bool(true));
}

#[test]
fn test_variable_can_change_from_bool_to_string() {
    let code = r#"
flag = false
flag = "yes"
"#;
    assert_eq!(eval_var(code, "flag"), Value::String("yes".to_string()));
}

#[test]
#[allow(clippy::approx_constant)] // intentionally parsing the literal "3.14" from source
fn test_variable_can_change_from_int_to_float() {
    let code = "
n = 1
n = 3.14
";
    assert_eq!(eval_var(code, "n"), Value::Float(3.14));
}
