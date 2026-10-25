// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Command-line argument parsing and types.
//!
//! Halo is interpreter-only: `run`, `check`, `tokens` and `ast` are the only
//! source-file commands. `build` and `llvm` are rejected with an explicit
//! message so a stale script fails clearly instead of trying to open a file
//! called `build`.

use std::process;

// ── Subcommands ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subcommand {
    /// Interpret a source file (default when no subcommand is given).
    Run,
    /// Parse the file and report errors without executing anything.
    Check,
    /// Print the token stream produced by the lexer.
    Tokens,
    /// Print the Abstract Syntax Tree produced by the parser.
    Ast,
    /// Print full help.
    Help,
    /// Print short usage.
    Usage,
    /// Print version information.
    Version,
}

// ── CLI arguments ─────────────────────────────────────────────────────────────

/// Parsed command-line arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    /// Which command to execute.
    pub subcommand: Subcommand,
    /// Path to the `.halo` source file (empty for help/usage/version).
    pub file: String,
    /// `--verbose` / `-v`: print progress information for each phase.
    pub verbose: bool,
}

// ── Argument parsing ──────────────────────────────────────────────────────────

/// Parse the process arguments. Exits the process after printing usage when
/// Halo is invoked with no arguments.
pub fn parse_args() -> Result<Args, String> {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    if argv.is_empty() {
        super::helpers::print_usage();
        process::exit(0);
    }

    parse_args_from(&argv)
}

/// Parse an argument list that does **not** include the program name.
///
/// This is the testable core of [`parse_args`].
pub fn parse_args_from(argv: &[String]) -> Result<Args, String> {
    let first = argv.first().map(String::as_str);

    let (subcommand, rest_start) = match first {
        Some("run") => (Subcommand::Run, 1),
        Some("check") => (Subcommand::Check, 1),
        Some("tokens") => (Subcommand::Tokens, 1),
        Some("ast") => (Subcommand::Ast, 1),
        Some("help" | "--help" | "-h") => (Subcommand::Help, 1),
        Some("version" | "--version" | "-V") => (Subcommand::Version, 1),
        Some("build" | "llvm") => {
            return Err(format!(
                "The '{}' subcommand was removed: Halo is interpreter-only.\n\
                 Use 'halo run <file.halo>' to execute the file.",
                first.unwrap()
            ))
        }
        // Any other word (including a leading option) is handled by the
        // implicit `run` command, whose option loop produces a precise error.
        Some(_) => (Subcommand::Run, 0),
        None => (Subcommand::Usage, 0),
    };

    if matches!(subcommand, Subcommand::Usage) {
        return Ok(Args {
            subcommand,
            file: String::new(),
            verbose: false,
        });
    }

    if matches!(subcommand, Subcommand::Help | Subcommand::Version) {
        if argv.len() > rest_start {
            return Err(format!(
                "'{}' does not take extra arguments.",
                subcommand_name(subcommand)
            ));
        }
        return Ok(Args {
            subcommand,
            file: String::new(),
            verbose: false,
        });
    }

    let mut file: Option<String> = None;
    let mut verbose = false;

    for arg in &argv[rest_start..] {
        match arg.as_str() {
            "--verbose" | "-v" => verbose = true,
            "-o" | "--output" | "-O" | "--opt" | "--toolchain" | "--emit-llvm" | "-r" | "--run" => {
                return Err(format!(
                    "Option '{arg}' was removed: Halo no longer produces native binaries.\n\
                     Use 'halo run <file.halo>' to execute the file."
                ))
            }
            value if value.starts_with('-') => {
                return Err(format!(
                    "Unknown option '{value}'.\nRun 'halo help' for usage."
                ))
            }
            value => {
                if file.is_some() {
                    return Err(format!(
                        "Unexpected extra argument '{value}'. Only one source file is supported."
                    ));
                }
                file = Some(value.to_string());
            }
        }
    }

    let file = file.ok_or_else(|| {
        format!(
            "No source file provided.\nUsage: halo {} <file.halo>",
            subcommand_name(subcommand)
        )
    })?;

    Ok(Args {
        subcommand,
        file,
        verbose,
    })
}

/// Get the name of a subcommand.
pub fn subcommand_name(sc: Subcommand) -> &'static str {
    match sc {
        Subcommand::Run => "run",
        Subcommand::Check => "check",
        Subcommand::Tokens => "tokens",
        Subcommand::Ast => "ast",
        Subcommand::Help => "help",
        Subcommand::Usage => "usage",
        Subcommand::Version => "version",
    }
}
