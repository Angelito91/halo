// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::cli::args::Args;
use crate::cli::helpers::{evaluate, file_stem, parse_tokens, read_source, run_command, tokenize};
use crate::compiler::Compilation;
use crate::diagnostics::{colors, CompilationStats, Diagnostic};
use std::fs;
use std::process;

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
    let ast_node_count = program.items.len();
    let mut function_count = 0;
    let mut variable_count = 0;

    for item in &program.items {
        match item {
            crate::parser::ast::TopLevel::Function { .. } => function_count += 1,
            crate::parser::ast::TopLevel::GlobalVar { .. } => variable_count += 1,
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
            ast_nodes: ast_node_count,
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

pub fn link_with_clang(ll_path: &str, bin_path: &str, args: &Args) -> bool {
    if args.verbose {
        eprintln!("🔨 Linking with clang…");
    }

    let mut cmd = process::Command::new("clang");
    cmd.args([ll_path, args.opt_level.clang_flag(), "-o", bin_path, "-lm"]);

    if !run_command(cmd, "clang", args.verbose) {
        eprintln!("❌ clang failed. Is clang installed?");
        return false;
    }

    if args.verbose {
        eprintln!("✅ Linked via clang\n");
    }
    true
}

/// Link using `llc` (IR → assembly) then `cc` (assembly → binary).
/// This is the former `haloc` toolchain, now available as `halo build --toolchain llc`.
pub fn link_with_llc(ll_path: &str, bin_path: &str, args: &Args) -> bool {
    let asm_path = format!("{bin_path}.s");

    if args.verbose {
        eprintln!("🔨 Compiling IR to assembly (llc)…");
    }

    let mut llc = process::Command::new("llc");
    llc.arg(ll_path).arg("-o").arg(&asm_path);
    if args.opt_level != crate::compiler::OptLevel::O0 {
        llc.arg(args.opt_level.clang_flag());
    }

    if !run_command(llc, "llc", args.verbose) {
        eprintln!("❌ llc failed. Is llc installed?");
        return false;
    }

    if args.verbose {
        eprintln!("✅ Assembly: {asm_path}\n");
        eprintln!("🔗 Linking with cc…");
    }

    let mut cc = process::Command::new("cc");
    cc.arg(&asm_path).arg("-o").arg(bin_path);

    if !run_command(cc, "cc", args.verbose) {
        eprintln!("❌ cc failed. Is a C compiler installed?");
        let _ = fs::remove_file(&asm_path);
        return false;
    }

    let _ = fs::remove_file(&asm_path);

    if args.verbose {
        eprintln!("✅ Linked via llc + cc\n");
    }
    true
}

pub fn cmd_check(args: &Args) -> i32 {
    let source = match read_source(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {e}");
            return 1;
        }
    };

    let tokens = tokenize(&source);

    match parse_tokens(tokens, &source, &args.file) {
        Ok(program) => {
            println!(
                "{}  {} — {} {}",
                colors::success("✅"),
                colors::highlight(&args.file),
                colors::success("OK"),
                colors::muted(&format!("({} top-level item(s))", program.items.len()))
            );
            0
        }
        Err(diagnostics) => {
            eprintln!(
                "{}  {} — {} {}",
                colors::error("❌"),
                colors::highlight(&args.file),
                colors::error(&format!("{} error(s)", diagnostics.len())),
                colors::muted(":")
            );
            eprintln!();
            for diag in &diagnostics {
                eprintln!("{}", diag.to_string_formatted());
            }
            1
        }
    }
}

pub fn cmd_tokens(args: &Args) -> i32 {
    let source = match read_source(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {e}");
            return 1;
        }
    };

    let tokens = tokenize(&source);

    println!("╔════════════════════════════════════════╗");
    println!("║       📋 Token Stream — {:<14} ║", file_stem(&args.file));
    println!("╚════════════════════════════════════════╝");
    println!();

    for (i, tok) in tokens.iter().enumerate() {
        use crate::lexer::TokenKind;
        match tok.kind {
            TokenKind::Eof => {
                println!("{i:4}  {}", colors::muted("EOF"));
            }
            TokenKind::Newline => {
                println!(
                    "{:4}  {:<20}          {}",
                    i,
                    colors::highlight("NEWLINE"),
                    colors::muted(&format!("{}:{}", tok.position.line, tok.position.column))
                );
            }
            _ => {
                println!(
                    "{i:4}  {:<20} {:>10}    {}",
                    colors::info(&format!("{:?}", tok.kind)),
                    colors::success(&format!("'{}'", tok.lexeme)),
                    colors::muted(&format!("{}:{}", tok.position.line, tok.position.column))
                );
            }
        }
    }

    println!();
    println!("{} token(s)", colors::highlight(&tokens.len().to_string()));
    0
}

pub fn cmd_ast(args: &Args) -> i32 {
    let source = match read_source(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {e}");
            return 1;
        }
    };

    let tokens = tokenize(&source);

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

    println!("╔════════════════════════════════════════╗");
    println!(
        "║   🌳 Abstract Syntax Tree — {:<11} ║",
        file_stem(&args.file)
    );
    println!("╚════════════════════════════════════════╝");
    println!();

    for (i, item) in program.items.iter().enumerate() {
        println!(
            "{}  {}",
            colors::highlight(&format!("[{i}]")),
            colors::info(toplevel_kind(item))
        );
        match item {
            crate::parser::ast::TopLevel::Function {
                name, params, body, ..
            } => {
                println!(
                    "    {}  : {}",
                    colors::success("name"),
                    colors::highlight(name)
                );
                if params.is_empty() {
                    println!(
                        "    {}  : {}",
                        colors::success("params"),
                        colors::muted("(none)")
                    );
                } else {
                    println!(
                        "    {}  : {}",
                        colors::success("params"),
                        colors::highlight(&params.join(", "))
                    );
                }
                println!(
                    "    {}  : {} {}",
                    colors::success("body"),
                    colors::highlight(&body.stmts.len().to_string()),
                    colors::muted("statement(s)")
                );
                for (j, stmt) in body.stmts.iter().enumerate() {
                    println!(
                        "      {}  {}",
                        colors::muted(&format!("[{j}]")),
                        colors::success(&stmt.to_string())
                    );
                }
            }
            crate::parser::ast::TopLevel::GlobalVar { name, init, .. } => {
                println!(
                    "    {}  : {}",
                    colors::success("name"),
                    colors::highlight(name)
                );
                match init {
                    Some(expr) => println!(
                        "    {}  : {}",
                        colors::success("init"),
                        colors::highlight(&expr.to_string())
                    ),
                    None => println!(
                        "    {}  : {}",
                        colors::success("init"),
                        colors::muted("(none)")
                    ),
                }
            }
            crate::parser::ast::TopLevel::Stmt { stmt, .. } => {
                println!(
                    "    {}  : {}",
                    colors::success("stmt"),
                    colors::highlight(&stmt.to_string())
                );
            }
        }
        println!();
    }

    println!(
        "{} top-level item(s)",
        colors::highlight(&program.items.len().to_string())
    );
    0
}

pub fn toplevel_kind(item: &crate::parser::ast::TopLevel) -> &'static str {
    match item {
        crate::parser::ast::TopLevel::Function { .. } => "Function",
        crate::parser::ast::TopLevel::GlobalVar { .. } => "GlobalVar",
        crate::parser::ast::TopLevel::Stmt { .. } => "Stmt",
    }
}

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
