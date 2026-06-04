// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Statement code generation.
//!
//! This module is responsible for generating LLVM IR for all statement types:
//! - Variable declarations
//! - Expression statements
//! - If/else conditionals
//! - While loops
//! - Return statements
//! - Break/continue (for loops)
//!
//! Each statement type is handled by dedicated submodules for clarity and maintainability.

pub mod break_stmt;
pub mod continue_stmt;
pub mod expr_stmt;
pub mod if_stmt;
pub mod return_stmt;
pub mod var_decl;
pub mod while_loop;

use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Statement;
use inkwell::values::FunctionValue;

/// Main entry point for statement code generation.
/// Dispatches to appropriate submodule based on statement type.
/// Returns whether the statement terminates (prevents unreachable code after return/break).
pub fn emit_statement<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    stmt: &Statement,
    function: FunctionValue<'ctx>,
) -> Result<bool, String> {
    match stmt {
        Statement::Expr(expr) => expr_stmt::emit_expr_stmt(gen, expr),
        Statement::VarDecl { name, init, .. } => var_decl::emit_var_decl(gen, name, init, function),
        Statement::If {
            cond,
            then_branch,
            else_if_branches,
            else_branch,
            ..
        } => if_stmt::emit_if_stmt(
            gen,
            function,
            cond,
            then_branch,
            else_if_branches,
            else_branch,
        ),
        Statement::While { cond, body, .. } => while_loop::emit_while_loop(gen, function, cond, body),
        Statement::Break { .. } => break_stmt::emit_break(gen),
        Statement::Continue { .. } => continue_stmt::emit_continue(gen),
        Statement::Return { value, .. } => return_stmt::emit_return(gen, value),
    }
}
