// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

mod compiler;
mod interpreter;
mod lexer;
mod parser;

use compiler::{Compilation, OptLevel};
use interpreter::Evaluator;
use lexer::{Lexer, TokenKind};
use parser::Parser;

// ── CLI argument types ────────────────────────────────────────────────────────

/// Parsed command-line arguments.
struct Args {
    /// Path to the `.halo` source file.
    file_path: String,
    /// `--ast` / `-a`: print the AST before running.
    show_ast: bool,
    /// `--tokens` / `-t`: print the token stream before running.
    show_tokens: bool,
    /// `--verbose` / `-v`: enable detailed progress output.
    verbose: bool,
    /// `--compile` / `-c`: compile to a native binary via clang.
    compile: bool,
    /// `--emit-llvm`: write LLVM IR next to the source file.
    emit_llvm: bool,
    /// `--run` / `-r`: compile to a temp binary and execute immediately.
    run: bool,
    /// `-o <path>`: output path for `--compile` / `--run`.
    output: Option<String>,
    /// `-O<N>` / `--opt-level <N>`: LLVM optimisation level (default: 2).
    opt_level: OptLevel,
}

/// Parse `std::env::args()` into an [`Args`] struct.
///
/// Returns `Err` with a human-readable message on any invalid input.
fn parse_args() -> Result<Args, String> {
    let raw: Vec<String> = env::args().collect();

    if raw.len() < 2 {
        return Err("No source file specified. Run 'halo --help' for usage.".to_string());
    }

    let mut file_path = String::new();
    let mut show_ast = false;
    let mut show_tokens = false;
    let mut verbose = false;
    let mut compile = false;
    let mut emit_llvm = false;
    let mut run = false;
    let mut output: Option<String> = None;
    let mut opt_level = OptLevel::O2;

    let mut i = 1;
    while i < raw.len() {
        match raw[i].as_str() {
            "--ast" | "-a" => show_ast = true,
            "--tokens" | "-t" => show_tokens = true,
            "--verbose" | "-v" => verbose = true,
            "--compile" | "-c" => compile = true,
            "--emit-llvm" => emit_llvm = true,
            "--run" | "-r" => run = true,
            "-o" => {
                i += 1;
                if i >= raw.len() {
                    return Err("Expected an output path after '-o'.".to_string());
                }
                output = Some(raw[i].clone());
            }
            "--opt-level" | "-O" => {
                i += 1;
                if i >= raw.len() {
                    return Err("Expected a level (0-3) after '--opt-level'.".to_string());
                }
                opt_level = parse_opt_level(&raw[i])?;
            }
            // Compact forms: -O0  -O1  -O2  -O3
            "-O0" => opt_level = OptLevel::O0,
            "-O1" => opt_level = OptLevel::O1,
            "-O2" => opt_level = OptLevel::O2,
            "-O3" => opt_level = OptLevel::O3,
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            "--version" | "-V" => {
                println!("Halo Programming Language v0.2.0");
                process::exit(0);
            }
            arg if !arg.starts_with('-') => {
                file_path = arg.to_string();
            }
            arg => {
                return Err(format!(
                    "Unknown option: '{arg}'. Run 'halo --help' for usage."
                ));
            }
        }
        i += 1;
    }

    if file_path.is_empty() {
        return Err("No source file specified. Run 'halo --help' for usage.".to_string());
    }

    Ok(Args {
        file_path,
        show_ast,
        show_tokens,
        verbose,
        compile,
        emit_llvm,
        run,
        output,
        opt_level,
    })
}

/// Convert a `"0"`–`"3"` string to an [`OptLevel`].
fn parse_opt_level(s: &str) -> Result<OptLevel, String> {
    match s {
        "0" => Ok(OptLevel::O0),
        "1" => Ok(OptLevel::O1),
        "2" => Ok(OptLevel::O2),
        "3" => Ok(OptLevel::O3),
        other => Err(format!(
            "Invalid optimisation level '{other}'. Expected 0, 1, 2, or 3."
        )),
    }
}

/// Prints help message
fn print_help() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║    🌟 Halo Programming Language v0.2.0 🌟     ║");
    println!("╚════════════════════════════════════════════════╝\n");

    println!("USAGE:");
    println!("    halo [OPTIONS] <FILE>\n");

    println!("OPTIONS:");
    println!("    -a, --ast          Display the Abstract Syntax Tree");
    println!("    -t, --tokens       Display the tokens from lexer");
    println!("    -v, --verbose      Enable verbose output");
    println!("    -c, --compile      Compile to a native binary via clang");
    println!("        --emit-llvm    Write LLVM IR (.ll) next to the source file");
    println!("    -r, --run          Compile and run immediately (uses a temp binary)");
    println!("    -o <path>          Output path for --compile or --run");
    println!("    -O, --opt-level N  LLVM optimisation level: 0=none 1 2 3 (default: 2)");
    println!("    -h, --help         Print this help message");
    println!("    -V, --version      Print version information\n");

    println!("EXAMPLES:");
    println!("    halo script.halo                   Interpret script.halo");
    println!("    halo --compile script.halo         Compile to ./script");
    println!("    halo --compile -o out script.halo  Compile to ./out");
    println!("    halo --emit-llvm script.halo       Emit script.ll");
    println!("    halo --run script.halo             Compile and run immediately");
    println!("    halo --run -O3 script.halo         Compile with max optimisation and run");
    println!("    halo --ast script.halo             Show AST then interpret\n");

    println!("FILE EXTENSION:");
    println!("    .halo            Halo source code file\n");

    println!("DOCUMENTATION:");
    println!("    For language documentation, see SYNTAX.md");
}

