use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::expressions;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;

macro_rules! b {
    ($gen:expr) => {
        &$gen.builder.builder
    };
}

fn get_fn<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    name: &str,
    fn_type: inkwell::types::FunctionType<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    if let Some(f) = gen.module.get_function(name) {
        return Ok(f);
    }
    Ok(gen.module.add_function(name, fn_type, None))
}

fn get_strlen<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i64_ty = gen.type_mapper.i64_type();
    let ptr_ty = gen.type_mapper.ptr_type();
    get_fn(gen, "strlen", i64_ty.fn_type(&[ptr_ty.into()], false))
}

fn get_malloc<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i64_ty = gen.type_mapper.i64_type();
    let ptr_ty = gen.type_mapper.ptr_type();
    get_fn(gen, "malloc", ptr_ty.fn_type(&[i64_ty.into()], false))
}

fn get_memcpy<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i64_ty = gen.type_mapper.i64_type();
    let ptr_ty = gen.type_mapper.ptr_type();
    get_fn(
        gen,
        "memcpy",
        ptr_ty.fn_type(&[ptr_ty.into(), ptr_ty.into(), i64_ty.into()], false),
    )
}

fn get_strcpy<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let ptr_ty = gen.type_mapper.ptr_type();
    get_fn(
        gen,
        "strcpy",
        ptr_ty.fn_type(&[ptr_ty.into(), ptr_ty.into()], false),
    )
}

fn get_toupper<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i32_ty = gen.type_mapper.i32_type();
    get_fn(gen, "toupper", i32_ty.fn_type(&[i32_ty.into()], false))
}

fn get_tolower<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i32_ty = gen.type_mapper.i32_type();
    get_fn(gen, "tolower", i32_ty.fn_type(&[i32_ty.into()], false))
}

fn get_isspace<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i32_ty = gen.type_mapper.i32_type();
    get_fn(gen, "isspace", i32_ty.fn_type(&[i32_ty.into()], false))
}

fn get_strstr<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let ptr_ty = gen.type_mapper.ptr_type();
    get_fn(
        gen,
        "strstr",
        ptr_ty.fn_type(&[ptr_ty.into(), ptr_ty.into()], false),
    )
}

fn get_strcmp<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i32_ty = gen.type_mapper.i32_type();
    let ptr_ty = gen.type_mapper.ptr_type();
    get_fn(
        gen,
        "strcmp",
        i32_ty.fn_type(&[ptr_ty.into(), ptr_ty.into()], false),
    )
}

fn get_strncmp<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i32_ty = gen.type_mapper.i32_type();
    let ptr_ty = gen.type_mapper.ptr_type();
    let i64_ty = gen.type_mapper.i64_type();
    get_fn(
        gen,
        "strncmp",
        i32_ty.fn_type(&[ptr_ty.into(), ptr_ty.into(), i64_ty.into()], false),
    )
}

fn expect_string<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    expr: &Expression,
    fn_name: &str,
    arg_idx: usize,
) -> Result<inkwell::values::PointerValue<'ctx>, String> {
    let val = expressions::emit_expression(gen, expr)?;
    if !val.is_pointer_value() {
        return Err(format!(
            "{}() expects argument {} to be a string",
            fn_name,
            arg_idx + 1
        ));
    }
    Ok(val.into_pointer_value())
}

fn ptr_off<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    ptr: inkwell::values::PointerValue<'ctx>,
    offset: inkwell::values::IntValue<'ctx>,
) -> Result<inkwell::values::PointerValue<'ctx>, String> {
    let i64_ty = gen.type_mapper.i64_type();
    let ptr_ty = gen.type_mapper.ptr_type();
    let base = b!(gen)
        .build_ptr_to_int(ptr, i64_ty, "base")
        .map_err(|_| "ptr_to_int failed")?;
    let addr = b!(gen)
        .build_int_add(base, offset, "addr")
        .map_err(|_| "add failed")?;
    b!(gen)
        .build_int_to_ptr(addr, ptr_ty, "elem")
        .map_err(|_| "int_to_ptr failed".to_string())
}

