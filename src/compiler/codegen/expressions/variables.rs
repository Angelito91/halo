// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;

use crate::compiler::builder::IRBuilder;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::{BasicValueEnum, GlobalValue};

/// Emit a variable reference by looking it up in local slots or module globals
pub fn emit_var_ref<'ctx>(
    builder: &IRBuilder<'ctx>,
    globals: &HashMap<String, GlobalValue<'ctx>>,
    name: &str,
) -> Result<BasicValueEnum<'ctx>, String> {
    // Try local alloca first
    if let Some(slot) = builder.get_slot(name) {
        return builder
            .build_load(slot.ty, slot.ptr, name)
            .map_err(|_| format!("Failed to load '{}'", name));
    }

    // Then try module globals
    if let Some(&gv) = globals.get(name) {
        let ty: inkwell::types::BasicTypeEnum = gv
            .get_value_type()
            .try_into()
            .map_err(|_| format!("Global '{}' has a non-basic type", name))?;
        return builder
            .build_load(ty, gv.as_pointer_value(), name)
            .map_err(|_| format!("Failed to load global '{}'", name));
    }

    Err(format!("Undefined variable: '{}'", name))
}

/// Emit a variable assignment, evaluating the expression and storing to local slot or global
pub fn emit_var_assign<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    name: &str,
    value_expr: &Expression,
) -> Result<BasicValueEnum<'ctx>, String> {
    let val = super::emit_expression(gen, value_expr)?;
    let is_bool = val.is_int_value() && val.into_int_value().get_type().get_bit_width() == 1;

    if let Some(slot) = gen.builder.get_slot(name) {
        // Coerce the new value to the slot's existing storage type
        let to_store = gen.coerce_to_storage(val, slot.ty)?;
        gen.builder
            .build_store(slot.ptr, to_store)
            .map_err(|_| format!("Failed to store into '{}'", name))?;
        if is_bool {
            gen.builder.mark_bool(name);
        } else {
            gen.builder.unmark_bool(name);
        }
        return Ok(to_store);
    }

    // Then try module globals
    if let Some(&gv) = gen.globals.get(name) {
        let slot_ty = gv
            .get_value_type()
            .try_into()
            .map_err(|_| format!("Global '{}' has a non-basic type", name))?;
        let to_store = gen.coerce_to_storage(val, slot_ty)?;
        gen.builder
            .build_store(gv.as_pointer_value(), to_store)
            .map_err(|_| format!("Failed to store into global '{}'", name))?;
        if is_bool {
            gen.bool_globals.insert(name.to_string());
        } else {
            gen.bool_globals.remove(name);
        }
        return Ok(to_store);
    }

    Err(format!("Assignment to undeclared variable: '{}'", name))
}
