// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// Array built-ins (split out of a single `methods.rs`).

use crate::interpreter::Value;

/// Maximum number of elements `range()` will materialise.
///
/// Mirrors the interpreter's loop guard so that a runaway `range()` fails
/// loudly instead of exhausting memory.
const MAX_RANGE_LENGTH: i64 = 1_000_000;

/// `range(end)` → `[0, 1, …, end-1]` and `range(start, end)` →
/// `[start, …, end-1]`.
///
/// Both bounds must be integers. `end <= start` yields an empty array; Halo
/// has no negative steps yet.
pub fn builtin_range(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() || args.len() > 2 {
        return Err(format!(
            "range() expects 1 or 2 arguments, got {}",
            args.len()
        ));
    }

    let mut bounds = Vec::with_capacity(args.len());
    for arg in args {
        match arg {
            Value::Number(n) => bounds.push(*n),
            other => {
                return Err(format!(
                    "range() expects integer arguments, got '{}'",
                    other.type_name()
                ))
            }
        }
    }

    let (start, end) = match bounds.as_slice() {
        [end] => (0, *end),
        [start, end] => (*start, *end),
        _ => unreachable!("range() arity checked above"),
    };

    let length = end.saturating_sub(start);
    if length > MAX_RANGE_LENGTH {
        return Err(format!(
            "range({start}, {end}) produces {length} elements, above the limit of {MAX_RANGE_LENGTH}"
        ));
    }

    Ok(Value::Array((start..end).map(Value::Number).collect()))
}

// ──────────────────────────────────────────────────────────────────────────────
// Array methods
// ──────────────────────────────────────────────────────────────────────────────

/// `push(array, element)` - Appends an element to the array and returns the new array.
pub fn push_value(array: Value, element: Value) -> Result<Value, String> {
    match array {
        Value::Array(mut arr) => {
            arr.push(element);
            Ok(Value::Array(arr))
        }
        other => Err(format!(
            "push() expects first argument to be an array, got '{}'",
            other.type_name()
        )),
    }
}

/// Builtin form of [`push_value`].
pub fn builtin_push(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "push() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let array = args[0].clone();
    let element = args[1].clone();
    push_value(array, element)
}

/// `pop(array)` - Removes and returns the last element of the array (null if empty).
pub fn pop_value(array: Value) -> Result<Value, String> {
    match array {
        Value::Array(mut arr) => Ok(arr.pop().unwrap_or(Value::Null)),
        other => Err(format!(
            "pop() expects an array, got '{}'",
            other.type_name()
        )),
    }
}

/// Builtin form of [`pop_value`].
pub fn builtin_pop(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "pop() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    pop_value(args[0].clone())
}

/// `append(array, array)` - Combines two arrays and returns a new array.
pub fn append_value(left: Value, right: Value) -> Result<Value, String> {
    match (left, right) {
        (Value::Array(mut left), Value::Array(right)) => {
            left.extend(right);
            Ok(Value::Array(left))
        }
        (Value::Array(_), other) => Err(format!(
            "append() expects second argument to be an array, got '{}'",
            other.type_name()
        )),
        (other, _) => Err(format!(
            "append() expects first argument to be an array, got '{}'",
            other.type_name()
        )),
    }
}

/// Builtin form of [`append_value`].
pub fn builtin_append(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "append() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let left = args[0].clone();
    let right = args[1].clone();
    append_value(left, right)
}

/// `first(array)` - Returns the first element of the array (null if empty).
pub fn first_value(array: Value) -> Result<Value, String> {
    match array {
        Value::Array(arr) => Ok(arr.first().cloned().unwrap_or(Value::Null)),
        other => Err(format!(
            "first() expects an array, got '{}'",
            other.type_name()
        )),
    }
}

/// Builtin form of [`first_value`].
pub fn builtin_first(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "first() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    first_value(args[0].clone())
}

/// `last(array)` - Returns the last element of the array (null if empty).
pub fn last_value(array: Value) -> Result<Value, String> {
    match array {
        Value::Array(arr) => Ok(arr.last().cloned().unwrap_or(Value::Null)),
        other => Err(format!(
            "last() expects an array, got '{}'",
            other.type_name()
        )),
    }
}

/// Builtin form of [`last_value`].
pub fn builtin_last(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "last() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    last_value(args[0].clone())
}

/// `reverse(array or string)` - Returns a new array with elements in reverse order, or reversed string.
pub fn reverse_value(value: Value) -> Result<Value, String> {
    match value {
        Value::Array(mut arr) => {
            arr.reverse();
            Ok(Value::Array(arr))
        }
        Value::String(s) => Ok(Value::String(s.chars().rev().collect())),
        other => Err(format!(
            "reverse() expects an array or string, got '{}'",
            other.type_name()
        )),
    }
}

/// Builtin form of [`reverse_value`].
pub fn builtin_reverse(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "reverse() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    reverse_value(args[0].clone())
}

/// `join(array, separator)` - Converts array elements to a string joined by separator.
pub fn join_value(array: Value, separator: Value) -> Result<Value, String> {
    let separator = match separator {
        Value::String(s) => s,
        other => {
            return Err(format!(
                "join() expects second argument to be a string, got '{}'",
                other.type_name()
            ))
        }
    };

    match array {
        Value::Array(arr) => {
            let strings: Vec<String> = arr.iter().map(|v| v.to_string_value()).collect();
            Ok(Value::String(strings.join(&separator)))
        }
        other => Err(format!(
            "join() expects first argument to be an array, got '{}'",
            other.type_name()
        )),
    }
}

/// Builtin form of [`join_value`].
pub fn builtin_join(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "join() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let array = args[0].clone();
    let separator = args[1].clone();
    join_value(array, separator)
}

/// `sort(array)` - Returns a new array with elements sorted.
pub fn sort_value(array: Value) -> Result<Value, String> {
    match array {
        Value::Array(mut arr) => {
            // Determine the element type: numbers, floats, or strings.
            let has_string = arr.iter().any(|e| matches!(e, Value::String(_)));
            let has_number = arr
                .iter()
                .any(|e| matches!(e, Value::Number(_) | Value::Float(_)));

            if has_string && has_number {
                return Err("sort() cannot mix strings and numbers in the same array".to_string());
            }

            if has_string {
                // Sort strings lexicographically.
                arr.sort_by(|a, b| {
                    let a_str = match a {
                        Value::String(s) => s.clone(),
                        _ => String::new(),
                    };
                    let b_str = match b {
                        Value::String(s) => s.clone(),
                        _ => String::new(),
                    };
                    a_str.cmp(&b_str)
                });
            } else {
                // Sort numbers (int or float).
                arr.sort_by(|a, b| {
                    let a_num = match a {
                        Value::Number(n) => *n as f64,
                        Value::Float(f) => *f,
                        _ => 0.0,
                    };
                    let b_num = match b {
                        Value::Number(n) => *n as f64,
                        Value::Float(f) => *f,
                        _ => 0.0,
                    };
                    a_num
                        .partial_cmp(&b_num)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }

            Ok(Value::Array(arr))
        }
        other => Err(format!(
            "sort() expects an array, got '{}'",
            other.type_name()
        )),
    }
}

/// Builtin form of [`sort_value`].
pub fn builtin_sort(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "sort() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    sort_value(args[0].clone())
}
