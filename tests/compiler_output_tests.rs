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

#[test]
fn compiled_trim_should_keep_inner_text_untouched() {
    let code = "print(\"Hola Mundo\".trim())";
    let stdout = compile_link_and_run("trim_full", code);
    assert_eq!(stdout, "Hola Mundo\n");
}

#[test]
fn compiled_trim_should_remove_leading_and_trailing_whitespace() {
    let code = "print(\"  ab  \".trim())";
    let stdout = compile_link_and_run("trim_spaces", code);
    assert_eq!(stdout, "ab\n");
}

#[test]
fn compiled_trim_should_keep_a_single_char() {
    let code = "print(\"X\".trim())";
    let stdout = compile_link_and_run("trim_single", code);
    assert_eq!(stdout, "X\n");
}

#[test]
fn compiled_contains_should_report_true_when_substring_is_present() {
    // The interpreter prints booleans as true/false; compiled mode prints
    // them as 1/0 until the output-format gap is closed. Branch on the
    // result to check the value without pinning the boolean format.
    let code = "if \"Hola Mundo\".contains(\"la Mu\") { print(\"has-it\") }";
    let stdout = compile_link_and_run("contains_true", code);
    assert_eq!(stdout, "has-it\n");
}

#[test]
fn compiled_contains_should_report_false_when_substring_is_absent() {
    let code = "if \"Hola Mundo\".contains(\"zzz\") { print(\"x\") } else { print(\"not-there\") }";
    let stdout = compile_link_and_run("contains_false", code);
    assert_eq!(stdout, "not-there\n");
}

#[test]
fn compiled_for_range_prints_every_iteration() {
    let code = "for i in range(4) { print(i) }";
    let stdout = compile_link_and_run("for_range", code);
    assert_eq!(stdout, "0\n1\n2\n3\n");
}

#[test]
fn compiled_for_range_with_start_and_end() {
    let code = "for i in range(2, 4) { print(i) }";
    let stdout = compile_link_and_run("for_range_ab", code);
    assert_eq!(stdout, "2\n3\n");
}

#[test]
fn compiled_for_break_exits_early() {
    let code = "for i in range(100) {\nif i == 2 { break }\nprint(i)\n}";
    let stdout = compile_link_and_run("for_break", code);
    assert_eq!(stdout, "0\n1\n");
}

#[test]
fn compiled_for_variable_shadows_an_outer_name_and_gives_it_back() {
    // The loop variable is scoped to the loop in the interpreter; compiled
    // mode must at least not clobber the outer binding (which here holds a
    // string, a different type from the loop counter).
    let code = "name = \"hi\"\nfor name in range(2) { print(name) }\nprint(name)";
    let stdout = compile_link_and_run("for_shadow", code);
    assert_eq!(stdout, "0\n1\nhi\n");
}

#[test]
fn compiled_for_continue_skips_to_the_next_iteration() {
    let code = "for i in range(5) {\nif i % 2 == 0 { continue }\nprint(i)\n}";
    let stdout = compile_link_and_run("for_continue", code);
    assert_eq!(stdout, "1\n3\n");
}

#[test]
#[should_panic(expected = "not yet supported")]
fn compiled_for_over_an_array_reports_a_clear_error() {
    compile_link_and_run("for_array_err", "for x in [1, 2] { print(x) }");
}
