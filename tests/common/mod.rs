// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

// Each integration-test crate compiles `common` independently, so the Rust
// compiler flags helpers that a particular crate doesn't import as unused.
// All functions here are intentionally shared across multiple test files.
#![allow(dead_code)]

use halo::interpreter::{Evaluator, Value};
use halo::lexer::{Lexer, TokenKind};
use halo::parser::Parser;

// ── Core pipeline ─────────────────────────────────────────────────────────────

/// Lex, parse, and evaluate a Halo source snippet.
///
/// Returns `Ok(last_value)` on success, or `Err(message)` on any
/// lex / parse / runtime error.
pub fn eval_code(code: &str) -> Result<Value, String> {
    let mut lexer = Lexer::new(code.to_string());
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        let done = tok.kind == TokenKind::Eof;
        tokens.push(tok);
        if done {
            break;
        }
    }

    let program = Parser::new(tokens)
        .parse()
        .map_err(|errs| errs.join("; "))?;

    Evaluator::new().eval_program(&program)
}

// ── Value extractors ──────────────────────────────────────────────────────────

/// Evaluate `code`, then read back the named variable `var` as the result.
///
/// Appends `\n<var>` to `code` so that `eval_program` returns the variable's
/// current value as the last evaluated expression.
///
/// # Panics
///
/// Panics if evaluation fails or the variable is not defined.
pub fn eval_var(code: &str, var: &str) -> Value {
    let full = format!("{}\n{}", code, var);
    eval_code(&full).unwrap_or_else(|e| panic!("eval_var('{var}'): {e}"))
}

/// Evaluate `code`, read back `var`, and return it as an `i64`.
///
/// # Panics
///
/// Panics if evaluation fails or the variable does not hold a `Value::Number`.
pub fn eval_number(code: &str, var: &str) -> i64 {
    match eval_var(code, var) {
        Value::Number(n) => n,
        other => panic!("eval_number('{var}'): expected Number, got {other:?}\n  code: {code}"),
    }
}

/// Evaluate `code`, read back `var`, and return it as an `f64`.
///
/// Accepts both `Value::Float` and `Value::Number` (widened to `f64`).
///
/// # Panics
///
/// Panics if evaluation fails or the variable holds neither a Float nor a Number.
pub fn eval_float(code: &str, var: &str) -> f64 {
    match eval_var(code, var) {
        Value::Float(f) => f,
        Value::Number(n) => n as f64,
        other => {
            panic!("eval_float('{var}'): expected Float/Number, got {other:?}\n  code: {code}")
        }
    }
}

/// Evaluate `code`, read back `var`, and return it as a `bool`.
///
/// # Panics
///
/// Panics if evaluation fails or the variable does not hold a `Value::Bool`.
pub fn eval_bool(code: &str, var: &str) -> bool {
    match eval_var(code, var) {
        Value::Bool(b) => b,
        other => panic!("eval_bool('{var}'): expected Bool, got {other:?}\n  code: {code}"),
    }
}

/// Evaluate `code`, read back `var`, and return it as a `String`.
///
/// # Panics
///
/// Panics if evaluation fails or the variable does not hold a `Value::String`.
pub fn eval_string(code: &str, var: &str) -> String {
    match eval_var(code, var) {
        Value::String(s) => s,
        other => panic!("eval_string('{var}'): expected String, got {other:?}\n  code: {code}"),
    }
}

// ── Function-return helpers ───────────────────────────────────────────────────
//
// These variants evaluate a snippet whose *last expression* is a function call
// that returns the value directly (no intermediate variable needed).  They rely
// on `eval_code` returning the last evaluated value.

/// Evaluate `code` and return the *last evaluated expression* as an `i64`.
///
/// Use this when the last top-level item is a function call that returns
/// an integer directly, e.g.:
///
/// ```text
/// factorial(n) { … }
/// factorial(5)   ← this return value is captured
/// ```
///
/// # Panics
///
/// Panics if evaluation fails or the last value is not a `Value::Number`.
pub fn eval_last_number(code: &str) -> i64 {
    match eval_code(code) {
        Ok(Value::Number(n)) => n,
        Ok(other) => panic!("eval_last_number: expected Number, got {other:?}\n  code: {code}"),
        Err(e) => panic!("eval_last_number: {e}\n  code: {code}"),
    }
}

/// Evaluate `code` and return the *last evaluated expression* as an `f64`.
///
/// Accepts `Value::Float` and `Value::Number` (widened).
///
/// # Panics
///
/// Panics if evaluation fails or the last value is neither Float nor Number.
pub fn eval_last_float(code: &str) -> f64 {
    match eval_code(code) {
        Ok(Value::Float(f)) => f,
        Ok(Value::Number(n)) => n as f64,
        Ok(other) => {
            panic!("eval_last_float: expected Float/Number, got {other:?}\n  code: {code}")
        }
        Err(e) => panic!("eval_last_float: {e}\n  code: {code}"),
    }
}

/// Evaluate `code` and return the *last evaluated expression* as a `String`.
///
/// # Panics
///
/// Panics if evaluation fails or the last value is not a `Value::String`.
pub fn eval_last_string(code: &str) -> String {
    match eval_code(code) {
        Ok(Value::String(s)) => s,
        Ok(other) => panic!("eval_last_string: expected String, got {other:?}\n  code: {code}"),
        Err(e) => panic!("eval_last_string: {e}\n  code: {code}"),
    }
}

/// Evaluate `code` and return the *last evaluated expression* as a `bool`.
///
/// Use this when the last top-level item is a boolean expression or a function
/// call that returns a boolean directly, e.g.:
///
/// ```text
/// true && false   ← this return value is captured
/// ```
///
/// # Panics
///
/// Panics if evaluation fails or the last value is not a `Value::Bool`.
pub fn eval_last_bool(code: &str) -> bool {
    match eval_code(code) {
        Ok(Value::Bool(b)) => b,
        Ok(other) => panic!("eval_last_bool: expected Bool, got {other:?}\n  code: {code}"),
        Err(e) => panic!("eval_last_bool: {e}\n  code: {code}"),
    }
}

// ── Error assertion helpers ───────────────────────────────────────────────────

/// Assert that evaluating `code` produces a runtime error whose message
/// contains `needle`.
///
/// # Panics
///
/// Panics if evaluation unexpectedly succeeds, or if the error message
/// does not contain `needle`.
pub fn assert_runtime_error(code: &str, needle: &str) {
    match eval_code(code) {
        Err(msg) => assert!(
            msg.contains(needle),
            "expected error containing {needle:?}, got: {msg:?}\n  code: {code}"
        ),
        Ok(v) => panic!(
            "expected runtime error containing {needle:?}, but eval succeeded with {v:?}\n  code: {code}"
        ),
    }
}

/// Assert that parsing `code` produces a parse error whose message
/// contains `needle`.
///
/// # Panics
///
/// Panics if parsing unexpectedly succeeds or the error does not contain
/// `needle`.
pub fn assert_parse_error(code: &str, needle: &str) {
    let mut lexer = Lexer::new(code.to_string());
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        let done = tok.kind == TokenKind::Eof;
        tokens.push(tok);
        if done {
            break;
        }
    }
    match Parser::new(tokens).parse() {
        Err(errs) => {
            let joined = errs.join("; ");
            assert!(
                joined.contains(needle),
                "expected parse error containing {needle:?}, got: {joined:?}\n  code: {code}"
            );
        }
        Ok(_) => panic!(
            "expected parse error containing {needle:?}, but parse succeeded\n  code: {code}"
        ),
    }
}
