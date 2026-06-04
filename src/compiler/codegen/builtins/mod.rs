// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Built-in functions.
//!
//! This module is responsible for code generation for built-in functions:
//! - I/O: `print()`
//! - Type operations: `type()`, `str()`, `int()`, `float()`
//! - Array operations: `len()`
//! - Math operations: `abs()`
//! - String operations: `str()` (concatenation, conversion)
//!
//! Each category is organized in its own submodule for clarity and maintainability.

pub mod io;

use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;

/// Main entry point for built-in function code generation.
/// Returns Some(value) if the name is a built-in function, None otherwise.
pub fn emit_builtin<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    name: &str,
    args: &[Expression],
) -> Result<Option<BasicValueEnum<'ctx>>, String> {
    match name {
        "print" | "println" => io::emit_print(gen, args, name == "println").map(Some),
        _ => Ok(None),
    }
}
