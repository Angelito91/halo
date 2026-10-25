// The Halo Programming Language — string builtins: trim.
use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

use super::helpers::*;

pub fn emit_trim<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 1 {
        return Err(format!("trim() expects 1 arg, got {}", args.len()));
    }
    let s = expect_string(gen, &args[0], "trim", 0)?;
    let i64_ty = gen.type_mapper.i64_type();
    let i8_ty = gen.type_mapper.i8_type();
    let i32_ty = gen.type_mapper.i32_type();
    let zero = i64_ty.const_zero();
    let one = i64_ty.const_int(1, false);

    let len = call_strlen(gen, s)?;
    let cf = b!(gen)
        .get_insert_block()
        .and_then(|bb| bb.get_parent())
        .ok_or("no fn")?;
    let start_ptr = Emitter::emit_entry_alloca(&gen.builder, cf, "trim.start", i64_ty.into());
    b!(gen).build_store(start_ptr, zero).map_err(|_| "store")?;
    let sc = gen.context.append_basic_block(cf, "trim.sc");
    let sb = gen.context.append_basic_block(cf, "trim.sb");
    let sx = gen.context.append_basic_block(cf, "trim.sx");
    b!(gen)
        .build_unconditional_branch(sc)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(sc);
    let st = b!(gen)
        .build_load(i64_ty, start_ptr, "st")
        .map_err(|_| "load")?
        .into_int_value();
    let lt = b!(gen)
        .build_int_compare(IntPredicate::SLT, st, len, "trim.lt")
        .map_err(|_| "cmp")?;
    let issp = get_isspace(gen)?;
    b!(gen)
        .build_conditional_branch(lt, sb, sx)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(sb);
    let bc = load_u8(gen, s, st)?;
    let bc32 = b!(gen)
        .build_int_z_extend(bc, i32_ty, "zext")
        .map_err(|_| "zext")?;
    let sp = call_libc1(gen, issp, bc32)?;
    let is_sp = b!(gen)
        .build_int_compare(IntPredicate::NE, sp, i32_ty.const_zero(), "issp")
        .map_err(|_| "cmp")?;
    let next = b!(gen).build_int_add(st, one, "inc").map_err(|_| "add")?;
    // Only record the advanced index when the byte was whitespace: on the
    // non-space exit edge the start must stay at `st`, otherwise every
    // trimmed string loses one byte before the first kept character.
    let next_or_st = b!(gen)
        .build_select(is_sp, next, st, "inc.or.same")
        .map_err(|_| "select")?;
    b!(gen)
        .build_store(start_ptr, next_or_st)
        .map_err(|_| "store")?;
    b!(gen)
        .build_conditional_branch(is_sp, sc, sx)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(sx);
    let start_final = b!(gen)
        .build_load(i64_ty, start_ptr, "sf")
        .map_err(|_| "load")?
        .into_int_value();

    let end_ptr = Emitter::emit_entry_alloca(&gen.builder, cf, "trim.end", i64_ty.into());
    b!(gen).build_store(end_ptr, len).map_err(|_| "store")?;
    let ec = gen.context.append_basic_block(cf, "trim.ec");
    let eb = gen.context.append_basic_block(cf, "trim.eb");
    let ex = gen.context.append_basic_block(cf, "trim.ex");
    b!(gen)
        .build_unconditional_branch(ec)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(ec);
    let ed = b!(gen)
        .build_load(i64_ty, end_ptr, "ed")
        .map_err(|_| "load")?
        .into_int_value();
    let gt = b!(gen)
        .build_int_compare(IntPredicate::SGT, ed, start_final, "trim.gt")
        .map_err(|_| "cmp")?;
    b!(gen)
        .build_conditional_branch(gt, eb, ex)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(eb);
    let eidx = b!(gen).build_int_sub(ed, one, "sub").map_err(|_| "sub")?;
    let ec_byte = load_u8(gen, s, eidx)?;
    let ec32 = b!(gen)
        .build_int_z_extend(ec_byte, i32_ty, "zext")
        .map_err(|_| "zext")?;
    let esp = call_libc1(gen, issp, ec32)?;
    let e_is_sp = b!(gen)
        .build_int_compare(IntPredicate::NE, esp, i32_ty.const_zero(), "eissp")
        .map_err(|_| "cmp")?;
    // Symmetric to the leading-whitespace scan: the index must not shrink
    // on the exit edge, or every trimmed string loses its last byte.
    let eidx_or_ed = b!(gen)
        .build_select(e_is_sp, eidx, ed, "eidx.or.same")
        .map_err(|_| "select")?;
    b!(gen)
        .build_store(end_ptr, eidx_or_ed)
        .map_err(|_| "store")?;
    b!(gen)
        .build_conditional_branch(e_is_sp, ec, ex)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(ex);
    let end_final = b!(gen)
        .build_load(i64_ty, end_ptr, "ef")
        .map_err(|_| "load")?
        .into_int_value();
    let new_len = b!(gen)
        .build_int_sub(end_final, start_final, "nlen")
        .map_err(|_| "sub")?;
    let alloc_size = b!(gen)
        .build_int_add(new_len, one, "alloc")
        .map_err(|_| "add")?;
    let result = call_malloc(gen, alloc_size)?;
    let memcpy_fn = get_memcpy(gen)?;
    let src = ptr_off(gen, s, start_final)?;
    b!(gen)
        .build_call(
            memcpy_fn,
            &[result.into(), src.into(), new_len.into()],
            "trim.memcpy",
        )
        .map_err(|_| "memcpy")?;
    let null_pos = ptr_off(gen, result, new_len)?;
    b!(gen)
        .build_store(null_pos, i8_ty.const_zero())
        .map_err(|_| "null")?;
    Ok(result.into())
}
