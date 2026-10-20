// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Command-line argument parsing and types.

use crate::compiler::OptLevel;
use std::process;
use std::str::FromStr;

// ── Subcommands ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum Subcommand {
    /// Interpret a source file (default when no subcommand is given).
    Run,
    /// Compile to a native binary via LLVM.
    Build,
    /// Parse the file and report errors without executing anything.
    Check,
    /// Print the token stream produced by the lexer.
    Tokens,
    /// Print the Abstract Syntax Tree produced by the parser.
    Ast,
    /// Emit LLVM IR to a `.ll` file (does not link).
    Llvm,
}

// ── Toolchain ─────────────────────────────────────────────────────────────────

/// Which external toolchain to use for the final link step in `build`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Toolchain {
    /// Use `clang <file.ll> -o <out>` (default, single step).
    #[default]
    Clang,
    /// Use `llc <file.ll> -o <file.s>` then `cc <file.s> -o <out>`.
    /// Useful when only `llc` and a plain C compiler are available.
    Llc,
}

impl FromStr for Toolchain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clang" => Ok(Toolchain::Clang),
            "llc" => Ok(Toolchain::Llc),
            other => Err(format!(
                "Unknown toolchain '{other}'. Valid options are: clang, llc."
            )),
        }
    }
}

impl Toolchain {
    pub fn name(self) -> &'static str {
        match self {
            Toolchain::Clang => "clang",
            Toolchain::Llc => "llc + cc",
        }
    }
}

// ── CLI arguments ─────────────────────────────────────────────────────────────

/// Parsed command-line arguments.
pub struct Args {
    /// Which subcommand to execute.
    pub subcommand: Subcommand,
    /// Path to the `.halo` source file.
    pub file: String,
    /// `-o <path>`: output path for `build`.
    pub output: Option<String>,
    /// `-O<N>` / `--opt <N>`: LLVM optimisation level (default: 2).
    pub opt_level: OptLevel,
    /// `--verbose` / `-v`: print progress information for each pipeline phase.
    pub verbose: bool,
    /// `--run` / `-r`: after `build`, execute the resulting binary immediately.
    pub run_after_build: bool,
    /// `--toolchain <name>`: linker toolchain for `build` (default: clang).
    pub toolchain: Toolchain,
    /// `--emit-llvm`: keep the intermediate `.ll` file after `build`.
    pub emit_llvm: bool,
}

// ── Argument parsing ──────────────────────────────────────────────────────────

/// Parse command-line arguments into an `Args` structure.
pub fn parse_args() -> Result<Args, String> {
    let argv: Vec<String> = std::env::args().collect();

    // No arguments at all — print short usage and exit cleanly.
    if argv.len() < 2 {
        super::helpers::print_usage();
        process::exit(0);
    }

    // Detect subcommand or fallback to `run`.
    let (subcommand, rest_start) = match argv[1].as_str() {
        "run" => (Subcommand::Run, 2),
        "build" => (Subcommand::Build, 2),
        "check" => (Subcommand::Check, 2),
        "tokens" => (Subcommand::Tokens, 2),
        "ast" => (Subcommand::Ast, 2),
        "llvm" => (Subcommand::Llvm, 2),
        "help" | "--help" | "-h" => {
            super::helpers::print_help();
            process::exit(0);
        }
        "version" | "--version" | "-V" => {
            super::helpers::print_version();
            process::exit(0);
        }
        // Any unknown word that does not start with '-' is treated as a file
        // path with the implicit `run` subcommand.
        arg if !arg.starts_with('-') => (Subcommand::Run, 1),
        arg => {
            return Err(format!(
                "Unknown subcommand or option '{arg}'.\n\
                 Run 'halo help' for usage."
            ))
        }
    };

    if argv.len() <= rest_start {
        return Err(format!(
            "No source file provided.\n\
             Usage: halo {} <file.halo>",
            subcommand_name(&subcommand)
        ));
    }

    let mut file = String::new();
    let mut output: Option<String> = None;
    let mut opt_level = OptLevel::O2;
    let mut verbose = false;
    let mut run_after_build = false;
    let mut toolchain = Toolchain::default();
    let mut emit_llvm = false;

    let mut i = rest_start;
    while i < argv.len() {
        match argv[i].as_str() {
            "-o" | "--output" => {
                i += 1;
                if i >= argv.len() {
                    return Err("Expected an output path after '-o'.".into());
                }
                output = Some(argv[i].clone());
            }
            "--opt" | "-O" => {
                i += 1;
                if i >= argv.len() {
                    return Err("Expected a level (0-3) after '--opt'.".into());
                }
                opt_level = parse_opt_level(&argv[i])?;
            }
            "-O0" => opt_level = OptLevel::O0,
            "-O1" => opt_level = OptLevel::O1,
            "-O2" => opt_level = OptLevel::O2,
            "-O3" => opt_level = OptLevel::O3,
            "--verbose" | "-v" => verbose = true,
            "--run" | "-r" => run_after_build = true,
            "--emit-llvm" => emit_llvm = true,
            "--toolchain" => {
                i += 1;
                if i >= argv.len() {
                    return Err("Expected a toolchain name after '--toolchain'.".into());
                }
                toolchain = Toolchain::from_str(&argv[i])?;
            }
            // Anything not starting with '-' is the source file.
            arg if !arg.starts_with('-') => {
                if !file.is_empty() {
                    return Err(format!(
                        "Unexpected extra argument '{arg}'. Only one source file is supported."
                    ));
                }
                file = arg.to_string();
            }
            arg => {
                return Err(format!(
                    "Unknown option '{arg}'.\n\
                     Run 'halo help' for usage."
                ))
            }
        }
        i += 1;
    }

    if file.is_empty() {
        return Err(format!(
            "No source file provided.\n\
             Usage: halo {} <file.halo>",
            subcommand_name(&subcommand)
        ));
    }

    // `--run` only makes sense with `build`.
    if run_after_build && !matches!(subcommand, Subcommand::Build) {
        return Err(
            "'--run' / '-r' is only valid with the 'build' subcommand.\n\
             Hint: use 'halo run <file>' to interpret directly."
                .into(),
        );
    }

    // `--toolchain` only makes sense with `build`.
    if toolchain != Toolchain::Clang && !matches!(subcommand, Subcommand::Build) {
        return Err("'--toolchain' is only valid with the 'build' subcommand.".into());
    }

    // `--emit-llvm` only makes sense with `build`.
    if emit_llvm && !matches!(subcommand, Subcommand::Build) {
        return Err("'--emit-llvm' is only valid with the 'build' subcommand.\n\
             Hint: use 'halo llvm <file>' to emit IR directly."
            .into());
    }

    Ok(Args {
        subcommand,
        file,
        output,
        opt_level,
        verbose,
        run_after_build,
        toolchain,
        emit_llvm,
    })
}

/// Get the name of a subcommand.
pub fn subcommand_name(sc: &Subcommand) -> &'static str {
    match sc {
        Subcommand::Run => "run",
        Subcommand::Build => "build",
        Subcommand::Check => "check",
        Subcommand::Tokens => "tokens",
        Subcommand::Ast => "ast",
        Subcommand::Llvm => "llvm",
    }
}

/// Parse an optimization level from a string.
pub fn parse_opt_level(s: &str) -> Result<OptLevel, String> {
    match s {
        "0" => Ok(OptLevel::O0),
        "1" => Ok(OptLevel::O1),
        "2" => Ok(OptLevel::O2),
        "3" => Ok(OptLevel::O3),
        other => Err(format!(
            "Invalid optimisation level '{other}'. Expected 0, 1, 2, or 3."
        )),
    }
}
