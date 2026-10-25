// The Halo Programming Language CLI Module
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! CLI module containing argument parsing, command handlers, and UI utilities.
//!
//! This module organizes all command-line interface functionality:
//! - `args`: Argument parsing and CLI types
//! - `run`, `check`, `tokens`, `ast`: one module per subcommand
//! - `helpers`: UI output, source loading and pipeline helpers

pub mod args;
pub mod ast;
pub mod check;
pub mod helpers;
pub mod run;
pub mod tokens;

pub use args::{parse_args, parse_args_from, subcommand_name, Args, Subcommand};
pub use {ast::cmd_ast, check::cmd_check, run::cmd_run, tokens::cmd_tokens};
