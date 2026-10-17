// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// String built-ins and their helpers (split out of a single `methods.rs`).

use super::super::core::Evaluator;
use crate::interpreter::value::Value;
use crate::parser::ast::Expression;

// ──────────────────────────────────────────────────────────────────────────────
// Helper functions for reducing code duplication
// ──────────────────────────────────────────────────────────────────────────────

/// Helper to extract a string value from an evaluated expression with error handling.
fn extract_string(value: Value, method_name: &str, arg_pos: usize) -> Result<String, String> {
    match value {
        Value::String(s) => Ok(s),
        other => Err(format!(
            "{}() expects argument {} to be a string, got {}",
            method_name,
            arg_pos + 1,
            other.type_name()
        )),
    }
}

/// Helper to extract a number value from an evaluated expression with error handling.
fn extract_number(value: Value, method_name: &str, arg_pos: usize) -> Result<i64, String> {
    match value {
        Value::Number(n) => Ok(n),
        other => Err(format!(
            "{}() expects argument {} to be a number, got {}",
            method_name,
            arg_pos + 1,
            other.type_name()
        )),
    }
}

/// Helper to apply a string transformation to the first argument.
/// Validates argument count and applies the transformer function.
fn apply_string_transformer<F>(
    evaluator: &mut Evaluator,
    args: &[Expression],
    method_name: &str,
    expected_args: usize,
    transformer: F,
) -> Result<Value, String>
where
    F: Fn(&str) -> String,
{
    if args.len() != expected_args {
        return Err(format!(
            "{}() expects exactly {} argument(s), got {}",
            method_name,
            expected_args,
            args.len()
        ));
    }
    let val = evaluator.eval_expr(&args[0])?;
    let s = extract_string(val, method_name, 0)?;
    Ok(Value::String(transformer(&s)))
}

/// Helper to build method call args by prepending the object as a string literal.
pub(crate) fn build_string_call_args(object: &Value, args: &[Expression]) -> Vec<Expression> {
    let mut call_args = vec![Expression::StringLiteral(
        object.to_string_value(),
        crate::parser::ast::Position { line: 0, column: 0 },
    )];
    call_args.extend(args.iter().cloned());
    call_args
}

/// Helper for string methods that take 2 string arguments and return a boolean.
fn apply_string_predicate<F>(
    evaluator: &mut Evaluator,
    args: &[Expression],
    method_name: &str,
    predicate: F,
) -> Result<Value, String>
where
    F: Fn(&str, &str) -> bool,
{
    if args.len() != 2 {
        return Err(format!(
            "{}() expects exactly 2 arguments, got {}",
            method_name,
            args.len()
        ));
    }
    let text = extract_string(evaluator.eval_expr(&args[0])?, method_name, 0)?;
    let other = extract_string(evaluator.eval_expr(&args[1])?, method_name, 1)?;
    Ok(Value::Bool(predicate(&text, &other)))
}

/// Helper for string methods that take variable numbers of string arguments and apply a transformation.
fn apply_multi_string_transformer<F>(
    evaluator: &mut Evaluator,
    args: &[Expression],
    method_name: &str,
    expected_args: usize,
    transformer: F,
) -> Result<Value, String>
where
    F: Fn(&[String]) -> String,
{
    if args.len() != expected_args {
        return Err(format!(
            "{}() expects exactly {} argument(s), got {}",
            method_name,
            expected_args,
            args.len()
        ));
    }
    let mut string_args = Vec::new();
    for (idx, arg) in args.iter().enumerate() {
        let val = evaluator.eval_expr(arg)?;
        let s = extract_string(val, method_name, idx)?;
        string_args.push(s);
    }
    Ok(Value::String(transformer(&string_args)))
}

// ──────────────────────────────────────────────────────────────────────────────
// String and container methods
// ──────────────────────────────────────────────────────────────────────────────

