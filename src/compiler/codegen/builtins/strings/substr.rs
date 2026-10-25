// The Halo Programming Language — string builtins: substr.
use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::expressions;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

use super::helpers::*;

pub fn emit_substr<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 3 {
        return Err(format!("substr expects 3 args, got {}", args.len()));
    }
    let s = expect_string(gen, &args[0], "substr", 0)?;
    let sv = expressions::emit_expression(gen, &args[1])?;
    let lv = expressions::emit_expression(gen, &args[2])?;
    if sv.is_pointer_value() || sv.is_float_value() {
        return Err("substr: arg2 must be int".to_string());
    }
    if lv.is_pointer_value() || lv.is_float_value() {
        return Err("substr: arg3 must be int".to_string());
    }
    let i64_ty = gen.type_mapper.i64_type();
    let i8_ty = gen.type_mapper.i8_type();
    let ptr_ty = gen.type_mapper.ptr_type();
    let zero = i64_ty.const_zero();
    let one = i64_ty.const_int(1, false);

    let start = sv.into_int_value();
    let length = lv.into_int_value();
    let s_len = call_strlen(gen, s)?;
    let cf = b!(gen)
        .get_insert_block()
        .and_then(|bb| bb.get_parent())
        .ok_or("no fn")?;
    let rs = Emitter::emit_entry_alloca(&gen.builder, cf, "ss.rs", ptr_ty.into());
    let empty_str = Emitter::get_or_create_string(gen, "", "ss.empty");

    let bad = b!(gen)
        .build_int_compare(IntPredicate::SLT, start, zero, "ss.sn")
        .map_err(|_| "cmp")?;
    let bl = b!(gen)
        .build_int_compare(IntPredicate::SLT, length, zero, "ss.ln")
        .map_err(|_| "cmp")?;
    let bad_bool = b!(gen).build_or(bad, bl, "ss.bb").map_err(|_| "or")?;

    let errb = gen.context.append_basic_block(cf, "ss.err");
    let okb = gen.context.append_basic_block(cf, "ss.ok");
    let endb = gen.context.append_basic_block(cf, "ss.end");
    b!(gen)
        .build_conditional_branch(bad_bool, errb, okb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(errb);
    b!(gen)
        .build_store(rs, empty_str.as_pointer_value())
        .map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(endb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(okb);
    let past = b!(gen)
        .build_int_compare(IntPredicate::SGE, start, s_len, "ss.past")
        .map_err(|_| "cmp")?;
    let pastb = gen.context.append_basic_block(cf, "ss.pastb");
    let copyb = gen.context.append_basic_block(cf, "ss.copyb");
    b!(gen)
        .build_conditional_branch(past, pastb, copyb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(pastb);
    b!(gen)
        .build_store(rs, empty_str.as_pointer_value())
        .map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(endb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(copyb);
    let rem = b!(gen)
        .build_int_sub(s_len, start, "rem")
        .map_err(|_| "sub")?;
    let ltr = b!(gen)
        .build_int_compare(IntPredicate::SLT, length, rem, "ltr")
        .map_err(|_| "cmp")?;
    let act = b!(gen)
        .build_select(ltr, length, rem, "act")
        .map_err(|_| "select")?
        .into_int_value();
    let alloc_sz = b!(gen).build_int_add(act, one, "asz").map_err(|_| "add")?;
    let alloc_r = call_malloc(gen, alloc_sz)?;
    let memcpy_fn = get_memcpy(gen)?;
    let src = ptr_off(gen, s, start)?;
    b!(gen)
        .build_call(
            memcpy_fn,
            &[alloc_r.into(), src.into(), act.into()],
            "ss.memcpy",
        )
        .map_err(|_| "memcpy")?;
    let np = ptr_off(gen, alloc_r, act)?;
    b!(gen)
        .build_store(np, i8_ty.const_zero())
        .map_err(|_| "null")?;
    b!(gen).build_store(rs, alloc_r).map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(endb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(endb);
    let rv = b!(gen)
        .build_load(ptr_ty, rs, "ss.rv")
        .map_err(|_| "load")?;
    Ok(rv)
}
