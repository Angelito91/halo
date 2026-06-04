// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::expressions;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;

/// Emit code for an expression statement
pub fn emit_expr_stmt<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    expr: &Expression,
) -> Result<bool, String> {
    expressions::emit_expression(gen, expr)?;
    Ok(false)
}
