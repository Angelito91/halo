// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

pub mod builder;
pub mod codegen;
pub mod types;

pub use codegen::CodeGenerator;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetMachine;

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
// Owns the LLVM Context, Module and CodeGenerator for the lifetime of a single
// compilation. The context and module live in boxes so their addresses stay
// stable when `Compilation` is moved; that is what makes the internal
// `'static` borrows sound. The generator is the first field so it is dropped
// before the module and the context it borrows from.
//
// Usage:
//
//   let mut comp = Compilation::new("my_module");
//   comp.codegen().compile(&program)?;
//   comp.emit_llvm("out.ll")?;
//
// ─────────────────────────────────────────────────────────────────────────────
pub struct Compilation {
    generator: CodeGenerator<'static>,
    module: Box<Module<'static>>,
    // Declared last: dropped after the generator and the module. The field is
    // never read directly; it owns the context the generator borrows from.
    #[allow(dead_code)]
    context: Box<Context>,
}

impl Compilation {
    /// Create a new compilation unit with the given module name.
    pub fn new(module_name: &str) -> Self {
        let context = Box::new(Context::create());
        // SAFETY: `context` is boxed, so its address is stable even after the
        // `Box` is moved into `Compilation`. The context outlives every value
        // that borrows from it (drop order: generator, module, context).
        let ctx_ref: &'static Context = unsafe { &*(&*context as *const Context) };

        let module = Box::new(ctx_ref.create_module(module_name));
        // Pin the module to the host triple. Without this, LLVM emits its own
        // default triple and clang warns while linking because it has to
        // override it with the host triple.
        module.set_triple(&TargetMachine::get_default_triple());
        // SAFETY: `module` is boxed for the same reason as the context. It is
        // declared before the context, so it is dropped before the context.
        let mod_ref: &'static Module<'static> = unsafe { &*(&*module as *const Module<'static>) };

        let generator = CodeGenerator::new(ctx_ref, mod_ref);

        Compilation {
            generator,
            module,
            context,
        }
    }

    /// Borrow the `CodeGenerator` tied to this compilation's context and module.
    ///
    /// The returned reference borrows `self`, so it cannot outlive the
    /// `Compilation` even though the generator itself carries a `'static`
    /// lifetime internally.
    pub fn codegen(&mut self) -> &mut CodeGenerator<'static> {
        &mut self.generator
    }

    /// Run LLVM optimisation passes on the module at the given level.
    /// Call this after `codegen().compile(...)` and before `emit_llvm`.
    pub fn optimise(&mut self, level: OptLevel) -> Result<(), String> {
        self.generator.optimise(level.as_u32())
    }

    /// Write the compiled LLVM IR to a `.ll` file.
    pub fn emit_llvm(&self, filename: &str) -> Result<(), String> {
        self.module
            .print_to_file(filename)
            .map_err(|e| format!("Failed to write LLVM IR: {e}"))
    }
}
