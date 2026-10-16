// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Expression code generation.
//!
//! This module is responsible for generating LLVM IR for all expression types:
//! - Literals (numbers, floats, booleans, strings)
//! - Variable references and assignments
//! - Binary operations (arithmetic, comparison, logical)
//! - Unary operations (negation, NOT)
//! - Function calls
//! - Array literals and indexing
//! - Method calls
//!
//! Each expression type is handled by dedicated submodules for clarity and maintainability.

pub mod arrays;
pub mod binary_ops;
pub mod function_call;
pub mod literals;
pub mod method_call;
pub mod unary_ops;
pub mod variables;

use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;

/// Main entry point for expression code generation.
/// Dispatches to appropriate submodule based on expression type.
pub fn emit_expression<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    expr: &Expression,
) -> Result<BasicValueEnum<'ctx>, String> {
    match expr {
        // Literals
        Expression::Number(n, _) => Ok(literals::emit_number(&gen.type_mapper, *n)),
        Expression::Float(f, _) => Ok(literals::emit_float(&gen.type_mapper, *f)),
        Expression::Bool(b, _) => Ok(literals::emit_bool(&gen.type_mapper, *b)),
        Expression::StringLiteral(s, _) => literals::emit_string(gen, s),

        // Variables
        Expression::Var(name, _) => variables::emit_var_ref(&gen.builder, &gen.globals, name),
        Expression::Assign { name, value, .. } => variables::emit_var_assign(gen, name, value),

        // Unary operations
        Expression::Unary { operator, expr, .. } => unary_ops::emit_unary(gen, operator, expr),

        // Binary operations
        Expression::Binary {
            left, op, right, ..
        } => binary_ops::emit_binop(gen, left, op, right),

        // Function calls
        Expression::Call { name, args, .. } => function_call::emit_function_call(gen, name, args),

        // Arrays
        Expression::ArrayLiteral { elements, .. } => arrays::emit_array_literal(gen, elements),
        Expression::Index { array, index, .. } => arrays::emit_array_index(gen, array, index),
        Expression::IndexAssign { .. } => {
            Err("Array index assignment is not yet supported in compiled mode".to_string())
        }

        // Method calls
        Expression::MethodCall {
            object,
            method,
            args,
            ..
        } => method_call::emit_method_call(gen, object, method, args),
    }
}
