// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::compiler::types::TypeMapper;
use inkwell::values::BasicValueEnum;

/// Emit a number literal (i64)
pub fn emit_number<'ctx>(type_mapper: &TypeMapper<'ctx>, value: i64) -> BasicValueEnum<'ctx> {
    type_mapper.i64_type().const_int(value as u64, true).into()
}

/// Emit a float literal (f64)
pub fn emit_float<'ctx>(type_mapper: &TypeMapper<'ctx>, value: f64) -> BasicValueEnum<'ctx> {
    type_mapper.f64_type().const_float(value).into()
}

/// Emit a bool literal (i1)
pub fn emit_bool<'ctx>(type_mapper: &TypeMapper<'ctx>, value: bool) -> BasicValueEnum<'ctx> {
    type_mapper
        .i1_type()
        .const_int(if value { 1 } else { 0 }, false)
        .into()
}

/// Emit a string literal with proper global string constant creation
pub fn emit_string<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    value: &str,
) -> Result<BasicValueEnum<'ctx>, String> {
    let gv = Emitter::get_or_create_string(gen, value, &format!("str.lit.{}", value.len()));
    Ok(gv.as_pointer_value().into())
}
