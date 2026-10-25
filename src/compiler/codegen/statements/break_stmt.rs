// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::generator::CodeGenerator;

/// Emit code for break statement
pub fn emit_break<'ctx>(gen: &mut CodeGenerator<'ctx>) -> Result<bool, String> {
    let exit_bb = gen
        .loop_stack
        .last()
        .map(|(_, exit)| *exit)
        .ok_or_else(|| "`break` used outside of a loop".to_string())?;
    gen.builder
        .build_unconditional_branch(exit_bb)
        .map_err(|_| "Failed to build break branch")?;
    Ok(true)
}
