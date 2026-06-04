// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, GlobalValue};
use std::collections::HashMap;

use crate::compiler::builder::IRBuilder;
use crate::compiler::types::TypeMapper;

/// GeneratorContext centralizes all shared state needed during code generation.
///
/// This struct consolidates references to LLVM primitives (context, module, builder)
/// and application-specific state (functions, globals, loop control flow) into a
/// single, coherent structure. This allows:
///
/// - **Reduced parameter passing**: Instead of passing 8+ arguments to every function,
///   we pass a single `GeneratorContext` reference.
/// - **Clear state ownership**: All mutable state is contained here, making
///   dependencies explicit.
/// - **Easier refactoring**: Adding new state (e.g., variable scope, optimization hints)
///   only requires modification here.
///
/// # Lifetime
///
/// The `'ctx` lifetime is tied to the LLVM `Context`. All LLVM values
/// (modules, functions, blocks) must not outlive this lifetime.
///
/// # Fields
///
/// - `context`: LLVM Context - root of all type/value universe.
/// - `module`: LLVM Module - contains functions and globals being generated.
/// - `builder`: IR builder - used to emit LLVM instructions.
/// - `type_mapper`: Maps Halo types to LLVM types (e.g., `int` → `i64`).
/// - `functions`: All user-defined functions keyed by name.
/// - `globals`: Module-level global variables keyed by name.
/// - `string_globals`: Cached format strings for `printf` (printf support).
/// - `loop_stack`: Stack of (condition_block, exit_block) for `break`/`continue`.
pub struct GeneratorContext<'ctx> {
    /// LLVM Context: root of the type system.
    pub context: &'ctx Context,

    /// LLVM Module: target compilation unit containing all generated code.
    pub module: &'ctx Module<'ctx>,

    /// IR Builder: emits LLVM instructions into basic blocks.
    pub builder: &'ctx mut IRBuilder<'ctx>,

    /// Type mapper: converts Halo types to LLVM types.
    pub type_mapper: &'ctx TypeMapper<'ctx>,

    /// User-defined functions, keyed by name (e.g., "factorial" → FunctionValue).
    pub functions: &'ctx HashMap<String, FunctionValue<'ctx>>,

    /// Module-level globals, keyed by name (e.g., "x" → GlobalValue).
    pub globals: &'ctx HashMap<String, GlobalValue<'ctx>>,

    /// Cached format strings for printf operations.
    /// Used to avoid redundant string constant allocations.
    pub string_globals: &'ctx HashMap<String, GlobalValue<'ctx>>,

    /// Stack of loop control flow blocks for break/continue support.
    ///
    /// Each entry is `(condition_block, exit_block)`:
    /// - `condition_block`: The block where the loop condition is checked.
    /// - `exit_block`: The block to branch to for `break` statements.
    pub loop_stack: &'ctx mut Vec<(BasicBlock<'ctx>, BasicBlock<'ctx>)>,
}

impl<'ctx> GeneratorContext<'ctx> {
    /// Creates a new GeneratorContext.
    ///
    /// This is a low-level constructor. In practice, use `CodeGenerator::new()` which
    /// properly initializes all fields.
    pub fn new(
        context: &'ctx Context,
        module: &'ctx Module<'ctx>,
        builder: &'ctx mut IRBuilder<'ctx>,
        type_mapper: &'ctx TypeMapper<'ctx>,
        functions: &'ctx HashMap<String, FunctionValue<'ctx>>,
        globals: &'ctx HashMap<String, GlobalValue<'ctx>>,
        string_globals: &'ctx HashMap<String, GlobalValue<'ctx>>,
        loop_stack: &'ctx mut Vec<(BasicBlock<'ctx>, BasicBlock<'ctx>)>,
    ) -> Self {
        GeneratorContext {
            context,
            module,
            builder,
            type_mapper,
            functions,
            globals,
            string_globals,
            loop_stack,
        }
    }

    /// Pushes a new loop context onto the stack.
    ///
    /// Called when entering a loop to track where `break` and `continue` should branch.
    pub fn push_loop(&mut self, cond_bb: BasicBlock<'ctx>, exit_bb: BasicBlock<'ctx>) {
        self.loop_stack.push((cond_bb, exit_bb));
    }

    /// Pops the current loop context from the stack.
    ///
    /// Called when exiting a loop. Returns the loop blocks if they exist.
    pub fn pop_loop(&mut self) -> Option<(BasicBlock<'ctx>, BasicBlock<'ctx>)> {
        self.loop_stack.pop()
    }

    /// Peeks at the current loop context without removing it.
    ///
    /// Returns `Some((cond_bb, exit_bb))` if inside a loop, `None` otherwise.
    pub fn current_loop(&self) -> Option<(BasicBlock<'ctx>, BasicBlock<'ctx>)> {
        self.loop_stack.last().copied()
    }

    /// Checks if we are currently inside a loop.
    pub fn in_loop(&self) -> bool {
        !self.loop_stack.is_empty()
    }
}