fn load_u8<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    ptr: inkwell::values::PointerValue<'ctx>,
    i: inkwell::values::IntValue<'ctx>,
) -> Result<inkwell::values::IntValue<'ctx>, String> {
    let elem = ptr_off(gen, ptr, i)?;
    b!(gen)
        .build_load(gen.type_mapper.i8_type(), elem, "load_u8")
        .map(|v| v.into_int_value())
        .map_err(|_| "load_u8 failed".to_string())
}

fn store_u8<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    ptr: inkwell::values::PointerValue<'ctx>,
    i: inkwell::values::IntValue<'ctx>,
    val: inkwell::values::IntValue<'ctx>,
) -> Result<(), String> {
    let elem = ptr_off(gen, ptr, i)?;
    b!(gen)
        .build_store(elem, val)
        .map(|_| ())
        .map_err(|_| "store_u8 failed".to_string())
}

fn call_strlen<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    s: inkwell::values::PointerValue<'ctx>,
) -> Result<inkwell::values::IntValue<'ctx>, String> {
    let fn_val = get_strlen(gen)?;
    let cs = b!(gen)
        .build_call(fn_val, &[s.into()], "strlen")
        .map_err(|_| "strlen call failed")?;
    match cs.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(v) => Ok(v.into_int_value()),
        _ => Err("strlen: unexpected".to_string()),
    }
}

fn call_malloc<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    size: inkwell::values::IntValue<'ctx>,
) -> Result<inkwell::values::PointerValue<'ctx>, String> {
    let fn_val = get_malloc(gen)?;
    let cs = b!(gen)
        .build_call(fn_val, &[size.into()], "malloc")
        .map_err(|_| "malloc call failed")?;
    match cs.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(v) => Ok(v.into_pointer_value()),
        _ => Err("malloc: unexpected".to_string()),
    }
}

fn call_libc1<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    fn_val: inkwell::values::FunctionValue<'ctx>,
    arg: inkwell::values::IntValue<'ctx>,
) -> Result<inkwell::values::IntValue<'ctx>, String> {
    let cs = b!(gen)
        .build_call(fn_val, &[arg.into()], "libc1")
        .map_err(|_| "libc1 call failed")?;
    match cs.try_as_basic_value() {
        inkwell::values::ValueKind::Basic(v) => Ok(v.into_int_value()),
        _ => Err("libc1: unexpected".to_string()),
    }
}

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
    b!(gen).build_store(start_ptr, next).map_err(|_| "store")?;
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
    b!(gen).build_store(end_ptr, eidx).map_err(|_| "store")?;
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

pub fn emit_starts_with<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 2 {
        return Err(format!("starts_with expects 2 args, got {}", args.len()));
    }
    let s = expect_string(gen, &args[0], "starts_with", 0)?;
    let prefix = expect_string(gen, &args[1], "starts_with", 1)?;
    let i64_ty = gen.type_mapper.i64_type();
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
    Ok(b!(gen)
        .build_int_z_extend(eq, i64_ty, "sw.ext")
        .map_err(|_| "zext")?
        .into())
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
    let i64_ty = gen.type_mapper.i64_type();
    let i32_ty = gen.type_mapper.i32_type();
    let zero = i64_ty.const_zero();

    let s_len = call_strlen(gen, s)?;
    let suffix_len = call_strlen(gen, suffix)?;
    let cf = b!(gen)
        .get_insert_block()
        .and_then(|bb| bb.get_parent())
        .ok_or("no fn")?;
    let rp = Emitter::emit_entry_alloca(&gen.builder, cf, "ew.r", i64_ty.into());

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
    b!(gen)
        .build_store(
            rp,
            b!(gen)
                .build_int_z_extend(eq, i64_ty, "ew.ext")
                .map_err(|_| "zext")?,
        )
        .map_err(|_| "store")?;
    b!(gen)
        .build_unconditional_branch(eb)
        .map_err(|_| "branch")?;

    b!(gen).position_at_end(eb);
    let rv = b!(gen)
        .build_load(i64_ty, rp, "ew.rv")
        .map_err(|_| "load")?;
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
    Ok(b!(gen)
        .build_int_z_extend(found, i64_ty, "ext")
        .map_err(|_| "zext")?
        .into())
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
    let ba = b!(gen).build_int_add(bad, bl, "ss.ba").map_err(|_| "add")?;
    let bad_bool = b!(gen)
        .build_int_compare(IntPredicate::NE, ba, zero, "ss.bb")
        .map_err(|_| "cmp")?;

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
