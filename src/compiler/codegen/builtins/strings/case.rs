// The Halo Programming Language — string builtins: upper/lower/reverse.
use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

use super::helpers::*;

pub fn emit_upper<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 1 {
        return Err(format!("upper() expects 1 arg, got {}", args.len()));
    }
    let s = expect_string(gen, &args[0], "upper", 0)?;
    let i64_ty = gen.type_mapper.i64_type();
    let i8_ty = gen.type_mapper.i8_type();

    let len = call_strlen(gen, s)?;
    let one = i64_ty.const_int(1, false);
    let size = b!(gen)
        .build_int_add(len, one, "upper.size")
        .map_err(|_| "add failed")?;
    let result = call_malloc(gen, size)?;
    let strcpy_fn = get_strcpy(gen)?;
    b!(gen)
        .build_call(strcpy_fn, &[result.into(), s.into()], "upper.copy")
        .map_err(|_| "strcpy failed")?;

    let cf = b!(gen)
        .get_insert_block()
        .and_then(|bb| bb.get_parent())
        .ok_or("no fn")?;
    let i_ptr = Emitter::emit_entry_alloca(&gen.builder, cf, "upper.i", i64_ty.into());
    b!(gen)
        .build_store(i_ptr, i64_ty.const_zero())
        .map_err(|_| "store")?;

    let cond = gen.context.append_basic_block(cf, "upper.cond");
    let body = gen.context.append_basic_block(cf, "upper.body");
    let exit = gen.context.append_basic_block(cf, "upper.exit");
    b!(gen)
        .build_unconditional_branch(cond)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(cond);
    let i = b!(gen)
        .build_load(i64_ty, i_ptr, "i")
        .map_err(|_| "load")?
        .into_int_value();
    let cnd = b!(gen)
        .build_int_compare(IntPredicate::SLT, i, len, "cmp")
        .map_err(|_| "cmp")?;
    b!(gen)
        .build_conditional_branch(cnd, body, exit)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(body);
    let c = load_u8(gen, result, i)?;
    let c32 = b!(gen)
        .build_int_z_extend(c, gen.type_mapper.i32_type(), "zext")
        .map_err(|_| "zext")?;
    let tu = get_toupper(gen)?;
    let uc = call_libc1(gen, tu, c32)?;
    let uc8 = b!(gen)
        .build_int_truncate(uc, i8_ty, "trunc")
        .map_err(|_| "trunc")?;
    store_u8(gen, result, i, uc8)?;
    let ni = b!(gen).build_int_add(i, one, "inc").map_err(|_| "add")?;
    b!(gen).build_store(i_ptr, ni).map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(cond)
        .map_err(|_| "branch")?;
    b!(gen).position_at_end(exit);
    Ok(result.into())
}

pub fn emit_lower<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 1 {
        return Err(format!("lower() expects 1 arg, got {}", args.len()));
    }
    let s = expect_string(gen, &args[0], "lower", 0)?;
    let i64_ty = gen.type_mapper.i64_type();
    let i8_ty = gen.type_mapper.i8_type();

    let len = call_strlen(gen, s)?;
    let one = i64_ty.const_int(1, false);
    let size = b!(gen)
        .build_int_add(len, one, "lower.size")
        .map_err(|_| "add")?;
    let result = call_malloc(gen, size)?;
    let strcpy_fn = get_strcpy(gen)?;
    b!(gen)
        .build_call(strcpy_fn, &[result.into(), s.into()], "lower.copy")
        .map_err(|_| "strcpy")?;

    let cf = b!(gen)
        .get_insert_block()
        .and_then(|bb| bb.get_parent())
        .ok_or("no fn")?;
    let i_ptr = Emitter::emit_entry_alloca(&gen.builder, cf, "lower.i", i64_ty.into());
    b!(gen)
        .build_store(i_ptr, i64_ty.const_zero())
        .map_err(|_| "store")?;
    let cond = gen.context.append_basic_block(cf, "lower.cond");
    let body = gen.context.append_basic_block(cf, "lower.body");
    let exit = gen.context.append_basic_block(cf, "lower.exit");
    b!(gen)
        .build_unconditional_branch(cond)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(cond);
    let i = b!(gen)
        .build_load(i64_ty, i_ptr, "i")
        .map_err(|_| "load")?
        .into_int_value();
    let cnd = b!(gen)
        .build_int_compare(IntPredicate::SLT, i, len, "cmp")
        .map_err(|_| "cmp")?;
    b!(gen)
        .build_conditional_branch(cnd, body, exit)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(body);
    let c = load_u8(gen, result, i)?;
    let c32 = b!(gen)
        .build_int_z_extend(c, gen.type_mapper.i32_type(), "zext")
        .map_err(|_| "zext")?;
    let tl = get_tolower(gen)?;
    let lc = call_libc1(gen, tl, c32)?;
    let lc8 = b!(gen)
        .build_int_truncate(lc, i8_ty, "trunc")
        .map_err(|_| "trunc")?;
    store_u8(gen, result, i, lc8)?;
    let ni = b!(gen).build_int_add(i, one, "inc").map_err(|_| "add")?;
    b!(gen).build_store(i_ptr, ni).map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(cond)
        .map_err(|_| "branch")?;
    b!(gen).position_at_end(exit);
    Ok(result.into())
}

pub fn emit_reverse<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 1 {
        return Err(format!("reverse expects 1 arg, got {}", args.len()));
    }
    let s = expect_string(gen, &args[0], "reverse", 0)?;
    let i64_ty = gen.type_mapper.i64_type();
    let i8_ty = gen.type_mapper.i8_type();
    let one = i64_ty.const_int(1, false);

    let len = call_strlen(gen, s)?;
    let size = b!(gen)
        .build_int_add(len, one, "rev.size")
        .map_err(|_| "add")?;
    let result = call_malloc(gen, size)?;
    let cf = b!(gen)
        .get_insert_block()
        .and_then(|bb| bb.get_parent())
        .ok_or("no fn")?;
    let i_ptr = Emitter::emit_entry_alloca(&gen.builder, cf, "rev.i", i64_ty.into());
    b!(gen)
        .build_store(i_ptr, i64_ty.const_zero())
        .map_err(|_| "store")?;

    let cond = gen.context.append_basic_block(cf, "rev.cond");
    let body = gen.context.append_basic_block(cf, "rev.body");
    let exit = gen.context.append_basic_block(cf, "rev.exit");
    b!(gen)
        .build_unconditional_branch(cond)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(cond);
    let i = b!(gen)
        .build_load(i64_ty, i_ptr, "i")
        .map_err(|_| "load")?
        .into_int_value();
    let cnd = b!(gen)
        .build_int_compare(IntPredicate::SLT, i, len, "cmp")
        .map_err(|_| "cmp")?;
    b!(gen)
        .build_conditional_branch(cnd, body, exit)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(body);
    let last = b!(gen).build_int_sub(len, one, "last").map_err(|_| "sub")?;
    let src_idx = b!(gen).build_int_sub(last, i, "srci").map_err(|_| "sub")?;
    let c = load_u8(gen, s, src_idx)?;
    store_u8(gen, result, i, c)?;
    let ni = b!(gen).build_int_add(i, one, "inc").map_err(|_| "add")?;
    b!(gen).build_store(i_ptr, ni).map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(cond)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(exit);
    store_u8(gen, result, len, i8_ty.const_zero())?;
    Ok(result.into())
}
