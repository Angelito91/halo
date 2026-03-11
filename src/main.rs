// The Halo Programming Language
// Command-line interface for executing .halo files
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

use compiler::Compilation;
use interpreter::Evaluator;
use lexer::Lexer;
use lexer::TokenType;
use parser::Parser;

/// Represents the CLI arguments
struct Args {
    file_path: String,
    show_ast: bool,
    show_tokens: bool,
    verbose: bool,
    /// --compile  : compile to native binary via clang
    compile: bool,
    /// --emit-llvm: write .ll IR file next to the source
    emit_llvm: bool,
    /// --run      : compile to a temp binary and execute it immediately
    run: bool,
    /// -o <path>  : output path for --compile / --run
    output: Option<String>,
}

/// Parses command-line arguments
fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("No file specified".to_string());
    }

    let mut file_path = String::new();
    let mut show_ast = false;
    let mut show_tokens = false;
    let mut verbose = false;
    let mut compile = false;
    let mut emit_llvm = false;
    let mut run = false;
    let mut output: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--ast" | "-a" => show_ast = true,
            "--tokens" | "-t" => show_tokens = true,
            "--verbose" | "-v" => verbose = true,
            "--compile" | "-c" => compile = true,
            "--emit-llvm" => emit_llvm = true,
            "--run" | "-r" => run = true,
            "-o" => {
                i += 1;
                if i >= args.len() {
                    return Err("Expected path after -o".to_string());
                }
                output = Some(args[i].clone());
            }
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
                return Err(format!("Unknown option: {}", arg));
            }
        }
        i += 1;
    }

    if file_path.is_empty() {
        return Err("No file specified".to_string());
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
    })
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
    println!("    -h, --help         Print this help message");
    println!("    -V, --version      Print version information\n");

    println!("EXAMPLES:");
    println!("    halo script.halo                   Interpret script.halo");
    println!("    halo --compile script.halo         Compile to ./script");
    println!("    halo --compile -o out script.halo  Compile to ./out");
    println!("    halo --emit-llvm script.halo       Emit script.ll");
    println!("    halo --run script.halo             Compile and run immediately");
    println!("    halo --ast script.halo             Show AST then interpret\n");

    println!("FILE EXTENSION:");
    println!("    .halo            Halo source code file\n");

    println!("DOCUMENTATION:");
    println!("    For language documentation, see SYNTAX.md");
}

/// Reads a file from disk
fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Cannot read file '{}': {}", path, e))
}

/// Reads from stdin
#[allow(dead_code)]
fn read_stdin() -> Result<String, String> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|e| format!("Error reading from stdin: {}", e))?;
    Ok(buffer)
}

/// Tokenizes the source code
fn tokenize(code: &str) -> Result<Vec<lexer::Token>, String> {
    let mut lexer = Lexer::new(code.to_string());
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        let is_eof = token.token_type == lexer::TokenType::EOF;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    Ok(tokens)
}

/// Parses tokens into an AST
fn parse(tokens: Vec<lexer::Token>) -> Result<parser::ast::Program, Vec<String>> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// Evaluates the AST using the tree-walking interpreter
fn evaluate(program: &parser::ast::Program) -> Result<interpreter::Value, String> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program)
}

/// Compile the program to LLVM IR, then to a native binary via clang.
/// Returns the path to the produced binary.
fn compile_program(
    program: &parser::ast::Program,
    source_path: &str,
    output_path: Option<&str>,
    emit_llvm: bool,
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
        .args([&ll_path, "-o", &bin_path, "-lm"])
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

/// Displays tokens in a formatted way
fn display_tokens(tokens: &[lexer::Token]) {
    println!("╔════════════════════════════════════════╗");
    println!("║         📋 TOKENS (Lexer Output)       ║");
    println!("╚════════════════════════════════════════╝\n");

    for (i, token) in tokens.iter().enumerate() {
        if token.token_type == TokenType::EOF {
            println!("{:3}. [EOF]", i);
        } else if token.token_type == TokenType::Newline {
            println!(
                "{:3}. [NEWLINE] at {}:{}",
                i, token.position.line, token.position.column
            );
        } else {
            println!(
                "{:3}. {:?} '{}' at {}:{}",
                i, token.token_type, token.lexeme, token.position.line, token.position.column
            );
        }
    }
    println!();
}

/// Displays the AST in a formatted way
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

/// Main entry point
fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            eprintln!("Run 'halo --help' for usage information");
            process::exit(1);
        }
    };

    // Print header
    if args.verbose {
        println!("╔════════════════════════════════════════╗");
        println!("║    🌟 Halo Interpreter v0.2.0 🌟     ║");
        println!("╚════════════════════════════════════════╝\n");
    }

    // Read file
    if args.verbose {
        println!("📂 Reading file: {}", args.file_path);
    }

    let code = match read_file(&args.file_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("❌ {}", e);
            process::exit(1);
        }
    };

    if args.verbose {
        println!("✅ File read successfully ({} bytes)\n", code.len());
    }

    // Tokenize
    if args.verbose {
        println!("🔍 Tokenizing...");
    }

    let tokens = match tokenize(&code) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("❌ Tokenization error: {}", e);
            process::exit(1);
        }
    };

    if args.verbose {
        println!("✅ Tokenization successful ({} tokens)\n", tokens.len());
    }

    if args.show_tokens {
        display_tokens(&tokens);
    }

    // Parse
    if args.verbose {
        println!("📝 Parsing...");
    }

    let program = match parse(tokens) {
        Ok(program) => program,
        Err(errors) => {
            eprintln!("❌ Parsing error:");
            for error in errors {
                eprintln!("   {}", error);
            }
            process::exit(1);
        }
    };

    if args.verbose {
        println!("✅ Parsing successful\n");
    }

    if args.show_ast {
        display_ast(&program);
    }

    // ── Compile mode ──────────────────────────────────────────────────────────
    if args.compile || args.emit_llvm || args.run {
        if args.verbose {
            println!("🏗️  Compiling via LLVM...");
            println!("─────────────────────────────────────────\n");
        }

        let bin_path = match compile_program(
            &program,
            &args.file_path,
            args.output.as_deref(),
            args.emit_llvm,
            args.verbose,
        ) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("❌ Compilation error: {}", e);
                process::exit(1);
            }
        };

        if args.run {
            // Execute the compiled binary
            if args.verbose {
                println!("🚀 Running: ./{}", bin_path);
                println!("─────────────────────────────────────────\n");
            }

            let status = std::process::Command::new(format!("./{}", bin_path))
                .status()
                .unwrap_or_else(|e| {
                    eprintln!("❌ Failed to run '{}': {}", bin_path, e);
                    process::exit(1);
                });

            // Clean up temp binary when using --run without explicit -o
            if args.output.is_none() {
                let _ = fs::remove_file(&bin_path);
            }

            if args.verbose {
                println!("\n─────────────────────────────────────────");
                println!("✅ Process exited with code {}", status.code().unwrap_or(0));
            }

            process::exit(status.code().unwrap_or(0));
        }

        if !args.verbose {
            println!("✅ Compiled successfully → {}", bin_path);
        }

        return;
    }

    // ── Interpreter mode (default) ────────────────────────────────────────────
    if args.verbose {
        println!("▶️  Evaluating...");
        println!("─────────────────────────────────────────\n");
    }

    match evaluate(&program) {
        Ok(_) => {
            if args.verbose {
                println!("\n─────────────────────────────────────────");
                println!("✅ Execution completed successfully");
            }
        }
        Err(e) => {
            eprintln!("\n❌ Runtime error: {}", e);
            process::exit(1);
        }
    }
}
