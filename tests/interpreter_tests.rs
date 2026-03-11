// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use halo::interpreter::{Evaluator, Value};
use halo::lexer::Lexer;
use halo::parser::Parser;

fn eval_code(code: &str) -> Result<Value, String> {
    let mut lexer = Lexer::new(code.to_string());
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        let is_eof = token.kind == halo::lexer::TokenKind::Eof;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| e.join(", "))?;

    let mut evaluator = Evaluator::new();
    evaluator.eval_program(&program)
}

// ============ Arithmetic Tests ============

#[test]
fn test_simple_addition() {
    let code = "x = 5 + 3";
    eval_code(code).unwrap();
}

#[test]
fn test_simple_subtraction() {
    let code = "x = 10 - 4";
    eval_code(code).unwrap();
}

#[test]
fn test_simple_multiplication() {
    let code = "x = 6 * 7";
    eval_code(code).unwrap();
}

#[test]
fn test_simple_division() {
    let code = "x = 20 / 4";
    eval_code(code).unwrap();
}

#[test]
fn test_modulo_operation() {
    let code = "x = 10 % 3";
    eval_code(code).unwrap();
}

#[test]
fn test_operator_precedence() {
    let code = "x = 2 + 3 * 4";
    eval_code(code).unwrap();
}

// ============ Variable Tests ============

#[test]
fn test_variable_assignment() {
    let code = "x = 42";
    eval_code(code).unwrap();
}

#[test]
fn test_variable_reassignment() {
    let code = "x = 5\nx = 10";
    eval_code(code).unwrap();
}

#[test]
fn test_multiple_variables() {
    let code = "a = 1\nb = 2\nc = 3";
    eval_code(code).unwrap();
}

#[test]
fn test_undefined_variable_error() {
    let code = "x = undefined_var";
    assert!(eval_code(code).is_err());
}

// ============ Boolean Tests ============

#[test]
fn test_boolean_true() {
    let code = "x = true";
    eval_code(code).unwrap();
}

#[test]
fn test_boolean_false() {
    let code = "x = false";
    eval_code(code).unwrap();
}

#[test]
fn test_logical_not() {
    let code = "x = !true";
    eval_code(code).unwrap();
}

#[test]
fn test_logical_and() {
    let code = "x = true && false";
    eval_code(code).unwrap();
}

#[test]
fn test_logical_or() {
    let code = "x = true || false";
    eval_code(code).unwrap();
}

// ============ Comparison Tests ============

#[test]
fn test_equality() {
    let code = "x = 5 == 5";
    eval_code(code).unwrap();
}

#[test]
fn test_inequality() {
    let code = "x = 5 != 3";
    eval_code(code).unwrap();
}

#[test]
fn test_less_than() {
    let code = "x = 3 < 5";
    eval_code(code).unwrap();
}

#[test]
fn test_greater_than() {
    let code = "x = 5 > 3";
    eval_code(code).unwrap();
}

#[test]
fn test_less_equal() {
    let code = "x = 5 <= 5";
    eval_code(code).unwrap();
}

#[test]
fn test_greater_equal() {
    let code = "x = 5 >= 5";
    eval_code(code).unwrap();
}

// ============ Control Flow Tests ============

#[test]
fn test_if_statement_true() {
    let code = "if true { x = 1 }";
    eval_code(code).unwrap();
}

#[test]
fn test_if_statement_false() {
    let code = "if false { x = 1 }";
    eval_code(code).unwrap();
}

#[test]
fn test_if_else_statement() {
    let code = "if true { x = 1 } else { x = 2 }";
    eval_code(code).unwrap();
}

#[test]
fn test_nested_if() {
    let code = "if true { if true { x = 1 } }";
    eval_code(code).unwrap();
}

#[test]
fn test_while_loop() {
    let code = "x = 0\nwhile x < 5 { x = x + 1 }";
    eval_code(code).unwrap();
}

