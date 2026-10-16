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
        if val.is_float_value() {
            // Pass float as f64 (the function signature uses f64 params when needed)
            call_args.push(val.into_float_value().into());
        } else if val.is_pointer_value() {
            // Pass string pointer as i64 (ptrtoint) — all user function params are i64
            let i64_ty = gen.type_mapper.i64_type();
            let ptr_int = gen
                .builder
                .builder
                .build_ptr_to_int(val.into_pointer_value(), i64_ty, "arg.ptr2int")
                .map_err(|_| "Failed to convert string arg to int")?;
            call_args.push(ptr_int.into());
        } else {
            let widened = Emitter::widen_to_i64(gen, val)?;
            call_args.push(widened.into());
        }
    }

    let call_site = gen
        .builder
        .builder
        .build_call(function, &call_args, "call")
        .map_err(|_| format!("Failed to call '{}'", name))?;

    let ret_ty = gen.fn_return_types.get(name).copied().unwrap_or("i64");
    let raw = match call_site.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(val) => val,
        inkwell::values::ValueKind::Instruction(_) => {
            return Ok(gen.type_mapper.i64_type().const_int(0, false).into());
        }
    };

    match ret_ty {
        "string" => {
            // The function returns a pointer, but it was declared as ptr-type.
            // LLVM already returns the right type.
            Ok(raw)
        }
        "f64" => Ok(raw),
        _ => Ok(raw), // i64 — fine as-is
    }
}
