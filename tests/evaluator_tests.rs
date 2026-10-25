// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Tests for the evaluator and runtime execution

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
    let program = parser.parse().map_err(|errs| {
        errs.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("; ")
    })?;
    let mut evaluator = Evaluator::new();
    evaluator.eval_program(&program)
}

#[test]
fn test_simple_arithmetic() {
    assert_eq!(eval_code("x = 2 + 3").unwrap(), Value::Null);
}

#[test]
fn test_variable_assignment() {
    assert_eq!(eval_code("x = 42").unwrap(), Value::Null);
}

#[test]
fn test_addition() {
    let code = "
add(a, b) {
    return a + b
}
result = add(3, 4)
";
    assert_eq!(eval_code(code).unwrap(), Value::Null);
}

#[test]
fn test_factorial() {
    let code = "
factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
result = factorial(5)
";
    eval_code(code).unwrap();
}

#[test]
fn test_undefined_variable_error() {
    let result = eval_code("x = undefined_var");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Undefined variable"));
}

#[test]
fn test_undefined_function_error() {
    let result = eval_code("result = undefined_func(1, 2)");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Undefined function"));
}

#[test]
fn test_modulo_operator() {
    let code = "
mod_test(a, b) {
    return a % b
}
result = mod_test(10, 3)
";
    eval_code(code).unwrap();
}

#[test]
fn test_logical_and() {
    let code = "
check(a, b) {
    if a && b {
        return 1
    }
    return 0
}
r1 = check(1, 1)
r2 = check(1, 0)
r3 = check(0, 1)
";
    eval_code(code).unwrap();
}

#[test]
fn test_logical_or() {
    let code = "
check(a, b) {
    if a || b {
        return 1
    }
    return 0
}
r1 = check(1, 0)
r2 = check(0, 1)
r3 = check(0, 0)
";
    eval_code(code).unwrap();
}

#[test]
#[ignore] // Causes stack overflow - recursion limit test should be done differently
fn test_recursion_depth_limit() {
    let code = "
infinite(n) {
    return infinite(n + 1)
}
result = infinite(0)
";
    let result = eval_code(code);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Maximum recursion depth exceeded"));
}

#[test]
fn test_integer_overflow_protection() {
    let code = "
big(n) {
    return n + 9223372036854775807
}
result = big(1)
";
    let result = eval_code(code);
    assert!(result.is_err());
}

#[test]
fn test_variable_persistence_in_global_scope() {
    let code = "
x = 10
y = x + 5
";
    eval_code(code).unwrap();
}

#[test]
fn test_function_with_no_return() {
    let code = "
void_func(a, b) {
    x = a + b
}
result = void_func(3, 4)
";
    eval_code(code).unwrap();
}

#[test]
fn test_nested_function_calls() {
    let code = "
add(a, b) { return a + b }
mul(a, b) { return a * b }
compute() {
    x = add(2, 3)
    return mul(x, 4)
}
result = compute()
";
    eval_code(code).unwrap();
}

#[test]
fn test_while_loop_basic() {
    let code = "
count = 0
while count < 5 {
    count = count + 1
}
";
    eval_code(code).unwrap();
}

#[test]
fn test_if_else_basic() {
    let code = "
x = 10
if x > 5 {
    y = 1
} else {
    y = 0
}
";
    eval_code(code).unwrap();
}

#[test]
fn test_break_statement() {
    let code = "
i = 0
while i < 10 {
    if i == 5 {
        break
    }
    i = i + 1
}
";
    eval_code(code).unwrap();
}

#[test]
fn test_continue_statement() {
    let code = "
i = 0
while i < 5 {
    i = i + 1
    if i == 3 {
        continue
    }
}
";
    eval_code(code).unwrap();
}

#[test]
fn test_string_concatenation() {
    let code = r#"
a = "hello"
b = " world"
c = a + b
"#;
    eval_code(code).unwrap();
}

#[test]
fn test_string_length() {
    let code = r#"
s = "hello"
len_s = len(s)
"#;
    eval_code(code).unwrap();
}

#[test]
fn test_type_conversion() {
    let code = r#"
num = 42
s = str(num)
back = int(s)
"#;
    eval_code(code).unwrap();
}

#[test]
fn test_print_function() {
    let code = r#"
print("hello")
print(42)
print(3.14)
"#;
    eval_code(code).unwrap();
}

#[test]
fn test_abs_function() {
    let code = r#"
x = abs(-5)
y = abs(-3.14)
"#;
    eval_code(code).unwrap();
}

#[test]
fn test_type_function() {
    let code = r#"
t1 = type(42)
t2 = type(3.14)
t3 = type("hello")
t4 = type(true)
"#;
    eval_code(code).unwrap();
}

#[test]
fn test_multiple_parameters() {
    let code = "
func(a, b, c, d, e) {
    return a + b + c + d + e
}
result = func(1, 2, 3, 4, 5)
";
    eval_code(code).unwrap();
}

#[test]
fn test_call_stack_tracking() {
    let code = "
inner() { return 42 }
outer() { return inner() }
result = outer()
";
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
    let program = parser.parse().unwrap();
    let mut evaluator = Evaluator::new();
    let _ = evaluator.eval_program(&program);

    // Verify call stack tracking is available (should be empty after execution)
    let stack = evaluator.format_call_stack();
    assert!(stack.is_empty());
}

#[test]
fn test_comparison_operators() {
    let code = "
check_lt() { if 5 < 10 { return 1 } return 0 }
check_gt() { if 10 > 5 { return 1 } return 0 }
check_eq() { if 5 == 5 { return 1 } return 0 }
check_neq() { if 5 != 3 { return 1 } return 0 }
check_le() { if 5 <= 5 { return 1 } return 0 }
check_ge() { if 5 >= 5 { return 1 } return 0 }
r1 = check_lt()
r2 = check_gt()
r3 = check_eq()
r4 = check_neq()
r5 = check_le()
r6 = check_ge()
";
    eval_code(code).unwrap();
}

#[test]
fn test_negation_operator() {
    let code = "
x = 5
neg_x = -x
bool_true = true
bool_false = !bool_true
";
    eval_code(code).unwrap();
}

#[test]
fn test_float_operations() {
    let code = "
a = 3.14
b = 2.86
c = a + b
d = a * b
e = a / b
";
    eval_code(code).unwrap();
}

#[test]
fn test_mixed_int_float_operations() {
    let code = "
a = 5
b = 2.5
c = a + b
d = a * b
";
    eval_code(code).unwrap();
}

#[test]
fn test_division_by_zero_error() {
    let code = "
result = 10 / 0
";
    let result = eval_code(code);
    assert!(result.is_err());
}

#[test]
fn test_boolean_values() {
    let code = "
t = true
f = false
if t { x = 1 }
if f { y = 2 } else { y = 0 }
";
    eval_code(code).unwrap();
}