/// `contains(array or string, value)` - Returns true if the array/string contains the value/substring.
pub fn builtin_contains(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "contains() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let container_val = evaluator.eval_expr(&args[0])?;
    let search_val = evaluator.eval_expr(&args[1])?;

    match (container_val, search_val) {
        (Value::Array(arr), search) => {
            let found = arr.iter().any(|v| v.equals(&search));
            Ok(Value::Bool(found))
        }
        (Value::String(s), Value::String(substring)) => Ok(Value::Bool(s.contains(&substring))),
        (Value::String(_), other) => Err(format!(
            "contains() with string expects second argument to be a string, got '{}'",
            other.type_name()
        )),
        (other, _) => Err(format!(
            "contains() expects first argument to be an array or string, got '{}'",
            other.type_name()
        )),
    }
}

/// `index_of(array or string, value)` - Returns the index of the value/substring (-1 if not found).
pub fn builtin_index_of(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "index_of() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let container_val = evaluator.eval_expr(&args[0])?;
    let search_val = evaluator.eval_expr(&args[1])?;

    match (container_val, search_val) {
        (Value::Array(arr), search) => {
            let index = arr
                .iter()
                .position(|v| v.equals(&search))
                .map(|i| i as i64)
                .unwrap_or(-1);
            Ok(Value::Number(index))
        }
        (Value::String(s), Value::String(substring)) => match s.find(&substring) {
            Some(idx) => Ok(Value::Number(idx as i64)),
            None => Ok(Value::Number(-1)),
        },
        (Value::String(_), other) => Err(format!(
            "index_of() with string expects second argument to be a string, got '{}'",
            other.type_name()
        )),
        (other, _) => Err(format!(
            "index_of() expects first argument to be an array or string, got '{}'",
            other.type_name()
        )),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// String methods
// ──────────────────────────────────────────────────────────────────────────────

/// `upper(text)` - Converts a string to uppercase.
pub fn builtin_upper(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    apply_string_transformer(evaluator, args, "upper", 1, |s| s.to_uppercase())
}

/// `lower(text)` - Converts a string to lowercase.
pub fn builtin_lower(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    apply_string_transformer(evaluator, args, "lower", 1, |s| s.to_lowercase())
}

/// `trim(text)` - Removes leading and trailing whitespace from a string.
pub fn builtin_trim(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    apply_string_transformer(evaluator, args, "trim", 1, |s| s.trim().to_string())
}

/// `starts_with(text, prefix)` - Returns true if the string starts with the prefix.
pub fn builtin_starts_with(
    evaluator: &mut Evaluator,
    args: &[Expression],
) -> Result<Value, String> {
    apply_string_predicate(evaluator, args, "starts_with", |text, prefix| {
        text.starts_with(prefix)
    })
}

/// `ends_with(text, suffix)` - Returns true if the string ends with the suffix.
pub fn builtin_ends_with(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    apply_string_predicate(evaluator, args, "ends_with", |text, suffix| {
        text.ends_with(suffix)
    })
}

/// `replace(text, old, new)` - Replaces all occurrences of old with new.
pub fn builtin_replace(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    apply_multi_string_transformer(evaluator, args, "replace", 3, |strings| {
        strings[0].replace(&strings[1], &strings[2])
    })
}

/// `substr(text, start, length)` - Extracts a substring starting at start with given length.
pub fn builtin_substr(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "substr() expects exactly 3 arguments, got {}",
            args.len()
        ));
    }
    let text = extract_string(evaluator.eval_expr(&args[0])?, "substr", 0)?;
    let start = extract_number(evaluator.eval_expr(&args[1])?, "substr", 1)?;
    let length = extract_number(evaluator.eval_expr(&args[2])?, "substr", 2)?;

    if start < 0 {
        return Err(format!(
            "substr() start index cannot be negative, got {}",
            start
        ));
    }
    if length < 0 {
        return Err(format!(
            "substr() length cannot be negative, got {}",
            length
        ));
    }

    let start_usize = start as usize;
    let length_usize = length as usize;

    let chars: Vec<char> = text.chars().collect();
    let end = std::cmp::min(start_usize + length_usize, chars.len());

    if start_usize >= chars.len() {
        return Ok(Value::String(String::new()));
    }

    let substring: String = chars[start_usize..end].iter().collect();
    Ok(Value::String(substring))
}
