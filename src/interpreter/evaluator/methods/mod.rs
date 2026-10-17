// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// Array and string method implementations, plus the method dispatcher
// (split out of a single `methods.rs`).

use super::super::value::Value;
use super::core::Evaluator;
use crate::parser::ast::Expression;

mod arrays;
mod strings;

pub use arrays::*;
pub use strings::*;

use strings::build_string_call_args;

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
            builtin_upper(evaluator, &build_string_call_args(object, args))
        }
        (Value::String(_), "lower") => {
            if !args.is_empty() {
                return Err(format!(
                    "String method lower() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            builtin_lower(evaluator, &build_string_call_args(object, args))
        }
        (Value::String(_), "trim") => {
            if !args.is_empty() {
                return Err(format!(
                    "String method trim() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            builtin_trim(evaluator, &build_string_call_args(object, args))
        }
        (Value::String(_), "reverse") => {
            if !args.is_empty() {
                return Err(format!(
                    "String method reverse() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            builtin_reverse(evaluator, &build_string_call_args(object, args))
        }
        (Value::String(_), "starts_with") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method starts_with() expects 1 argument, got {}",
                    args.len()
                ));
            }
            builtin_starts_with(evaluator, &build_string_call_args(object, args))
        }
        (Value::String(_), "ends_with") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method ends_with() expects 1 argument, got {}",
                    args.len()
                ));
            }
            builtin_ends_with(evaluator, &build_string_call_args(object, args))
        }
        (Value::String(_), "contains") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method contains() expects 1 argument, got {}",
                    args.len()
                ));
            }
            builtin_contains(evaluator, &build_string_call_args(object, args))
        }
        (Value::String(_), "index_of") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method index_of() expects 1 argument, got {}",
                    args.len()
                ));
            }
            builtin_index_of(evaluator, &build_string_call_args(object, args))
        }
        (Value::String(_), "replace") => {
            if args.len() != 2 {
                return Err(format!(
                    "String method replace() expects 2 arguments, got {}",
                    args.len()
                ));
            }
            builtin_replace(evaluator, &build_string_call_args(object, args))
        }
        (Value::String(_), "substr") => {
            if args.len() != 2 {
                return Err(format!(
                    "String method substr() expects 2 arguments, got {}",
                    args.len()
                ));
            }
            builtin_substr(evaluator, &build_string_call_args(object, args))
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
