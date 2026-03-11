// The Halo Programming Language
// Code generation from AST to LLVM IR
// Version: 0.2.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, GlobalValue};
use inkwell::AddressSpace;
use std::collections::HashMap;

use crate::compiler::builder::IRBuilder;
use crate::compiler::types::TypeMapper;
use crate::parser::ast::{BinOp, Block, Expression, Program, Statement, TopLevel};
use inkwell::basic_block::BasicBlock;

// ─────────────────────────────────────────────────────────────────────────────
// CodeGenerator
// ─────────────────────────────────────────────────────────────────────────────

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: &'ctx Module<'ctx>,
    builder: IRBuilder<'ctx>,
    type_mapper: TypeMapper<'ctx>,
    /// All user-defined functions, keyed by name.
    functions: HashMap<String, FunctionValue<'ctx>>,
    /// Module-level global variables (name → GlobalValue).
    globals: HashMap<String, GlobalValue<'ctx>>,
    /// Cached format-string globals for printf calls.
    string_globals: HashMap<String, GlobalValue<'ctx>>,
    /// Stack of (loop_cond_bb, loop_exit_bb) for break/continue support.
    loop_stack: Vec<(BasicBlock<'ctx>, BasicBlock<'ctx>)>,
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
    /// - `GlobalVar` with the synthetic name `__expr` (top-level expressions
    ///   such as `print(42)` or `while …`) → collected and emitted inside a
    ///   generated `main` function so they execute at program start.
    pub fn compile(&mut self, program: &Program) -> Result<(), String> {
        self.declare_printf();

        // ── Pass 1: declare all named function signatures ──────────────────
        for item in &program.items {
            if let TopLevel::Function { name, params, .. } = item {
                self.declare_function(name, params)?;
            }
        }

        // ── Pass 2: generate named functions and true globals ───────────────
        // While doing so, collect top-level statements (the __expr items) that
        // must go into main.
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

                TopLevel::GlobalVar { name, init, .. } => {
                    if let Some(expr) = init {
                        self.generate_global_var(name, expr)?;
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

    /// Write LLVM IR text to a `.ll` file.
    pub fn emit_llvm(&self, filename: &str) -> Result<(), String> {
        self.module
            .print_to_file(filename)
            .map_err(|e| format!("Failed to write LLVM IR: {}", e))
    }

    /// Print LLVM IR to stdout (useful for debugging).
    pub fn print_ir(&self) {
        println!("{}", self.module.print_to_string().to_string());
    }

    /// Access the underlying LLVM module.
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
        for stmt in stmts {
            self.generate_statement(stmt, main_fn)?;
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

    // =========================================================================
    // Function management
    // =========================================================================

    /// First-pass: register the function signature so it can be called.
    fn declare_function(&mut self, name: &str, params: &[String]) -> Result<(), String> {
        let fn_type = self.type_mapper.fn_i64_n(params.len());
        let function = self.module.add_function(name, fn_type, None);
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
            let alloca = self.emit_entry_alloca(function, param_name, storage_ty);

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
        if !terminated {
            if self
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
            _ => Err(format!(
                "Global variable initializer must be a constant literal"
            )),
        }
    }

    // =========================================================================
    // Block / statement generation
    // =========================================================================

    /// Generate all statements in a `Block`.
    /// Returns `true` if the block is guaranteed to terminate (all paths return).
    fn generate_block(
        &mut self,
        block: &Block,
        function: FunctionValue<'ctx>,
    ) -> Result<bool, String> {
        for stmt in &block.stmts {
            let terminated = self.generate_statement(stmt, function)?;
            if terminated {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Generate code for a single statement.
    /// Returns `true` if this statement is a terminator (i.e. `return`).
    fn generate_statement(
        &mut self,
        stmt: &Statement,
        function: FunctionValue<'ctx>,
    ) -> Result<bool, String> {
        match stmt {
            // ── Expression statement ─────────────────────────────────────────
            Statement::Expr(expr) => {
                self.generate_expression(expr)?;
                Ok(false)
            }

            // ── Variable declaration ─────────────────────────────────────────
            Statement::VarDecl { name, init, .. } => {
                let init_val = match init {
                    Some(expr) => self.generate_expression(expr)?,
                    None => self.type_mapper.i64_type().const_int(0, false).into(),
                };

                let storage_ty = self.type_mapper.storage_type_of(init_val);
                let alloca = self.emit_entry_alloca(function, name, storage_ty);
                let to_store = self.coerce_to_storage(init_val, storage_ty)?;

                self.builder
                    .builder
                    .build_store(alloca, to_store)
                    .map_err(|_| format!("Failed to store into '{}'", name))?;

                self.builder.set_alloca(name.clone(), alloca, storage_ty);
                Ok(false)
            }

            // ── If / else if / else ──────────────────────────────────────────
            Statement::If {
                cond,
                then_branch,
                else_if_branches,
                else_branch,
                ..
            } => {
                let merge_bb = self.context.append_basic_block(function, "if.merge");

                // Helper closure: emit a branch to merge_bb if the current
                // block doesn't already have a terminator.
                // We track whether every reachable path terminates.
                let mut all_terminated = true;

                // ── then ──────────────────────────────────────────────────────
                let cond_val = self.generate_expression(cond)?;
                let cond_i1 = self.coerce_to_bool(cond_val)?;

                let then_bb = self.context.append_basic_block(function, "if.then");
                // next_bb will be the first else-if or (if none) the else/merge block.
                let next_bb = self.context.append_basic_block(function, "if.next");

                self.builder
                    .builder
                    .build_conditional_branch(cond_i1, then_bb, next_bb)
                    .map_err(|_| "Failed to build if conditional branch")?;

                self.builder.builder.position_at_end(then_bb);
                let then_terminated = self.generate_block(then_branch, function)?;
                if !then_terminated
                    && self
                        .builder
                        .builder
                        .get_insert_block()
                        .and_then(|b| b.get_terminator())
                        .is_none()
                {
                    self.builder
                        .builder
                        .build_unconditional_branch(merge_bb)
                        .map_err(|_| "Failed to branch to if.merge from then")?;
                }
                if !then_terminated {
                    all_terminated = false;
                }

                // ── else if chains ────────────────────────────────────────────
                let mut current_next_bb = next_bb;
                for (idx, branch) in else_if_branches.iter().enumerate() {
                    self.builder.builder.position_at_end(current_next_bb);

                    let branch_cond_val = self.generate_expression(&branch.cond)?;
                    let branch_cond_i1 = self.coerce_to_bool(branch_cond_val)?;

                    let branch_then_bb = self
                        .context
                        .append_basic_block(function, &format!("elif.then.{}", idx));
                    let branch_next_bb = self
                        .context
                        .append_basic_block(function, &format!("elif.next.{}", idx));

                    self.builder
                        .builder
                        .build_conditional_branch(branch_cond_i1, branch_then_bb, branch_next_bb)
                        .map_err(|_| "Failed to build else-if conditional branch")?;

                    self.builder.builder.position_at_end(branch_then_bb);
                    let branch_terminated = self.generate_block(&branch.body, function)?;
                    if !branch_terminated
                        && self
                            .builder
                            .builder
                            .get_insert_block()
                            .and_then(|b| b.get_terminator())
                            .is_none()
                    {
                        self.builder
                            .builder
                            .build_unconditional_branch(merge_bb)
                            .map_err(|_| "Failed to branch to if.merge from else-if")?;
                    }
                    if !branch_terminated {
                        all_terminated = false;
                    }

                    current_next_bb = branch_next_bb;
                }

                // ── else (or fall-through) ─────────────────────────────────────
                self.builder.builder.position_at_end(current_next_bb);
                let else_terminated = if let Some(else_b) = else_branch {
                    let t = self.generate_block(else_b, function)?;
                    if !t
                        && self
                            .builder
                            .builder
                            .get_insert_block()
                            .and_then(|b| b.get_terminator())
                            .is_none()
                    {
                        self.builder
                            .builder
                            .build_unconditional_branch(merge_bb)
                            .map_err(|_| "Failed to branch to if.merge from else")?;
                    }
                    t
                } else {
                    // No else branch → fall through to merge.
                    self.builder
                        .builder
                        .build_unconditional_branch(merge_bb)
                        .map_err(|_| "Failed to branch to if.merge (no else)")?;
                    false
                };

                if !else_terminated {
                    all_terminated = false;
                }

                self.builder.builder.position_at_end(merge_bb);

                // Only report termination when every branch (including the
                // implicit fall-through when there is no else) terminates.
                let has_else = else_branch.is_some();
                Ok(all_terminated && has_else)
            }

            // ── While loop ───────────────────────────────────────────────────
            Statement::While { cond, body, .. } => {
                let cond_bb = self.context.append_basic_block(function, "while.cond");
                let body_bb = self.context.append_basic_block(function, "while.body");
                let exit_bb = self.context.append_basic_block(function, "while.exit");

                self.builder
                    .builder
                    .build_unconditional_branch(cond_bb)
                    .map_err(|_| "Failed to branch to while.cond")?;

                // ── condition ──
                self.builder.builder.position_at_end(cond_bb);
                let cond_val = self.generate_expression(cond)?;
                let cond_i1 = self.coerce_to_bool(cond_val)?;
                self.builder
                    .builder
                    .build_conditional_branch(cond_i1, body_bb, exit_bb)
                    .map_err(|_| "Failed to build while conditional branch")?;

                // ── body ──
                self.builder.builder.position_at_end(body_bb);
                // Push loop context so break/continue can find the right blocks.
                self.loop_stack.push((cond_bb, exit_bb));
                let body_terminated = self.generate_block(body, function)?;
                self.loop_stack.pop();

                // Only jump back if the body didn't already terminate.
                if !body_terminated
                    && self
                        .builder
                        .builder
                        .get_insert_block()
                        .and_then(|b| b.get_terminator())
                        .is_none()
                {
                    self.builder
                        .builder
                        .build_unconditional_branch(cond_bb)
                        .map_err(|_| "Failed to branch back to while.cond")?;
                }

                self.builder.builder.position_at_end(exit_bb);
                Ok(false)
            }

            // ── Break ────────────────────────────────────────────────────────
            Statement::Break { .. } => {
                let exit_bb = self
                    .loop_stack
                    .last()
                    .map(|(_, exit)| *exit)
                    .ok_or_else(|| "`break` used outside of a loop".to_string())?;
                self.builder
                    .builder
                    .build_unconditional_branch(exit_bb)
                    .map_err(|_| "Failed to build break branch")?;
                Ok(true)
            }

            // ── Continue ─────────────────────────────────────────────────────
            Statement::Continue { .. } => {
                let cond_bb = self
                    .loop_stack
                    .last()
                    .map(|(cond, _)| *cond)
                    .ok_or_else(|| "`continue` used outside of a loop".to_string())?;
                self.builder
                    .builder
                    .build_unconditional_branch(cond_bb)
                    .map_err(|_| "Failed to build continue branch")?;
                Ok(true)
            }

            // ── Return ───────────────────────────────────────────────────────
            Statement::Return { value, .. } => {
                let ret_val: BasicValueEnum = match value {
                    Some(expr) => {
                        let v = self.generate_expression(expr)?;
                        // Functions always return i64; widen if needed.
                        self.widen_to_i64(v)?.into()
                    }
                    None => self.type_mapper.i64_type().const_int(0, false).into(),
                };

                self.builder
                    .builder
                    .build_return(Some(&ret_val))
                    .map_err(|_| "Failed to build return")?;
                Ok(true)
            }
        }
    }

    // =========================================================================
    // Expression generation
    // =========================================================================

    fn generate_expression(&mut self, expr: &Expression) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            // ── Literals ─────────────────────────────────────────────────────
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
            // ── String literal ───────────────────────────────────────────────
            Expression::StringLiteral(s, _) => {
                let gv = self.get_or_create_string(s, &format!("str.lit.{}", s.len()));
                Ok(gv.as_pointer_value().into())
            }

            // ── Variable read ────────────────────────────────────────────────
            Expression::Var(name, _) => {
                // Try local alloca first, then module globals.
                if let Some(slot) = self.builder.get_slot(name) {
                    self.builder
                        .builder
                        .build_load(slot.ty, slot.ptr, name)
                        .map_err(|_| format!("Failed to load '{}'", name))
                } else if let Some(&gv) = self.globals.get(name.as_str()) {
                    let ty: BasicTypeEnum = gv
                        .get_value_type()
                        .try_into()
                        .map_err(|_| format!("Global '{}' has a non-basic type", name))?;
                    self.builder
                        .builder
                        .build_load(ty, gv.as_pointer_value(), name)
                        .map_err(|_| format!("Failed to load global '{}'", name))
                } else {
                    Err(format!("Undefined variable: '{}'", name))
                }
            }

            // ── Unary operators ──────────────────────────────────────────────
            Expression::Unary { operator, expr, .. } => {
                let val = self.generate_expression(expr)?;
                match operator.as_str() {
                    "-" => {
                        if val.is_float_value() {
                            let neg = self
                                .builder
                                .builder
                                .build_float_neg(val.into_float_value(), "fneg")
                                .map_err(|_| "Failed to negate float")?;
                            Ok(neg.into())
                        } else {
                            let iv = self.widen_to_i64(val)?;
                            let neg = self
                                .builder
                                .builder
                                .build_int_neg(iv, "ineg")
                                .map_err(|_| "Failed to negate integer")?;
                            Ok(neg.into())
                        }
                    }
                    "!" => {
                        let iv = self.coerce_to_bool(val)?;
                        let not = self
                            .builder
                            .builder
                            .build_not(iv, "not")
                            .map_err(|_| "Failed to apply NOT")?;
                        Ok(not.into())
                    }
                    op => Err(format!("Unknown unary operator: '{}'", op)),
                }
            }

            // ── Binary operators ─────────────────────────────────────────────
            Expression::Binary {
                left, op, right, ..
            } => {
                let lv = self.generate_expression(left)?;
                let rv = self.generate_expression(right)?;
                self.generate_binary_op(lv, op, rv)
            }

            // ── Assignment ───────────────────────────────────────────────────
            Expression::Assign { name, value, .. } => {
                let val = self.generate_expression(value)?;

                if let Some(slot) = self.builder.get_slot(name) {
                    // Coerce the new value to the slot's existing storage type.
                    let to_store = self.coerce_to_storage(val, slot.ty)?;
                    self.builder
                        .builder
                        .build_store(slot.ptr, to_store)
                        .map_err(|_| format!("Failed to store into '{}'", name))?;
                    Ok(to_store)
                } else if let Some(&gv) = self.globals.get(name.as_str()) {
                    let slot_ty: BasicTypeEnum = gv
                        .get_value_type()
                        .try_into()
                        .map_err(|_| format!("Global '{}' has a non-basic type", name))?;
                    let to_store = self.coerce_to_storage(val, slot_ty)?;
                    self.builder
                        .builder
                        .build_store(gv.as_pointer_value(), to_store)
                        .map_err(|_| format!("Failed to store into global '{}'", name))?;
                    Ok(to_store)
                } else {
                    Err(format!("Assignment to undeclared variable: '{}'", name))
                }
            }

            // ── Function calls ───────────────────────────────────────────────
            Expression::Call { name, args, .. } => self.generate_call(name, args),
        }
    }

    // =========================================================================
    // Binary operations
    // =========================================================================

    fn generate_binary_op(
        &mut self,
        left: BasicValueEnum<'ctx>,
        op: &BinOp,
        right: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        if left.is_float_value() || right.is_float_value() {
            self.generate_float_binary_op(left, op, right)
        } else {
            self.generate_int_binary_op(left, op, right)
        }
    }

    fn generate_int_binary_op(
        &mut self,
        left: BasicValueEnum<'ctx>,
        op: &BinOp,
        right: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Widen both operands to i64 so bit-widths always match.
        let lv = self.widen_to_i64(left)?;
        let rv = self.widen_to_i64(right)?;
        let b = &self.builder.builder;

        match op {
            BinOp::Add => Ok(b.build_int_add(lv, rv, "add").map_err(|_| "add")?.into()),
            BinOp::Sub => Ok(b.build_int_sub(lv, rv, "sub").map_err(|_| "sub")?.into()),
            BinOp::Mul => Ok(b.build_int_mul(lv, rv, "mul").map_err(|_| "mul")?.into()),
            BinOp::Div => Ok(b
                .build_int_signed_div(lv, rv, "div")
                .map_err(|_| "div")?
                .into()),
            BinOp::Mod => Ok(b
                .build_int_signed_rem(lv, rv, "mod")
                .map_err(|_| "mod")?
                .into()),
            BinOp::Eq => Ok(b
                .build_int_compare(inkwell::IntPredicate::EQ, lv, rv, "eq")
                .map_err(|_| "eq")?
                .into()),
            BinOp::Neq => Ok(b
                .build_int_compare(inkwell::IntPredicate::NE, lv, rv, "neq")
                .map_err(|_| "neq")?
                .into()),
            BinOp::Lt => Ok(b
                .build_int_compare(inkwell::IntPredicate::SLT, lv, rv, "lt")
                .map_err(|_| "lt")?
                .into()),
            BinOp::Gt => Ok(b
                .build_int_compare(inkwell::IntPredicate::SGT, lv, rv, "gt")
                .map_err(|_| "gt")?
                .into()),
            BinOp::Le => Ok(b
                .build_int_compare(inkwell::IntPredicate::SLE, lv, rv, "le")
                .map_err(|_| "le")?
                .into()),
            BinOp::Ge => Ok(b
                .build_int_compare(inkwell::IntPredicate::SGE, lv, rv, "ge")
                .map_err(|_| "ge")?
                .into()),
            BinOp::And => Ok(b.build_and(lv, rv, "and").map_err(|_| "and")?.into()),
            BinOp::Or => Ok(b.build_or(lv, rv, "or").map_err(|_| "or")?.into()),
        }
    }

    fn generate_float_binary_op(
        &mut self,
        left: BasicValueEnum<'ctx>,
        op: &BinOp,
        right: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Promote both operands to f64.
        let lv = self.promote_to_f64(left)?;
        let rv = self.promote_to_f64(right)?;
        let b = &self.builder.builder;

        match op {
            BinOp::Add => Ok(b
                .build_float_add(lv, rv, "fadd")
                .map_err(|_| "fadd")?
                .into()),
            BinOp::Sub => Ok(b
                .build_float_sub(lv, rv, "fsub")
                .map_err(|_| "fsub")?
                .into()),
            BinOp::Mul => Ok(b
                .build_float_mul(lv, rv, "fmul")
                .map_err(|_| "fmul")?
                .into()),
            BinOp::Div => Ok(b
                .build_float_div(lv, rv, "fdiv")
                .map_err(|_| "fdiv")?
                .into()),
            BinOp::Mod => Ok(b
                .build_float_rem(lv, rv, "frem")
                .map_err(|_| "frem")?
                .into()),
            BinOp::Eq => Ok(b
                .build_float_compare(inkwell::FloatPredicate::OEQ, lv, rv, "feq")
                .map_err(|_| "feq")?
                .into()),
            BinOp::Neq => Ok(b
                .build_float_compare(inkwell::FloatPredicate::ONE, lv, rv, "fneq")
                .map_err(|_| "fneq")?
                .into()),
            BinOp::Lt => Ok(b
                .build_float_compare(inkwell::FloatPredicate::OLT, lv, rv, "flt")
                .map_err(|_| "flt")?
                .into()),
            BinOp::Gt => Ok(b
                .build_float_compare(inkwell::FloatPredicate::OGT, lv, rv, "fgt")
                .map_err(|_| "fgt")?
                .into()),
            BinOp::Le => Ok(b
                .build_float_compare(inkwell::FloatPredicate::OLE, lv, rv, "fle")
                .map_err(|_| "fle")?
                .into()),
            BinOp::Ge => Ok(b
                .build_float_compare(inkwell::FloatPredicate::OGE, lv, rv, "fge")
                .map_err(|_| "fge")?
                .into()),
            // Logical ops: compare each side to 0.0, then combine as i1.
            BinOp::And | BinOp::Or => {
                let zero = self.type_mapper.f64_type().const_float(0.0);
                let lb = b
                    .build_float_compare(inkwell::FloatPredicate::ONE, lv, zero, "ftob_l")
                    .map_err(|_| "ftob_l")?;
                let rb = b
                    .build_float_compare(inkwell::FloatPredicate::ONE, rv, zero, "ftob_r")
                    .map_err(|_| "ftob_r")?;
                let result = match op {
                    BinOp::And => b.build_and(lb, rb, "and").map_err(|_| "and")?,
                    _ => b.build_or(lb, rb, "or").map_err(|_| "or")?,
                };
                Ok(result.into())
            }
        }
    }

    // =========================================================================
    // Function calls
    // =========================================================================

    fn generate_call(
        &mut self,
        name: &str,
        args: &[Expression],
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match name {
            // ── print(...) ───────────────────────────────────────────────────
            "print" | "println" => {
                if args.is_empty() {
                    // No arguments: just emit a newline.
                    self.emit_printf_str("\n")?;
                    return Ok(self.type_mapper.i64_type().const_int(0, false).into());
                }

                for (i, arg_expr) in args.iter().enumerate() {
                    if i > 0 {
                        // Separate multiple arguments with a space.
                        self.emit_printf_str(" ")?;
                    }
                    let val = self.generate_expression(arg_expr)?;
                    self.emit_printf_for_value(val)?;
                }

                // Always end with a newline.
                self.emit_printf_str("\n")?;
                Ok(self.type_mapper.i64_type().const_int(0, false).into())
            }

            // ── User-defined functions ────────────────────────────────────────
            _ => {
                let function = *self
                    .functions
                    .get(name)
                    .ok_or_else(|| format!("Undefined function: '{}'", name))?;

                let mut call_args: Vec<BasicMetadataValueEnum<'ctx>> = Vec::new();
                for arg_expr in args {
                    let val = self.generate_expression(arg_expr)?;
                    // All user functions take i64 parameters; coerce accordingly.
                    if val.is_float_value() {
                        return Err(format!(
                            "Cannot pass a float directly to function '{}'; \
                             only integer arguments are supported in this version",
                            name
                        ));
                    }
                    let widened = self.widen_to_i64(val)?;
                    call_args.push(widened.into());
                }

                let call_site = self
                    .builder
                    .builder
                    .build_call(function, &call_args, "call")
                    .map_err(|_| format!("Failed to call '{}'", name))?;

                match call_site.try_as_basic_value() {
                    inkwell::values::ValueKind::Basic(val) => Ok(val),
                    inkwell::values::ValueKind::Instruction(_) => {
                        Ok(self.type_mapper.i64_type().const_int(0, false).into())
                    }
                }
            }
        }
    }

    // =========================================================================
    // Printf helpers
    // =========================================================================

    /// Emit a `printf` call for a literal string (e.g. `"\n"`, `" "`).
    fn emit_printf_str(&mut self, s: &str) -> Result<(), String> {
        let printf = self
            .module
            .get_function("printf")
            .ok_or("printf not declared")?;
        let fmt = self.get_or_create_string(s, &format!("str_{}", s.len()));
        let fmt_ptr: BasicMetadataValueEnum = fmt.as_pointer_value().into();
        self.builder
            .builder
            .build_call(printf, &[fmt_ptr], "printf_str")
            .map_err(|_| format!("Failed to call printf for {:?}", s))?;
        Ok(())
    }

    /// Emit a `printf` call that prints `val` with an appropriate format string.
    fn emit_printf_for_value(&mut self, val: BasicValueEnum<'ctx>) -> Result<(), String> {
        let printf = self
            .module
            .get_function("printf")
            .ok_or("printf not declared")?;

        if val.is_float_value() {
            // Ensure the value is f64 (promote f32 if necessary).
            let fv = self.promote_to_f64(val)?;
            let fmt = self.get_or_create_string("%f", "fmt_f64");
            let fmt_ptr: BasicMetadataValueEnum = fmt.as_pointer_value().into();
            let fval: BasicMetadataValueEnum = fv.into();
            self.builder
                .builder
                .build_call(printf, &[fmt_ptr, fval], "printf_f")
                .map_err(|_| "Failed to call printf (float)")?;
        } else if val.is_pointer_value() {
            // Pointer values are assumed to be null-terminated C strings (%s).
            let fmt = self.get_or_create_string("%s", "fmt_str");
            let fmt_ptr: BasicMetadataValueEnum = fmt.as_pointer_value().into();
            let sval: BasicMetadataValueEnum = val.into_pointer_value().into();
            self.builder
                .builder
                .build_call(printf, &[fmt_ptr, sval], "printf_s")
                .map_err(|_| "Failed to call printf (string)")?;
        } else {
            // All integer/bool values → widen to i64 and print as %lld.
            let wide = self.widen_to_i64(val)?;
            let fmt = self.get_or_create_string("%lld", "fmt_i64");
            let fmt_ptr: BasicMetadataValueEnum = fmt.as_pointer_value().into();
            let ival: BasicMetadataValueEnum = wide.into();
            self.builder
                .builder
                .build_call(printf, &[fmt_ptr, ival], "printf_i")
                .map_err(|_| "Failed to call printf (int)")?;
        }
        Ok(())
    }

    /// Get (or lazily create) a global null-terminated string constant.
    fn get_or_create_string(&mut self, s: &str, name: &str) -> GlobalValue<'ctx> {
        if let Some(&gv) = self.string_globals.get(s) {
            return gv;
        }
        let gv = self
            .builder
            .builder
            .build_global_string_ptr(s, name)
            .expect("Failed to create global string");
        self.string_globals.insert(s.to_string(), gv);
        gv
    }

    // =========================================================================
    // Alloca helper
    // =========================================================================

    /// Emit an `alloca` instruction at the very start of the function's entry
    /// block so that mem2reg can promote it to a register.
    fn emit_entry_alloca(
        &self,
        function: FunctionValue<'ctx>,
        name: &str,
        ty: BasicTypeEnum<'ctx>,
    ) -> inkwell::values::PointerValue<'ctx> {
        // Temporarily reposition to the start of the entry block.
        let entry_bb = function
            .get_first_basic_block()
            .expect("function has no entry block");
        let saved_pos = self.builder.builder.get_insert_block();

        match entry_bb.get_first_instruction() {
            Some(first_instr) => self.builder.builder.position_before(&first_instr),
            None => self.builder.builder.position_at_end(entry_bb),
        }

        let alloca = self
            .builder
            .builder
            .build_alloca(ty, name)
            .expect("Failed to build alloca");

        // Restore the builder's original position.
        if let Some(bb) = saved_pos {
            self.builder.builder.position_at_end(bb);
        }

        alloca
    }

    // =========================================================================
    // Type coercion helpers
    // =========================================================================

    /// Coerce `val` so it can be stored in a slot of type `slot_ty`.
    ///
    /// - float → float slot : extend f32→f64 if needed
    /// - int/bool → i64 slot : sign-extend
    /// - mismatch (float into int slot or vice-versa): error
    fn coerce_to_storage(
        &mut self,
        val: BasicValueEnum<'ctx>,
        slot_ty: BasicTypeEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match slot_ty {
            BasicTypeEnum::FloatType(_) => Ok(self.promote_to_f64(val)?.into()),
            BasicTypeEnum::IntType(it) => {
                if val.is_float_value() {
                    return Err("Cannot store a float value into an integer variable".to_string());
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
            _ => Ok(val), // pass-through for pointer/aggregate types
        }
    }

    /// Compare `val` to zero and return an `i1` for use in branch conditions.
    fn coerce_to_bool(
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

    /// Sign-extend any integer value to `i64`. No-op if already `i64`.
    /// Returns an error if called with a float (use `promote_to_f64` instead).
    fn widen_to_i64(
        &mut self,
        val: BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::IntValue<'ctx>, String> {
        if val.is_float_value() {
            return Err("widen_to_i64: value is a float — use promote_to_f64 instead".to_string());
        }
        let iv = val.into_int_value();
        if iv.get_type().get_bit_width() == 64 {
            return Ok(iv);
        }
        self.builder
            .builder
            .build_int_s_extend(iv, self.type_mapper.i64_type(), "sext")
            .map_err(|_| "Failed to sign-extend to i64".to_string())
    }

    /// Promote any numeric value to `f64`.
    /// - `f64` → no-op
    /// - `f32` → `fpext`
    /// - integer → `sitofp`
    fn promote_to_f64(
        &mut self,
        val: BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::FloatValue<'ctx>, String> {
        if val.is_float_value() {
            let fv = val.into_float_value();
            if fv.get_type().get_bit_width() == 64 {
                return Ok(fv);
            }
            return self
                .builder
                .builder
                .build_float_ext(fv, self.type_mapper.f64_type(), "fpext")
                .map_err(|_| "Failed to extend float to f64".to_string());
        }
        let iv = val.into_int_value();
        self.builder
            .builder
            .build_signed_int_to_float(iv, self.type_mapper.f64_type(), "sitofp")
            .map_err(|_| "Failed to convert int to f64".to_string())
    }
}
