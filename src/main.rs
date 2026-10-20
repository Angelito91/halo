// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use std::process;

// ── Re-exports ────────────────────────────────────────────────────────────────
//
// The binary is a thin wrapper over the library: every module lives in
// `halo::*` exactly once. Declaring them here as private `mod`s would build a
// second, unreachable copy of the tree inside the binary crate, and every
// public item the CLI does not use would be reported as dead code.

use halo::cli::{cmd_ast, cmd_build, cmd_check, cmd_llvm, cmd_run, cmd_tokens};
use halo::cli::{parse_args, Subcommand};

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
