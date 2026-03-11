// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

mod common;

use common::{assert_runtime_error, eval_last_number, eval_var};
use halo::interpreter::Value;

// ── Basic definition and call ─────────────────────────────────────────────────

#[test]
fn test_function_returns_constant() {
    let code = "
answer() {
    return 42
}
answer()
";
    assert_eq!(eval_last_number(code), 42);
}

#[test]
fn test_function_single_param() {
    let code = "
double(x) {
    return x * 2
}
double(21)
";
    assert_eq!(eval_last_number(code), 42);
}

#[test]
fn test_function_two_params_add() {
    let code = "
add(a, b) {
    return a + b
}
add(17, 25)
";
    assert_eq!(eval_last_number(code), 42);
}

#[test]
fn test_function_two_params_sub() {
    let code = "
sub(a, b) {
    return a - b
}
sub(50, 8)
";
    assert_eq!(eval_last_number(code), 42);
}

#[test]
fn test_function_two_params_mul() {
    let code = "
mul(a, b) {
    return a * b
}
mul(6, 7)
";
    assert_eq!(eval_last_number(code), 42);
}

#[test]
fn test_function_three_params() {
    let code = "
sum3(a, b, c) {
    return a + b + c
}
sum3(10, 20, 12)
";
    assert_eq!(eval_last_number(code), 42);
}

#[test]
fn test_function_result_stored_in_variable() {
    let code = "
square(n) {
    return n * n
}
result = square(7)
";
    assert_eq!(eval_var(code, "result"), Value::Number(49));
}

#[test]
fn test_function_call_used_in_expression() {
    let code = "
square(n) { return n * n }
square(3) + square(4)
";
    // 9 + 16 = 25
    assert_eq!(eval_last_number(code), 25);
}

#[test]
fn test_function_call_as_argument() {
    let code = "
inc(n) { return n + 1 }
double(n) { return n * 2 }
double(inc(4))
";
    // double(5) = 10
    assert_eq!(eval_last_number(code), 10);
}

#[test]
fn test_nested_calls_three_deep() {
    let code = "
inc(n) { return n + 1 }
double(n) { return n * 2 }
negate(n) { return 0 - n }
negate(double(inc(4)))
";
    // negate(double(5)) = negate(10) = -10
    assert_eq!(eval_last_number(code), -10);
}

// ── Void functions ────────────────────────────────────────────────────────────

#[test]
fn test_void_function_returns_null() {
    let code = "
do_nothing() {
    local_x = 1
}
result = do_nothing()
";
    // A void function (no explicit return) must return null.
    // Local variables inside the function are not visible outside.
    assert_eq!(eval_var(code, "result"), Value::Null);
}

#[test]
fn test_early_return_no_value() {
    let code = "
set_flag(cond) {
    if cond { return 1 }
    return 99
}
result = set_flag(true)
";
    // Plain `return` with a value must exit before reaching the second return.
    assert_eq!(eval_var(code, "result"), Value::Number(1));
}

// ── Arity errors ──────────────────────────────────────────────────────────────

#[test]
fn test_too_few_arguments_errors() {
    let code = "
add(a, b) { return a + b }
add(1)
";
    assert_runtime_error(code, "expects");
}

#[test]
fn test_too_many_arguments_errors() {
    let code = "
add(a, b) { return a + b }
add(1, 2, 3)
";
    assert_runtime_error(code, "expects");
}

#[test]
fn test_zero_param_called_with_arg_errors() {
    let code = "
greet() { return 1 }
greet(42)
";
    assert_runtime_error(code, "expects");
}

#[test]
fn test_undefined_function_errors() {
    assert_runtime_error("ghost()", "Undefined function");
}

// ── Local variable scope ──────────────────────────────────────────────────────

#[test]
fn test_local_var_does_not_leak_to_caller() {
    let code = "
foo() {
    secret = 99
}
secret = 0
foo()
";
    // The local `secret` inside foo must not overwrite the outer one.
    assert_eq!(eval_var(code, "secret"), Value::Number(0));
}

