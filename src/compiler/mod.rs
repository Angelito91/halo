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

    /// Write the compiled LLVM IR to a `.ll` file.
    pub fn emit_llvm(&self, filename: &str) -> Result<(), String> {
        self.module
            .as_ref()
            .unwrap()
            .print_to_file(filename)
            .map_err(|e| format!("Failed to write LLVM IR: {}", e))
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
