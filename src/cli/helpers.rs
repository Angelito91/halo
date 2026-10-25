// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::cli::args::Args;
use crate::diagnostics::{location_from_source, Diagnostic};
use crate::interpreter::Evaluator;
use crate::lexer::{Lexer, TokenKind};
use crate::parser::Parser;
use crate::utils::{BANNER, VERSION};
use std::fs;
use std::path::Path;
use std::process;

pub fn print_version() {
    println!("{BANNER} v{VERSION}");
}

pub fn print_usage() {
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

pub fn print_help() {
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

pub fn read_source(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Cannot read '{path}': {e}"))
}

pub fn file_stem(path: &str) -> &str {
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output")
}

pub fn tokenize(source: &str) -> Vec<crate::lexer::Token> {
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

pub fn parse_tokens(
    tokens: Vec<crate::lexer::Token>,
    source: &str,
    file: &str,
) -> Result<crate::parser::ast::Program, Vec<Diagnostic>> {
    match Parser::new(tokens).parse() {
        Ok(program) => Ok(program),
        Err(errors) => Err(errors
            .into_iter()
            .map(|error| {
                let location = location_from_source(file, source, error.line, error.column, 1);
                Diagnostic::error(error.message).with_location(location)
            })
            .collect()),
    }
}

pub fn evaluate(
    program: &crate::parser::ast::Program,
) -> Result<crate::interpreter::Value, String> {
    Evaluator::new().eval_program(program)
}

pub fn run_command(mut cmd: std::process::Command, step_name: &str, verbose: bool) -> bool {
    if verbose {
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
