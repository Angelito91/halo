use crate::compiler::codegen::builtins;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;

/// Emit a method call.
///
/// Method calls like `"hello".upper()` are translated to the corresponding
/// built-in function call with the object as the first argument.
pub fn emit_method_call<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    object: &Expression,
    method: &str,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    match method {
        "upper" | "lower" | "trim" | "reverse" => {
            if !args.is_empty() {
                return Err(format!(
                    "String method {}() expects 0 arguments, got {}",
                    method,
                    args.len()
                ));
            }
            let with_args = build_method_args(object, args);
            dispatch_method(gen, method, &with_args)
        }
        "starts_with" | "ends_with" | "contains" | "index_of" => {
            if args.len() != 1 {
                return Err(format!(
                    "String method {}() expects 1 argument, got {}",
                    method,
                    args.len()
                ));
            }
            let with_args = build_method_args(object, args);
            dispatch_method(gen, method, &with_args)
        }
        "replace" => {
            if args.len() != 2 {
                return Err(format!(
                    "String method replace() expects 2 arguments, got {}",
                    args.len()
                ));
            }
            let with_args = build_method_args(object, args);
            dispatch_method(gen, method, &with_args)
        }
        "substr" => {
            if args.len() != 2 {
                return Err(format!(
                    "String method substr() expects 2 arguments, got {}",
                    args.len()
                ));
            }
            let with_args = build_method_args(object, args);
            dispatch_method(gen, method, &with_args)
        }
        "push" | "pop" | "first" | "last" | "join" | "sort" => Err(format!(
            "Method '{}' is not yet supported in compiled mode (arrays are not implemented)",
            method
        )),
        _ => Err(format!("Unknown method '{}'", method)),
    }
}

fn build_method_args(object: &Expression, extra_args: &[Expression]) -> Vec<Expression> {
    let mut with_args = vec![object.clone()];
    with_args.extend(extra_args.iter().cloned());
    with_args
}

fn dispatch_method<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    method: &str,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if let Some(result) = builtins::emit_builtin(gen, method, args)? {
        return Ok(result);
    }
    Err(format!(
        "Built-in function '{}' not found in compiled mode",
        method
    ))
}
