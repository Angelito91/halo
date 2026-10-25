// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::expressions;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::{Block, Expression};
use inkwell::values::{FunctionValue, IntValue};

/// Emit code for `for <var> in <iterable> { … }`.
///
/// Only `range(end)` / `range(start, end)` are supported in compiled mode.
/// The loop lowers to the same branch structure a `while` loop uses, with an
/// extra increment block so that `continue` skips to the next iteration:
///
/// ```text
///   init ─▶ cond ──true──▶ body ──▶ incr ──▶ cond
///             │false
///             ▼
///           exit
/// ```
///
/// Iterating over arrays is rejected with a clear error: compiled binaries
/// do not support array values yet.
pub fn emit_for_loop<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    function: FunctionValue<'ctx>,
    var: &str,
    iterable: &Expression,
    body: &Block,
) -> Result<bool, String> {
    let (start_val, end_val) = emit_range_bounds(gen, iterable)?;

    let i64_type = gen.type_mapper.i64_type();

    // Loop variable slot: a fresh i64 alloca that temporarily shadows any
    // outer binding of the same name (which may hold another type, e.g. a
    // float). The previous mapping is restored once the loop is done.
    let outer_slot = gen.builder.get_slot(var);
    let outer_is_bool = gen.builder.is_bool_local(var);
    let var_slot = Emitter::emit_entry_alloca(&gen.builder, function, var, i64_type.into());
    gen.builder
        .set_alloca(var.to_string(), var_slot, i64_type.into());
    gen.builder.unmark_bool(var);

    // The upper bound is evaluated once, before the first iteration.
    let end_slot = Emitter::emit_entry_alloca(&gen.builder, function, "for.end", i64_type.into());

    gen.builder
        .build_store(var_slot, start_val)
        .map_err(|_| "Failed to initialise the for loop variable")?;
    gen.builder
        .build_store(end_slot, end_val)
        .map_err(|_| "Failed to store the for loop bound")?;

    let cond_bb = gen.context.append_basic_block(function, "for.cond");
    let body_bb = gen.context.append_basic_block(function, "for.body");
    let incr_bb = gen.context.append_basic_block(function, "for.incr");
    let exit_bb = gen.context.append_basic_block(function, "for.exit");

    gen.builder
        .build_unconditional_branch(cond_bb)
        .map_err(|_| "Failed to branch to for.cond")?;

    // ── condition: i < end ──
    gen.builder.position_at_end(cond_bb);
    let current = gen
        .builder
        .build_load(i64_type, var_slot, "for.i")
        .map_err(|_| "Failed to load the for loop variable")?
        .into_int_value();
    let end = gen
        .builder
        .build_load(i64_type, end_slot, "for.end.v")
        .map_err(|_| "Failed to load the for loop bound")?
        .into_int_value();
    let cond = gen
        .builder
        .build_int_compare(inkwell::IntPredicate::SLT, current, end, "for.cond.v")
        .map_err(|_| "Failed to compare the for loop bounds")?;
    gen.builder
        .build_conditional_branch(cond, body_bb, exit_bb)
        .map_err(|_| "Failed to build the for conditional branch")?;

    // ── body ──
    gen.builder.position_at_end(body_bb);
    // `continue` must run the increment step before re-checking the bound.
    gen.loop_stack.push((incr_bb, exit_bb));
    let body_terminated = gen.generate_block(body, function)?;
    gen.loop_stack.pop();

    if !body_terminated
        && gen
            .builder
            .get_insert_block()
            .and_then(|b| b.get_terminator())
            .is_none()
    {
        gen.builder
            .build_unconditional_branch(incr_bb)
            .map_err(|_| "Failed to branch to for.incr")?;
    }

    // ── increment: i = i + 1 ──
    gen.builder.position_at_end(incr_bb);
    let current = gen
        .builder
        .build_load(i64_type, var_slot, "for.i.inc")
        .map_err(|_| "Failed to load the for loop variable")?
        .into_int_value();
    let next = gen
        .builder
        .build_int_add(current, i64_type.const_int(1, false), "for.i.next")
        .map_err(|_| "Failed to increment the for loop variable")?;
    gen.builder
        .build_store(var_slot, next)
        .map_err(|_| "Failed to store the for loop variable")?;
    gen.builder
        .build_unconditional_branch(cond_bb)
        .map_err(|_| "Failed to branch back to for.cond")?;

    gen.builder.position_at_end(exit_bb);

    // The loop variable only exists inside the loop: restore whatever the
    // name meant before, so code after the loop sees the outer binding.
    match outer_slot {
        Some(slot) => gen.builder.set_alloca(var.to_string(), slot.ptr, slot.ty),
        None => gen.builder.remove_slot(var),
    }
    if outer_slot.is_some() && outer_is_bool {
        gen.builder.mark_bool(var);
    } else {
        gen.builder.unmark_bool(var);
    }

    Ok(false)
}

/// Lower the supported `for` iterables to a `(start, end)` integer pair.
///
/// `range(n)` → `(0, n)`, `range(a, b)` → `(a, b)`. Anything else (arrays,
/// variables) is rejected until compiled arrays exist.
fn emit_range_bounds<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    iterable: &Expression,
) -> Result<(IntValue<'ctx>, IntValue<'ctx>), String> {
    let i64_type = gen.type_mapper.i64_type();
    let zero = i64_type.const_zero();

    match iterable {
        Expression::Call { name, args, .. } if name == "range" => {
            let (start_expr, end_expr) = match args.as_slice() {
                [end] => (None, end),
                [start, end] => (Some(start), end),
                _ => {
                    return Err(format!(
                        "range() expects 1 or 2 arguments, got {}",
                        args.len()
                    ))
                }
            };

            let start_val = match start_expr {
                Some(expr) => emit_int_bound(gen, expr, "range() start")?,
                None => zero,
            };
            let end_val = emit_int_bound(gen, end_expr, "range() end")?;
            Ok((start_val, end_val))
        }
        other => Err(format!(
            "for over {} is not yet supported in compiled mode: only range(n) and range(a, b) can be iterated (compiled binaries do not support arrays yet)",
            describe_iterable(other)
        )),
    }
}

/// Emit one `range()` bound, rejecting non-integer expressions.
fn emit_int_bound<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    expr: &Expression,
    what: &str,
) -> Result<IntValue<'ctx>, String> {
    let value = expressions::emit_expression(gen, expr)?;
    if value.is_float_value() {
        return Err(format!("{what} must be an integer expression"));
    }
    Emitter::widen_to_i64(gen, value)
}

/// Human-readable name for an unsupported iterable, used in error messages.
fn describe_iterable(expr: &Expression) -> &'static str {
    match expr {
        Expression::ArrayLiteral { .. } => "an array literal",
        Expression::Var(_, _) => "a variable",
        Expression::StringLiteral(_, _) => "a string",
        _ => "this expression",
    }
}
