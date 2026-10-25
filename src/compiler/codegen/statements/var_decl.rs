// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::expressions;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::FunctionValue;

/// Emit code for variable declarations
pub fn emit_var_decl<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    name: &str,
    init: &Option<Expression>,
    function: FunctionValue<'ctx>,
) -> Result<bool, String> {
    let init_val = match init {
        Some(expr) => expressions::emit_expression(gen, expr)?,
        None => gen.type_mapper.i64_type().const_int(0, false).into(),
    };

    let storage_ty = gen.type_mapper.storage_type_of(init_val);
    let is_bool =
        init_val.is_int_value() && init_val.into_int_value().get_type().get_bit_width() == 1;

    // Reuse the existing alloca slot if this variable was already
    // declared in the current function scope.  Creating a second
    // alloca for the same name would split loads and stores across
    // two different memory locations, producing an infinite loop
    // (loads always read the initial value while stores write to
    // the second, unreachable alloca).
    let alloca = if let Some(slot) = gen.builder.get_slot(name) {
        slot.ptr
    } else {
        let a = Emitter::emit_entry_alloca(&gen.builder, function, name, storage_ty);
        gen.builder.set_alloca(name.to_string(), a, storage_ty);
        a
    };

    if is_bool {
        gen.builder.mark_bool(name);
    } else {
        gen.builder.unmark_bool(name);
    }

    let to_store = gen.coerce_to_storage(init_val, storage_ty)?;

    gen.builder
        .build_store(alloca, to_store)
        .map_err(|_| format!("Failed to store into '{}'", name))?;

    Ok(false)
}
