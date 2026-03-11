// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use std::fs;
use std::path::Path;
use std::process;

mod compiler;
mod interpreter;
mod lexer;
mod parser;

use compiler::{Compilation, OptLevel};
use interpreter::Evaluator;
use lexer::{Lexer, TokenKind};
use parser::Parser;

// ── Constants ─────────────────────────────────────────────────────────────────

const VERSION: &str = "0.2.0";
const BANNER: &str = "🌟 Halo Programming Language";

// ── Subcommands ───────────────────────────────────────────────────────────────

#[derive(Debug)]
enum Subcommand {
    /// Interpret a source file (default when no subcommand is given).
    Run,
    /// Compile to a native binary via LLVM.
    Build,
    /// Parse the file and report errors without executing anything.
    Check,
    /// Print the token stream produced by the lexer.
    Tokens,
    /// Print the Abstract Syntax Tree produced by the parser.
    Ast,
    /// Emit LLVM IR to a `.ll` file (does not link).
    Llvm,
}

// ── Toolchain ─────────────────────────────────────────────────────────────────

/// Which external toolchain to use for the final link step in `build`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Toolchain {
    /// Use `clang <file.ll> -o <out>` (default, single step).
    #[default]
    Clang,
    /// Use `llc <file.ll> -o <file.s>` then `cc <file.s> -o <out>`.
    /// Useful when only `llc` and a plain C compiler are available.
    Llc,
}

impl Toolchain {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "clang" => Ok(Toolchain::Clang),
            "llc" => Ok(Toolchain::Llc),
            other => Err(format!(
                "Unknown toolchain '{other}'. Valid options are: clang, llc."
            )),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Toolchain::Clang => "clang",
            Toolchain::Llc => "llc + cc",
        }
    }
}

// ── CLI arguments ─────────────────────────────────────────────────────────────

struct Args {
    /// Which subcommand to execute.
    subcommand: Subcommand,
    /// Path to the `.halo` source file.
    file: String,
    /// `-o <path>`: output path for `build`.
    output: Option<String>,
    /// `-O<N>` / `--opt <N>`: LLVM optimisation level (default: 2).
    opt_level: OptLevel,
    /// `--verbose` / `-v`: print progress information for each pipeline phase.
    verbose: bool,
    /// `--run` / `-r`: after `build`, execute the resulting binary immediately.
    run_after_build: bool,
    /// `--toolchain <name>`: linker toolchain for `build` (default: clang).
    toolchain: Toolchain,
    /// `--emit-llvm`: keep the intermediate `.ll` file after `build`.
    emit_llvm: bool,
}

// ── Argument parsing ──────────────────────────────────────────────────────────

