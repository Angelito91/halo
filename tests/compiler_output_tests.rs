// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Integration tests for the native compilation pipeline:
//! source → lexer → parser → LLVM IR → clang → executable → stdout.
//!
//! These tests pin the contract between the interpreter's user-visible
//! output and what compiled binaries print. The interpreter always ends
//! `print()` output with a newline; compiled binaries must match.

use halo::compiler::Compilation;
use halo::lexer::{Lexer, TokenKind};
use halo::parser::ast::Program;
use halo::parser::Parser;
use std::fs;
use std::process::Command;

/// Tokenize a source string the same way the CLI does: scan until EOF.
fn tokenize_for_tests(source: &str) -> Vec<halo::lexer::Token> {
    let mut lexer = Lexer::new(source.to_string());
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        let is_eof = tok.kind == TokenKind::Eof;
        tokens.push(tok);
        if is_eof {
            break;
        }
    }
    tokens
}

/// Compile `code` to LLVM IR, link it with clang, run the binary, and
/// return the raw stdout bytes as a String.
fn compile_link_and_run(test_id: &str, code: &str) -> String {
    let program: Program = match Parser::new(tokenize_for_tests(code)).parse() {
        Ok(p) => p,
        Err(errors) => panic!("parse failed for {test_id}: {errors:?}"),
    };

    let dir = std::env::temp_dir().join(format!("halo_codegen_test_{}", test_id));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp dir");

    let ll_path = dir.join(format!("{test_id}.ll"));
    let bin_path = dir.join(test_id);

    let mut comp = Compilation::new(test_id);
    comp.codegen()
        .compile(&program)
        .expect("codegen failed");
    comp.emit_llvm(ll_path.to_str().unwrap())
        .expect("emit llvm failed");

    let status = Command::new("clang")
        .arg(ll_path.to_str().unwrap())
        .arg("-O0")
        .arg("-o")
        .arg(bin_path.to_str().unwrap())
        .arg("-lm")
        .status()
        .expect("clang must be installed for compiled-mode tests");
    assert!(
        status.success(),
        "clang linking failed for {test_id}"
    );

    let output = Command::new(&bin_path)
        .output()
        .expect("run linked binary");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn compiled_print_should_end_with_newline() {
    let stdout = compile_link_and_run("print_newline", r#"print("hi")"#);
    assert_eq!(stdout, "hi\n");
}

#[test]
fn compiled_multiple_prints_should_each_have_their_own_line() {
    let code = "print(\"a\")\nprint(123)";
    let stdout = compile_link_and_run("print_lines", code);
    assert_eq!(stdout, "a\n123\n");
}