// ── Pipeline helpers ──────────────────────────────────────────────────────────

/// Read the entire contents of `path` into a `String`.
fn read_source_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Cannot read '{path}': {e}"))
}

/// Read all of stdin into a `String`.
#[allow(dead_code)]
fn read_stdin() -> Result<String, String> {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .map_err(|e| format!("Error reading stdin: {e}"))?;
    Ok(buf)
}

/// Lex `source` into a token stream, including the terminal [`TokenKind::Eof`].
fn tokenize(source: &str) -> Vec<lexer::Token> {
    let mut lexer = Lexer::new(source.to_string());
    // Collect tokens until (and including) Eof, then stop.
    std::iter::from_fn(move || {
        let tok = lexer.next_token();
        let done = tok.kind == TokenKind::Eof;
        Some((tok, done))
    })
    .scan(false, |finished, (tok, is_eof)| {
        if *finished {
            return None;
        }
        *finished = is_eof;
        Some(tok)
    })
    .collect()
}

/// Parse a token stream into a [`Program`] AST.
fn parse(tokens: Vec<lexer::Token>) -> Result<parser::ast::Program, Vec<String>> {
    Parser::new(tokens).parse()
}

/// Evaluate a [`Program`] with the tree-walking interpreter.
fn evaluate(program: &parser::ast::Program) -> Result<interpreter::Value, String> {
    Evaluator::new().eval_program(program)
}