#[test]
fn test_param_does_not_overwrite_outer_variable() {
    let code = "
square(x) {
    return x * x
}
x = 5
square(10)
";
    // Outer `x` must still be 5 after the call.
    assert_eq!(eval_var(code, "x"), Value::Number(5));
}

#[test]
fn test_multiple_calls_do_not_share_locals() {
    let code = "
bump(n) {
    local = n + 1
    return local
}
a = bump(1)
b = bump(10)
";
    assert_eq!(eval_var(code, "a"), Value::Number(2));
    assert_eq!(eval_var(code, "b"), Value::Number(11));
}

#[test]
fn test_function_sees_global_variable() {
    let code = "
base = 100
add_base(n) {
    return n + base
}
add_base(5)
";
    assert_eq!(eval_last_number(code), 105);
}

// ── Call-before-definition ────────────────────────────────────────────────────

#[test]
fn test_call_before_definition_works() {
    // The interpreter pre-registers all functions before executing statements,
    // so calling a function that is textually defined later must succeed.
    let code = "
result = triple(7)
triple(n) { return n * 3 }
";
    assert_eq!(eval_var(code, "result"), Value::Number(21));
}

// ── Recursion ─────────────────────────────────────────────────────────────────

#[test]
fn test_factorial_base_case_0() {
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
fn test_factorial_base_case_1() {
    let code = "
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
factorial(1)
";
    assert_eq!(eval_last_number(code), 1);
}

#[test]
fn test_factorial_5() {
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
fn test_factorial_10() {
    let code = "
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
factorial(10)
";
    assert_eq!(eval_last_number(code), 3_628_800);
}

#[test]
fn test_fibonacci_0() {
    let code = "
fib(n) {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}
fib(0)
";
    assert_eq!(eval_last_number(code), 0);
}

#[test]
fn test_fibonacci_1() {
    let code = "
fib(n) {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}
fib(1)
";
    assert_eq!(eval_last_number(code), 1);
}

#[test]
fn test_fibonacci_6() {
    let code = "
fib(n) {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}
fib(6)
";
    // 0,1,1,2,3,5,8 → fib(6)=8
    assert_eq!(eval_last_number(code), 8);
}

#[test]
fn test_fibonacci_10() {
    let code = "
fib(n) {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}
fib(10)
";
    assert_eq!(eval_last_number(code), 55);
}

#[test]
fn test_recursive_sum_1_to_n() {
    let code = "
sum(n) {
    if n <= 0 { return 0 }
    return n + sum(n - 1)
}
sum(10)
";
    assert_eq!(eval_last_number(code), 55);
}

#[test]
fn test_recursive_power() {
    let code = "
pow(base, exp) {
    if exp == 0 { return 1 }
    return base * pow(base, exp - 1)
}
pow(2, 8)
";
    assert_eq!(eval_last_number(code), 256);
}

#[test]
fn test_recursive_countdown_returns_zero() {
    let code = "
countdown(n) {
    if n <= 0 { return 0 }
    return countdown(n - 1)
}
countdown(20)
";
    assert_eq!(eval_last_number(code), 0);
}

#[test]
fn test_recursive_gcd() {
    // Euclidean GCD.
    let code = "
gcd(a, b) {
    if b == 0 { return a }
    return gcd(b, a % b)
}
gcd(48, 18)
";
    assert_eq!(eval_last_number(code), 6);
}

#[test]
fn test_recursive_gcd_coprime() {
    let code = "
gcd(a, b) {
    if b == 0 { return a }
    return gcd(b, a % b)
}
gcd(13, 7)
";
    assert_eq!(eval_last_number(code), 1);
}

// ── Mutual recursion ──────────────────────────────────────────────────────────

#[test]
fn test_mutual_recursion_is_even() {
    let code = "
is_even(n) {
    if n == 0 { return 1 }
    return is_odd(n - 1)
}
is_odd(n) {
    if n == 0 { return 0 }
    return is_even(n - 1)
}
is_even(8)
";
    assert_eq!(eval_last_number(code), 1);
}

#[test]
fn test_mutual_recursion_is_odd() {
    let code = "
is_even(n) {
    if n == 0 { return 1 }
    return is_odd(n - 1)
}
is_odd(n) {
    if n == 0 { return 0 }
    return is_even(n - 1)
}
is_odd(7)
";
    assert_eq!(eval_last_number(code), 1);
}

#[test]
fn test_mutual_recursion_odd_even_boundary() {
    let code = "
is_even(n) {
    if n == 0 { return 1 }
    return is_odd(n - 1)
}
is_odd(n) {
    if n == 0 { return 0 }
    return is_even(n - 1)
}
is_even(1)
";
    assert_eq!(eval_last_number(code), 0);
}

// ── Iterative equivalents ─────────────────────────────────────────────────────

#[test]
fn test_iterative_factorial_matches_recursive() {
    let code_iter = "
factorial_iter(n) {
    result = 1
    i = 2
    while i <= n {
        result = result * i
        i = i + 1
    }
    return result
}
factorial_iter(7)
";
    let code_rec = "
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
factorial(7)
";
    assert_eq!(eval_last_number(code_iter), eval_last_number(code_rec));
}

#[test]
fn test_iterative_fibonacci_matches_recursive() {
    let code_iter = "
fib_iter(n) {
    if n <= 1 { return n }
    a = 0
    b = 1
    i = 2
    while i <= n {
        tmp = a + b
        a = b
        b = tmp
        i = i + 1
    }
    return b
}
fib_iter(10)
";
    // fib(10) = 55
    assert_eq!(eval_last_number(code_iter), 55);
}

// ── Recursion depth limit ─────────────────────────────────────────────────────

#[test]
fn test_infinite_recursion_hits_depth_limit() {
    // Run in a thread with a larger stack so the Rust call stack does not
    // overflow before the interpreter's own recursion-depth guard fires.
    let code = "
infinite(n) { return infinite(n + 1) }
infinite(0)
";
    let code = code.to_string();
    let handle = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024) // 64 MiB
        .spawn(move || {
            common::assert_runtime_error(&code, "recursion depth exceeded");
        })
        .expect("failed to spawn thread");
    handle.join().expect("thread panicked");
}

#[test]
fn test_deeply_nested_but_finite_recursion_succeeds() {
    // 10 levels deep — well within the limit even in test builds (limit = 15).
    let code = "
deep(n) {
    if n <= 0 { return 0 }
    return deep(n - 1)
}
deep(10)
";
    assert_eq!(eval_last_number(code), 0);
}

// ── Multiple functions defined, only one called ───────────────────────────────

#[test]
fn test_only_called_function_runs() {
    let code = "
side_effect = 0
touch() { side_effect = 99 }
safe() { return 1 }
safe()
";
    // `touch` is defined but never called; side_effect must stay 0.
    assert_eq!(eval_var(code, "side_effect"), Value::Number(0));
}

// ── Functions with loops ──────────────────────────────────────────────────────

#[test]
fn test_function_with_while_loop() {
    let code = "
count_down(n) {
    total = 0
    while n > 0 {
        total = total + n
        n = n - 1
    }
    return total
}
count_down(5)
";
    // 5+4+3+2+1 = 15
    assert_eq!(eval_last_number(code), 15);
}

#[test]
fn test_function_with_break() {
    let code = "
first_gt(limit) {
    i = 1
    while i <= 100 {
        if i > limit { break }
        i = i + 1
    }
    return i
}
first_gt(7)
";
    assert_eq!(eval_last_number(code), 8);
}

#[test]
fn test_function_with_continue_counts_odds() {
    let code = "
count_odd(n) {
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
count_odd(10)
";
    assert_eq!(eval_last_number(code), 5);
}

// ── Chained function calls as expressions ────────────────────────────────────

#[test]
fn test_chained_calls_in_arithmetic() {
    let code = "
inc(n) { return n + 1 }
square(n) { return n * n }
square(inc(3)) + square(inc(4))
";
    // square(4) + square(5) = 16 + 25 = 41
    assert_eq!(eval_last_number(code), 41);
}

#[test]
fn test_function_result_as_condition() {
    let code = "
is_positive(n) {
    if n > 0 { return 1 }
    return 0
}
result = 0
if is_positive(5) { result = 1 }
";
    assert_eq!(eval_var(code, "result"), Value::Number(1));
}
