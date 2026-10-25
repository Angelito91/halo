// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::cli::args::Args;
use crate::cli::helpers::{
    file_stem, link_with_clang, link_with_llc, parse_tokens, read_source, tokenize,
};
use crate::compiler::Compilation;
use crate::diagnostics::CompilationStats;
use crate::parser::ast::TopLevel;
use std::fs;
use std::process;

pub fn cmd_build(args: &Args) -> i32 {
    let source = match read_source(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {e}");
            return 1;
        }
    };

    if args.verbose {
        eprintln!("╔════════════════════════════════════════╗");
        eprintln!("║     🏗️  Halo Build Pipeline            ║");
        eprintln!("╚════════════════════════════════════════╝");
        eprintln!();
        eprintln!("📂 Source    : {}", args.file);
        eprintln!("🔧 Toolchain : {}", args.toolchain.name());
    }

    if args.verbose {
        eprintln!("🔍 Tokenising…");
    }
    let tokens = tokenize(&source);
    let token_count = tokens.len();
    if args.verbose {
        eprintln!("✅ {} tokens\n", token_count);
    }

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

    let item_count = program.items.len();
    let mut function_count = 0;
    let mut variable_count = 0;

    for item in &program.items {
        match item {
            TopLevel::Function { .. } => function_count += 1,
            TopLevel::GlobalVar { .. } => variable_count += 1,
            _ => {}
        }
    }

    if args.verbose {
        eprintln!("✅ {} top-level item(s)\n", item_count);
    }

    let stem = file_stem(&args.file);
    let bin_path = args.output.as_deref().unwrap_or(stem).to_string();
    let ll_path = format!("{stem}.ll");

    if args.verbose {
        eprintln!("⚙️  Generating LLVM IR…");
    }
    let mut comp = Compilation::new(stem);
    if let Err(e) = comp.codegen().compile(&program) {
        eprintln!("❌ Code generation error: {e}");
        return 1;
    }
    if args.verbose {
        eprintln!("✅ LLVM IR ready\n");
    }

    if args.opt_level != crate::compiler::OptLevel::O0 {
        if args.verbose {
            eprintln!("⚡ Optimising (O{})…", args.opt_level.as_u32());
        }
        if let Err(e) = comp.optimise(args.opt_level) {
            eprintln!("❌ Optimisation error: {e}");
            return 1;
        }
        if args.verbose {
            eprintln!("✅ Optimisation done\n");
        }
    }

    if let Err(e) = comp.emit_llvm(&ll_path) {
        eprintln!("❌ Failed to write LLVM IR: {e}");
        return 1;
    }
    if args.verbose && args.emit_llvm {
        eprintln!("💾 LLVM IR saved to: {ll_path}\n");
    }

    let link_ok = match args.toolchain {
        crate::cli::args::Toolchain::Clang => link_with_clang(&ll_path, &bin_path, args),
        crate::cli::args::Toolchain::Llc => link_with_llc(&ll_path, &bin_path, args),
    };

    if !args.emit_llvm {
        let _ = fs::remove_file(&ll_path);
    }

    if !link_ok {
        return 1;
    }

    if args.verbose {
        eprintln!("✅ Binary → {bin_path}\n");

        let stats = CompilationStats {
            tokens: token_count,
            top_level_items: item_count,
            functions: function_count,
            variables: variable_count,
            lines_of_code: source.lines().count(),
        };
        stats.print_summary();

        eprintln!("╔════════════════════════════════════════╗");
        eprintln!("║      ✅ Build successful!              ║");
        eprintln!("╚════════════════════════════════════════╝");
    } else {
        println!("✅ {bin_path}");
    }

    if args.run_after_build {
        if args.verbose {
            eprintln!("\n🚀 Running ./{bin_path}");
            eprintln!("─────────────────────────────────────────");
        }

        let status = process::Command::new(format!("./{bin_path}"))
            .status()
            .unwrap_or_else(|e| {
                eprintln!("❌ Failed to run '{bin_path}': {e}");
                process::exit(1);
            });

        if args.output.is_none() {
            let _ = fs::remove_file(&bin_path);
        }

        if args.verbose {
            eprintln!("─────────────────────────────────────────");
            eprintln!("✅ Exited with code {}", status.code().unwrap_or(0));
        }

        return status.code().unwrap_or(0);
    }

    0
}
