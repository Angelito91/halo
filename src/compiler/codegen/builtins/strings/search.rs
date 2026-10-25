// The Halo Programming Language — string builtins: starts_with/ends_with/contains/index_of.
use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

use super::helpers::*;

pub fn emit_starts_with<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 2 {
        return Err(format!("starts_with expects 2 args, got {}", args.len()));
    }
    let s = expect_string(gen, &args[0], "starts_with", 0)?;
    let prefix = expect_string(gen, &args[1], "starts_with", 1)?;
    let i32_ty = gen.type_mapper.i32_type();

    let prefix_len = call_strlen(gen, prefix)?;
    let strncmp_fn = get_strncmp(gen)?;
    let cs = b!(gen)
        .build_call(
            strncmp_fn,
            &[s.into(), prefix.into(), prefix_len.into()],
            "sw",
        )
        .map_err(|_| "strncmp")?;
    let result = match cs.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(v) => v.into_int_value(),
        _ => return Err("strncmp: unexpected".to_string()),
    };
    let eq = b!(gen)
        .build_int_compare(IntPredicate::EQ, result, i32_ty.const_zero(), "sw.eq")
        .map_err(|_| "cmp")?;
    Ok(eq.into())
}

pub fn emit_ends_with<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 2 {
        return Err(format!("ends_with expects 2 args, got {}", args.len()));
    }
    let s = expect_string(gen, &args[0], "ends_with", 0)?;
    let suffix = expect_string(gen, &args[1], "ends_with", 1)?;
    let i1_ty = gen.type_mapper.i1_type();
    let i32_ty = gen.type_mapper.i32_type();
    let zero = i1_ty.const_zero();

    let s_len = call_strlen(gen, s)?;
    let suffix_len = call_strlen(gen, suffix)?;
    let cf = b!(gen)
        .get_insert_block()
        .and_then(|bb| bb.get_parent())
        .ok_or("no fn")?;
    let rp = Emitter::emit_entry_alloca(&gen.builder, cf, "ew.r", i1_ty.into());

    let toolong = b!(gen)
        .build_int_compare(IntPredicate::SGT, suffix_len, s_len, "ew.tl")
        .map_err(|_| "cmp")?;
    let tlb = gen.context.append_basic_block(cf, "ew.tlb");
    let chb = gen.context.append_basic_block(cf, "ew.chb");
    let eb = gen.context.append_basic_block(cf, "ew.eb");
    b!(gen)
        .build_conditional_branch(toolong, tlb, chb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(tlb);
    b!(gen).build_store(rp, zero).map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(eb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(chb);
    let offset = b!(gen)
        .build_int_sub(s_len, suffix_len, "ew.off")
        .map_err(|_| "sub")?;
    let cmp_ptr = ptr_off(gen, s, offset)?;
    let strcmp_fn = get_strcmp(gen)?;
    let cs = b!(gen)
        .build_call(strcmp_fn, &[cmp_ptr.into(), suffix.into()], "ew.strcmp")
        .map_err(|_| "strcmp")?;
    let cr = match cs.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(v) => v.into_int_value(),
        _ => return Err("strcmp: unexpected".to_string()),
    };
    let eq = b!(gen)
        .build_int_compare(IntPredicate::EQ, cr, i32_ty.const_zero(), "ew.eq")
        .map_err(|_| "cmp")?;
    b!(gen).build_store(rp, eq).map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(eb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(eb);
    let rv = b!(gen).build_load(i1_ty, rp, "ew.rv").map_err(|_| "load")?;
    Ok(rv)
}

pub fn emit_contains<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 2 {
        return Err(format!("contains expects 2 args, got {}", args.len()));
    }
    let s = expect_string(gen, &args[0], "contains", 0)?;
    let needle = expect_string(gen, &args[1], "contains", 1)?;
    let i64_ty = gen.type_mapper.i64_type();

    let strstr_fn = get_strstr(gen)?;
    let cs = b!(gen)
        .build_call(strstr_fn, &[s.into(), needle.into()], "contains.strstr")
        .map_err(|_| "strstr")?;
    let rp = match cs.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
        _ => return Err("strstr: unexpected".to_string()),
    };
    let rp_int = b!(gen)
        .build_ptr_to_int(rp, i64_ty, "rp_i")
        .map_err(|_| "ptr2int")?;
    let found = b!(gen)
        .build_int_compare(IntPredicate::NE, rp_int, i64_ty.const_zero(), "found")
        .map_err(|_| "cmp")?;
    Ok(found.into())
}

pub fn emit_index_of<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 2 {
        return Err(format!("index_of expects 2 args, got {}", args.len()));
    }
    let s = expect_string(gen, &args[0], "index_of", 0)?;
    let needle = expect_string(gen, &args[1], "index_of", 1)?;
    let i64_ty = gen.type_mapper.i64_type();

    let strstr_fn = get_strstr(gen)?;
    let cs = b!(gen)
        .build_call(strstr_fn, &[s.into(), needle.into()], "io.strstr")
        .map_err(|_| "strstr")?;
    let rp = match cs.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
        _ => return Err("strstr: unexpected".to_string()),
    };
    let rp_int = b!(gen)
        .build_ptr_to_int(rp, i64_ty, "rp_i")
        .map_err(|_| "ptr2int")?;
    let is_null = b!(gen)
        .build_int_compare(IntPredicate::EQ, rp_int, i64_ty.const_zero(), "isnull")
        .map_err(|_| "cmp")?;

    let cf = b!(gen)
        .get_insert_block()
        .and_then(|bb| bb.get_parent())
        .ok_or("no fn")?;
    let rs = Emitter::emit_entry_alloca(&gen.builder, cf, "io.rs", i64_ty.into());
    let fb = gen.context.append_basic_block(cf, "io.fb");
    let nfb = gen.context.append_basic_block(cf, "io.nfb");
    let eb = gen.context.append_basic_block(cf, "io.eb");
    b!(gen)
        .build_conditional_branch(is_null, nfb, fb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(nfb);
    b!(gen)
        .build_store(rs, i64_ty.const_int(u64::MAX, false))
        .map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(eb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(fb);
    let s_int = b!(gen)
        .build_ptr_to_int(s, i64_ty, "si")
        .map_err(|_| "ptr2int")?;
    let idx = b!(gen)
        .build_int_sub(rp_int, s_int, "idx")
        .map_err(|_| "sub")?;
    b!(gen).build_store(rs, idx).map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(eb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(eb);
    let rv = b!(gen)
        .build_load(i64_ty, rs, "io.rv")
        .map_err(|_| "load")?;
    Ok(rv)
}
