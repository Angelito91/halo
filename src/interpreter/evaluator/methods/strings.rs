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
// Helpers
// ──────────────────────────────────────────────────────────────────────────────

/// Extract a string from an already-evaluated value.
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

/// Extract a number from an already-evaluated value.
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

// ──────────────────────────────────────────────────────────────────────────────
// Value-level operations (shared by built-ins and method dispatch)
// ──────────────────────────────────────────────────────────────────────────────

/// `contains(array or string, value)` on already-evaluated values.
pub fn contains_value(container: Value, search: Value) -> Result<Value, String> {
    match (container, search) {
        (Value::Array(arr), search) => Ok(Value::Bool(arr.iter().any(|v| v.equals(&search)))),
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

/// `index_of(array or string, value)` on already-evaluated values.
pub fn index_of_value(container: Value, search: Value) -> Result<Value, String> {
    match (container, search) {
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

/// Uppercase a value that must be a string.
pub fn upper_value(value: Value) -> Result<Value, String> {
    Ok(Value::String(
        extract_string(value, "upper", 0)?.to_uppercase(),
    ))
}

/// Lowercase a value that must be a string.
pub fn lower_value(value: Value) -> Result<Value, String> {
    Ok(Value::String(
        extract_string(value, "lower", 0)?.to_lowercase(),
    ))
}

/// Trim a value that must be a string.
pub fn trim_value(value: Value) -> Result<Value, String> {
    Ok(Value::String(
        extract_string(value, "trim", 0)?.trim().to_string(),
    ))
}

/// `starts_with(text, prefix)` on already-evaluated values.
pub fn starts_with_value(text: Value, prefix: Value) -> Result<Value, String> {
    let text = extract_string(text, "starts_with", 0)?;
    let prefix = extract_string(prefix, "starts_with", 1)?;
    Ok(Value::Bool(text.starts_with(&prefix)))
}

/// `ends_with(text, suffix)` on already-evaluated values.
pub fn ends_with_value(text: Value, suffix: Value) -> Result<Value, String> {
    let text = extract_string(text, "ends_with", 0)?;
    let suffix = extract_string(suffix, "ends_with", 1)?;
    Ok(Value::Bool(text.ends_with(&suffix)))
}

/// `replace(text, old, new)` on already-evaluated values.
pub fn replace_value(text: Value, old: Value, new: Value) -> Result<Value, String> {
    let text = extract_string(text, "replace", 0)?;
    let old = extract_string(old, "replace", 1)?;
    let new = extract_string(new, "replace", 2)?;
    Ok(Value::String(text.replace(&old, &new)))
}

/// `substr(text, start, length)` on already-evaluated values.
pub fn substr_value(text: Value, start: Value, length: Value) -> Result<Value, String> {
    let text = extract_string(text, "substr", 0)?;
    let start = extract_number(start, "substr", 1)?;
    let length = extract_number(length, "substr", 2)?;

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
    let end = std::cmp::min(start_usize.saturating_add(length_usize), chars.len());

    if start_usize >= chars.len() {
        return Ok(Value::String(String::new()));
    }

    Ok(Value::String(chars[start_usize..end].iter().collect()))
}

// ──────────────────────────────────────────────────────────────────────────────
// Built-ins
// ──────────────────────────────────────────────────────────────────────────────

/// `contains(array or string, value)` - Returns true if the array/string contains the value/substring.
pub fn builtin_contains(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "contains() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let container = evaluator.eval_expr(&args[0])?;
    let search = evaluator.eval_expr(&args[1])?;
    contains_value(container, search)
}

/// `index_of(array or string, value)` - Returns the index of the value/substring (-1 if not found).
pub fn builtin_index_of(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "index_of() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let container = evaluator.eval_expr(&args[0])?;
    let search = evaluator.eval_expr(&args[1])?;
    index_of_value(container, search)
}

/// `upper(text)` - Converts a string to uppercase.
pub fn builtin_upper(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "upper() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    upper_value(evaluator.eval_expr(&args[0])?)
}

/// `lower(text)` - Converts a string to lowercase.
pub fn builtin_lower(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "lower() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    lower_value(evaluator.eval_expr(&args[0])?)
}

/// `trim(text)` - Removes leading and trailing whitespace from a string.
pub fn builtin_trim(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "trim() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    trim_value(evaluator.eval_expr(&args[0])?)
}

/// `starts_with(text, prefix)` - Returns true if the string starts with the prefix.
pub fn builtin_starts_with(
    evaluator: &mut Evaluator,
    args: &[Expression],
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "starts_with() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let text = evaluator.eval_expr(&args[0])?;
    let prefix = evaluator.eval_expr(&args[1])?;
    starts_with_value(text, prefix)
}

/// `ends_with(text, suffix)` - Returns true if the string ends with the suffix.
pub fn builtin_ends_with(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "ends_with() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let text = evaluator.eval_expr(&args[0])?;
    let suffix = evaluator.eval_expr(&args[1])?;
    ends_with_value(text, suffix)
}

/// `replace(text, old, new)` - Replaces all occurrences of old with new.
pub fn builtin_replace(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "replace() expects exactly 3 arguments, got {}",
            args.len()
        ));
    }
    let text = evaluator.eval_expr(&args[0])?;
    let old = evaluator.eval_expr(&args[1])?;
    let new = evaluator.eval_expr(&args[2])?;
    replace_value(text, old, new)
}

/// `substr(text, start, length)` - Extracts a substring starting at start with given length.
pub fn builtin_substr(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "substr() expects exactly 3 arguments, got {}",
            args.len()
        ));
    }
    let text = evaluator.eval_expr(&args[0])?;
    let start = evaluator.eval_expr(&args[1])?;
    let length = evaluator.eval_expr(&args[2])?;
    substr_value(text, start, length)
}