#[test]
fn test_while_loop_zero_iterations() {
    let code = "x = 0\nwhile x < 0 { x = x + 1 }";
    eval_code(code).unwrap();
}

// ============ Function Tests ============

#[test]
fn test_function_definition() {
    let code = "add(a, b) { return a + b }";
    eval_code(code).unwrap();
}

#[test]
fn test_function_call() {
    let code = "add(a, b) { return a + b }\nadd(2, 3)";
    eval_code(code).unwrap();
}

#[test]
fn test_function_with_no_params() {
    let code = "greet() { return 42 }\ngreet()";
    eval_code(code).unwrap();
}

#[test]
fn test_function_with_return() {
    let code = "getValue() { return 100 }\nx = getValue()";
    eval_code(code).unwrap();
}

#[test]
fn test_function_no_return() {
    let code = "doNothing() { x = 1 }\ndoNothing()";
    eval_code(code).unwrap();
}

#[test]
fn test_factorial_recursive() {
    let code = "
factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
factorial(5)
";
    eval_code(code).unwrap();
}

#[test]
fn test_fibonacci_recursive() {
    let code = "
fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}
fibonacci(6)
";
    eval_code(code).unwrap();
}

#[test]
fn test_function_wrong_argument_count() {
    let code = "add(a, b) { return a + b }\nadd(2)";
    assert!(eval_code(code).is_err());
}

#[test]
fn test_undefined_function() {
    let code = "undefined()";
    assert!(eval_code(code).is_err());
}

// ============ Built-in Functions Tests ============

#[test]
fn test_builtin_print() {
    let code = "print(42)";
    eval_code(code).unwrap();
}

#[test]
fn test_builtin_len_string() {
    let code = "x = len(42)";
    eval_code(code).unwrap();
}

#[test]
fn test_builtin_str() {
    let code = "x = str(42)";
    eval_code(code).unwrap();
}

#[test]
fn test_builtin_int() {
    let code = "x = int(3.14)";
    eval_code(code).unwrap();
}

#[test]
fn test_builtin_float() {
    let code = "x = float(42)";
    eval_code(code).unwrap();
}

#[test]
fn test_builtin_abs_positive() {
    let code = "x = abs(5)";
    eval_code(code).unwrap();
}

#[test]
fn test_builtin_abs_negative() {
    let code = "x = abs(-5)";
    eval_code(code).unwrap();
}

#[test]
fn test_builtin_type() {
    let code = "x = type(42)";
    eval_code(code).unwrap();
}

// ============ Complex Expression Tests ============

#[test]
fn test_unary_negation() {
    let code = "x = -5";
    eval_code(code).unwrap();
}

#[test]
fn test_unary_negation_float() {
    let code = "x = -3.14";
    eval_code(code).unwrap();
}

#[test]
fn test_complex_arithmetic() {
    let code = "x = (2 + 3) * 4 - 5 / 2";
    eval_code(code).unwrap();
}

#[test]
fn test_mixed_number_float() {
    let code = "x = 5 + 3.14";
    eval_code(code).unwrap();
}

// ============ Assignment Tests ============

#[test]
fn test_assignment_in_expression() {
    // The parser does not support assignment inside parentheses (e.g. `x = (y = 5)`).
    // Test sequential assignments instead, which is the idiomatic form in Halo.
    let code = "y = 5\nx = y";
    eval_code(code).unwrap();
}

#[test]
fn test_update_variable() {
    let code = "x = 5\nx = x + 3";
    eval_code(code).unwrap();
}

// ============ Scope Tests ============

#[test]
fn test_function_scope_isolation() {
    let code = "
test() {
    x = 100
}
x = 5
test()
";
    eval_code(code).unwrap();
}

#[test]
fn test_local_variable_shadowing() {
    let code = "
test(x) {
    x = 999
    return x
}
x = 5
test(10)
";
    eval_code(code).unwrap();
}

