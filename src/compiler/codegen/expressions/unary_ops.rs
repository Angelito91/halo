// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;

/// Emit code for unary operations: negation (-) and NOT (!)
pub fn emit_unary<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    operator: &str,
    expr: &Expression,
) -> Result<BasicValueEnum<'ctx>, String> {
    let val = super::emit_expression(gen, expr)?;

    match operator {
        "-" => {
            if val.is_float_value() {
                let neg = gen
                    .builder
                    .builder
                    .build_float_neg(val.into_float_value(), "fneg")
                    .map_err(|_| "Failed to negate float")?;
                Ok(neg.into())
            } else {
                let iv = Emitter::widen_to_i64(gen, val)?;
                let neg = gen
                    .builder
                    .builder
                    .build_int_neg(iv, "ineg")
                    .map_err(|_| "Failed to negate integer")?;
                Ok(neg.into())
            }
        }
        "!" => {
            let iv = gen.coerce_to_bool(val)?;
            let not = gen
                .builder
                .builder
                .build_not(iv, "not")
                .map_err(|_| "Failed to apply NOT")?;
            Ok(not.into())
        }
        op => Err(format!("Unknown unary operator: '{}'", op)),
    }
}
