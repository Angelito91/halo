// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// `haloc` — standalone compiler front-end.
//
// Reads a `.halo` source file, produces LLVM IR, and then links a native
// binary using the system `llc` + `cc` toolchain.  For most use-cases the
// `halo --compile` sub-command (which uses clang directly) is simpler; this
// binary is kept for environments where only `llc` and a plain C compiler are
// available.

use std::env;
use std::fs;
use std::process::{exit, Command};

use halo::compiler::Compilation;
use halo::lexer::{Lexer, TokenKind};
use halo::parser::Parser;

// ── Configuration ─────────────────────────────────────────────────────────────

struct Config {
    /// Path to the `.halo` source file.
    input_path: String,
    /// Path for the final linked executable (default: `a.out`).
    output_path: String,
    /// When `true`, keep the intermediate `.ll` file on disk.
    emit_llvm: bool,
    /// When `true`, print progress messages to stdout.
    verbose: bool,
    /// When `true`, pass `-O2` to `llc`.
    optimize: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_path: String::new(),
            output_path: "a.out".to_string(),
            emit_llvm: false,
            verbose: false,
            optimize: false,
        }
    }
}

// ── Argument parsing ──────────────────────────────────────────────────────────

fn parse_args() -> Result<Config, String> {
    let raw: Vec<String> = env::args().collect();

    if raw.len() < 2 {
        return Err("Usage: haloc <input.halo> [options]".to_string());
    }

    let mut cfg = Config::default();
    let mut i = 1;

    while i < raw.len() {
        match raw[i].as_str() {
            "-o" => {
                i += 1;
                if i >= raw.len() {
                    return Err("Expected an output filename after '-o'.".to_string());
                }
                cfg.output_path = raw[i].clone();
            }
            "-emit-llvm" => cfg.emit_llvm = true,
            "-v" | "--verbose" => cfg.verbose = true,
            "-O" | "-O2" => cfg.optimize = true,
            "-h" | "--help" => {
                print_help();
                exit(0);
            }
            arg if !arg.starts_with('-') => cfg.input_path = arg.to_string(),
            arg => return Err(format!("Unknown option: '{arg}'. Run 'haloc --help'.")),
        }
        i += 1;
    }

    if cfg.input_path.is_empty() {
        return Err("No input file specified. Run 'haloc --help'.".to_string());
    }

    Ok(cfg)
}

fn print_help() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║    🌟 Halo Compiler (haloc) v0.2.0            ║");
    println!("╚════════════════════════════════════════════════╝\n");
    println!("USAGE:");
    println!("    haloc [OPTIONS] <INPUT.halo>\n");
    println!("OPTIONS:");
    println!("    -o <output>      Output filename (default: a.out)");
    println!("    -emit-llvm       Keep the intermediate LLVM IR (.ll) file");
    println!("    -O               Enable optimisations (passes -O2 to llc)");
    println!("    -v, --verbose    Print progress messages");
    println!("    -h, --help       Print this help message\n");
    println!("EXAMPLES:");
    println!("    haloc program.halo              Compile to ./a.out");
    println!("    haloc program.halo -o program   Compile to ./program");
    println!("    haloc program.halo -emit-llvm   Keep program.ll on disk\n");
    println!("NOTE:");
    println!("    For most use-cases, prefer: halo --compile <file.halo>");
}

// ── Pipeline helpers ──────────────────────────────────────────────────────────

/// Die with an error message and exit code 1.
macro_rules! fatal {
    ($($arg:tt)*) => {{
        eprintln!("❌ {}", format!($($arg)*));
        exit(1);
    }};
}

