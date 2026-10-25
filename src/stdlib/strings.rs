// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// String built-ins and their helpers (split out of a single `methods.rs`).

use crate::interpreter::Value;

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
pub fn builtin_contains(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "contains() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let container = args[0].clone();
    let search = args[1].clone();
    contains_value(container, search)
}

/// `index_of(array or string, value)` - Returns the index of the value/substring (-1 if not found).
pub fn builtin_index_of(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "index_of() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let container = args[0].clone();
    let search = args[1].clone();
    index_of_value(container, search)
}

/// `upper(text)` - Converts a string to uppercase.
pub fn builtin_upper(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "upper() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    upper_value(args[0].clone())
}

/// `lower(text)` - Converts a string to lowercase.
pub fn builtin_lower(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "lower() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    lower_value(args[0].clone())
}

/// `trim(text)` - Removes leading and trailing whitespace from a string.
pub fn builtin_trim(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "trim() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    trim_value(args[0].clone())
}

/// `starts_with(text, prefix)` - Returns true if the string starts with the prefix.
pub fn builtin_starts_with(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "starts_with() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let text = args[0].clone();
    let prefix = args[1].clone();
    starts_with_value(text, prefix)
}

/// `ends_with(text, suffix)` - Returns true if the string ends with the suffix.
pub fn builtin_ends_with(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "ends_with() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let text = args[0].clone();
    let suffix = args[1].clone();
    ends_with_value(text, suffix)
}

/// `replace(text, old, new)` - Replaces all occurrences of old with new.
pub fn builtin_replace(args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "replace() expects exactly 3 arguments, got {}",
            args.len()
        ));
    }
    let text = args[0].clone();
    let old = args[1].clone();
    let new = args[2].clone();
    replace_value(text, old, new)
}

/// `substr(text, start, length)` - Extracts a substring starting at start with given length.
pub fn builtin_substr(args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "substr() expects exactly 3 arguments, got {}",
            args.len()
        ));
    }
    let text = args[0].clone();
    let start = args[1].clone();
    let length = args[2].clone();
    substr_value(text, start, length)
}
