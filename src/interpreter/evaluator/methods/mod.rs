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

/// Try to evaluate a method call on an object.
pub fn try_eval_method_call(
    evaluator: &mut Evaluator,
    object: &Value,
    method: &str,
    args: &[Expression],
) -> Result<Value, String> {
    match (object, method) {
        // String methods
        (Value::String(s), "upper") => {
            if !args.is_empty() {
                return Err(format!(
                    "String method upper() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            upper_value(Value::String(s.clone()))
        }
        (Value::String(s), "lower") => {
            if !args.is_empty() {
                return Err(format!(
                    "String method lower() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            lower_value(Value::String(s.clone()))
        }
        (Value::String(s), "trim") => {
            if !args.is_empty() {
                return Err(format!(
                    "String method trim() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            trim_value(Value::String(s.clone()))
        }
        (Value::String(_), "reverse") => {
            if !args.is_empty() {
                return Err(format!(
                    "String method reverse() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            reverse_value(object.clone())
        }
        (Value::String(_), "starts_with") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method starts_with() expects 1 argument, got {}",
                    args.len()
                ));
            }
            starts_with_value(object.clone(), evaluator.eval_expr(&args[0])?)
        }
        (Value::String(_), "ends_with") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method ends_with() expects 1 argument, got {}",
                    args.len()
                ));
            }
            ends_with_value(object.clone(), evaluator.eval_expr(&args[0])?)
        }
        (Value::String(_), "contains") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method contains() expects 1 argument, got {}",
                    args.len()
                ));
            }
            contains_value(object.clone(), evaluator.eval_expr(&args[0])?)
        }
        (Value::String(_), "index_of") => {
            if args.len() != 1 {
                return Err(format!(
                    "String method index_of() expects 1 argument, got {}",
                    args.len()
                ));
            }
            index_of_value(object.clone(), evaluator.eval_expr(&args[0])?)
        }
        (Value::String(_), "replace") => {
            if args.len() != 2 {
                return Err(format!(
                    "String method replace() expects 2 arguments, got {}",
                    args.len()
                ));
            }
            let old = evaluator.eval_expr(&args[0])?;
            let new = evaluator.eval_expr(&args[1])?;
            replace_value(object.clone(), old, new)
        }
        (Value::String(_), "substr") => {
            if args.len() != 2 {
                return Err(format!(
                    "String method substr() expects 2 arguments, got {}",
                    args.len()
                ));
            }
            let start = evaluator.eval_expr(&args[0])?;
            let length = evaluator.eval_expr(&args[1])?;
            substr_value(object.clone(), start, length)
        }

        // Array methods
        (Value::Array(_), "push") => {
            if args.len() != 1 {
                return Err(format!(
                    "Array method push() expects 1 argument, got {}",
                    args.len()
                ));
            }
            let element = evaluator.eval_expr(&args[0])?;
            push_value(object.clone(), element)
        }
        (Value::Array(_), "pop") => {
            if !args.is_empty() {
                return Err(format!(
                    "Array method pop() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            pop_value(object.clone())
        }
        (Value::Array(_), "first") => {
            if !args.is_empty() {
                return Err(format!(
                    "Array method first() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            first_value(object.clone())
        }
        (Value::Array(_), "last") => {
            if !args.is_empty() {
                return Err(format!(
                    "Array method last() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            last_value(object.clone())
        }
        (Value::Array(_), "reverse") => {
            if !args.is_empty() {
                return Err(format!(
                    "Array method reverse() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            reverse_value(object.clone())
        }
        (Value::Array(_), "join") => {
            if args.len() != 1 {
                return Err(format!(
                    "Array method join() expects 1 argument, got {}",
                    args.len()
                ));
            }
            join_value(object.clone(), evaluator.eval_expr(&args[0])?)
        }
        (Value::Array(_), "contains") => {
            if args.len() != 1 {
                return Err(format!(
                    "Array method contains() expects 1 argument, got {}",
                    args.len()
                ));
            }
            contains_value(object.clone(), evaluator.eval_expr(&args[0])?)
        }
        (Value::Array(_), "index_of") => {
            if args.len() != 1 {
                return Err(format!(
                    "Array method index_of() expects 1 argument, got {}",
                    args.len()
                ));
            }
            index_of_value(object.clone(), evaluator.eval_expr(&args[0])?)
        }
        (Value::Array(_), "sort") => {
            if !args.is_empty() {
                return Err(format!(
                    "Array method sort() expects 0 arguments, got {}",
                    args.len()
                ));
            }
            sort_value(object.clone())
        }

        // Unknown method
        (obj, method_name) => Err(format!(
            "Unknown method '{}' for type '{}'",
            method_name,
            obj.type_name()
        )),
    }
}
