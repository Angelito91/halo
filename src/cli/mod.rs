// The Halo Programming Language CLI Module
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! CLI module containing argument parsing, command handlers, and UI utilities.
//!
//! This module organizes all command-line interface functionality:
//! - `args`: Argument parsing and CLI types
//! - `commands`: Subcommand handlers (run, build, check, etc.)
//! - `helpers`: UI output and helper functions

pub mod args;
pub mod commands;
pub mod helpers;

pub use args::{parse_args, Subcommand};
#[allow(unused_imports)]
pub use commands::{cmd_ast, cmd_build, cmd_check, cmd_llvm, cmd_run, cmd_tokens};
