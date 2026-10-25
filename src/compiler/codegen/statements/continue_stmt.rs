// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::compiler::codegen::generator::CodeGenerator;

/// Emit code for continue statement
pub fn emit_continue<'ctx>(gen: &mut CodeGenerator<'ctx>) -> Result<bool, String> {
    let cond_bb = gen
        .loop_stack
        .last()
        .map(|(cond, _)| *cond)
        .ok_or_else(|| "`continue` used outside of a loop".to_string())?;
    gen.builder
        .build_unconditional_branch(cond_bb)
        .map_err(|_| "Failed to build continue branch")?;
    Ok(true)
}