fn parse_args() -> Result<Args, String> {
    let argv: Vec<String> = std::env::args().collect();

    // No arguments at all — print short usage and exit cleanly.
    if argv.len() < 2 {
        print_usage();
        process::exit(0);
    }

    // Detect subcommand or fallback to `run`.
    let (subcommand, rest_start) = match argv[1].as_str() {
        "run" => (Subcommand::Run, 2),
        "build" => (Subcommand::Build, 2),
        "check" => (Subcommand::Check, 2),
        "tokens" => (Subcommand::Tokens, 2),
        "ast" => (Subcommand::Ast, 2),
        "llvm" => (Subcommand::Llvm, 2),
        "help" | "--help" | "-h" => {
            print_help();
            process::exit(0);
        }
        "version" | "--version" | "-V" => {
            print_version();
            process::exit(0);
        }
        // Any unknown word that does not start with '-' is treated as a file
        // path with the implicit `run` subcommand.
        arg if !arg.starts_with('-') => (Subcommand::Run, 1),
        arg => {
            return Err(format!(
                "Unknown subcommand or option '{arg}'.\n\
                 Run 'halo help' for usage."
            ))
        }
    };

    if argv.len() <= rest_start {
        return Err(format!(
            "No source file provided.\n\
             Usage: halo {} <file.halo>",
            subcommand_name(&subcommand)
        ));
    }

    let mut file = String::new();
    let mut output: Option<String> = None;
    let mut opt_level = OptLevel::O2;
    let mut verbose = false;
    let mut run_after_build = false;
    let mut toolchain = Toolchain::default();
    let mut emit_llvm = false;

    let mut i = rest_start;
    while i < argv.len() {
        match argv[i].as_str() {
            "-o" | "--output" => {
                i += 1;
                if i >= argv.len() {
                    return Err("Expected an output path after '-o'.".into());
                }
                output = Some(argv[i].clone());
            }
            "--opt" | "-O" => {
                i += 1;
                if i >= argv.len() {
                    return Err("Expected a level (0-3) after '--opt'.".into());
                }
                opt_level = parse_opt_level(&argv[i])?;
            }
            "-O0" => opt_level = OptLevel::O0,
            "-O1" => opt_level = OptLevel::O1,
            "-O2" => opt_level = OptLevel::O2,
            "-O3" => opt_level = OptLevel::O3,
            "--verbose" | "-v" => verbose = true,
            "--run" | "-r" => run_after_build = true,
            "--emit-llvm" => emit_llvm = true,
            "--toolchain" => {
                i += 1;
                if i >= argv.len() {
                    return Err("Expected a toolchain name after '--toolchain'.".into());
                }
                toolchain = Toolchain::from_str(&argv[i])?;
            }
            // Anything not starting with '-' is the source file.
            arg if !arg.starts_with('-') => {
                if !file.is_empty() {
                    return Err(format!(
                        "Unexpected extra argument '{arg}'. Only one source file is supported."
                    ));
                }
                file = arg.to_string();
            }
            arg => {
                return Err(format!(
                    "Unknown option '{arg}'.\n\
                     Run 'halo help' for usage."
                ))
            }
        }
        i += 1;
    }

    if file.is_empty() {
        return Err(format!(
            "No source file provided.\n\
             Usage: halo {} <file.halo>",
            subcommand_name(&subcommand)
        ));
    }

    // `--run` only makes sense with `build`.
    if run_after_build && !matches!(subcommand, Subcommand::Build) {
        return Err(
            "'--run' / '-r' is only valid with the 'build' subcommand.\n\
             Hint: use 'halo run <file>' to interpret directly."
                .into(),
        );
    }

    // `--toolchain` only makes sense with `build`.
    if toolchain != Toolchain::Clang && !matches!(subcommand, Subcommand::Build) {
        return Err("'--toolchain' is only valid with the 'build' subcommand.".into());
    }

    // `--emit-llvm` only makes sense with `build`.
    if emit_llvm && !matches!(subcommand, Subcommand::Build) {
        return Err("'--emit-llvm' is only valid with the 'build' subcommand.\n\
             Hint: use 'halo llvm <file>' to emit IR directly."
            .into());
    }

    Ok(Args {
        subcommand,
        file,
        output,
        opt_level,
        verbose,
        run_after_build,
        toolchain,
        emit_llvm,
    })
}

fn subcommand_name(sc: &Subcommand) -> &'static str {
    match sc {
        Subcommand::Run => "run",
        Subcommand::Build => "build",
        Subcommand::Check => "check",
        Subcommand::Tokens => "tokens",
        Subcommand::Ast => "ast",
        Subcommand::Llvm => "llvm",
    }
}

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

// ── Help / version output ─────────────────────────────────────────────────────

fn print_version() {
    println!("{BANNER} v{VERSION}");
}

fn print_usage() {
    println!("{BANNER} v{VERSION}");
    println!();
    println!("USAGE:");
    println!("    halo <SUBCOMMAND> <file.halo> [OPTIONS]");
    println!("    halo <file.halo>              (shorthand for 'halo run')");
    println!();
    println!("SUBCOMMANDS:");
    println!("    run     Interpret a source file (default)");
    println!("    build   Compile to a native binary");
    println!("    check   Parse and report errors without running");
    println!("    tokens  Print the token stream");
    println!("    ast     Print the Abstract Syntax Tree");
    println!("    llvm    Emit LLVM IR to a .ll file");
    println!();
    println!("Run 'halo help' for full documentation.");
}