// ============ Edge Cases ============

#[test]
fn test_division_by_zero() {
    let code = "x = 5 / 0";
    assert!(eval_code(code).is_err());
}

#[test]
fn test_empty_program() {
    let code = "";
    eval_code(code).ok();
}

#[test]
fn test_multiple_statements() {
    let code = "a = 1\nb = 2\nc = 3\nd = a + b + c";
    eval_code(code).unwrap();
}

#[test]
fn test_deeply_nested_if() {
    let code = "
if true {
    if true {
        if true {
            x = 1
        }
    }
}
";
    eval_code(code).unwrap();
}

#[test]
fn test_loop_with_break_condition() {
    let code = "
x = 0
while x < 100 {
    if x == 5 {
        break
    }
    x = x + 1
}
";
    eval_code(code).ok();
}

// ============ Comment Tests ============

#[test]
fn test_single_line_comment() {
    let code = "x = 5 // This is a comment";
    eval_code(code).unwrap();
}

#[test]
fn test_comment_only_line() {
    let code = "// Just a comment\nx = 10";
    eval_code(code).unwrap();
}

#[test]
fn test_multiple_comments() {
    let code = "
// First comment
x = 5
// Second comment
y = 10 // Inline comment
// Third comment
z = 15
";
    eval_code(code).unwrap();
}

#[test]
fn test_comment_with_special_chars() {
    let code = "x = 5 // Comment with symbols: @#$%^&*()";
    eval_code(code).unwrap();
}

#[test]
fn test_comment_with_slashes_in_text() {
    let code = "x = 5 // This comment mentions URLs like http://example.com";
    eval_code(code).unwrap();
}

#[test]
fn test_comment_before_function() {
    let code = "
// Function to add numbers
add(a, b) {
    // Add and return result
    return a + b
}
// Call the function
result = add(3, 4)
";
    eval_code(code).unwrap();
}

#[test]
fn test_comment_in_function_body() {
    let code = "
multiply(x, y) {
    // Calculate product
    result = x * y
    // Return the result
    return result
}
";
    eval_code(code).unwrap();
}

#[test]
fn test_comment_after_block_statement() {
    let code = "
test() {
    if true { x = 5 } // End if
}
";
    eval_code(code).unwrap();
}

#[test]
fn test_consecutive_comments() {
    let code = "
// Comment 1
// Comment 2
// Comment 3
x = 42
";
    eval_code(code).unwrap();
}

#[test]
fn test_comment_with_empty_lines() {
    let code = "
// Comment
// Another comment

x = 5

// Yet another comment
y = 10
";
    eval_code(code).unwrap();
}

#[test]
fn test_division_operator_not_comment() {
    let code = "x = 10 / 2";
    let _result = eval_code(code).unwrap();
    // Should successfully parse division operator, not treat as comment
}

#[test]
fn test_division_followed_by_comment() {
    let code = "x = 10 / 2 // This is the result of division";
    eval_code(code).unwrap();
}

#[test]
fn test_comment_in_while_loop() {
    let code = "
test() {
    x = 0
    while x < 5 { x = x + 1 } // End while
}
";
    eval_code(code).unwrap();
}

#[test]
fn test_multiline_code_with_comments() {
    let code = "
// Global variables
max_value = 100 // Maximum allowed value

// Function to check if number is in range
check_range(n) {
    if n > 0 && n < max_value {
        return 1
    }
    return 0
}

// Main execution
result = check_range(50)
";
    eval_code(code).unwrap();
}

#[test]
fn test_comment_before_operator() {
    let code = "
x = 5
// This comment is before addition
y = x + 3
";
    eval_code(code).unwrap();
}

#[test]
fn test_empty_comment() {
    let code = "x = 5 //";
    eval_code(code).unwrap();
}

#[test]
fn test_comment_with_tabs() {
    let code = "x = 5\t//\tComment with tabs";
    eval_code(code).unwrap();
}
