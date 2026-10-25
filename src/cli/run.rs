// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::cli::args::Args;
use crate::cli::helpers::{evaluate, parse_tokens, read_source, tokenize};
use crate::diagnostics::Diagnostic;

pub fn cmd_run(args: &Args) -> i32 {
    let source = match read_source(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {e}");
            return 1;
        }
    };

    if args.verbose {
        println!("📂 {}", args.file);
        println!("🔍 Tokenising…");
    }

    let tokens = tokenize(&source);

    if args.verbose {
        println!("✅ {} tokens", tokens.len());
        println!("📝 Parsing…");
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

    if args.verbose {
        println!("✅ {} top-level item(s)", program.items.len());
        println!("▶️  Interpreting…");
        println!("─────────────────────────────────────────");
    }

    match evaluate(&program) {
        Ok(_) => {
            if args.verbose {
                println!("─────────────────────────────────────────");
                println!("✅ Finished successfully");
            }
            0
        }
        Err(e) => {
            eprintln!();
            let diag = Diagnostic::error(format!("Runtime error: {}", e));
            eprintln!("{}", diag.to_string_formatted());
            1
        }
    }
}
