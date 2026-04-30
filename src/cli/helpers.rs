// ── UI Output Functions ──────────────────────────────────────────────────────

use crate::utils::{BANNER, VERSION};

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

// ── Pipeline helpers ──────────────────────────────────────────────────────────

use crate::diagnostics::{location_from_source, Diagnostic};
use crate::interpreter::Evaluator;
use crate::lexer::{Lexer, TokenKind};
use crate::parser::Parser;
use std::fs;
use std::path::Path;

/// Read the entire contents of `path` into a `String`.
pub fn read_source(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Cannot read '{path}': {e}"))
}

/// Derive the file stem from a source path (`"examples/foo.halo"` → `"foo"`).
pub fn file_stem(path: &str) -> &str {
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output")
}

/// Lex `source` into a token stream including the terminal `Eof` token.
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

/// Parse a token stream into a [`Program`] AST.
/// Takes source code for better error context and snippets.
pub fn parse_tokens(
    tokens: Vec<crate::lexer::Token>,
    source: &str,
    file: &str,
) -> Result<crate::parser::ast::Program, Vec<Diagnostic>> {
    match Parser::new(tokens).parse() {
        Ok(program) => Ok(program),
        Err(error_strings) => {
            let diagnostics = error_strings
                .iter()
                .map(|err| {
                    // Extract line and column from error message
                    // Format: "Error at LINE:COLUMN: MESSAGE"

                    // Try to parse "Error at LINE:COLUMN:" pattern
                    if let Some(at_idx) = err.find("at ") {
                        let rest = &err[at_idx + 3..];
                        let parts: Vec<&str> = rest.split(':').collect();

                        // We expect at least: ["LINE", "COLUMN", "rest of message"]
                        if parts.len() >= 3 {
                            if let (Ok(line_num), Ok(col_num)) = (
                                parts[0].trim().parse::<u32>(),
                                parts[1].trim().parse::<u32>(),
                            ) {
                                let location =
                                    location_from_source(file, source, line_num, col_num, 1);

                                // Extract just the message part (everything after the second colon)
                                let message = parts[2..].join(":").trim().to_string();

                                return Diagnostic::error(message).with_location(location);
                            }
                        }
                    }

                    // Fallback: return the error as-is
                    Diagnostic::error(err)
                })
                .collect();
            Err(diagnostics)
        }
    }
}

/// Evaluate a [`Program`] with the tree-walking interpreter.
pub fn evaluate(
    program: &crate::parser::ast::Program,
) -> Result<crate::interpreter::Value, String> {
    Evaluator::new().eval_program(program)
}

/// Execute an external command and return success status.
pub fn run_command(mut cmd: std::process::Command, step_name: &str, verbose: bool) -> bool {
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
