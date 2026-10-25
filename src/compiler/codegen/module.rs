// The Halo Programming Language — module/function emission for CodeGenerator.

use super::emitter::Emitter;
use super::generator::CodeGenerator;
use crate::parser::ast::{Block, Expression, Statement};
use inkwell::module::Linkage;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue};
use inkwell::AddressSpace;

impl<'ctx> CodeGenerator<'ctx> {
    // =========================================================================
    // Top-level main generation
    // =========================================================================

    /// Emit a C-compatible `main` function that runs all top-level statements.
    ///
    /// Signature: `i32 main()`
    pub(crate) fn generate_main(&mut self, stmts: &[Statement]) -> Result<(), String> {
        let i32_ty = self.type_mapper.i32_type();
        let main_type = i32_ty.fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_type, None);

        self.builder.reset_scope();

        let entry_bb = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry_bb);

        // Execute every top-level statement.
        let main_name = String::from("main");
        for stmt in stmts {
            self.generate_statement(stmt, main_fn, &main_name)?;
        }

        // Return 0 from main if the block has no terminator yet.
        if self
            .builder
            .get_insert_block()
            .and_then(|b: inkwell::basic_block::BasicBlock| b.get_terminator())
            .is_none()
        {
            self.builder
                .build_return(Some(&i32_ty.const_int(0, false)))
                .map_err(|_| "Failed to build return from main")?;
        }

        Ok(())
    }

    // =========================================================================
    // External declarations
    // =========================================================================

    pub(crate) fn declare_printf(&mut self) {
        if self.module.get_function("printf").is_none() {
            let printf_type = self.type_mapper.fn_printf();
            self.module
                .add_function("printf", printf_type, Some(Linkage::External));
        }
    }

    /// Declare the string runtime helpers we use (strlen, malloc, strcpy, strcat).
    /// All take `i8*` and return either `i64` or `i8*`.
    pub(crate) fn declare_string_runtime(&mut self) {
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
        // int snprintf(char *buf, size_t size, const char *fmt, ...)
        if self.module.get_function("snprintf").is_none() {
            let i32_type = self.type_mapper.i32_type();
            let ty = i32_type.fn_type(&[i8_ptr.into(), i64_type.into(), i8_ptr.into()], true);
            self.module
                .add_function("snprintf", ty, Some(Linkage::External));
        }
        // char *strchr(const char *s, int c)
        if self.module.get_function("strchr").is_none() {
            let i32_type = self.type_mapper.i32_type();
            let ty = i8_ptr.fn_type(&[i8_ptr.into(), i32_type.into()], false);
            self.module
                .add_function("strchr", ty, Some(Linkage::External));
        }
        // long long strtoll(const char *s, char **endptr, int base)
        if self.module.get_function("strtoll").is_none() {
            let ty = i64_type.fn_type(&[i8_ptr.into(), i8_ptr.into()], false);
            self.module
                .add_function("strtoll", ty, Some(Linkage::External));
        }
        // double strtod(const char *s, char **endptr)
        if self.module.get_function("strtod").is_none() {
            let f64_type = self.type_mapper.f64_type();
            let ty = f64_type.fn_type(&[i8_ptr.into(), i8_ptr.into()], false);
            self.module
                .add_function("strtod", ty, Some(Linkage::External));
        }
    }

    // =========================================================================
    // Function management
    // =========================================================================

    /// First-pass: register the function signature so it can be called.
    pub(crate) fn declare_function(&mut self, name: &str, params: &[String]) -> Result<(), String> {
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
    pub(crate) fn generate_function(
        &mut self,
        name: &str,
        params: &[String],
        body: &Block,
    ) -> Result<(), String> {
        let function = *self
            .functions
            .get(name)
            .ok_or_else(|| format!("Function '{}' not declared", name))?;

        self.builder.reset_scope();

        // Create the entry block and position the builder there.
        let entry_bb = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_bb);

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
                .get_insert_block()
                .and_then(|b| b.get_terminator())
                .is_none()
        {
            let ret_ty = self.fn_return_types.get(name).copied().unwrap_or("i64");
            let zero: BasicValueEnum = match ret_ty {
                "string" => self.type_mapper.ptr_type().const_null().into(),
                "f64" => self.type_mapper.f64_type().const_zero().into(),
                "bool" => self.type_mapper.i1_type().const_zero().into(),
                _ => self.type_mapper.i64_type().const_int(0, false).into(),
            };
            self.builder
                .build_return(Some(&zero))
                .map_err(|_| format!("Failed to build implicit return in '{}'", name))?;
        }

        Ok(())
    }

    // =========================================================================
    // Global variable generation
    // =========================================================================

    /// Generate a global with a constant-expression initializer.
    pub(crate) fn generate_global_var(
        &mut self,
        name: &str,
        init: &Expression,
    ) -> Result<(), String> {
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
        if matches!(init, Expression::Bool(_, _)) {
            self.bool_globals.insert(name.to_string());
        } else {
            self.bool_globals.remove(name);
        }
        Ok(())
    }

    /// Generate an uninitialised (zero) global variable.
    pub(crate) fn generate_global_var_typed(
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
}
