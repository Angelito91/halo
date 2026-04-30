// Array and string methods for Halo interpreter

use super::super::value::Value;
use super::core::Evaluator;
use crate::parser::ast::Expression;

/// Try to evaluate a method call on an object.
pub fn try_eval_method_call(
    evaluator: &mut Evaluator,
    object: &Value,
    method: &str,
    args: &[Expression],
) -> Result<Value, String> {
    match (object, method) {
        // String methods
        (Value::String(_), "upper") => {
            if !args.is_empty() {
                return Err(format!(
                    "String method upper() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            let mut call_args = vec![Expression::StringLiteral(
                object.to_string_value(),
                crate::parser::ast::Position { line: 0, column: 0 },
            )];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_upper(evaluator, &call_args)
        }
        (Value::String(_), "lower") => {
            if !args.is_empty() {
                return Err(format!(
                    "String method lower() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            let mut call_args = vec![Expression::StringLiteral(
                object.to_string_value(),
                crate::parser::ast::Position { line: 0, column: 0 },
            )];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_lower(evaluator, &call_args)
        }
        (Value::String(_), "trim") => {
            if !args.is_empty() {
                return Err(format!(
                    "String method trim() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            let mut call_args = vec![Expression::StringLiteral(
                object.to_string_value(),
                crate::parser::ast::Position { line: 0, column: 0 },
            )];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_trim(evaluator, &call_args)
        }
        (Value::String(_), "reverse") => {
            if !args.is_empty() {
                return Err(format!(
                    "String method reverse() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            let mut call_args = vec![Expression::StringLiteral(
                object.to_string_value(),
                crate::parser::ast::Position { line: 0, column: 0 },
            )];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_reverse(evaluator, &call_args)
        }
        (Value::String(_), "starts_with") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method starts_with() expects 1 argument, got {}",
                    args.len()
                ));
            }
            let mut call_args = vec![Expression::StringLiteral(
                object.to_string_value(),
                crate::parser::ast::Position { line: 0, column: 0 },
            )];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_starts_with(evaluator, &call_args)
        }
        (Value::String(_), "ends_with") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method ends_with() expects 1 argument, got {}",
                    args.len()
                ));
            }
            let mut call_args = vec![Expression::StringLiteral(
                object.to_string_value(),
                crate::parser::ast::Position { line: 0, column: 0 },
            )];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_ends_with(evaluator, &call_args)
        }
        (Value::String(_), "contains") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method contains() expects 1 argument, got {}",
                    args.len()
                ));
            }
            let mut call_args = vec![Expression::StringLiteral(
                object.to_string_value(),
                crate::parser::ast::Position { line: 0, column: 0 },
            )];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_contains(evaluator, &call_args)
        }
        (Value::String(_), "index_of") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method index_of() expects 1 argument, got {}",
                    args.len()
                ));
            }
            let mut call_args = vec![Expression::StringLiteral(
                object.to_string_value(),
                crate::parser::ast::Position { line: 0, column: 0 },
            )];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_index_of(evaluator, &call_args)
        }
        (Value::String(_), "replace") => {
            if args.len() != 2 {
                return Err(format!(
                    "String method replace() expects 2 arguments, got {}",
                    args.len()
                ));
            }
            let mut call_args = vec![Expression::StringLiteral(
                object.to_string_value(),
                crate::parser::ast::Position { line: 0, column: 0 },
            )];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_replace(evaluator, &call_args)
        }
        (Value::String(_), "substr") => {
            if args.len() != 2 {
                return Err(format!(
                    "String method substr() expects 2 arguments, got {}",
                    args.len()
                ));
            }
            let mut call_args = vec![Expression::StringLiteral(
                object.to_string_value(),
                crate::parser::ast::Position { line: 0, column: 0 },
            )];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_substr(evaluator, &call_args)
        }

        // Array methods
        (Value::Array(_), "push") => {
            if args.len() != 1 {
                return Err(format!(
                    "Array method push() expects 1 argument, got {}",
                    args.len()
                ));
            }
            let array_expr = evaluator.value_to_expression(object);
            let mut call_args = vec![array_expr];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_push(evaluator, &call_args)
        }
        (Value::Array(_), "pop") => {
            if !args.is_empty() {
                return Err(format!(
                    "Array method pop() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            let array_expr = evaluator.value_to_expression(object);
            builtin_pop(evaluator, &[array_expr])
        }
        (Value::Array(_), "first") => {
            if !args.is_empty() {
                return Err(format!(
                    "Array method first() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            let array_expr = evaluator.value_to_expression(object);
            builtin_first(evaluator, &[array_expr])
        }
        (Value::Array(_), "last") => {
            if !args.is_empty() {
                return Err(format!(
                    "Array method last() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            let array_expr = evaluator.value_to_expression(object);
            builtin_last(evaluator, &[array_expr])
        }
        (Value::Array(_), "reverse") => {
            if !args.is_empty() {
                return Err(format!(
                    "Array method reverse() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            let array_expr = evaluator.value_to_expression(object);
            builtin_reverse(evaluator, &[array_expr])
        }
        (Value::Array(_), "join") => {
            if args.len() != 1 {
                return Err(format!(
                    "Array method join() expects 1 argument, got {}",
                    args.len()
                ));
            }
            let array_expr = evaluator.value_to_expression(object);
            let mut call_args = vec![array_expr];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_join(evaluator, &call_args)
        }
        (Value::Array(_), "contains") => {
            if args.len() != 1 {
                return Err(format!(
                    "Array method contains() expects 1 argument, got {}",
                    args.len()
                ));
            }
            let array_expr = evaluator.value_to_expression(object);
            let mut call_args = vec![array_expr];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_contains(evaluator, &call_args)
        }
        (Value::Array(_), "index_of") => {
            if args.len() != 1 {
                return Err(format!(
                    "Array method index_of() expects 1 argument, got {}",
                    args.len()
                ));
            }
            let array_expr = evaluator.value_to_expression(object);
            let mut call_args = vec![array_expr];
            for arg in args {
                call_args.push(arg.clone());
            }
            builtin_index_of(evaluator, &call_args)
        }
        (Value::Array(_), "sort") => {
            if !args.is_empty() {
                return Err(format!(
                    "Array method sort() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            let array_expr = evaluator.value_to_expression(object);
            builtin_sort(evaluator, &[array_expr])
        }

        // Unknown method
        (obj, method_name) => Err(format!(
            "Unknown method '{}' for type '{}'",
            method_name,
            obj.type_name()
        )),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Array methods
// ──────────────────────────────────────────────────────────────────────────────

/// `push(array, element)` - Appends an element to the array and returns the new array.
pub fn builtin_push(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "push() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let arr_val = evaluator.eval_expr(&args[0])?;
    let elem = evaluator.eval_expr(&args[1])?;

    match arr_val {
        Value::Array(mut arr) => {
            arr.push(elem);
            Ok(Value::Array(arr))
        }
        other => Err(format!(
            "push() expects first argument to be an array, got '{}'",
            other.type_name()
        )),
    }
}

/// `pop(array)` - Removes and returns the last element of the array (null if empty).
pub fn builtin_pop(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "pop() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let arr_val = evaluator.eval_expr(&args[0])?;

    match arr_val {
        Value::Array(mut arr) => {
            let result = arr.pop().unwrap_or(Value::Null);
            Ok(result)
        }
        other => Err(format!(
            "pop() expects an array, got '{}'",
            other.type_name()
        )),
    }
}

/// `first(array)` - Returns the first element of the array (null if empty).
pub fn builtin_first(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "first() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let arr_val = evaluator.eval_expr(&args[0])?;

    match arr_val {
        Value::Array(arr) => Ok(arr.first().cloned().unwrap_or(Value::Null)),
        other => Err(format!(
            "first() expects an array, got '{}'",
            other.type_name()
        )),
    }
}

/// `last(array)` - Returns the last element of the array (null if empty).
pub fn builtin_last(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "last() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let arr_val = evaluator.eval_expr(&args[0])?;

    match arr_val {
        Value::Array(arr) => Ok(arr.last().cloned().unwrap_or(Value::Null)),
        other => Err(format!(
            "last() expects an array, got '{}'",
            other.type_name()
        )),
    }
}

/// `reverse(array or string)` - Returns a new array with elements in reverse order, or reversed string.
pub fn builtin_reverse(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "reverse() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let val = evaluator.eval_expr(&args[0])?;

    match val {
        Value::Array(mut arr) => {
            arr.reverse();
            Ok(Value::Array(arr))
        }
        Value::String(s) => {
            let reversed: String = s.chars().rev().collect();
            Ok(Value::String(reversed))
        }
        other => Err(format!(
            "reverse() expects an array or string, got '{}'",
            other.type_name()
        )),
    }
}

/// `join(array, separator)` - Converts array elements to a string joined by separator.
pub fn builtin_join(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "join() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let arr_val = evaluator.eval_expr(&args[0])?;
    let sep_val = evaluator.eval_expr(&args[1])?;

    let sep = match sep_val {
        Value::String(s) => s,
        other => {
            return Err(format!(
                "join() expects second argument to be a string, got '{}'",
                other.type_name()
            ))
        }
    };

    match arr_val {
        Value::Array(arr) => {
            let strings: Vec<String> = arr.iter().map(|v| v.to_string_value()).collect();
            Ok(Value::String(strings.join(&sep)))
        }
        other => Err(format!(
            "join() expects first argument to be an array, got '{}'",
            other.type_name()
        )),
    }
}

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
        (Value::String(s), Value::String(substring)) => {
            let index = s.find(&substring).map(|i| i as i64).unwrap_or(-1);
            Ok(Value::Number(index))
        }
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

/// `sort(array)` - Returns a new array with elements sorted (only for numbers).
pub fn builtin_sort(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "sort() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let arr_val = evaluator.eval_expr(&args[0])?;

    match arr_val {
        Value::Array(mut arr) => {
            // Validate that all elements are numbers or floats
            for elem in &arr {
                match elem {
                    Value::Number(_) | Value::Float(_) => {}
                    other => {
                        return Err(format!(
                            "sort() only works with arrays of numbers, found '{}'",
                            other.type_name()
                        ))
                    }
                }
            }

            // Sort the array
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

            Ok(Value::Array(arr))
        }
        other => Err(format!(
            "sort() expects an array, got '{}'",
            other.type_name()
        )),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// String methods
// ──────────────────────────────────────────────────────────────────────────────

/// `upper(text)` - Converts a string to uppercase.
pub fn builtin_upper(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "upper() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let val = evaluator.eval_expr(&args[0])?;

    match val {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        other => Err(format!(
            "upper() expects a string, got '{}'",
            other.type_name()
        )),
    }
}

/// `lower(text)` - Converts a string to lowercase.
pub fn builtin_lower(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "lower() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let val = evaluator.eval_expr(&args[0])?;

    match val {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        other => Err(format!(
            "lower() expects a string, got '{}'",
            other.type_name()
        )),
    }
}

/// `trim(text)` - Removes leading and trailing whitespace from a string.
pub fn builtin_trim(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "trim() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let val = evaluator.eval_expr(&args[0])?;

    match val {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        other => Err(format!(
            "trim() expects a string, got '{}'",
            other.type_name()
        )),
    }
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
    let text_val = evaluator.eval_expr(&args[0])?;
    let prefix_val = evaluator.eval_expr(&args[1])?;

    let text = match text_val {
        Value::String(s) => s,
        other => {
            return Err(format!(
                "starts_with() expects first argument to be a string, got '{}'",
                other.type_name()
            ))
        }
    };

    let prefix = match prefix_val {
        Value::String(s) => s,
        other => {
            return Err(format!(
                "starts_with() expects second argument to be a string, got '{}'",
                other.type_name()
            ))
        }
    };

    Ok(Value::Bool(text.starts_with(&prefix)))
}

/// `ends_with(text, suffix)` - Returns true if the string ends with the suffix.
pub fn builtin_ends_with(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "ends_with() expects exactly 2 arguments, got {}",
            args.len()
        ));
    }
    let text_val = evaluator.eval_expr(&args[0])?;
    let suffix_val = evaluator.eval_expr(&args[1])?;

    let text = match text_val {
        Value::String(s) => s,
        other => {
            return Err(format!(
                "ends_with() expects first argument to be a string, got '{}'",
                other.type_name()
            ))
        }
    };

    let suffix = match suffix_val {
        Value::String(s) => s,
        other => {
            return Err(format!(
                "ends_with() expects second argument to be a string, got '{}'",
                other.type_name()
            ))
        }
    };

    Ok(Value::Bool(text.ends_with(&suffix)))
}

/// `replace(text, old, new)` - Replaces all occurrences of old with new.
pub fn builtin_replace(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "replace() expects exactly 3 arguments, got {}",
            args.len()
        ));
    }
    let text_val = evaluator.eval_expr(&args[0])?;
    let old_val = evaluator.eval_expr(&args[1])?;
    let new_val = evaluator.eval_expr(&args[2])?;

    let text = match text_val {
        Value::String(s) => s,
        other => {
            return Err(format!(
                "replace() expects first argument to be a string, got '{}'",
                other.type_name()
            ))
        }
    };

    let old = match old_val {
        Value::String(s) => s,
        other => {
            return Err(format!(
                "replace() expects second argument to be a string, got '{}'",
                other.type_name()
            ))
        }
    };

    let new = match new_val {
        Value::String(s) => s,
        other => {
            return Err(format!(
                "replace() expects third argument to be a string, got '{}'",
                other.type_name()
            ))
        }
    };

    Ok(Value::String(text.replace(&old, &new)))
}

/// `substr(text, start, length)` - Extracts a substring starting at start with given length.
pub fn builtin_substr(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "substr() expects exactly 3 arguments, got {}",
            args.len()
        ));
    }
    let text_val = evaluator.eval_expr(&args[0])?;
    let start_val = evaluator.eval_expr(&args[1])?;
    let length_val = evaluator.eval_expr(&args[2])?;

    let text = match text_val {
        Value::String(s) => s,
        other => {
            return Err(format!(
                "substr() expects first argument to be a string, got '{}'",
                other.type_name()
            ))
        }
    };

    let start = match start_val {
        Value::Number(n) => n,
        other => {
            return Err(format!(
                "substr() expects second argument to be a number, got '{}'",
                other.type_name()
            ))
        }
    };

    let length = match length_val {
        Value::Number(n) => n,
        other => {
            return Err(format!(
                "substr() expects third argument to be a number, got '{}'",
                other.type_name()
            ))
        }
    };

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
