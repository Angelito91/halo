// The Halo Programming Language
// Compiler module - LLVM-based code generation
// Version: 0.2.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

pub mod builder;
pub mod codegen;
pub mod types;

pub use codegen::CodeGenerator;

use inkwell::context::Context;
use inkwell::module::Module;

/// Optimisation level passed to LLVM passes and clang.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptLevel {
    O0,
    O1,
    #[default]
    O2,
    O3,
}

impl OptLevel {
    pub fn as_u32(self) -> u32 {
        match self {
            OptLevel::O0 => 0,
            OptLevel::O1 => 1,
            OptLevel::O2 => 2,
            OptLevel::O3 => 3,
        }
    }

    pub fn clang_flag(self) -> &'static str {
        match self {
            OptLevel::O0 => "-O0",
            OptLevel::O1 => "-O1",
            OptLevel::O2 => "-O2",
            OptLevel::O3 => "-O3",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Compilation
//
// Owns the LLVM Context and Module for the lifetime of a single compilation.
// This avoids the need for Box::leak in callers: create a Compilation, borrow
// a CodeGenerator from it, compile, then let the Compilation drop cleanly.
//
// Usage:
//
//   let mut comp = Compilation::new("my_module");
//   comp.codegen().compile(&program)?;
//   comp.emit_llvm("out.ll")?;
//
// ─────────────────────────────────────────────────────────────────────────────
pub struct Compilation {
    // SAFETY: `module` borrows from `context`, so it must be dropped first.
    // The field declaration order guarantees that in Rust (fields are dropped
    // in declaration order, reversed — i.e. last declared is dropped first).
    // We therefore declare `module` before `context` so that `module` is
    // dropped before `context`.
    module: Option<Module<'static>>,
    context: Box<Context>,
}

impl Compilation {
    /// Create a new compilation unit with the given module name.
    pub fn new(module_name: &str) -> Self {
        // Leak the context so we can hand out `'static` references.
        // The Box is stored so we can reclaim it on Drop.
        let context = Box::new(Context::create());

        // SAFETY: We extend the lifetime to `'static` because both `context`
        // and `module` live inside this struct and are dropped together in the
        // correct order (module before context, guaranteed by field order and
        // Rust's drop order).
        let ctx_ref: &'static Context = unsafe { &*(&*context as *const Context) };
        let module = ctx_ref.create_module(module_name);

        Compilation {
            module: Some(module),
            context,
        }
    }

    /// Borrow a `CodeGenerator` tied to this compilation's context and module.
    pub fn codegen(&mut self) -> CodeGenerator<'static> {
        let ctx_ref: &'static Context = unsafe { &*(&*self.context as *const Context) };
        let mod_ref: &'static Module<'static> =
            unsafe { &*(self.module.as_ref().unwrap() as *const Module<'static>) };
        CodeGenerator::new(ctx_ref, mod_ref)
    }

    /// Run LLVM optimisation passes on the module at the given level.
    /// Call this after `codegen().compile(...)` and before `emit_llvm` / `emit_object`.
    pub fn optimise(&mut self, level: OptLevel) -> Result<(), String> {
        self.codegen().optimise(level.as_u32())
    }

    /// Write the compiled LLVM IR to a `.ll` file.
    pub fn emit_llvm(&self, filename: &str) -> Result<(), String> {
        self.module
            .as_ref()
            .unwrap()
            .print_to_file(filename)
            .map_err(|e| format!("Failed to write LLVM IR: {}", e))
    }

    /// Emit a native object file directly (no clang needed for this step).
    #[allow(dead_code)]
    pub fn emit_object(&mut self, filename: &str) -> Result<(), String> {
        self.codegen().emit_object(filename)
    }

    /// Print the LLVM IR to stdout.
    pub fn print_ir(&self) {
        println!(
            "{}",
            self.module.as_ref().unwrap().print_to_string().to_string()
        );
    }
}

impl Drop for Compilation {
    fn drop(&mut self) {
        // Drop the module explicitly before the context is freed.
        drop(self.module.take());
        // `self.context` is dropped automatically after this.
    }
}
