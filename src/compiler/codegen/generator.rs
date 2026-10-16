// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, GlobalValue};
use inkwell::AddressSpace;
use std::collections::HashMap;

use crate::compiler::builder::IRBuilder;
use crate::compiler::types::TypeMapper;
use crate::parser::ast::{BinOp, Block, Expression, Program, Statement, TopLevel};
use inkwell::basic_block::BasicBlock;

use super::emitter::Emitter;

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
    /// Cached format-string globals for printf calls.
    pub string_globals: HashMap<String, GlobalValue<'ctx>>,
    /// Stack of (loop_cond_bb, loop_exit_bb) for break/continue support.
    pub loop_stack: Vec<(BasicBlock<'ctx>, BasicBlock<'ctx>)>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module: &'ctx Module<'ctx>) -> Self {
        let builder = IRBuilder::new(context, module);
        let type_mapper = TypeMapper::new(context);

        CodeGenerator {
            context,
            module,
            builder,
            type_mapper,
            functions: HashMap::new(),
            fn_return_types: HashMap::new(),
            globals: HashMap::new(),
            string_globals: HashMap::new(),
            loop_stack: Vec::new(),
        }
    }

    // =========================================================================
    // Public API
    // =========================================================================

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

    /// Write LLVM IR text to a `.ll` file.
    #[allow(dead_code)] // PLANNED FOR v0.3.0: LLVM IR file emission
    pub fn emit_llvm(&self, filename: &str) -> Result<(), String> {
        self.module
            .print_to_file(filename)
            .map_err(|e| format!("Failed to write LLVM IR: {}", e))
    }

    /// Compile directly to a native object file (bypasses clang).
    #[allow(dead_code)] // PLANNED FOR v0.3.0: Direct object file emission
    pub fn emit_object(&self, filename: &str) -> Result<(), String> {
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
            .ok_or("Failed to create TargetMachine for object emission")?;

        machine
            .write_to_file(
                self.module,
                FileType::Object,
                std::path::Path::new(filename),
            )
            .map_err(|e| format!("Failed to write object file: {}", e))?;

        Ok(())
    }

    /// Print LLVM IR to stdout (useful for debugging).
    #[allow(dead_code)]
    pub fn print_ir(&self) {
        println!("{}", self.module.print_to_string().to_string());
    }

    /// Access the underlying LLVM module.
    #[allow(dead_code)]
    pub fn get_module(&self) -> &'ctx Module<'ctx> {
        self.module
    }

    // =========================================================================
    // Top-level main generation
    // =========================================================================

    /// Emit a C-compatible `main` function that runs all top-level statements.
    ///
    /// Signature: `i32 main()`
    fn generate_main(&mut self, stmts: &[Statement]) -> Result<(), String> {
        let i32_ty = self.type_mapper.i32_type();
        let main_type = i32_ty.fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_type, None);

        self.builder.set_function(main_fn);

        let entry_bb = self.context.append_basic_block(main_fn, "entry");
        self.builder.builder.position_at_end(entry_bb);

        // Execute every top-level statement.
        let main_name = String::from("main");
        for stmt in stmts {
            self.generate_statement(stmt, main_fn, &main_name)?;
        }

        // Return 0 from main if the block has no terminator yet.
        if self
            .builder
            .builder
            .get_insert_block()
            .and_then(|b: inkwell::basic_block::BasicBlock| b.get_terminator())
            .is_none()
        {
            self.builder
                .builder
                .build_return(Some(&i32_ty.const_int(0, false)))
                .map_err(|_| "Failed to build return from main")?;
        }

        Ok(())
    }

    // =========================================================================
    // External declarations
    // =========================================================================

    fn declare_printf(&mut self) {
        if self.module.get_function("printf").is_none() {
            let printf_type = self.type_mapper.fn_printf();
            self.module
                .add_function("printf", printf_type, Some(Linkage::External));
        }
    }

    /// Declare the string runtime helpers we use (strlen, malloc, strcpy, strcat).
    /// All take `i8*` and return either `i64` or `i8*`.
    fn declare_string_runtime(&mut self) {
        let i64_type = self.type_mapper.i64_type();
        let i8_ptr = self
            .type_mapper
            .context
            .ptr_type(inkwell::AddressSpace::default());
        // size_t strlen(const char *s) — declared as i64 (i8*) for simplicity
        if self.module.get_function("strlen").is_none() {
            let ty = i64_type.fn_type(&[i8_ptr.into()], false);
            self.module
                .add_function("strlen", ty, Some(Linkage::External));
        }
        // void *malloc(size_t size)
        if self.module.get_function("malloc").is_none() {
            let ty = i8_ptr.fn_type(&[i64_type.into()], false);
            self.module
                .add_function("malloc", ty, Some(Linkage::External));
        }
        // char *strcpy(char *dest, const char *src)
        if self.module.get_function("strcpy").is_none() {
            let ty = i8_ptr.fn_type(&[i8_ptr.into(), i8_ptr.into()], false);
            self.module
                .add_function("strcpy", ty, Some(Linkage::External));
        }
        // char *strcat(char *dest, const char *src)
        if self.module.get_function("strcat").is_none() {
            let ty = i8_ptr.fn_type(&[i8_ptr.into(), i8_ptr.into()], false);
            self.module
                .add_function("strcat", ty, Some(Linkage::External));
        }
    }

    // =========================================================================
    // Function management
    // =========================================================================

    /// First-pass: register the function signature so it can be called.
    fn declare_function(&mut self, name: &str, params: &[String]) -> Result<(), String> {
        let ret_ty = self.fn_return_types.get(name).copied().unwrap_or("i64");
        let fn_type = self.type_mapper.fn_type_for(params.len(), ret_ty);
        let function = self.module.add_function(name, fn_type, None);

        // Mark functions with attributes that enable more aggressive optimisation.
        //   nounwind  – the function never throws a C++ exception
        //   norecurse – conservative hint; mem2reg works better with it
        let ctx = self.context;
        let nounwind_kind = inkwell::attributes::Attribute::get_named_enum_kind_id("nounwind");
        let nounwind = ctx.create_enum_attribute(nounwind_kind, 0);
        function.add_attribute(inkwell::attributes::AttributeLoc::Function, nounwind);

        self.functions.insert(name.to_string(), function);
        Ok(())
    }

    /// Second-pass: generate the function body.
    fn generate_function(
        &mut self,
        name: &str,
        params: &[String],
        body: &Block,
    ) -> Result<(), String> {
        let function = *self
            .functions
            .get(name)
            .ok_or_else(|| format!("Function '{}' not declared", name))?;

        self.builder.set_function(function);

        // Create the entry block and position the builder there.
        let entry_bb = self.context.append_basic_block(function, "entry");
        self.builder.builder.position_at_end(entry_bb);

        // ── Allocate all parameters as mutable stack slots ──
        for (i, param_name) in params.iter().enumerate() {
            let param_val = function
                .get_nth_param(i as u32)
                .ok_or_else(|| format!("Missing parameter {} in '{}'", i, name))?;
            param_val.set_name(param_name);

            // Decide storage type: float params stay f64, ints become i64.
            let storage_ty = self.type_mapper.storage_type_of(param_val);
            let alloca =
                Emitter::emit_entry_alloca(&self.builder, function, param_name, storage_ty);

            // Widen boolean/narrow int params before storing.
            let to_store = self.coerce_to_storage(param_val, storage_ty)?;
            self.builder
                .builder
                .build_store(alloca, to_store)
                .map_err(|_| format!("Failed to store param '{}'", param_name))?;

            self.builder
                .set_alloca(param_name.to_string(), alloca, storage_ty);
        }

        // ── Generate statements ──
        let terminated = self.generate_block(body, function)?;

        // ── Implicit return 0 if the last block has no terminator ──
        if !terminated
            && self
                .builder
                .builder
                .get_insert_block()
                .and_then(|b| b.get_terminator())
                .is_none()
        {
            let zero = self.type_mapper.i64_type().const_int(0, false);
            self.builder
                .builder
                .build_return(Some(&zero))
                .map_err(|_| format!("Failed to build implicit return in '{}'", name))?;
        }

        Ok(())
    }

    // =========================================================================
    // Global variable generation
    // =========================================================================

    /// Generate a global with a constant-expression initializer.
    fn generate_global_var(&mut self, name: &str, init: &Expression) -> Result<(), String> {
        let value = self.generate_const_expr(init)?;
        let ty = self.type_mapper.storage_type_of(value);
        let to_store = self.coerce_to_storage(value, ty)?;

        let global = self
            .module
            .add_global(ty, Some(AddressSpace::default()), name);
        global.set_linkage(Linkage::Internal);
        global.set_initializer(&to_store);
        // Constant globals allow the optimiser to fold loads entirely.
        // We leave them non-constant here because Halo allows reassignment,
        // but we mark them unnamed_addr so the linker can merge identical ones.
        global.set_unnamed_addr(true);
        self.globals.insert(name.to_string(), global);
        Ok(())
    }

    /// Generate an uninitialised (zero) global variable.
    fn generate_global_var_typed(
        &mut self,
        name: &str,
        _ty: Option<BasicTypeEnum<'ctx>>,
    ) -> Result<(), String> {
        let ty: BasicTypeEnum = self.type_mapper.i64_type().into();
        let global = self
            .module
            .add_global(ty, Some(AddressSpace::default()), name);
        global.set_linkage(Linkage::Internal);
        global.set_initializer(&self.type_mapper.i64_type().const_zero());
        self.globals.insert(name.to_string(), global);
        Ok(())
    }

    /// Evaluate a *constant* expression (used for global initializers only).
    /// Only literals are valid here; variables and calls are not.
    fn generate_const_expr(&self, expr: &Expression) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            Expression::Number(n, _) => Ok(self
                .type_mapper
                .i64_type()
                .const_int(*n as u64, true)
                .into()),
            Expression::Float(f, _) => Ok(self.type_mapper.f64_type().const_float(*f).into()),
            Expression::Bool(b, _) => Ok(self
                .type_mapper
                .i1_type()
                .const_int(if *b { 1 } else { 0 }, false)
                .into()),
            Expression::StringLiteral(_, _) => {
                Err("String literals are not supported as global variable initializers".to_string())
            }
            _ => Err("Global variable initializer must be a constant literal".to_string()),
        }
    }

    // =========================================================================
    // Block / statement generation
    // =========================================================================

    /// Generate all statements in a `Block`.
    /// Returns `true` if the block is guaranteed to terminate (all paths return).
    pub fn generate_block(
        &mut self,
        block: &Block,
        function: FunctionValue<'ctx>,
    ) -> Result<bool, String> {
        // Look up the function name from the FunctionValue using fn_return_types.
        let fn_name = self
            .functions
            .iter()
            .find(|(_, fv)| **fv == function)
            .map(|(name, _)| name.clone())
            .unwrap_or_default();
        for stmt in &block.stmts {
            let terminated = self.generate_statement(stmt, function, &fn_name)?;
            if terminated {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Generate code for a single statement.
    /// Returns `true` if this statement is a terminator (i.e. `return`).
    /// Delegates to the statements module for actual code generation.
    fn generate_statement(
        &mut self,
        stmt: &Statement,
        function: FunctionValue<'ctx>,
        fn_name: &str,
    ) -> Result<bool, String> {
        super::statements::emit_statement(self, stmt, function, fn_name)
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
                self.builder
                    .builder
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
                .builder
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
                .builder
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

/// Scan a function body and determine its return type.
/// Returns one of "i64", "f64", or "string".
fn determine_return_type(body: &Block) -> &'static str {
    // First pass: check explicit return expressions
    for stmt in &body.stmts {
        if let Some(ty) = return_type_of_stmt(stmt) {
            if ty == "string" || ty == "f64" {
                return ty;
            }
        }
    }

    // Second pass: check if the function body contains string operations
    // (string concat `+`, string repeat `*`, or string literals being assigned).
    // If so, it might return a string via a variable.
    if contains_string_op(body) {
        return "string";
    }

    // Fallback: check for any non-i64 return type
    for stmt in &body.stmts {
        if let Some(ty) = return_type_of_stmt(stmt) {
            return ty;
        }
    }

    "i64"
}

/// Check if a block contains any string-related operations.
fn contains_string_op(body: &Block) -> bool {
    for stmt in &body.stmts {
        if stmt_has_string(stmt) {
            return true;
        }
    }
    false
}

fn stmt_has_string(stmt: &Statement) -> bool {
    match stmt {
        Statement::Return { value: Some(expr), .. } => expr_has_string(expr),
        Statement::Expr(expr) => expr_has_string(expr),
        Statement::VarDecl { init: Some(expr), .. } => expr_has_string(expr),
        Statement::VarDecl { init: None, .. } => false,
        Statement::Return { value: None, .. } => false,
        Statement::If { then_branch, else_branch, else_if_branches, .. } => {
            for s in &then_branch.stmts {
                if stmt_has_string(s) {
                    return true;
                }
            }
            if let Some(else_b) = else_branch {
                for s in &else_b.stmts {
                    if stmt_has_string(s) {
                        return true;
                    }
                }
            }
            for branch in else_if_branches {
                if expr_has_string(&branch.cond) {
                    return true;
                }
                for s in &branch.body.stmts {
                    if stmt_has_string(s) {
                        return true;
                    }
                }
            }
            false
        }
        Statement::While { cond, body, .. } => {
            expr_has_string(cond) || body.stmts.iter().any(|s| stmt_has_string(s))
        }
        Statement::Break { .. } | Statement::Continue { .. } => false,
    }
}

fn expr_has_string(expr: &Expression) -> bool {
    match expr {
        Expression::StringLiteral(_, _) => true,
        Expression::Binary { left, op, right, .. } => {
            matches!(op, BinOp::Add | BinOp::Mul) && (expr_has_string(left) || expr_has_string(right))
        }
        Expression::Var(_, _) => false, // variable — could be string, but conservative
        _ => false,
    }
}

/// Check if a statement contains a return expression that reveals the return type.
fn return_type_of_stmt(stmt: &Statement) -> Option<&'static str> {
    match stmt {
        Statement::Return { value: Some(expr), .. } => return_type_of_expr(expr),
        Statement::If { then_branch, else_branch, else_if_branches, .. } => {
            for s in &then_branch.stmts {
                if let Some(ty) = return_type_of_stmt(s) {
                    return Some(ty);
                }
            }
            if let Some(else_b) = else_branch {
                for s in &else_b.stmts {
                    if let Some(ty) = return_type_of_stmt(s) {
                        return Some(ty);
                    }
                }
            }
            for branch in else_if_branches {
                for s in &branch.body.stmts {
                    if let Some(ty) = return_type_of_stmt(s) {
                        return Some(ty);
                    }
                }
            }
            None
        }
        Statement::While { body, .. } => {
            for s in &body.stmts {
                if let Some(ty) = return_type_of_stmt(s) {
                    return Some(ty);
                }
            }
            None
        }
        _ => None,
    }
}

/// Check what type a return expression would yield.
fn return_type_of_expr(expr: &Expression) -> Option<&'static str> {
    match expr {
        Expression::StringLiteral(_, _) => Some("string"),
        Expression::Float(_, _) => Some("f64"),
        Expression::Number(_, _) => Some("i64"),
        Expression::Bool(_, _) => Some("i64"),
        Expression::Binary { left, op, right, .. } => {
            // String concat/repeat → string return
            if matches!(op, BinOp::Add | BinOp::Mul) {
                if return_type_of_expr(left) == Some("string")
                    || return_type_of_expr(right) == Some("string")
                {
                    return Some("string");
                }
            }
            if matches!(op, BinOp::Eq | BinOp::Neq | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge
                | BinOp::And | BinOp::Or)
            {
                return Some("i64"); // bools are stored as i64
            }
            // Arithmetic → i64 or f64
            if return_type_of_expr(left) == Some("f64") || return_type_of_expr(right) == Some("f64")
            {
                return Some("f64");
            }
            Some("i64")
        }
        _ => Some("i64"),
    }
}

/// Returns true if the given expression is a valid constant initializer for
/// a global variable. LLVM requires global initializers to be compile-time
/// constants; we only support number, float, and bool literals. Strings and
/// any computed value must be promoted to a local assignment inside `main`.
fn is_constant_initializer(expr: &Expression) -> bool {
    matches!(
        expr,
        Expression::Number(_, _) | Expression::Float(_, _) | Expression::Bool(_, _)
    )
}