fn print_help() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║        {BANNER} v{VERSION}        ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();
    println!("USAGE:");
    println!("    halo <SUBCOMMAND> [OPTIONS] <file.halo>");
    println!("    halo <file.halo>                (shorthand for 'halo run')");
    println!();
    println!("SUBCOMMANDS:");
    println!("    run    <file>   Interpret the source file with the tree-walking");
    println!("                   interpreter (this is the default when no subcommand");
    println!("                   is given).");
    println!();
    println!("    build  <file>   Compile to a native binary using LLVM.");
    println!("                   Options:");
    println!(
        "                     -o, --output <path>        Output binary path (default: <stem>)"
    );
    println!("                     -O0 / -O1 / -O2 / -O3      Optimisation level (default: -O2)");
    println!("                     --opt <N>                   Same as -ON");
    println!("                     --toolchain <clang|llc>    Linker toolchain (default: clang)");
    println!("                       clang  — clang <file.ll> -o <out>  (single step, default)");
    println!("                       llc    — llc <file.ll> | cc <file.s> -o <out>  (two steps)");
    println!("                     --emit-llvm                Keep the intermediate .ll file");
    println!("                     -r, --run                  Execute the binary after building");
    println!("                     -v, --verbose              Show each compilation phase");
    println!();
    println!("    check  <file>   Lex and parse the file, report any errors, then exit.");
    println!("                   Exit code 0 = OK, 1 = errors.");
    println!();
    println!("    tokens <file>   Print every token produced by the lexer and exit.");
    println!();
    println!("    ast    <file>   Print the Abstract Syntax Tree produced by the parser.");
    println!();
    println!("    llvm   <file>   Generate LLVM IR and write it to <stem>.ll.");
    println!("                   Options:");
    println!("                     -o, --output <path>        Output .ll file path");
    println!("                     -O0 / -O1 / -O2 / -O3      Optimisation level (default: -O2)");
    println!("                     -v, --verbose              Show codegen progress");
    println!();
    println!("GLOBAL OPTIONS:");
    println!("    -v, --verbose   Verbose output (supported by run, build, llvm)");
    println!("    -h, --help      Print this help message");
    println!("    -V, --version   Print version information");
    println!();
    println!("EXAMPLES:");
    println!("    halo script.halo                             # interpret");
    println!("    halo run script.halo                         # same as above");
    println!("    halo build script.halo                       # compile → ./script  (clang)");
    println!("    halo build -o prog script.halo               # compile → ./prog");
    println!("    halo build -O3 -r script.halo                # compile with O3 and run");
    println!("    halo build --toolchain llc script.halo       # compile via llc + cc");
    println!("    halo build --toolchain llc --emit-llvm s.halo# compile via llc + cc, keep .ll");
    println!("    halo check script.halo                       # validate syntax only");
    println!("    halo tokens script.halo                      # inspect the token stream");
    println!("    halo ast    script.halo                      # inspect the AST");
    println!("    halo llvm   script.halo                      # emit script.ll");
    println!("    halo llvm -O3 script.halo                    # emit optimised script.ll");
    println!();
    println!("DOCUMENTATION:");
    println!("    Language reference: SYNTAX.md");
    println!("    Project overview:   README.md");
}

// ── Pipeline helpers ──────────────────────────────────────────────────────────

/// Read the entire contents of `path` into a `String`.
fn read_source(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Cannot read '{path}': {e}"))
}

/// Derive the file stem from a source path (`"examples/foo.halo"` → `"foo"`).
fn file_stem(path: &str) -> &str {
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output")
}

