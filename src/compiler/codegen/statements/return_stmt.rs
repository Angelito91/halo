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

/// Emit code for return statement
pub fn emit_return<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    value: &Option<Expression>,
) -> Result<bool, String> {
    let ret_val: BasicValueEnum<'ctx> = match value {
        Some(expr) => {
            let v = expressions::emit_expression(gen, expr)?;
            // Functions always return i64; widen if needed.
            Emitter::widen_to_i64(gen, v)?
        }
        None => gen.type_mapper.i64_type().const_int(0, false),
    }
    .into();

    gen.builder
        .builder
        .build_return(Some(&ret_val))
        .map_err(|_| "Failed to build return")?;
    Ok(true)
}
