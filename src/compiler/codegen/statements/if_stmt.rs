// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::{Block, Expression};
use inkwell::values::FunctionValue;

/// Emit code for if/else-if/else statements
pub fn emit_if_stmt<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    function: FunctionValue<'ctx>,
    _fn_name: &str,
    cond: &Expression,
    then_branch: &Block,
    else_if_branches: &[crate::parser::ast::ElseIfBranch],
    else_branch: &Option<Block>,
) -> Result<bool, String> {
    let merge_bb = gen.context.append_basic_block(function, "if.merge");

    let mut all_terminated = true;

    // ── then ──────────────────────────────────────────────────────
    let cond_val = super::super::expressions::emit_expression(gen, cond)?;
    let cond_i1 = gen.coerce_to_bool(cond_val)?;

    let then_bb = gen.context.append_basic_block(function, "if.then");
    let next_bb = gen.context.append_basic_block(function, "if.next");

    gen.builder
        .builder
        .build_conditional_branch(cond_i1, then_bb, next_bb)
        .map_err(|_| "Failed to build if conditional branch")?;

    gen.builder.builder.position_at_end(then_bb);
    let then_terminated = gen.generate_block(then_branch, function)?;
    if !then_terminated
        && gen
            .builder
            .builder
            .get_insert_block()
            .and_then(|b| b.get_terminator())
            .is_none()
    {
        gen.builder
            .builder
            .build_unconditional_branch(merge_bb)
            .map_err(|_| "Failed to branch to if.merge from then")?;
    }
    if !then_terminated {
        all_terminated = false;
    }

    // ── else if chains ────────────────────────────────────────────
    let mut current_next_bb = next_bb;
    for (idx, branch) in else_if_branches.iter().enumerate() {
        gen.builder.builder.position_at_end(current_next_bb);

        let branch_cond_val = super::super::expressions::emit_expression(gen, &branch.cond)?;
        let branch_cond_i1 = gen.coerce_to_bool(branch_cond_val)?;

        let branch_then_bb = gen
            .context
            .append_basic_block(function, &format!("elif.then.{}", idx));
        let branch_next_bb = gen
            .context
            .append_basic_block(function, &format!("elif.next.{}", idx));

        gen.builder
            .builder
            .build_conditional_branch(branch_cond_i1, branch_then_bb, branch_next_bb)
            .map_err(|_| "Failed to build else-if conditional branch")?;

        gen.builder.builder.position_at_end(branch_then_bb);
        let branch_terminated = gen.generate_block(&branch.body, function)?;
        if !branch_terminated
            && gen
                .builder
                .builder
                .get_insert_block()
                .and_then(|b| b.get_terminator())
                .is_none()
        {
            gen.builder
                .builder
                .build_unconditional_branch(merge_bb)
                .map_err(|_| "Failed to branch to if.merge from else-if")?;
        }
        if !branch_terminated {
            all_terminated = false;
        }

        current_next_bb = branch_next_bb;
    }

    // ── else (or fall-through) ─────────────────────────────────────
    gen.builder.builder.position_at_end(current_next_bb);
    let else_terminated = if let Some(else_b) = else_branch {
        let t = gen.generate_block(else_b, function)?;
        if !t
            && gen
                .builder
                .builder
                .get_insert_block()
                .and_then(|b| b.get_terminator())
                .is_none()
        {
            gen.builder
                .builder
                .build_unconditional_branch(merge_bb)
                .map_err(|_| "Failed to branch to if.merge from else")?;
        }
        t
    } else {
        // No else branch → fall through to merge.
        gen.builder
            .builder
            .build_unconditional_branch(merge_bb)
            .map_err(|_| "Failed to branch to if.merge (no else)")?;
        false
    };

    if !else_terminated {
        all_terminated = false;
    }

    gen.builder.builder.position_at_end(merge_bb);

    // Only report termination when every branch (including the
    // implicit fall-through when there is no else) terminates.
    let has_else = else_branch.is_some();
    Ok(all_terminated && has_else)
}
