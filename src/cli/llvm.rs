// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::cli::args::Args;
use crate::cli::helpers::{file_stem, parse_tokens, read_source, tokenize};
use crate::compiler::Compilation;

pub fn cmd_llvm(args: &Args) -> i32 {
    let source = match read_source(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {e}");
            return 1;
        }
    };

    if args.verbose {
        eprintln!("🔍 Tokenising…");
    }
    let tokens = tokenize(&source);

    if args.verbose {
        eprintln!("📝 Parsing…");
    }
    let program = match parse_tokens(tokens, &source, &args.file) {
        Ok(p) => p,
        Err(diagnostics) => {
            eprintln!();
            for diag in &diagnostics {
                eprintln!("{}", diag.to_string_formatted());
            }
            return 1;
        }
    };

    let stem = file_stem(&args.file);
    let ll_path = args.output.clone().unwrap_or_else(|| format!("{stem}.ll"));

    if args.verbose {
        eprintln!("⚙️  Generating LLVM IR…");
    }
    let mut comp = Compilation::new(stem);
    if let Err(e) = comp.codegen().compile(&program) {
        eprintln!("❌ Code generation error: {e}");
        return 1;
    }

    if args.opt_level != crate::compiler::OptLevel::O0 {
        if args.verbose {
            eprintln!("⚡ Optimising (O{})…", args.opt_level.as_u32());
        }
        if let Err(e) = comp.optimise(args.opt_level) {
            eprintln!("❌ Optimisation error: {e}");
            return 1;
        }
    }

    if let Err(e) = comp.emit_llvm(&ll_path) {
        eprintln!("❌ Failed to write '{ll_path}': {e}");
        return 1;
    }

    println!("✅ LLVM IR → {ll_path}");
    0
}
