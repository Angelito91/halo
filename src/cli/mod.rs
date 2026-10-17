// The Halo Programming Language CLI Module
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! CLI module containing argument parsing, command handlers, and UI utilities.
//!
//! This module organizes all command-line interface functionality:
//! - `args`: Argument parsing and CLI types
//! - `run`, `build`, `check`, `tokens`, `ast`, `llvm`: one module per subcommand
//! - `helpers`: UI output, linking and helper functions

pub mod args;
pub mod ast;
pub mod build;
pub mod check;
pub mod helpers;
pub mod llvm;
pub mod run;
pub mod tokens;

pub use args::{parse_args, Subcommand};
#[allow(unused_imports)]
pub use {
    ast::cmd_ast, build::cmd_build, check::cmd_check, llvm::cmd_llvm, run::cmd_run,
    tokens::cmd_tokens,
};
