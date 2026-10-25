// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::diagnostics::{location_from_source, Diagnostic};
use crate::interpreter::Evaluator;
use crate::lexer::{Lexer, TokenKind};
use crate::parser::Parser;
use crate::utils::{BANNER, VERSION};
use std::fs;
use std::path::Path;

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
    println!("    check   Parse and report errors without running");
    println!("    tokens  Print the token stream");
    println!("    ast     Print the Abstract Syntax Tree");
    println!();
    println!("Run 'halo help' for full documentation.");
}

pub fn print_help() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║        {BANNER} v{VERSION}        ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();
    println!("Halo is an interpreted scripting language.");
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
    println!("    check  <file>   Lex and parse the file, report any errors, then exit.");
    println!("                   Exit code 0 = OK, 1 = errors.");
    println!();
    println!("    tokens <file>   Print every token produced by the lexer and exit.");
    println!();
    println!("    ast    <file>   Print the Abstract Syntax Tree produced by the parser.");
    println!();
    println!("OPTIONS:");
    println!("    -v, --verbose   Print progress information while running a file");
    println!("    -h, --help      Print this help message");
    println!("    -V, --version   Print version information");
    println!();
    println!("EXAMPLES:");
    println!("    halo script.halo                 # interpret");
    println!("    halo run script.halo             # same as above");
    println!("    halo check script.halo           # validate syntax only");
    println!("    halo tokens script.halo          # inspect the token stream");
    println!("    halo ast    script.halo          # inspect the AST");
    println!();
    println!("DOCUMENTATION:");
    println!("    Project overview: README.md");
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
