// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::expressions;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;

/// Emit code for print() or println()
pub fn emit_print<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
    add_newline: bool,
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.is_empty() && !add_newline {
        // No arguments: just emit a newline.
        Emitter::emit_printf_str(gen, "\n")?;
        return Ok(gen.type_mapper.i64_type().const_int(0, false).into());
    }

    for (i, arg_expr) in args.iter().enumerate() {
        if i > 0 {
            // Separate multiple arguments with a space.
            Emitter::emit_printf_str(gen, " ")?;
        }
        let val = expressions::emit_expression(gen, arg_expr)?;
        Emitter::emit_printf_for_value(gen, val)?;
    }

    // Always end with a newline if add_newline is true.
    if add_newline {
        Emitter::emit_printf_str(gen, "\n")?;
    }
    Ok(gen.type_mapper.i64_type().const_int(0, false).into())
}
