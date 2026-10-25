// Pins the text that parser-dependent CLI subcommands print, so that
// refactors of the parser cannot silently change token or item order.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::Command;

/// What a CLI invocation produced.
#[derive(Debug)]
struct Output {
    stdout: String,
    stderr: String,
    code: Option<i32>,
}

/// Unique file name per (source, subcommand).
///
/// The tests run in parallel and a shared file name would let one test
/// overwrite another's source between the write and the spawn.
fn unique_name(source: &str, sub: &str) -> String {
    let mut h = DefaultHasher::new();
    source.hash(&mut h);
    sub.hash(&mut h);
    format!("prog-{:016x}.halo", h.finish())
}

fn run_halo_on(source: &str, sub: &str) -> Output {
    let dir = std::env::temp_dir().join("halo_cli_output_tests");
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join(unique_name(source, sub));
    std::fs::write(&src, source).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_halo"))
        .arg(sub)
        .arg(&src)
        .output()
        .expect("spawn halo");
    Output {
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        code: out.status.code(),
    }
}

fn run_halo(sub: &str) -> String {
    run_halo_on("x = 1 + 2\nprint(x)\n", sub).stdout
}

fn run_args(args: &[&str]) -> Output {
    let out = Command::new(env!("CARGO_BIN_EXE_halo"))
        .args(args)
        .output()
        .expect("spawn halo");
    Output {
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        code: out.status.code(),
    }
}

fn temp_source(name: &str, source: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("halo_cli_output_tests");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, source).unwrap();
    path
}

#[test]
fn tokens_subcommand_keeps_the_token_order() {
    let out = run_halo("tokens");
    let ident = out
        .find("Identifier")
        .unwrap_or_else(|| panic!("no Identifier in {out}"));
    let number = out
        .find("Number")
        .unwrap_or_else(|| panic!("no Number in {out}"));
    let plus = out.find('+').unwrap_or_else(|| panic!("no '+' in {out}"));
    // `x = 1 + 2`: the identifier comes before the number, and the `+` after
    // it. A lexer or parser split that reorders tokens breaks this.
    assert!(
        ident < number,
        "identifier must come before the number: {out}"
    );
    assert!(number < plus, "the number must come before '+': {out}");
}

#[test]
fn ast_subcommand_is_stable() {
    let out = run_halo("ast");
    // The AST printer names top-level bindings `GlobalVar` (not `VarDecl`),
    // and renders both items of `x = 1 + 2\nprint(x)` in order.
    assert!(out.contains("GlobalVar"), "unexpected ast output: {out}");
    assert!(out.contains("(1 + 2)"), "unexpected ast output: {out}");
    assert!(out.contains("print(x)"), "unexpected ast output: {out}");
    // Item order must be preserved: `x = 1 + 2` before the bare `print(x)`.
    assert!(
        out.find("(1 + 2)") < out.find("print(x)"),
        "ast items out of order: {out}"
    );
}

#[test]
fn run_subcommand_is_stable() {
    let out = run_halo("run");
    assert_eq!(out, "3\n");
}

#[test]
fn check_subcommand_is_stable() {
    let out = run_halo("check");
    // Real output: `✅  <path> — OK (2 top-level item(s))` (colorised).
    assert!(out.contains("OK"), "unexpected check output: {out}");
    assert!(
        out.contains("top-level item(s)"),
        "unexpected check output: {out}"
    );
}

#[test]
fn check_subcommand_fails_with_exit_code_1_on_a_syntax_error() {
    let out = run_halo_on("x = = 2\n", "check");
    assert_eq!(out.code, Some(1), "a parse error must exit with 1: {out:?}");
    assert!(
        !out.stdout.contains("OK"),
        "a broken program must not report OK: {out:?}"
    );
    assert!(
        out.stderr.contains("1 error(s)"),
        "the error count must be reported on stderr: {out:?}"
    );
}

#[test]
fn bare_file_is_the_run_shorthand() {
    let path = temp_source("shorthand.halo", "print(6 * 7)\n");
    let out = run_args(&[path.to_str().unwrap()]);
    assert_eq!(out.code, Some(0), "implicit run failed: {out:?}");
    assert_eq!(out.stdout, "42\n");
}

#[test]
fn help_lists_only_interpreter_commands() {
    let out = run_args(&["help"]);
    assert_eq!(out.code, Some(0), "halo help failed: {out:?}");
    for command in ["run", "check", "tokens", "ast"] {
        assert!(out.stdout.contains(command), "missing {command}: {out:?}");
    }
    assert!(
        !out.stdout.contains("build") && !out.stdout.contains("llvm"),
        "help must not advertise removed commands: {out:?}"
    );
}

#[test]
fn version_subcommand_is_available() {
    let out = run_args(&["version"]);
    assert_eq!(out.code, Some(0), "halo version failed: {out:?}");
    assert!(out.stdout.contains("0.2.0"), "unexpected version: {out:?}");
}

#[test]
fn removed_build_subcommand_fails_clearly() {
    let path = temp_source("removed-build.halo", "print(1)\n");
    let out = run_args(&["build", path.to_str().unwrap()]);
    assert_eq!(out.code, Some(1), "build must not run: {out:?}");
    assert!(
        out.stderr.contains("was removed") && out.stderr.contains("interpreter-only"),
        "build must explain the removal: {out:?}"
    );
}
