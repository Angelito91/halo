// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::{BinOp, Expression};
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

/// Helper: convert a BasicValueEnum to a PointerValue.
/// If the value is already a pointer, return as-is.
/// If it's an int, assume it's a ptrtoint'd pointer and inttoptr it back.
fn as_pointer<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    val: BasicValueEnum<'ctx>,
    name: &str,
) -> Result<inkwell::values::PointerValue<'ctx>, String> {
    if val.is_pointer_value() {
        return Ok(val.into_pointer_value());
    }
    let ptr_ty = gen.type_mapper.ptr_type();
    let iv = Emitter::widen_to_i64(gen, val)?;
    gen.builder
        .build_int_to_ptr(iv, ptr_ty, name)
        .map_err(|_| format!("as_pointer: int_to_ptr failed for {}", name))
}

/// Emit code for binary operations
pub fn emit_binop<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    left_expr: &Expression,
    op: &BinOp,
    right_expr: &Expression,
) -> Result<BasicValueEnum<'ctx>, String> {
    let lv = super::emit_expression(gen, left_expr)?;
    let rv = super::emit_expression(gen, right_expr)?;

    // Equality between values of different runtime categories is always
    // false in the interpreter (e.g. `1 == "1"` and `true == 1`). The
    // compiled backend would otherwise reuse strcmp/intcmp on mismatched
    // representations.
    if matches!(op, BinOp::Eq | BinOp::Neq) {
        let mixed_pointer = (lv.is_pointer_value() || rv.is_pointer_value())
            && !(lv.is_pointer_value() && rv.is_pointer_value());
        let mixed_bool = lv.is_int_value()
            && rv.is_int_value()
            && (lv.into_int_value().get_type().get_bit_width() == 1)
                != (rv.into_int_value().get_type().get_bit_width() == 1);
        if mixed_pointer || mixed_bool {
            let result = matches!(op, BinOp::Neq);
            return Ok(gen
                .type_mapper
                .i1_type()
                .const_int(result as u64, false)
                .into());
        }
    }

    // String operations: `+` (concatenation), `*` (repetition),
    // and `==` / `!=` (comparison via strcmp).
    if lv.is_pointer_value() || rv.is_pointer_value() {
        return match op {
            BinOp::Add => emit_string_concat(gen, lv, rv),
            BinOp::Mul => emit_string_repeat(gen, lv, rv),
            BinOp::Eq => emit_string_compare(gen, lv, rv, IntPredicate::EQ),
            BinOp::Neq => emit_string_compare(gen, lv, rv, IntPredicate::NE),
            _ => Err(format!(
                "Binary operator '{:?}' is not supported on strings in native compilation. \
                 Only '+' (concat), '*' (repeat), '==' and '!=' are supported.",
                op
            )),
        };
    }

    if lv.is_float_value() || rv.is_float_value() {
        emit_float_binop(gen, lv, op, rv)
    } else {
        emit_int_binop(gen, lv, op, rv)
    }
}

