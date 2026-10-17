// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use std::collections::HashMap;

/// A variable slot: the alloca pointer plus the LLVM type it holds.
/// LLVM 21 uses opaque pointers, so we store the type explicitly.
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
    #[allow(dead_code)] // PLANNED FOR v0.3.0: Function context tracking
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

    /// Forget the slot registered for `name`.
    ///
    /// Used by constructs that introduce a temporary binding — a `for` loop
    /// variable, for instance — so the name can go back to meaning whatever
    /// it meant before (or to nothing at all).
    pub fn remove_slot(&mut self, name: &str) {
        self.allocas.remove(name);
    }

    /// Emit a `store` of `value` into the alloca for `name`.
    /// Returns an error if the variable has not been declared.
    #[allow(dead_code)] // PLANNED FOR v0.3.0: Variable storage
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
    #[allow(dead_code)] // PLANNED FOR v0.3.0: Variable loading
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