/// Compile the program to LLVM IR, then to a native binary via clang.
/// Returns the path to the produced binary.
fn compile_program(
    program: &parser::ast::Program,
    source_path: &str,
    output_path: Option<&str>,
    emit_llvm: bool,
    opt_level: OptLevel,
    verbose: bool,
) -> Result<String, String> {
    // Derive default output paths from the source file name
    let stem = std::path::Path::new(source_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let ll_path = format!("{}.ll", stem);
    let bin_path = output_path
        .map(|p| p.to_string())
        .unwrap_or_else(|| stem.to_string());

    // --- Code generation ---
    if verbose {
        println!("⚙️  Generating LLVM IR...");
    }

    let mut comp = Compilation::new(stem);
    comp.codegen()
        .compile(program)
        .map_err(|e| format!("Code generation error: {}", e))?;

    // Run LLVM optimisation passes before emitting.
    if opt_level != OptLevel::O0 {
        if verbose {
            println!(
                "⚡ Running LLVM optimisation passes (O{})...",
                opt_level.as_u32()
            );
        }
        comp.optimise(opt_level)
            .map_err(|e| format!("Optimisation error: {}", e))?;
        if verbose {
            println!("✅ Optimisation done\n");
        }
    }

    if verbose {
        println!("✅ LLVM IR generated\n");
        println!("📄 LLVM IR:");
        println!("─────────────────────────────────────────");
        comp.print_ir();
        println!("─────────────────────────────────────────\n");
    }

    // --- Emit .ll file ---
    if emit_llvm || verbose {
        comp.emit_llvm(&ll_path)
            .map_err(|e| format!("Failed to emit LLVM IR: {}", e))?;
        if verbose || emit_llvm {
            println!("📝 LLVM IR written to: {}", ll_path);
        }
    }

    // --- Invoke clang to produce a native binary ---
    if verbose {
        println!("🔨 Compiling to native binary with clang...");
        println!("   clang {} -o {}", ll_path, bin_path);
    }

    // We always write the .ll first (even if --emit-llvm wasn't requested)
    // so that clang has something to compile.
    if !emit_llvm && !verbose {
        comp.emit_llvm(&ll_path)
            .map_err(|e| format!("Failed to write temporary IR: {}", e))?;
    }

    let clang_status = std::process::Command::new("clang")
        .args([&ll_path, opt_level.clang_flag(), "-o", &bin_path, "-lm"])
        .status()
        .map_err(|e| format!("Failed to invoke clang: {}. Is clang installed?", e))?;

    if !clang_status.success() {
        return Err(format!(
            "clang exited with status {}",
            clang_status.code().unwrap_or(-1)
        ));
    }

    // Remove the temporary .ll file unless the user explicitly asked for it
    if !emit_llvm {
        let _ = fs::remove_file(&ll_path);
    }

    if verbose {
        println!("✅ Binary written to: {}\n", bin_path);
    }

    Ok(bin_path)
}

// ── Display helpers ───────────────────────────────────────────────────────────

/// Print the token stream produced by the lexer.
fn display_tokens(tokens: &[lexer::Token]) {
    println!("╔════════════════════════════════════════╗");
    println!("║         📋 TOKENS (Lexer Output)       ║");
    println!("╚════════════════════════════════════════╝\n");

    for (i, tok) in tokens.iter().enumerate() {
        match tok.kind {
            TokenKind::Eof => println!("{i:3}. [EOF]"),
            TokenKind::Newline => println!(
                "{i:3}. [NEWLINE] at {}:{}",
                tok.position.line, tok.position.column
            ),
            _ => println!(
                "{i:3}. {:?} '{}' at {}:{}",
                tok.kind, tok.lexeme, tok.position.line, tok.position.column
            ),
        }
    }
    println!();
}

/// Print the top-level items of a parsed [`Program`].
fn display_ast(program: &parser::ast::Program) {
    println!("╔════════════════════════════════════════╗");
    println!("║      🌳 AST (Abstract Syntax Tree)     ║");
    println!("╚════════════════════════════════════════╝\n");

    for (i, item) in program.items.iter().enumerate() {
        println!("Item {}:", i);
        match item {
            parser::ast::TopLevel::Function {
                name, params, body, ..
            } => {
                println!("  Function: {}", name);
                println!("  Parameters: {}", params.join(", "));
                println!("  Body: {} statements", body.stmts.len());
                for (j, stmt) in body.stmts.iter().enumerate() {
                    println!("    Statement {}: {}", j, stmt);
                }
            }
            parser::ast::TopLevel::GlobalVar { name, init, .. } => {
                println!("  GlobalVar: {}", name);
                if let Some(expr) = init {
                    println!("  Initializer: {}", expr);
                } else {
                    println!("  Initializer: none");
                }
            }
            parser::ast::TopLevel::Stmt { stmt, .. } => {
                println!("  TopLevelStmt: {}", stmt);
            }
        }
        println!();
    }
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("❌ {e}");
            process::exit(1);
        }
    };

    // Print header
    if args.verbose {
        println!("╔════════════════════════════════════════╗");
        println!("║    🌟 Halo Interpreter v0.2.0 🌟     ║");
        println!("╚════════════════════════════════════════╝\n");
    }

    if args.verbose {
        println!("📂 Reading file: {}", args.file_path);
    }

    let source = match read_source_file(&args.file_path) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("❌ {e}");
            process::exit(1);
        }
    };

    if args.verbose {
        println!("✅ Read {} bytes\n", source.len());
        println!("🔍 Tokenising…");
    }

    // Lexing is infallible — the lexer never returns an error.
    let tokens = tokenize(&source);

    if args.verbose {
        println!("✅ {} tokens\n", tokens.len());
    }

    if args.show_tokens {
        display_tokens(&tokens);
    }

    if args.verbose {
        println!("📝 Parsing…");
    }

    let program = match parse(tokens) {
        Ok(prog) => prog,
        Err(errors) => {
            eprintln!("❌ Parse error(s):");
            for err in &errors {
                eprintln!("   {err}");
            }
            process::exit(1);
        }
    };

    if args.verbose {
        println!("✅ Parsed {} top-level item(s)\n", program.items.len());
    }

    if args.show_ast {
        display_ast(&program);
    }

    // ── Compile / emit-LLVM / run mode ───────────────────────────────────────
    if args.compile || args.emit_llvm || args.run {
        if args.verbose {
            println!("🏗️  Compiling via LLVM…");
            println!("─────────────────────────────────────────\n");
        }

        let bin_path = match compile_program(
            &program,
            &args.file_path,
            args.output.as_deref(),
            args.emit_llvm,
            args.opt_level,
            args.verbose,
        ) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("❌ Compilation error: {e}");
                process::exit(1);
            }
        };

        if args.run {
            if args.verbose {
                println!("🚀 Running: ./{bin_path}");
                println!("─────────────────────────────────────────\n");
            }

            let exit_status = std::process::Command::new(format!("./{bin_path}"))
                .status()
                .unwrap_or_else(|e| {
                    eprintln!("❌ Failed to execute '{bin_path}': {e}");
                    process::exit(1);
                });

            // Remove the temporary binary when no explicit output path was given.
            if args.output.is_none() {
                let _ = fs::remove_file(&bin_path);
            }

            if args.verbose {
                println!("\n─────────────────────────────────────────");
                println!("✅ Exited with code {}", exit_status.code().unwrap_or(0));
            }

            process::exit(exit_status.code().unwrap_or(0));
        }

        if !args.verbose {
            println!("✅ Compiled successfully → {bin_path}");
        }

        return;
    }

    // ── Interpreter mode (default) ────────────────────────────────────────────
    if args.verbose {
        println!("▶️  Interpreting…");
        println!("─────────────────────────────────────────\n");
    }

    match evaluate(&program) {
        Ok(_) => {
            if args.verbose {
                println!("\n─────────────────────────────────────────");
                println!("✅ Finished successfully");
            }
        }
        Err(e) => {
            eprintln!("\n❌ Runtime error: {e}");
            process::exit(1);
        }
    }
}