/// Concatenate two null-terminated C strings into a freshly allocated buffer.
fn emit_string_concat<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    left: BasicValueEnum<'ctx>,
    right: BasicValueEnum<'ctx>,
) -> Result<BasicValueEnum<'ctx>, String> {
    let left_ptr = Emitter::value_to_string(gen, left)?;
    let right_ptr = Emitter::value_to_string(gen, right)?;

    let strlen_fn = gen
        .module
        .get_function("strlen")
        .ok_or("strlen not declared")?;
    let malloc_fn = gen
        .module
        .get_function("malloc")
        .ok_or("malloc not declared")?;
    let strcpy_fn = gen
        .module
        .get_function("strcpy")
        .ok_or("strcpy not declared")?;
    let strcat_fn = gen
        .module
        .get_function("strcat")
        .ok_or("strcat not declared")?;

    let b = &gen.builder;
    let i64_type = gen.type_mapper.i64_type();
    let i8_ptr = gen
        .type_mapper
        .context
        .ptr_type(inkwell::AddressSpace::default());

    let left_len = match b
        .build_call(strlen_fn, &[left_ptr.into()], "left_len")
        .map_err(|_| "Failed to call strlen")?
        .try_as_basic_value()
    {
        inkwell::values::ValueKind::Basic(v) => v.into_int_value(),
        inkwell::values::ValueKind::Instruction(_) => {
            return Err("strlen returned instruction".to_string())
        }
    };
    let right_len = match b
        .build_call(strlen_fn, &[right_ptr.into()], "right_len")
        .map_err(|_| "Failed to call strlen")?
        .try_as_basic_value()
    {
        inkwell::values::ValueKind::Basic(v) => v.into_int_value(),
        inkwell::values::ValueKind::Instruction(_) => {
            return Err("strlen returned instruction".to_string())
        }
    };

    // Allocate len(left) + len(right) + 1 bytes.
    let sum = b
        .build_int_add(left_len, right_len, "sum_len")
        .map_err(|_| "Failed to add lengths")?;
    let one = i64_type.const_int(1, false);
    let alloc_size = b
        .build_int_add(sum, one, "alloc_size")
        .map_err(|_| "Failed to add 1")?;

    let buf = match b
        .build_call(malloc_fn, &[alloc_size.into()], "malloc_buf")
        .map_err(|_| "Failed to call malloc")?
        .try_as_basic_value()
    {
        inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
        inkwell::values::ValueKind::Instruction(_) => {
            return Err("malloc returned instruction".to_string())
        }
    };

    // Copy left into buf, then append right.
    b.build_call(strcpy_fn, &[buf.into(), left_ptr.into()], "strcpy_l")
        .map_err(|_| "Failed to call strcpy")?;
    b.build_call(strcat_fn, &[buf.into(), right_ptr.into()], "strcat_r")
        .map_err(|_| "Failed to call strcat")?;

    let _: BasicTypeEnum = i8_ptr.into();
    Ok(buf.into())
}

/// Repeat a string `n` times. `n` must be a non-negative integer.
fn emit_string_repeat<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    left: BasicValueEnum<'ctx>,
    right: BasicValueEnum<'ctx>,
) -> Result<BasicValueEnum<'ctx>, String> {
    let str_ptr = as_pointer(gen, left, "repeat.str")?;
    let n_val = right.into_int_value();

    let strlen_fn = gen
        .module
        .get_function("strlen")
        .ok_or("strlen not declared")?;
    let malloc_fn = gen
        .module
        .get_function("malloc")
        .ok_or("malloc not declared")?;
    let strcpy_fn = gen
        .module
        .get_function("strcpy")
        .ok_or("strcpy not declared")?;
    let strcat_fn = gen
        .module
        .get_function("strcat")
        .ok_or("strcat not declared")?;

    let b = &gen.builder;
    let i64_type = gen.type_mapper.i64_type();
    let i8_ptr = gen
        .type_mapper
        .context
        .ptr_type(inkwell::AddressSpace::default());

    let str_len = match b
        .build_call(strlen_fn, &[str_ptr.into()], "str_len")
        .map_err(|_| "Failed to call strlen")?
        .try_as_basic_value()
    {
        inkwell::values::ValueKind::Basic(v) => v.into_int_value(),
        inkwell::values::ValueKind::Instruction(_) => {
            return Err("strlen returned instruction".to_string())
        }
    };

    // alloc_size = len(s) * n + 1
    let alloc_size = b
        .build_int_mul(str_len, n_val, "alloc_size_pre")
        .map_err(|_| "Failed to multiply")?;
    let one = i64_type.const_int(1, false);
    let alloc_size = b
        .build_int_add(alloc_size, one, "alloc_size")
        .map_err(|_| "Failed to add 1")?;

    let buf = match b
        .build_call(malloc_fn, &[alloc_size.into()], "malloc_buf")
        .map_err(|_| "Failed to call malloc")?
        .try_as_basic_value()
    {
        inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
        inkwell::values::ValueKind::Instruction(_) => {
            return Err("malloc returned instruction".to_string())
        }
    };

    // Initialise buf with the first copy of the string.
    b.build_call(strcpy_fn, &[buf.into(), str_ptr.into()], "strcpy_init")
        .map_err(|_| "Failed to call strcpy")?;

    // The current insert block must be the loop header, so capture it.
    let cur_bb = b
        .get_insert_block()
        .ok_or("No current insert block for repeat loop")?;
    let function = cur_bb
        .get_parent()
        .ok_or("Insert block has no parent function")?;

    // Create loop blocks.
    let cond_bb = gen.context.append_basic_block(function, "repeat.cond");
    let body_bb = gen.context.append_basic_block(function, "repeat.body");
    let exit_bb = gen.context.append_basic_block(function, "repeat.exit");

    // Initialise the counter on the stack.
    let counter_alloca = b
        .build_alloca(i64_type, "repeat.i")
        .map_err(|_| "Failed to build alloca")?;
    b.build_store(counter_alloca, i64_type.const_int(1, false))
        .map_err(|_| "Failed to store initial counter")?;
    b.build_unconditional_branch(cond_bb)
        .map_err(|_| "Failed to branch to cond")?;

    // cond: counter < n
    b.position_at_end(cond_bb);
    let cur = b
        .build_load(i64_type, counter_alloca, "cur")
        .map_err(|_| "Failed to load counter")?
        .into_int_value();
    let cond = b
        .build_int_compare(inkwell::IntPredicate::SLT, cur, n_val, "repeat.cond")
        .map_err(|_| "Failed to build compare")?;
    b.build_conditional_branch(cond, body_bb, exit_bb)
        .map_err(|_| "Failed to build conditional branch")?;

    // body: strcat(buf, str); i = i + 1
    b.position_at_end(body_bb);
    b.build_call(strcat_fn, &[buf.into(), str_ptr.into()], "strcat_body")
        .map_err(|_| "Failed to call strcat")?;
    let cur2 = b
        .build_load(i64_type, counter_alloca, "cur2")
        .map_err(|_| "Failed to load counter")?
        .into_int_value();
    let next = b
        .build_int_add(cur2, i64_type.const_int(1, false), "next")
        .map_err(|_| "Failed to increment")?;
    b.build_store(counter_alloca, next)
        .map_err(|_| "Failed to store counter")?;
    b.build_unconditional_branch(cond_bb)
        .map_err(|_| "Failed to branch back")?;

    b.position_at_end(exit_bb);

    let _: BasicTypeEnum = i8_ptr.into();
    Ok(buf.into())
}