/// Lex `source` into a token stream including the terminal `Eof` token.
fn tokenize(source: &str) -> Vec<lexer::Token> {
    let mut lexer = Lexer::new(source.to_string());
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
fn parse_tokens(tokens: Vec<lexer::Token>) -> Result<parser::ast::Program, Vec<String>> {
    Parser::new(tokens).parse()
}

/// Evaluate a [`Program`] with the tree-walking interpreter.
fn evaluate(program: &parser::ast::Program) -> Result<interpreter::Value, String> {
    Evaluator::new().eval_program(program)
}

/// Run an external command, printing stderr and returning `false` on failure.
fn run_command(mut cmd: process::Command, step_name: &str, verbose: bool) -> bool {
    if verbose {
        // Print the command being executed.
        eprintln!("   $ {:?}", cmd);
    }
    match cmd.output() {
        Ok(out) if out.status.success() => true,
        Ok(out) => {
            eprintln!("❌ '{step_name}' failed:");
            let stderr = String::from_utf8_lossy(&out.stderr);
            if !stderr.is_empty() {
                eprintln!("{stderr}");
            }
            false
        }
        Err(e) => {
            eprintln!("❌ Could not launch '{step_name}': {e}");
            false
        }
    }
}

// ── Subcommand handlers ───────────────────────────────────────────────────────

/// `halo run <file>` — interpret the source file.
fn cmd_run(args: &Args) -> i32 {
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

    let program = match parse_tokens(tokens) {
        Ok(p) => p,
        Err(errors) => {
            eprintln!("❌ Parse error(s):");
            for e in &errors {
                eprintln!("   {e}");
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
            eprintln!("❌ Runtime error: {e}");
            1
        }
    }
}

/// `halo build <file>` — compile to a native binary via LLVM.
///
/// Supports two toolchains:
///   - `clang`  (default): `clang <file.ll> -o <out>`
///   - `llc`             : `llc <file.ll> -o <file.s>` → `cc <file.s> -o <out>`
fn cmd_build(args: &Args) -> i32 {
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

    // ── Lex ───────────────────────────────────────────────────────────────────
    if args.verbose {
        eprintln!("🔍 Tokenising…");
    }
    let tokens = tokenize(&source);
    if args.verbose {
        eprintln!("✅ {} tokens\n", tokens.len());
    }

    // ── Parse ─────────────────────────────────────────────────────────────────
    if args.verbose {
        eprintln!("📝 Parsing…");
    }
    let program = match parse_tokens(tokens) {
        Ok(p) => p,
        Err(errors) => {
            eprintln!("❌ Parse error(s):");
            for e in &errors {
                eprintln!("   {e}");
            }
            return 1;
        }
    };
    if args.verbose {
        eprintln!("✅ {} top-level item(s)\n", program.items.len());
    }

    // ── Codegen ───────────────────────────────────────────────────────────────
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

    // ── Optimise ──────────────────────────────────────────────────────────────
    if args.opt_level != OptLevel::O0 {
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

    // ── Emit .ll ──────────────────────────────────────────────────────────────
    if let Err(e) = comp.emit_llvm(&ll_path) {
        eprintln!("❌ Failed to write LLVM IR: {e}");
        return 1;
    }
    if args.verbose && args.emit_llvm {
        eprintln!("💾 LLVM IR saved to: {ll_path}\n");
    }

    // ── Link ──────────────────────────────────────────────────────────────────
    let link_ok = match args.toolchain {
        Toolchain::Clang => link_with_clang(&ll_path, &bin_path, args),
        Toolchain::Llc => link_with_llc(&ll_path, &bin_path, args),
    };

    // ── Clean up intermediate .ll ─────────────────────────────────────────────
    if !args.emit_llvm {
        let _ = fs::remove_file(&ll_path);
    }

    if !link_ok {
        return 1;
    }

    if args.verbose {
        eprintln!("✅ Binary → {bin_path}\n");
        eprintln!("╔════════════════════════════════════════╗");
        eprintln!("║      ✅ Build successful!              ║");
        eprintln!("╚════════════════════════════════════════╝");
    } else {
        println!("✅ {bin_path}");
    }

    // ── Optional: run immediately ─────────────────────────────────────────────
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

        // Remove temporary binary if the user did not specify an explicit output.
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

/// Link using `clang <ll_path> -o <bin_path>` (default toolchain).
fn link_with_clang(ll_path: &str, bin_path: &str, args: &Args) -> bool {
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
///
/// This is the former `haloc` toolchain, now available as
/// `halo build --toolchain llc`.
fn link_with_llc(ll_path: &str, bin_path: &str, args: &Args) -> bool {
    let asm_path = format!("{bin_path}.s");

    // ── Step A: llc — IR → assembly ───────────────────────────────────────────
    if args.verbose {
        eprintln!("🔨 Compiling IR to assembly (llc)…");
    }

    let mut llc = process::Command::new("llc");
    llc.arg(ll_path).arg("-o").arg(&asm_path);
    if args.opt_level != OptLevel::O0 {
        llc.arg(args.opt_level.clang_flag()); // llc accepts -O0 .. -O3 too
    }

    if !run_command(llc, "llc", args.verbose) {
        eprintln!("❌ llc failed. Is llc installed?");
        return false;
    }

    if args.verbose {
        eprintln!("✅ Assembly: {asm_path}\n");
        eprintln!("🔗 Linking with cc…");
    }

    // ── Step B: cc — assembly → executable ────────────────────────────────────
    let mut cc = process::Command::new("cc");
    cc.arg(&asm_path).arg("-o").arg(bin_path);

    if !run_command(cc, "cc", args.verbose) {
        eprintln!("❌ cc failed. Is a C compiler installed?");
        let _ = fs::remove_file(&asm_path);
        return false;
    }

    // Always remove the intermediate assembly file.
    let _ = fs::remove_file(&asm_path);

    if args.verbose {
        eprintln!("✅ Linked via llc + cc\n");
    }
    true
}

/// `halo check <file>` — lex + parse, report errors, exit 0 if clean.
fn cmd_check(args: &Args) -> i32 {
    let source = match read_source(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {e}");
            return 1;
        }
    };

    let tokens = tokenize(&source);

    match parse_tokens(tokens) {
        Ok(program) => {
            println!(
                "✅ {} — OK ({} top-level item(s))",
                args.file,
                program.items.len()
            );
            0
        }
        Err(errors) => {
            eprintln!("❌ {} — {} error(s):", args.file, errors.len());
            for e in &errors {
                eprintln!("   {e}");
            }
            1
        }
    }
}

/// `halo tokens <file>` — print the token stream and exit.
fn cmd_tokens(args: &Args) -> i32 {
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
        match tok.kind {
            TokenKind::Eof => {
                println!("{i:4}  EOF");
            }
            TokenKind::Newline => {
                println!(
                    "{i:4}  NEWLINE                     {}:{}",
                    tok.position.line, tok.position.column
                );
            }
            _ => {
                println!(
                    "{i:4}  {:<20} {:>10}    {}:{}",
                    format!("{:?}", tok.kind),
                    format!("'{}'", tok.lexeme),
                    tok.position.line,
                    tok.position.column
                );
            }
        }
    }

    println!();
    println!("{} token(s)", tokens.len());
    0
}

/// `halo ast <file>` — print the Abstract Syntax Tree and exit.
fn cmd_ast(args: &Args) -> i32 {
    let source = match read_source(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {e}");
            return 1;
        }
    };

    let tokens = tokenize(&source);

    let program = match parse_tokens(tokens) {
        Ok(p) => p,
        Err(errors) => {
            eprintln!("❌ Parse error(s):");
            for e in &errors {
                eprintln!("   {e}");
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
        println!("[{i}] {item_kind}", item_kind = toplevel_kind(item));
        match item {
            parser::ast::TopLevel::Function {
                name, params, body, ..
            } => {
                println!("    name   : {name}");
                if params.is_empty() {
                    println!("    params : (none)");
                } else {
                    println!("    params : {}", params.join(", "));
                }
                println!("    body   : {} statement(s)", body.stmts.len());
                for (j, stmt) in body.stmts.iter().enumerate() {
                    println!("      [{j}] {stmt}");
                }
            }
            parser::ast::TopLevel::GlobalVar { name, init, .. } => {
                println!("    name   : {name}");
                match init {
                    Some(expr) => println!("    init   : {expr}"),
                    None => println!("    init   : (none)"),
                }
            }
            parser::ast::TopLevel::Stmt { stmt, .. } => {
                println!("    stmt   : {stmt}");
            }
        }
        println!();
    }

    println!("{} top-level item(s)", program.items.len());
    0
}

fn toplevel_kind(item: &parser::ast::TopLevel) -> &'static str {
    match item {
        parser::ast::TopLevel::Function { .. } => "Function",
        parser::ast::TopLevel::GlobalVar { .. } => "GlobalVar",
        parser::ast::TopLevel::Stmt { .. } => "Stmt",
    }
}

/// `halo llvm <file>` — emit LLVM IR to a `.ll` file.
fn cmd_llvm(args: &Args) -> i32 {
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
    let program = match parse_tokens(tokens) {
        Ok(p) => p,
        Err(errors) => {
            eprintln!("❌ Parse error(s):");
            for e in &errors {
                eprintln!("   {e}");
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

    if args.opt_level != OptLevel::O0 {
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

