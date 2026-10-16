// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use super::super::value::Value;
use super::core::Evaluator;
use crate::parser::ast::Expression;

/// Try to evaluate a built-in function call.
/// Returns None if the name is not a built-in function.
pub fn try_eval_builtin(
    evaluator: &mut Evaluator,
    name: &str,
    args: &[Expression],
) -> Result<Option<Value>, String> {
    let result = match name {
        "print" => builtin_print(evaluator, args)?,
        "len" => builtin_single_arg(evaluator, name, args, |v| match v {
            Value::String(s) => Ok(Value::Number(s.len() as i64)),
            Value::Array(arr) => Ok(Value::Number(arr.len() as i64)),
            Value::Number(n) => Ok(Value::Number(n.to_string().len() as i64)),
            other => Err(format!(
                "len() does not support type '{}'",
                other.type_name()
            )),
        })?,
        "str" => builtin_single_arg(evaluator, name, args, |v| {
            Ok(Value::String(v.to_string_value()))
        })?,
        "int" => builtin_single_arg(evaluator, name, args, |v| Ok(Value::Number(v.to_int()?)))?,
        "float" => builtin_single_arg(evaluator, name, args, |v| Ok(Value::Float(v.to_number()?)))?,
        "abs" => builtin_single_arg(evaluator, name, args, |v| match v {
            Value::Number(n) => Ok(Value::Number(n.abs())),
            Value::Float(f) => Ok(Value::Float(f.abs())),
            other => Err(format!(
                "abs() does not support type '{}'",
                other.type_name()
            )),
        })?,
        "type" => builtin_single_arg(evaluator, name, args, |v| {
            Ok(Value::String(v.type_name().to_string()))
        })?,
        // Array methods that can also be called as functions
        "push" => super::methods::builtin_push(evaluator, args)?,
        "pop" => super::methods::builtin_pop(evaluator, args)?,
        "append" => super::methods::builtin_append(evaluator, args)?,
        "first" => super::methods::builtin_first(evaluator, args)?,
        "last" => super::methods::builtin_last(evaluator, args)?,
        "reverse" => super::methods::builtin_reverse(evaluator, args)?,
        "join" => super::methods::builtin_join(evaluator, args)?,
        "contains" => super::methods::builtin_contains(evaluator, args)?,
        "index_of" => super::methods::builtin_index_of(evaluator, args)?,
        "sort" => super::methods::builtin_sort(evaluator, args)?,
        "upper" => super::methods::builtin_upper(evaluator, args)?,
        "lower" => super::methods::builtin_lower(evaluator, args)?,
        "trim" => super::methods::builtin_trim(evaluator, args)?,
        "starts_with" => super::methods::builtin_starts_with(evaluator, args)?,
        "ends_with" => super::methods::builtin_ends_with(evaluator, args)?,
        "replace" => super::methods::builtin_replace(evaluator, args)?,
        "substr" => super::methods::builtin_substr(evaluator, args)?,
        // Not a built-in.
        _ => return Ok(None),
    };
    Ok(Some(result))
}

/// `print(v1, v2, …)` — prints each argument on its own line.
fn builtin_print(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    for arg in args {
        let val = evaluator.eval_expr(arg)?;
        println!("{val}");
    }
    Ok(Value::Null)
}

/// Helper for built-ins that accept exactly one argument.
/// Evaluates the single argument, then calls `f` with the resulting Value.
/// Returns an arity error if the argument count is wrong.
fn builtin_single_arg(
    evaluator: &mut Evaluator,
    name: &str,
    args: &[Expression],
    f: impl FnOnce(Value) -> Result<Value, String>,
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "{name}() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let val = evaluator.eval_expr(&args[0])?;
    f(val)
}