/// Compare two strings using `strcmp`, returning an i64 boolean (1 or 0).
fn emit_string_compare<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    left: BasicValueEnum<'ctx>,
    right: BasicValueEnum<'ctx>,
    pred: IntPredicate,
) -> Result<BasicValueEnum<'ctx>, String> {
    let left_ptr = as_pointer(gen, left, "strcmp.left")?;
    let right_ptr = as_pointer(gen, right, "strcmp.right")?;

    let strcmp_fn = gen
        .module
        .get_function("strcmp")
        .or_else(|| {
            let i32_ty = gen.type_mapper.i32_type();
            let ptr_ty = gen.type_mapper.ptr_type();
            let fn_ty = i32_ty.fn_type(&[ptr_ty.into(), ptr_ty.into()], false);
            gen.module.add_function("strcmp", fn_ty, None).into()
        })
        .ok_or("strcmp not declared")?;

    let b = &gen.builder;
    let i32_type = gen.type_mapper.i32_type();

    let cs = b
        .build_call(strcmp_fn, &[left_ptr.into(), right_ptr.into()], "strcmp")
        .map_err(|_| "strcmp call failed")?;
    let result = match cs.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(v) => v.into_int_value(),
        _ => return Err("strcmp: unexpected return".to_string()),
    };

    let zero = i32_type.const_zero();
    let cmp = b
        .build_int_compare(pred, result, zero, "strcmp.pred")
        .map_err(|_| "strcmp comparison failed")?;
    Ok(cmp.into())
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

    // Logical operators yield an i1 so `print` can render true/false.
    if matches!(op, BinOp::And | BinOp::Or) {
        let lb = gen.coerce_to_bool(lv.into())?;
        let rb = gen.coerce_to_bool(rv.into())?;
        let result = match op {
            BinOp::And => gen.builder.build_and(lb, rb, "and"),
            _ => gen.builder.build_or(lb, rb, "or"),
        }
        .map_err(|_| "logical op")?;
        return Ok(result.into());
    }

    let b = &gen.builder;

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
    let b = &gen.builder;

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
