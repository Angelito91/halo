// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::{Expression, IndexType};
use inkwell::values::BasicValueEnum;

/// Emit code for array literals: [1, 2, 3]
pub fn emit_array_literal<'ctx>(
    _gen: &mut CodeGenerator<'ctx>,
    _elements: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    Err("Array literals are not yet supported in compiled mode".to_string())
}

/// Emit code for array indexing: a[i]
pub fn emit_array_index<'ctx>(
    _gen: &mut CodeGenerator<'ctx>,
    _array: &Expression,
    _index: &IndexType,
) -> Result<BasicValueEnum<'ctx>, String> {
    Err("Array indexing is not yet supported in compiled mode".to_string())
}
