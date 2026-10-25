// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Built-in function dispatch. Arguments are already evaluated.

use crate::interpreter::Value;

/// Every name accepted by [`call_builtin`].
pub const BUILTIN_NAMES: &[&str] = &[
    "print",
    "println",
    "len",
    "str",
    "int",
    "float",
    "abs",
    "type",
    "range",
    "push",
    "pop",
    "append",
    "first",
    "last",
    "reverse",
    "join",
    "contains",
    "index_of",
    "sort",
    "upper",
    "lower",
    "trim",
    "starts_with",
    "ends_with",
    "replace",
    "substr",
];

/// Whether `name` is a built-in function.
pub fn is_builtin(name: &str) -> bool {
    BUILTIN_NAMES.contains(&name)
}

/// Evaluate a built-in function on already-evaluated arguments.
///
/// Returns `None` when `name` is not a built-in, so the evaluator can fall
/// back to user-defined functions.
pub fn call_builtin(name: &str, args: &[Value]) -> Result<Option<Value>, String> {
    let result = match name {
        "print" | "println" => builtin_print(args)?,
        "len" => builtin_single_arg(name, args, |v| match v {
            Value::String(s) => Ok(Value::Number(s.len() as i64)),
            Value::Array(arr) => Ok(Value::Number(arr.len() as i64)),
            Value::Number(n) => Ok(Value::Number(n.to_string().len() as i64)),
            other => Err(format!(
                "len() does not support type '{}'",
                other.type_name()
            )),
        })?,
        "str" => builtin_single_arg(name, args, |v| Ok(Value::String(v.to_string_value())))?,
        "int" => builtin_single_arg(name, args, |v| Ok(Value::Number(v.to_int()?)))?,
        "float" => builtin_single_arg(name, args, |v| Ok(Value::Float(v.to_number()?)))?,
        "abs" => builtin_single_arg(name, args, |v| match v {
            Value::Number(n) => Ok(Value::Number(n.abs())),
            Value::Float(f) => Ok(Value::Float(f.abs())),
            other => Err(format!(
                "abs() does not support type '{}'",
                other.type_name()
            )),
        })?,
        "type" => builtin_single_arg(name, args, |v| Ok(Value::String(v.type_name().to_string())))?,
        "range" => super::arrays::builtin_range(args)?,
        // Array functions.
        "push" => super::arrays::builtin_push(args)?,
        "pop" => super::arrays::builtin_pop(args)?,
        "append" => super::arrays::builtin_append(args)?,
        "first" => super::arrays::builtin_first(args)?,
        "last" => super::arrays::builtin_last(args)?,
        "reverse" => super::arrays::builtin_reverse(args)?,
        "join" => super::arrays::builtin_join(args)?,
        "sort" => super::arrays::builtin_sort(args)?,
        // String functions.
        "contains" => super::strings::builtin_contains(args)?,
        "index_of" => super::strings::builtin_index_of(args)?,
        "upper" => super::strings::builtin_upper(args)?,
        "lower" => super::strings::builtin_lower(args)?,
        "trim" => super::strings::builtin_trim(args)?,
        "starts_with" => super::strings::builtin_starts_with(args)?,
        "ends_with" => super::strings::builtin_ends_with(args)?,
        "replace" => super::strings::builtin_replace(args)?,
        "substr" => super::strings::builtin_substr(args)?,
        // Not a built-in.
        _ => return Ok(None),
    };
    Ok(Some(result))
}

/// `print(v1, v2, …)` / `println(v1, v2, …)` — print all arguments on one
/// line, separated by spaces, followed by a newline.
fn builtin_print(args: &[Value]) -> Result<Value, String> {
    let rendered: Vec<String> = args.iter().map(|v| v.to_string_value()).collect();
    println!("{}", rendered.join(" "));
    Ok(Value::Null)
}

/// Helper for built-ins that accept exactly one argument.
fn builtin_single_arg(
    name: &str,
    args: &[Value],
    f: impl FnOnce(Value) -> Result<Value, String>,
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "{name}() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    f(args[0].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_listed_builtin_is_dispatched() {
        for &name in BUILTIN_NAMES {
            assert!(is_builtin(name), "{name} is not recognised");
            assert!(
                !matches!(call_builtin(name, &[]), Ok(None)),
                "{name} is listed but not dispatched"
            );
        }
    }

    #[test]
    fn unknown_names_are_not_builtins() {
        assert!(!is_builtin("definitely_not_a_builtin"));
        assert!(matches!(
            call_builtin("definitely_not_a_builtin", &[]),
            Ok(None)
        ));
    }
}
