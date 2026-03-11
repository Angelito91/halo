// The Halo Programming Language
// IR Builder - wraps inkwell's Builder for LLVM IR generation
// Version: 0.2.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use std::collections::HashMap;

/// A variable slot: the alloca pointer plus the LLVM type it holds.
/// We store the type explicitly because LLVM 21 uses opaque pointers —
/// `PointerType` no longer carries element-type information.
#[derive(Clone, Copy)]
pub struct VarSlot<'ctx> {
    pub ptr: PointerValue<'ctx>,
    pub ty: BasicTypeEnum<'ctx>,
}

pub struct IRBuilder<'ctx> {
    pub builder: Builder<'ctx>,
    /// Maps variable names to their alloca'd stack slots.
    allocas: HashMap<String, VarSlot<'ctx>>,
    current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> IRBuilder<'ctx> {
    /// Create a new IRBuilder bound to the given context.
    pub fn new(context: &'ctx Context, _module: &'ctx Module<'ctx>) -> Self {
        IRBuilder {
            builder: context.create_builder(),
            allocas: HashMap::new(),
            current_function: None,
        }
    }

    /// Reset scope when entering a new function.
    pub fn set_function(&mut self, function: FunctionValue<'ctx>) {
        self.current_function = Some(function);
        self.allocas.clear();
    }

    /// Return the function currently being compiled, if any.
    pub fn current_function(&self) -> Option<FunctionValue<'ctx>> {
        self.current_function
    }

    /// Register an alloca slot for a named variable.
    /// `ty` is the type of the value stored in the slot (the pointee type).
    pub fn set_alloca(&mut self, name: String, ptr: PointerValue<'ctx>, ty: BasicTypeEnum<'ctx>) {
        self.allocas.insert(name, VarSlot { ptr, ty });
    }

    /// Look up the alloca slot for a variable.
    pub fn get_slot(&self, name: &str) -> Option<VarSlot<'ctx>> {
        self.allocas.get(name).copied()
    }

    /// Emit a `store` of `value` into the alloca for `name`.
    /// Returns an error if the variable has not been declared.
    pub fn store_variable(
        &mut self,
        name: &str,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let slot = self
            .allocas
            .get(name)
            .copied()
            .ok_or_else(|| format!("Variable '{}' has no alloca slot", name))?;
        self.builder
            .build_store(slot.ptr, value)
            .map_err(|_| format!("Failed to store into '{}'", name))?;
        Ok(())
    }

    /// Emit a `load` from the alloca for `name`.
    /// Returns an error if the variable has not been declared.
    pub fn load_variable(&mut self, name: &str) -> Result<BasicValueEnum<'ctx>, String> {
        let slot = self
            .allocas
            .get(name)
            .copied()
            .ok_or_else(|| format!("Undefined variable: '{}'", name))?;
        self.builder
            .build_load(slot.ty, slot.ptr, name)
            .map_err(|_| format!("Failed to load '{}'", name))
    }
}
