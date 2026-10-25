// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Behaviour of the interpreter-only CLI argument layer.

use halo::cli::{parse_args_from, Args, Subcommand};

fn parse(args: &[&str]) -> Result<Args, String> {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    parse_args_from(&owned)
}

fn file_command(args: &[&str], subcommand: Subcommand, file: &str, verbose: bool) {
    assert_eq!(
        parse(args).unwrap(),
        Args {
            subcommand,
            file: file.to_string(),
            verbose,
        }
    );
}

#[test]
fn bare_file_is_implicit_run() {
    file_command(&["script.halo"], Subcommand::Run, "script.halo", false);
}

#[test]
fn run_accepts_a_file_and_verbose() {
    file_command(
        &["run", "script.halo", "-v"],
        Subcommand::Run,
        "script.halo",
        true,
    );
}

#[test]
fn check_tokens_and_ast_are_recognised() {
    file_command(&["check", "a.halo"], Subcommand::Check, "a.halo", false);
    file_command(&["tokens", "a.halo"], Subcommand::Tokens, "a.halo", false);
    file_command(&["ast", "a.halo"], Subcommand::Ast, "a.halo", false);
}

#[test]
fn verbose_is_global_and_order_independent() {
    file_command(&["-v", "script.halo"], Subcommand::Run, "script.halo", true);
    file_command(
        &["check", "-v", "a.halo"],
        Subcommand::Check,
        "a.halo",
        true,
    );
}

#[test]
fn help_and_version_need_no_file() {
    assert_eq!(parse(&["help"]).unwrap().subcommand, Subcommand::Help);
    assert_eq!(parse(&["--help"]).unwrap().subcommand, Subcommand::Help);
    assert_eq!(parse(&["-h"]).unwrap().subcommand, Subcommand::Help);
    assert_eq!(parse(&["version"]).unwrap().subcommand, Subcommand::Version);
    assert_eq!(
        parse(&["--version"]).unwrap().subcommand,
        Subcommand::Version
    );
    assert_eq!(parse(&["-V"]).unwrap().subcommand, Subcommand::Version);
}

#[test]
fn help_and_version_reject_extra_arguments() {
    assert!(parse(&["help", "script.halo"]).is_err());
    assert!(parse(&["version", "script.halo"]).is_err());
}

#[test]
fn build_and_llvm_are_rejected_with_an_explanation() {
    for removed in ["build", "llvm"] {
        let err = parse(&[removed, "script.halo"]).unwrap_err();
        assert!(
            err.contains("was removed") && err.contains("interpreter-only"),
            "unexpected error for {removed}: {err}"
        );
    }
}

#[test]
fn removed_build_options_are_rejected() {
    for option in [
        "-o",
        "--output",
        "-O",
        "--opt",
        "--toolchain",
        "--emit-llvm",
        "-r",
        "--run",
    ] {
        let err = parse(&[option, "script.halo"]).unwrap_err();
        assert!(
            err.contains("was removed"),
            "option {option} should be rejected: {err}"
        );
    }
}

#[test]
fn missing_file_is_reported_with_usage() {
    let err = parse(&["run"]).unwrap_err();
    assert!(err.contains("No source file provided"), "{err}");
    assert!(err.contains("halo run <file.halo>"), "{err}");
}

#[test]
fn extra_files_are_rejected() {
    let err = parse(&["run", "a.halo", "b.halo"]).unwrap_err();
    assert!(err.contains("Only one source file"), "{err}");
}

#[test]
fn unknown_options_are_rejected() {
    let err = parse(&["--wat", "script.halo"]).unwrap_err();
    assert!(err.contains("Unknown option '--wat'"), "{err}");
}

#[test]
fn an_empty_argument_list_is_usage() {
    assert_eq!(parse(&[]).unwrap().subcommand, Subcommand::Usage);
}
