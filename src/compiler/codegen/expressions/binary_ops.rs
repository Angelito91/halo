// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::{BinOp, Expression};
use inkwell::values::BasicValueEnum;

/// Emit code for binary operations
pub fn emit_binop<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    left_expr: &Expression,
    op: &BinOp,
    right_expr: &Expression,
) -> Result<BasicValueEnum<'ctx>, String> {
    let lv = super::emit_expression(gen, left_expr)?;
    let rv = super::emit_expression(gen, right_expr)?;

    if lv.is_float_value() || rv.is_float_value() {
        emit_float_binop(gen, lv, op, rv)
    } else {
        emit_int_binop(gen, lv, op, rv)
    }
}

/// Emit integer binary operations
fn emit_int_binop<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    left: BasicValueEnum<'ctx>,
    op: &BinOp,
    right: BasicValueEnum<'ctx>,
) -> Result<BasicValueEnum<'ctx>, String> {
    // Widen both operands to i64 so bit-widths always match.
    let lv = Emitter::widen_to_i64(gen, left)?;
    let rv = Emitter::widen_to_i64(gen, right)?;
    let b = &gen.builder.builder;

    match op {
        BinOp::Add => Ok(b.build_int_add(lv, rv, "add").map_err(|_| "add")?.into()),
        BinOp::Sub => Ok(b.build_int_sub(lv, rv, "sub").map_err(|_| "sub")?.into()),
        BinOp::Mul => Ok(b.build_int_mul(lv, rv, "mul").map_err(|_| "mul")?.into()),
        BinOp::Div => Ok(b
            .build_int_signed_div(lv, rv, "div")
            .map_err(|_| "div")?
            .into()),
        BinOp::Mod => Ok(b
            .build_int_signed_rem(lv, rv, "mod")
            .map_err(|_| "mod")?
            .into()),
        BinOp::Eq => Ok(b
            .build_int_compare(inkwell::IntPredicate::EQ, lv, rv, "eq")
            .map_err(|_| "eq")?
            .into()),
        BinOp::Neq => Ok(b
            .build_int_compare(inkwell::IntPredicate::NE, lv, rv, "neq")
            .map_err(|_| "neq")?
            .into()),
        BinOp::Lt => Ok(b
            .build_int_compare(inkwell::IntPredicate::SLT, lv, rv, "lt")
            .map_err(|_| "lt")?
            .into()),
        BinOp::Gt => Ok(b
            .build_int_compare(inkwell::IntPredicate::SGT, lv, rv, "gt")
            .map_err(|_| "gt")?
            .into()),
        BinOp::Le => Ok(b
            .build_int_compare(inkwell::IntPredicate::SLE, lv, rv, "le")
            .map_err(|_| "le")?
            .into()),
        BinOp::Ge => Ok(b
            .build_int_compare(inkwell::IntPredicate::SGE, lv, rv, "ge")
            .map_err(|_| "ge")?
            .into()),
        BinOp::And => Ok(b.build_and(lv, rv, "and").map_err(|_| "and")?.into()),
        BinOp::Or => Ok(b.build_or(lv, rv, "or").map_err(|_| "or")?.into()),
    }
}

/// Emit floating-point binary operations
fn emit_float_binop<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    left: BasicValueEnum<'ctx>,
    op: &BinOp,
    right: BasicValueEnum<'ctx>,
) -> Result<BasicValueEnum<'ctx>, String> {
    // Promote both operands to f64.
    let lv = Emitter::promote_to_f64(gen, left)?;
    let rv = Emitter::promote_to_f64(gen, right)?;
    let b = &gen.builder.builder;

    match op {
        BinOp::Add => Ok(b
            .build_float_add(lv, rv, "fadd")
            .map_err(|_| "fadd")?
            .into()),
        BinOp::Sub => Ok(b
            .build_float_sub(lv, rv, "fsub")
            .map_err(|_| "fsub")?
            .into()),
        BinOp::Mul => Ok(b
            .build_float_mul(lv, rv, "fmul")
            .map_err(|_| "fmul")?
            .into()),
        BinOp::Div => Ok(b
            .build_float_div(lv, rv, "fdiv")
            .map_err(|_| "fdiv")?
            .into()),
        BinOp::Mod => Ok(b
            .build_float_rem(lv, rv, "frem")
            .map_err(|_| "frem")?
            .into()),
        BinOp::Eq => Ok(b
            .build_float_compare(inkwell::FloatPredicate::OEQ, lv, rv, "feq")
            .map_err(|_| "feq")?
            .into()),
        BinOp::Neq => Ok(b
            .build_float_compare(inkwell::FloatPredicate::ONE, lv, rv, "fneq")
            .map_err(|_| "fneq")?
            .into()),
        BinOp::Lt => Ok(b
            .build_float_compare(inkwell::FloatPredicate::OLT, lv, rv, "flt")
            .map_err(|_| "flt")?
            .into()),
        BinOp::Gt => Ok(b
            .build_float_compare(inkwell::FloatPredicate::OGT, lv, rv, "fgt")
            .map_err(|_| "fgt")?
            .into()),
        BinOp::Le => Ok(b
            .build_float_compare(inkwell::FloatPredicate::OLE, lv, rv, "fle")
            .map_err(|_| "fle")?
            .into()),
        BinOp::Ge => Ok(b
            .build_float_compare(inkwell::FloatPredicate::OGE, lv, rv, "fge")
            .map_err(|_| "fge")?
            .into()),
        // Logical ops: compare each side to 0.0, then combine as i1.
        BinOp::And | BinOp::Or => {
            let zero = gen.type_mapper.f64_type().const_float(0.0);
            let lb = b
                .build_float_compare(inkwell::FloatPredicate::ONE, lv, zero, "ftob_l")
                .map_err(|_| "ftob_l")?;
            let rb = b
                .build_float_compare(inkwell::FloatPredicate::ONE, rv, zero, "ftob_r")
                .map_err(|_| "ftob_r")?;
            let result = match op {
                BinOp::And => b.build_and(lb, rb, "and").map_err(|_| "and")?,
                _ => b.build_or(lb, rb, "or").map_err(|_| "or")?,
            };
            Ok(result.into())
        }
    }
}
