// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::{Block, Expression};
use inkwell::values::FunctionValue;

/// Emit code for while loops
pub fn emit_while_loop<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    function: FunctionValue<'ctx>,
    _fn_name: &str,
    cond: &Expression,
    body: &Block,
) -> Result<bool, String> {
    let cond_bb = gen.context.append_basic_block(function, "while.cond");
    let body_bb = gen.context.append_basic_block(function, "while.body");
    let exit_bb = gen.context.append_basic_block(function, "while.exit");

    gen.builder
        .build_unconditional_branch(cond_bb)
        .map_err(|_| "Failed to branch to while.cond")?;

    // ── condition ──
    gen.builder.position_at_end(cond_bb);
    let cond_val = super::super::expressions::emit_expression(gen, cond)?;
    let cond_i1 = gen.coerce_to_bool(cond_val)?;
    gen.builder
        .build_conditional_branch(cond_i1, body_bb, exit_bb)
        .map_err(|_| "Failed to build while conditional branch")?;

    // ── body ──
    gen.builder.position_at_end(body_bb);
    // Push loop context so break/continue can find the right blocks.
    gen.loop_stack.push((cond_bb, exit_bb));
    let body_terminated = gen.generate_block(body, function)?;
    gen.loop_stack.pop();

    // Only jump back if the body didn't already terminate.
    if !body_terminated
        && gen
            .builder
            .get_insert_block()
            .and_then(|b| b.get_terminator())
            .is_none()
    {
        gen.builder
            .build_unconditional_branch(cond_bb)
            .map_err(|_| "Failed to branch back to while.cond")?;
    }

    gen.builder.position_at_end(exit_bb);
    Ok(false)
}