/// Run an external command, printing stderr and exiting on failure.
fn run_command(mut cmd: Command, step_name: &str) {
    match cmd.output() {
        Ok(out) if out.status.success() => {}
        Ok(out) => {
            eprintln!("❌ {step_name} failed:");
            eprintln!("{}", String::from_utf8_lossy(&out.stderr));
            exit(1);
        }
        Err(e) => fatal!("Could not launch '{step_name}': {e}"),
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let cfg = parse_args().unwrap_or_else(|e| fatal!("{e}"));

    // ── Step 1: Read source ───────────────────────────────────────────────────

    if cfg.verbose {
        println!("📂 Reading: {}", cfg.input_path);
    }

    let source = fs::read_to_string(&cfg.input_path)
        .unwrap_or_else(|e| fatal!("Cannot read '{}': {e}", cfg.input_path));

    if cfg.verbose {
        println!("✅ {} bytes\n", source.len());
    }

    // ── Step 2: Lex ───────────────────────────────────────────────────────────

    if cfg.verbose {
        println!("🔍 Tokenising…");
    }

    // The lexer is infallible; collect tokens until (and including) Eof.
    let mut lexer = Lexer::new(source);
    let tokens: Vec<_> = std::iter::from_fn(|| {
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
    .collect();

    if cfg.verbose {
        println!("✅ {} tokens\n", tokens.len());
    }

    // ── Step 3: Parse ─────────────────────────────────────────────────────────

    if cfg.verbose {
        println!("📝 Parsing…");
    }

    let program = Parser::new(tokens).parse().unwrap_or_else(|errors| {
        eprintln!("❌ Parse error(s):");
        for err in &errors {
            eprintln!("   {err}");
        }
        exit(1);
    });

    if cfg.verbose {
        println!("✅ {} top-level item(s)\n", program.items.len());
    }

    // ── Step 4: Generate LLVM IR ──────────────────────────────────────────────

    if cfg.verbose {
        println!("⚙️  Generating LLVM IR…");
    }

    let mut compilation = Compilation::new("halo_program");
    compilation
        .codegen()
        .compile(&program)
        .unwrap_or_else(|e| fatal!("Code generation error: {e}"));

    if cfg.verbose {
        println!("✅ LLVM IR ready\n");
    }

    // ── Step 5: Emit .ll file ─────────────────────────────────────────────────

    let ll_path = format!("{}.ll", cfg.output_path);

    // Always write the .ll so that `llc` has something to compile.  The file
    // is removed at the end unless --emit-llvm was requested.
    compilation
        .emit_llvm(&ll_path)
        .unwrap_or_else(|e| fatal!("Failed to write LLVM IR: {e}"));

    if cfg.verbose && cfg.emit_llvm {
        println!("💾 LLVM IR saved to: {ll_path}\n");
    }

    // ── Step 6: llc — IR → assembly ──────────────────────────────────────────

    if cfg.verbose {
        println!("🔨 Compiling IR to assembly (llc)…");
    }

    let asm_path = format!("{}.s", cfg.output_path);
    let mut llc = Command::new("llc");
    llc.arg(&ll_path).arg("-o").arg(&asm_path);
    if cfg.optimize {
        llc.arg("-O2");
    }
    run_command(llc, "llc");

    if cfg.verbose {
        println!("✅ Assembly: {asm_path}\n");
    }

    // ── Step 7: cc — assembly → executable ───────────────────────────────────

    if cfg.verbose {
        println!("🔗 Linking executable (cc)…");
    }

    let mut cc = Command::new("cc");
    cc.arg(&asm_path).arg("-o").arg(&cfg.output_path);
    run_command(cc, "cc");

    if cfg.verbose {
        println!("✅ Executable: {}\n", cfg.output_path);
    }

    // ── Step 8: Clean up intermediates ───────────────────────────────────────

    if !cfg.emit_llvm {
        let _ = fs::remove_file(&ll_path);
    }
    // Always remove the assembly file; it is an internal artefact.
    let _ = fs::remove_file(&asm_path);

    // ── Done ──────────────────────────────────────────────────────────────────

    if cfg.verbose {
        println!("╔════════════════════════════════════════╗");
        println!("║      ✅ Compilation successful!        ║");
        println!("╚════════════════════════════════════════╝");
        println!("\n🚀 Run with: ./{}", cfg.output_path);
    } else {
        println!("✅ Compiled → {}", cfg.output_path);
    }
}
