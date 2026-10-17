// The Halo Programming Language — string builtins: shared LLVM/libc helpers.
use crate::compiler::codegen::expressions;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;

macro_rules! b {
    ($gen:expr) => {
        &$gen.builder.builder
    };
}
pub(crate) use b;

pub(crate) fn get_fn<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    name: &str,
    fn_type: inkwell::types::FunctionType<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    if let Some(f) = gen.module.get_function(name) {
        return Ok(f);
    }
    Ok(gen.module.add_function(name, fn_type, None))
}

pub(crate) fn get_strlen<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i64_ty = gen.type_mapper.i64_type();
    let ptr_ty = gen.type_mapper.ptr_type();
    get_fn(gen, "strlen", i64_ty.fn_type(&[ptr_ty.into()], false))
}

pub(crate) fn get_malloc<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i64_ty = gen.type_mapper.i64_type();
    let ptr_ty = gen.type_mapper.ptr_type();
    get_fn(gen, "malloc", ptr_ty.fn_type(&[i64_ty.into()], false))
}

pub(crate) fn get_memcpy<'ctx>(
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

pub(crate) fn get_strcpy<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let ptr_ty = gen.type_mapper.ptr_type();
    get_fn(
        gen,
        "strcpy",
        ptr_ty.fn_type(&[ptr_ty.into(), ptr_ty.into()], false),
    )
}

pub(crate) fn get_toupper<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i32_ty = gen.type_mapper.i32_type();
    get_fn(gen, "toupper", i32_ty.fn_type(&[i32_ty.into()], false))
}

pub(crate) fn get_tolower<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i32_ty = gen.type_mapper.i32_type();
    get_fn(gen, "tolower", i32_ty.fn_type(&[i32_ty.into()], false))
}

pub(crate) fn get_isspace<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let i32_ty = gen.type_mapper.i32_type();
    get_fn(gen, "isspace", i32_ty.fn_type(&[i32_ty.into()], false))
}

pub(crate) fn get_strstr<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
) -> Result<inkwell::values::FunctionValue<'ctx>, String> {
    let ptr_ty = gen.type_mapper.ptr_type();
    get_fn(
        gen,
        "strstr",
        ptr_ty.fn_type(&[ptr_ty.into(), ptr_ty.into()], false),
    )
}

pub(crate) fn get_strcmp<'ctx>(
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

pub(crate) fn get_strncmp<'ctx>(
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

pub(crate) fn expect_string<'ctx>(
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

pub(crate) fn ptr_off<'ctx>(
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

pub(crate) fn load_u8<'ctx>(
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

pub(crate) fn store_u8<'ctx>(
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

pub(crate) fn call_strlen<'ctx>(
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

pub(crate) fn call_malloc<'ctx>(
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

pub(crate) fn call_libc1<'ctx>(
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
