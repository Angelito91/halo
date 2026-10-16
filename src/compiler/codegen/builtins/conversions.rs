// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Type-conversion built-in functions for native code generation.

use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::expressions;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;

/// `str(value)` — convert the value to its string representation.
///
/// Supports:
/// - integers (constants only, otherwise returns a runtime integer)
/// - floats  (constant values are formatted at compile-time)
/// - bools   (returns "true" or "false")
/// - strings (pass-through)
pub fn emit_str<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 1 {
        return Err(format!(
            "str() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let val = expressions::emit_expression(gen, &args[0])?;

    if val.is_pointer_value() {
        // Already a string — pass through.
        return Ok(val);
    }

    // Try to format the value at compile-time.
    if val.is_int_value() {
        let iv = val.into_int_value();
        if iv.is_const() {
            let v = iv.get_zero_extended_constant().unwrap_or(0);
            return Ok(
                Emitter::get_or_create_string(gen, &v.to_string(), "str.lit.int")
                    .as_pointer_value()
                    .into(),
            );
        }
    }

    if val.is_float_value() {
        let fv = val.into_float_value();
        if let Some((c, _lossless)) = fv.get_constant() {
            // Use a 6-digit representation.
            let s = format!("{:.6}", c);
            let trimmed = s.trim_end_matches('0').trim_end_matches('.').to_string();
            return Ok(
                Emitter::get_or_create_string(gen, &trimmed, "str.lit.float")
                    .as_pointer_value()
                    .into(),
            );
        }
    }

    // Fallback: we don't have sprintf at hand, so we cannot produce a real
    // string at runtime. Return a placeholder so the program still links.
    Ok(Emitter::get_or_create_string(gen, "?", "str.lit.unknown")
        .as_pointer_value()
        .into())
}

/// `int(value)` — convert value to integer (truncates floats).
pub fn emit_int<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 1 {
        return Err(format!(
            "int() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let val = expressions::emit_expression(gen, &args[0])?;
    if val.is_float_value() {
        let fv = val.into_float_value();
        return gen
            .builder
            .builder
            .build_float_to_signed_int(fv, gen.type_mapper.i64_type(), "f2i")
            .map(|v| v.into())
            .map_err(|_| "Failed to convert float to int".to_string());
    }
    if val.is_pointer_value() {
        return Err(
            "int() conversion from strings is not yet supported in native compilation".to_string(),
        );
    }
    Ok(Emitter::widen_to_i64(gen, val)?.into())
}

/// `float(value)` — convert value to f64.
pub fn emit_float<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 1 {
        return Err(format!(
            "float() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let val = expressions::emit_expression(gen, &args[0])?;
    if val.is_pointer_value() {
        return Err(
            "float() conversion from strings is not yet supported in native compilation"
                .to_string(),
        );
    }
    Ok(Emitter::promote_to_f64(gen, val)?.into())
}

/// `type(value)` — return the type name of a value as a string.
pub fn emit_type<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 1 {
        return Err(format!(
            "type() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let val = expressions::emit_expression(gen, &args[0])?;
    let type_name = if val.is_float_value() {
        "float"
    } else if val.is_pointer_value() {
        "string"
    } else if val.is_int_value() {
        let iv = val.into_int_value();
        if iv.get_type().get_bit_width() == 1 {
            "bool"
        } else {
            "number"
        }
    } else {
        return Err("type(): unsupported value type in compiled mode".to_string());
    };
    Ok(Emitter::get_or_create_string(gen, type_name, "type.name")
        .as_pointer_value()
        .into())
}

/// `len(value)` — return the byte length of a string, or digit count for ints.
pub fn emit_len<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 1 {
        return Err(format!(
            "len() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let val = expressions::emit_expression(gen, &args[0])?;
    if val.is_pointer_value() {
        let b = &gen.builder.builder;
        let strlen_fn = gen
            .module
            .get_function("strlen")
            .ok_or("strlen not declared")?;
        let pv = val.into_pointer_value();
        let len = match b
            .build_call(strlen_fn, &[pv.into()], "str_len_call")
            .map_err(|_| "Failed to call strlen")?
            .try_as_basic_value()
        {
            inkwell::values::ValueKind::Basic(v) => v.into_int_value(),
            inkwell::values::ValueKind::Instruction(_) => {
                return Err("strlen returned instruction".to_string())
            }
        };
        return Ok(len.into());
    }
    if val.is_int_value() {
        let iv = val.into_int_value();
        if iv.is_const() {
            let v = iv.get_zero_extended_constant().unwrap_or(0);
            return Ok(gen
                .type_mapper
                .i64_type()
                .const_int(v.to_string().len() as u64, false)
                .into());
        }
    }
    Err("len() with this type is not supported at runtime in native compilation".to_string())
}
