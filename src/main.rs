// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use std::process;

// ── Modules ───────────────────────────────────────────────────────────────────

mod compiler;
mod diagnostics;
mod interpreter;
mod lexer;
mod parser;
mod utils;

mod cli;

// ── Re-exports ────────────────────────────────────────────────────────────────

use cli::{cmd_ast, cmd_build, cmd_check, cmd_llvm, cmd_run, cmd_tokens};
use cli::{parse_args, Subcommand};

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("❌ {e}");
            process::exit(1);
        }
    };

    let exit_code = match args.subcommand {
        Subcommand::Run => cmd_run(&args),
        Subcommand::Build => cmd_build(&args),
        Subcommand::Check => cmd_check(&args),
        Subcommand::Tokens => cmd_tokens(&args),
        Subcommand::Ast => cmd_ast(&args),
        Subcommand::Llvm => cmd_llvm(&args),
    };

    process::exit(exit_code);
}
