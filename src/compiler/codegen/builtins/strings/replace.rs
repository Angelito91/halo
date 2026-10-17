// The Halo Programming Language — string builtins: replace.
use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

use super::helpers::*;

pub fn emit_replace<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 3 {
        return Err(format!("replace expects 3 args, got {}", args.len()));
    }
    let s = expect_string(gen, &args[0], "replace", 0)?;
    let old = expect_string(gen, &args[1], "replace", 1)?;
    let new_str = expect_string(gen, &args[2], "replace", 2)?;
    let i64_ty = gen.type_mapper.i64_type();
    let i8_ty = gen.type_mapper.i8_type();
    let ptr_ty = gen.type_mapper.ptr_type();
    let zero = i64_ty.const_zero();
    let one = i64_ty.const_int(1, false);

    let s_len = call_strlen(gen, s)?;
    let old_len = call_strlen(gen, old)?;
    let new_len_val = call_strlen(gen, new_str)?;

    let cf = b!(gen)
        .get_insert_block()
        .and_then(|bb| bb.get_parent())
        .ok_or("no fn")?;
    let rs = Emitter::emit_entry_alloca(&gen.builder, cf, "rep.rs", ptr_ty.into());
    let strstr_fn = get_strstr(gen)?;
    let memcpy_fn = get_memcpy(gen)?;
    let strcpy_fn = get_strcpy(gen)?;

    let old_empty = b!(gen)
        .build_int_compare(IntPredicate::EQ, old_len, zero, "oe")
        .map_err(|_| "cmp")?;
    let nb = gen.context.append_basic_block(cf, "rep.nb");
    let cb = gen.context.append_basic_block(cf, "rep.cb");
    let ac = gen.context.append_basic_block(cf, "rep.ac");
    b!(gen)
        .build_conditional_branch(old_empty, nb, cb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(nb);
    let dup = call_malloc(
        gen,
        b!(gen)
            .build_int_add(s_len, one, "dup")
            .map_err(|_| "add")?,
    )?;
    b!(gen)
        .build_call(strcpy_fn, &[dup.into(), s.into()], "rep.strcpy")
        .map_err(|_| "strcpy")?;
    b!(gen).build_store(rs, dup).map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(ac)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(cb);
    let cnt_ptr = Emitter::emit_entry_alloca(&gen.builder, cf, "rep.cnt", i64_ty.into());
    b!(gen).build_store(cnt_ptr, zero).map_err(|_| "store")?;
    let pp = Emitter::emit_entry_alloca(&gen.builder, cf, "rep.pp", ptr_ty.into());
    b!(gen).build_store(pp, s).map_err(|_| "store")?;
    let cc = gen.context.append_basic_block(cf, "rep.cc");
    let cbody = gen.context.append_basic_block(cf, "rep.cbody");
    let cexit = gen.context.append_basic_block(cf, "rep.cexit");
    b!(gen)
        .build_unconditional_branch(cc)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(cc);
    let cs = b!(gen)
        .build_load(ptr_ty, pp, "cs")
        .map_err(|_| "load")?
        .into_pointer_value();
    let fcs = b!(gen)
        .build_call(strstr_fn, &[cs.into(), old.into()], "rep.strstr_cnt")
        .map_err(|_| "strstr")?;
    let fp = match fcs.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
        _ => return Err("strstr: unexpected".to_string()),
    };
    let fp_int = b!(gen)
        .build_ptr_to_int(fp, i64_ty, "fpi")
        .map_err(|_| "ptr2int")?;
    let isnull = b!(gen)
        .build_int_compare(IntPredicate::EQ, fp_int, zero, "isnull")
        .map_err(|_| "cmp")?;
    b!(gen)
        .build_conditional_branch(isnull, cexit, cbody)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(cbody);
    let cv = b!(gen)
        .build_load(i64_ty, cnt_ptr, "cv")
        .map_err(|_| "load")?
        .into_int_value();
    let cv1 = b!(gen).build_int_add(cv, one, "inc").map_err(|_| "add")?;
    b!(gen).build_store(cnt_ptr, cv1).map_err(|_| "store")?;
    let np2 = ptr_off(gen, fp, old_len)?;
    b!(gen).build_store(pp, np2).map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(cc)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(cexit);
    let count = b!(gen)
        .build_load(i64_ty, cnt_ptr, "cntf")
        .map_err(|_| "load")?
        .into_int_value();
    let sd = b!(gen)
        .build_int_sub(new_len_val, old_len, "sd")
        .map_err(|_| "sub")?;
    let td = b!(gen).build_int_mul(count, sd, "td").map_err(|_| "mul")?;
    let ns = b!(gen).build_int_add(s_len, td, "ns").map_err(|_| "add")?;
    let result = call_malloc(
        gen,
        b!(gen)
            .build_int_add(ns, one, "ralloc")
            .map_err(|_| "add")?,
    )?;
    b!(gen).build_store(rs, result).map_err(|_| "store")?;

    let sp2 = Emitter::emit_entry_alloca(&gen.builder, cf, "rep.sp2", ptr_ty.into());
    b!(gen).build_store(sp2, s).map_err(|_| "store")?;
    let dp2 = Emitter::emit_entry_alloca(&gen.builder, cf, "rep.dp2", ptr_ty.into());
    b!(gen).build_store(dp2, result).map_err(|_| "store")?;

    let bc = gen.context.append_basic_block(cf, "rep.bc");
    let bf = gen.context.append_basic_block(cf, "rep.bf");
    let bcopy = gen.context.append_basic_block(cf, "rep.bcopy");
    let bdone = gen.context.append_basic_block(cf, "rep.bdone");
    b!(gen)
        .build_unconditional_branch(bc)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(bc);
    let cs2 = b!(gen)
        .build_load(ptr_ty, sp2, "cs2")
        .map_err(|_| "load")?
        .into_pointer_value();
    let fcs2 = b!(gen)
        .build_call(strstr_fn, &[cs2.into(), old.into()], "rep.strstr_bld")
        .map_err(|_| "strstr")?;
    let fp2 = match fcs2.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
        _ => return Err("strstr: unexpected".to_string()),
    };
    let fp2_int = b!(gen)
        .build_ptr_to_int(fp2, i64_ty, "fp2i")
        .map_err(|_| "ptr2int")?;
    let isnull2 = b!(gen)
        .build_int_compare(IntPredicate::EQ, fp2_int, zero, "isnull2")
        .map_err(|_| "cmp")?;
    b!(gen)
        .build_conditional_branch(isnull2, bcopy, bf)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(bf);
    let cd = b!(gen)
        .build_load(ptr_ty, dp2, "cd")
        .map_err(|_| "load")?
        .into_pointer_value();
    let blen = {
        let fint = b!(gen)
            .build_ptr_to_int(fp2, i64_ty, "fi")
            .map_err(|_| "ptr2int")?;
        let cs_int = b!(gen)
            .build_ptr_to_int(cs2, i64_ty, "csi")
            .map_err(|_| "ptr2int")?;
        b!(gen)
            .build_int_sub(fint, cs_int, "blen")
            .map_err(|_| "sub")?
    };
    b!(gen)
        .build_call(
            memcpy_fn,
            &[cd.into(), cs2.into(), blen.into()],
            "rep.cpy_before",
        )
        .map_err(|_| "memcpy")?;
    let dab = ptr_off(gen, cd, blen)?;
    b!(gen)
        .build_call(
            memcpy_fn,
            &[dab.into(), new_str.into(), new_len_val.into()],
            "rep.cpy_new",
        )
        .map_err(|_| "memcpy")?;
    let dan = ptr_off(gen, dab, new_len_val)?;
    b!(gen).build_store(dp2, dan).map_err(|_| "store")?;
    let sao = ptr_off(gen, fp2, old_len)?;
    b!(gen).build_store(sp2, sao).map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(bc)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(bcopy);
    let cdf = b!(gen)
        .build_load(ptr_ty, dp2, "cdf")
        .map_err(|_| "load")?
        .into_pointer_value();
    let csf = b!(gen)
        .build_load(ptr_ty, sp2, "csf")
        .map_err(|_| "load")?
        .into_pointer_value();
    let rl = {
        let se = b!(gen)
            .build_ptr_to_int(s, i64_ty, "se")
            .map_err(|_| "ptr2int")?;
        let se2 = b!(gen).build_int_add(se, s_len, "se2").map_err(|_| "add")?;
        let csf_i = b!(gen)
            .build_ptr_to_int(csf, i64_ty, "csfi")
            .map_err(|_| "ptr2int")?;
        b!(gen).build_int_sub(se2, csf_i, "rl").map_err(|_| "sub")?
    };
    b!(gen)
        .build_call(
            memcpy_fn,
            &[cdf.into(), csf.into(), rl.into()],
            "rep.cpy_rem",
        )
        .map_err(|_| "memcpy")?;
    let fn2 = ptr_off(gen, cdf, rl)?;
    b!(gen)
        .build_store(fn2, i8_ty.const_zero())
        .map_err(|_| "null")?;
    b!(gen)
        .build_unconditional_branch(bdone)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(bdone);
    let fr = b!(gen)
        .build_load(ptr_ty, rs, "rep.fr")
        .map_err(|_| "load")?;
    Ok(fr)
}
