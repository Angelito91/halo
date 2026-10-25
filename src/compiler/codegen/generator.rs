// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, GlobalValue};
use std::collections::{HashMap, HashSet};

use crate::compiler::builder::IRBuilder;
use crate::compiler::types::TypeMapper;
use crate::parser::ast::{Program, Statement, TopLevel};
use inkwell::basic_block::BasicBlock;

use super::emitter::Emitter;
use super::functions::{determine_return_type, is_constant_initializer};

// ─────────────────────────────────────────────────────────────────────────────
// CodeGenerator
// ─────────────────────────────────────────────────────────────────────────────

pub struct CodeGenerator<'ctx> {
    pub context: &'ctx Context,
    pub module: &'ctx Module<'ctx>,
    pub builder: IRBuilder<'ctx>,
    pub type_mapper: TypeMapper<'ctx>,
    /// All user-defined functions, keyed by name.
    pub functions: HashMap<String, FunctionValue<'ctx>>,
    /// Return type of each user function ("i64", "f64", "string").
    pub fn_return_types: HashMap<String, &'static str>,
    /// Module-level global variables (name → GlobalValue).
    pub globals: HashMap<String, GlobalValue<'ctx>>,
    /// Global variables whose source-level type is `bool`.
    pub bool_globals: HashSet<String>,
    /// Cached format-string globals for printf calls.
    pub string_globals: HashMap<String, GlobalValue<'ctx>>,
    /// Stack of (loop_cond_bb, loop_exit_bb) for break/continue support.
    pub loop_stack: Vec<(BasicBlock<'ctx>, BasicBlock<'ctx>)>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module: &'ctx Module<'ctx>) -> Self {
        let builder = IRBuilder::new(context);
        let type_mapper = TypeMapper::new(context);

        CodeGenerator {
            context,
            module,
            builder,
            type_mapper,
            functions: HashMap::new(),
            fn_return_types: HashMap::new(),
            globals: HashMap::new(),
            bool_globals: HashSet::new(),
            string_globals: HashMap::new(),
            loop_stack: Vec::new(),
        }
    }

    // =========================================================================
    // Public API
    // =========================================================================

    /// Whether `name` refers to a variable with source-level boolean type.
    pub(crate) fn is_bool_var(&self, name: &str) -> bool {
        self.builder.is_bool_local(name) || self.bool_globals.contains(name)
    }

    /// Compile the entire program to LLVM IR.
    ///
    /// Top-level items are handled as follows:
    ///
    /// - `Function`   → compiled to a named LLVM function.
    /// - `GlobalVar` with a real name (e.g. `x = 3`) → module-level global
    ///   with a constant initializer.
    /// - `GlobalVar` with a non-constant initializer (e.g. `g = "hello"`)
    ///   → emitted as a local assignment at the start of `main`.
    /// - `GlobalVar` with the synthetic name `__expr` (top-level expressions
    ///   in the REPL) → collected and emitted inside `main`.
    pub fn compile(&mut self, program: &Program) -> Result<(), String> {
        self.declare_printf();
        self.declare_string_runtime();

        // ── Pass 0: determine return types for all functions ───────────────
        for item in &program.items {
            if let TopLevel::Function { name, body, .. } = item {
                let ret = determine_return_type(body);
                self.fn_return_types.insert(name.clone(), ret);
            }
        }

        // ── Pass 1: declare all named function signatures ──────────────────
        for item in &program.items {
            if let TopLevel::Function { name, params, .. } = item {
                self.declare_function(name, params)?;
            }
        }

        // ── Pass 2: generate named functions and true globals ───────────────
        // While doing so, collect top-level statements (the __expr items) that
        // must go into main. Variables with non-constant initializers
        // (e.g. string literals) are also collected so they are emitted as
        // local assignments inside `main` (LLVM does not allow non-const
        // values in global initializers).
        let mut top_level_stmts: Vec<Statement> = Vec::new();

        for item in &program.items {
            match item {
                TopLevel::Function {
                    name, params, body, ..
                } => self.generate_function(name, params, body)?,

                TopLevel::GlobalVar { name, init, .. } if name == "__expr" => {
                    // A bare top-level expression — turn it into a statement
                    // that will be emitted inside `main`.
                    if let Some(expr) = init {
                        top_level_stmts.push(Statement::Expr(expr.clone()));
                    }
                }

                // A bare top-level statement (if, while, break, continue, return).
                TopLevel::Stmt { stmt, .. } => {
                    top_level_stmts.push(stmt.clone());
                }

                TopLevel::GlobalVar { name, init, pos } => {
                    if let Some(expr) = init {
                        if is_constant_initializer(expr) {
                            self.generate_global_var(name, expr)?;
                        } else {
                            // Non-constant initializer (e.g. a string literal).
                            // Emit it as a local variable declaration inside
                            // `main` instead, since LLVM does not allow
                            // non-const values in global initializers.
                            top_level_stmts.push(Statement::VarDecl {
                                name: name.clone(),
                                init: Some(expr.clone()),
                                pos: *pos,
                            });
                        }
                    } else {
                        self.generate_global_var_typed(name, None)?;
                    }
                }
            }
        }

        // ── Pass 3: emit `main` if there are any top-level statements ───────
        if !top_level_stmts.is_empty() {
            self.generate_main(&top_level_stmts)?;
        }

        // Reject malformed IR at the source instead of letting clang produce a
        // confusing error (or, worse, emitting a broken binary).
        self.module
            .verify()
            .map_err(|e| format!("Invalid LLVM IR: {e}"))?;

        Ok(())
    }

    /// Run LLVM optimisation passes on the module.
    /// `level` maps to `-O` flags: 0 = no opts, 1 = -O1, 2 = -O2, 3 = -O3
    pub fn optimise(&self, level: u32) -> Result<(), String> {
        // Initialise the native target so TargetMachine can be created.
        Target::initialize_native(&InitializationConfig::default())
            .map_err(|e| format!("Failed to initialise native target: {}", e))?;

        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple)
            .map_err(|e| format!("Failed to get target from triple: {}", e))?;

        let machine = target
            .create_target_machine(
                &triple,
                "generic",
                "",
                inkwell::OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or("Failed to create TargetMachine")?;

        // Keep the module's data layout consistent with the target machine.
        // This is required for correct optimisation and removes a source of
        // target-dependent IR differences.
        self.module
            .set_data_layout(&machine.get_target_data().get_data_layout());

        // Build the pass pipeline string, mirroring clang -O<level>.
        let pipeline = match level {
            0 => return Ok(()), // nothing to do
            1 => "default<O1>",
            3 => "default<O3>",
            _ => "default<O2>", // level 2 and anything else → O2
        };

        let opts = PassBuilderOptions::create();
        opts.set_loop_interleaving(true);
        opts.set_loop_vectorization(true);
        opts.set_loop_unrolling(true);
        opts.set_merge_functions(true);

        self.module
            .run_passes(pipeline, &machine, opts)
            .map_err(|e| format!("Optimisation pass failed: {}", e))?;

        Ok(())
    }

    // =========================================================================
    // Type coercion helpers (called from emitter too)
    // =========================================================================

    /// Coerce `val` so it can be stored in a slot of type `slot_ty`.
    ///
    /// - float → float slot : extend f32→f64 if needed
    /// - int/bool → i64 slot : sign-extend
    /// - mismatch (float into int slot or vice-versa): error
    pub(crate) fn coerce_to_storage(
        &mut self,
        val: BasicValueEnum<'ctx>,
        slot_ty: BasicTypeEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match slot_ty {
            BasicTypeEnum::FloatType(_) => {
                if val.is_pointer_value() {
                    return Err("Cannot store a string (pointer) into a float variable".to_string());
                }
                Ok(Emitter::promote_to_f64(self, val)?.into())
            }
            BasicTypeEnum::IntType(it) => {
                if val.is_float_value() {
                    return Err("Cannot store a float value into an integer variable".to_string());
                }
                if val.is_pointer_value() {
                    return Err(
                        "Cannot store a string (pointer) into an integer variable".to_string()
                    );
                }
                let iv = val.into_int_value();
                if iv.get_type() == it {
                    return Ok(iv.into());
                }
                if iv.get_type().get_bit_width() == 1 && it.get_bit_width() > 1 {
                    return self
                        .builder
                        .build_int_z_extend(iv, it, "zext")
                        .map(|v| v.into())
                        .map_err(|_| {
                            format!(
                                "Failed to zero-extend bool to slot type i{}",
                                it.get_bit_width()
                            )
                        });
                }
                self.builder
                    .build_int_s_extend_or_bit_cast(iv, it, "coerce")
                    .map(|v| v.into())
                    .map_err(|_| "Failed to coerce int to slot type".to_string())
            }
            BasicTypeEnum::PointerType(_) => {
                if val.is_pointer_value() {
                    Ok(val)
                } else {
                    Err("Cannot store a non-pointer value into a string variable".to_string())
                }
            }
            _ => Ok(val), // pass-through for aggregate/other types
        }
    }

    /// Compare `val` to zero and return an `i1` for use in branch conditions.
    pub fn coerce_to_bool(
        &mut self,
        val: BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::IntValue<'ctx>, String> {
        if val.is_float_value() {
            self.builder
                .build_float_compare(
                    inkwell::FloatPredicate::ONE,
                    val.into_float_value(),
                    self.type_mapper.f64_type().const_float(0.0),
                    "fbool",
                )
                .map_err(|_| "Failed to coerce float to bool".to_string())
        } else {
            let iv = val.into_int_value();
            if iv.get_type().get_bit_width() == 1 {
                return Ok(iv);
            }
            self.builder
                .build_int_compare(
                    inkwell::IntPredicate::NE,
                    iv,
                    iv.get_type().const_zero(),
                    "ibool",
                )
                .map_err(|_| "Failed to coerce int to bool".to_string())
        }
    }
}
