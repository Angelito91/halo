// Pins the text that parser-dependent CLI subcommands print, so that
// refactors of the parser cannot silently change token or item order.

use std::process::Command;

fn run_halo(sub: &str) -> String {
    let dir = std::env::temp_dir().join("halo_cli_output_tests");
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("prog.halo");
    std::fs::write(&src, "x = 1 + 2\nprint(x)\n").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_halo"))
        .arg(sub)
        .arg(&src)
        .output()
        .expect("spawn halo");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn tokens_subcommand_is_stable() {
    let out = run_halo("tokens");
    assert!(out.contains("Identifier"), "unexpected tokens output: {out}");
    assert!(out.contains("Number"), "unexpected tokens output: {out}");
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
