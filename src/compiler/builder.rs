// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::PointerValue;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

/// A variable slot: the alloca pointer plus the LLVM type it holds.
/// LLVM 23 uses opaque pointers, so we store the type explicitly.
#[derive(Clone, Copy)]
pub struct VarSlot<'ctx> {
    pub ptr: PointerValue<'ctx>,
    pub ty: BasicTypeEnum<'ctx>,
}

pub struct IRBuilder<'ctx> {
    builder: Builder<'ctx>,
    /// Maps variable names to their alloca'd stack slots.
    allocas: HashMap<String, VarSlot<'ctx>>,
    /// Names of local variables whose source-level type is `bool`.
    ///
    /// Booleans are stored as i64 for uniform arithmetic, so this metadata is
    /// needed to print and stringify them as `true`/`false` instead of 1/0.
    bool_locals: HashSet<String>,
}

impl<'ctx> Deref for IRBuilder<'ctx> {
    type Target = Builder<'ctx>;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl<'ctx> IRBuilder<'ctx> {
    /// Create a new IRBuilder bound to the given context.
    pub fn new(context: &'ctx Context) -> Self {
        IRBuilder {
            builder: context.create_builder(),
            allocas: HashMap::new(),
            bool_locals: HashSet::new(),
        }
    }

    /// Reset scope when entering a new function.
    pub fn reset_scope(&mut self) {
        self.allocas.clear();
        self.bool_locals.clear();
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
        self.bool_locals.remove(name);
    }

    /// Record that `name` holds a source-level boolean.
    pub fn mark_bool(&mut self, name: &str) {
        self.bool_locals.insert(name.to_string());
    }

    /// Record that `name` no longer holds a source-level boolean.
    pub fn unmark_bool(&mut self, name: &str) {
        self.bool_locals.remove(name);
    }

    /// Whether `name` is a local variable with boolean source semantics.
    pub fn is_bool_local(&self, name: &str) -> bool {
        self.bool_locals.contains(name)
    }
}
