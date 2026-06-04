// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::builtins;
use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};

/// Emit code for function calls (both built-in and user-defined)
pub fn emit_function_call<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    name: &str,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    // Try built-in functions first
    if let Some(result) = builtins::emit_builtin(gen, name, args)? {
        return Ok(result);
    }

    // Otherwise, try user-defined functions
    let function = *gen
        .functions
        .get(name)
        .ok_or_else(|| format!("Undefined function: '{}'", name))?;

    let mut call_args: Vec<BasicMetadataValueEnum<'ctx>> = Vec::new();
    for arg_expr in args {
        let val = super::emit_expression(gen, arg_expr)?;
        // All user functions take i64 parameters; coerce accordingly.
        if val.is_float_value() {
            return Err(format!(
                "Cannot pass a float directly to function '{}'; \
                 only integer arguments are supported in this version",
                name
            ));
        }
        let widened = Emitter::widen_to_i64(gen, val)?;
        call_args.push(widened.into());
    }

    let call_site = gen
        .builder
        .builder
        .build_call(function, &call_args, "call")
        .map_err(|_| format!("Failed to call '{}'", name))?;

    match call_site.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(val) => Ok(val),
        inkwell::values::ValueKind::Instruction(_) => {
            Ok(gen.type_mapper.i64_type().const_int(0, false).into())
        }
    }
}
