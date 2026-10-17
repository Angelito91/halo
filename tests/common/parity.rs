//! Arnés diferencial: ejecuta un snippet en el intérprete y en un binario
//! nativo y compara comportamiento observable (stdout + ¿error?).
//!
//! Reglas anti-mentira (spec §4.2):
//!   - divergencia NO declarada  → falla
//!   - divergencia declarada que ya no diverge → falla (entrada obsoleta)
//!   - clang ausente → se salta (no falla)
#![allow(dead_code)]

use halo::compiler::Compilation;
use halo::lexer::{Lexer, TokenKind};
use halo::parser::Parser;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct RunResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

impl RunResult {
    /// Un proceso cuenta como error si no terminó con código 0.
    pub fn is_error(&self) -> bool {
        !matches!(self.exit_code, Some(0))
    }
}

pub struct BothModes {
    pub interpreter: RunResult,
    pub compiled: Option<RunResult>,
    /// Explica por qué no hay resultado compilado (fallo de compilación, clang ausente…).
    pub compiled_note: Option<String>,
}

pub struct KnownDivergence {
    pub id: &'static str,
    pub reason: &'static str,
    /// Fase que debe cerrar esta divergencia (P1, P2 o P3).
    pub phase: &'static str,
}

/// Divergencias medidas el 2026-10-17 (spec §1.1). Cada entrada debe
/// desaparecer cuando su fase la arregle: el arnés falla si sigue listada
/// pero ya no diverge.
pub const KNOWN_DIVERGENCES: &[KnownDivergence] = &[
    KnownDivergence { id: "bool_print",          reason: "compiled prints -1/0, interpreter true/false", phase: "P1" },
    KnownDivergence { id: "float_print",         reason: "compiled prints 6 decimals, interpreter minimal", phase: "P1" },
    KnownDivergence { id: "str_runtime_int",     reason: "str() only formats constants in compiled mode", phase: "P1" },
    KnownDivergence { id: "str_runtime_float",   reason: "str() only formats constants in compiled mode", phase: "P1" },
    KnownDivergence { id: "str_runtime_bool",    reason: "bool variable declaration fails to compile", phase: "P1" },
    KnownDivergence { id: "int_from_string",     reason: "int() from strings unsupported in compiled mode", phase: "P1" },
    KnownDivergence { id: "float_from_string",   reason: "float() from strings unsupported in compiled mode", phase: "P1" },
    KnownDivergence { id: "abs_float",           reason: "float formatting differs", phase: "P1" },
    KnownDivergence { id: "array_literal",       reason: "arrays unsupported in compiled mode", phase: "P3" },
    KnownDivergence { id: "array_negative_idx",  reason: "arrays unsupported in compiled mode", phase: "P3" },
    KnownDivergence { id: "array_methods",       reason: "arrays unsupported in compiled mode", phase: "P3" },
    KnownDivergence { id: "array_sort",          reason: "arrays unsupported in compiled mode", phase: "P3" },
    KnownDivergence { id: "array_join",          reason: "arrays unsupported in compiled mode", phase: "P3" },
    KnownDivergence { id: "for_over_array",      reason: "arrays unsupported in compiled mode", phase: "P3" },
    KnownDivergence { id: "range_as_value",      reason: "array values unsupported in compiled mode", phase: "P3" },
    KnownDivergence { id: "string_concat_num",   reason: "string + number prints nothing in compiled mode", phase: "P1" },
    KnownDivergence { id: "int_overflow",        reason: "compiled wraps silently, interpreter errors", phase: "P2" },
    KnownDivergence { id: "div_by_zero",         reason: "compiled has undefined behavior, interpreter errors", phase: "P2" },
    KnownDivergence { id: "mixed_equality",      reason: "printing a comparison result outputs nothing", phase: "P1" },
    KnownDivergence { id: "print_multiarg",      reason: "interpreter prints one line per arg, compiled joins with spaces", phase: "P1" },
    KnownDivergence { id: "print_noargs",        reason: "interpreter prints nothing, compiled prints a newline", phase: "P1" },
    KnownDivergence { id: "string_methods",      reason: "contains() result printed as 1 instead of true", phase: "P1" },
    KnownDivergence { id: "nested_call_str",     reason: "function returning a string call fails to compile", phase: "P1" },
    KnownDivergence { id: "return_none",         reason: "void return prints null vs 0", phase: "P1" },
    KnownDivergence { id: "bool_var_decl",       reason: "bool variable declaration fails to compile", phase: "P1" },
    KnownDivergence { id: "println_builtin",     reason: "println exists only in compiled mode", phase: "P1" },
];

fn hash_source(code: &str) -> u64 {
    let mut h = DefaultHasher::new();
    code.hash(&mut h);
    h.finish()
}

fn cache_dir() -> PathBuf {
    std::env::temp_dir().join("halo_parity_cache")
}

/// Ruta del binario `halo` que Cargo construye para esta suite.
fn halo_bin() -> &'static str {
    env!("CARGO_BIN_EXE_halo")
}

pub fn clang_available() -> bool {
    Command::new("clang")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Quita códigos ANSI (`\x1b[…m`) para poder comparar mensajes.
fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            for c2 in chars.by_ref() {
                if c2 == 'm' { break; }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn write_source(dir: &Path, hash: u64, code: &str) -> PathBuf {
    std::fs::create_dir_all(dir).expect("create cache dir");
    let src = dir.join(format!("{hash:016x}.halo"));
    std::fs::write(&src, code).expect("write source");
    src
}

/// Ejecuta `code` con el intérprete (`halo run`).
pub fn run_interpreter(code: &str) -> RunResult {
    let dir = cache_dir();
    let src = write_source(&dir, hash_source(code), code);
    let out = Command::new(halo_bin())
        .arg("run")
        .arg(&src)
        .output()
        .expect("spawn halo run");
    RunResult {
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: strip_ansi(&String::from_utf8_lossy(&out.stderr)),
        exit_code: out.status.code(),
    }
}

/// Compila `code` a binario nativo y lo ejecuta. `Err` explica el fallo.
pub fn run_compiled(code: &str) -> Result<RunResult, String> {
    let dir = cache_dir();
    let hash = hash_source(code);
    write_source(&dir, hash, code);
    let ll = dir.join(format!("{hash:016x}.ll"));
    let bin = dir.join(format!("{hash:016x}"));

    // Compilar solo si el binario aún no está en caché.
    if !bin.exists() {
        let tokens = tokenize(code);
        let program = Parser::new(tokens)
            .parse()
            .map_err(|e| format!("parse: {e:?}"))?;
        let name = format!("parity_{hash:016x}");
        let mut comp = Compilation::new(&name);
        comp.codegen()
            .compile(&program)
            .map_err(|e| format!("codegen: {e}"))?;
        comp.emit_llvm(ll.to_str().unwrap())
            .map_err(|e| format!("emit_llvm: {e}"))?;
        let status = Command::new("clang")
            .arg(ll.to_str().unwrap())
            .arg("-O0")
            .arg("-o")
            .arg(bin.to_str().unwrap())
            .arg("-lm")
            .status()
            .map_err(|e| format!("clang: {e}"))?;
        if !status.success() {
            return Err("clang linking failed".to_string());
        }
    }

    let out = Command::new(&bin).output().expect("run compiled binary");
    Ok(RunResult {
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: strip_ansi(&String::from_utf8_lossy(&out.stderr)),
        exit_code: out.status.code(),
    })
}

fn tokenize(code: &str) -> Vec<halo::lexer::Token> {
    let mut lexer = Lexer::new(code.to_string());
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        let eof = tok.kind == TokenKind::Eof;
        tokens.push(tok);
        if eof { break; }
    }
    tokens
}

pub fn both_modes(code: &str) -> BothModes {
    let interpreter = run_interpreter(code);
    let (compiled, note) = match run_compiled(code) {
        Ok(r) => (Some(r), None),
        Err(e) => (None, Some(e)),
    };
    BothModes { interpreter, compiled, compiled_note: note }
}

/// Compara ambos modos aplicando `table`. Devuelve `Err` con un mensaje
/// explicativo si hay una divergencia no declarada o una entrada obsoleta.
pub fn compare(id: &str, code: &str, table: &[KnownDivergence]) -> Result<(), String> {
    if !clang_available() {
        eprintln!("skipping parity test '{id}': clang not available");
        return Ok(());
    }

    let modes = both_modes(code);
    let declared = table.iter().find(|d| d.id == id);

    // Igualdad fuerte: mismo stdout, mismo exit code y, si ambos fallan,
    // mismo mensaje de error (normalizado sin ANSI). Así no se puede colar
    // un "ambos fallan pero por motivos distintos" como si fuera paridad.
    let same = match &modes.compiled {
        Some(c) => {
            modes.interpreter.stdout == c.stdout
                && modes.interpreter.exit_code == c.exit_code
                && modes.interpreter.is_error() == c.is_error()
                && (!modes.interpreter.is_error() || modes.interpreter.stderr == c.stderr)
        }
        None => false,
    };

    match (declared, same) {
        (None, true) => Ok(()),
        (None, false) => Err(format!(
            "undocumented divergence for '{id}'\n  interpreter: stdout={:?} error={} stderr={:?}\n  compiled:    {}\n  note: {}",
            modes.interpreter.stdout,
            modes.interpreter.is_error(),
            modes.interpreter.stderr,
            match &modes.compiled {
                Some(c) => format!("stdout={:?} error={}", c.stdout, c.is_error()),
                None => "not compiled".to_string(),
            },
            modes.compiled_note.clone().unwrap_or_else(|| "-".to_string()),
        )),
        (Some(d), false) => {
            let _ = d; // divergencia esperada por ahora
            Ok(())
        }
        (Some(d), true) => Err(format!(
            "stale entry for '{id}': it no longer diverges (was declared for {}); remove it from KNOWN_DIVERGENCES",
            d.phase
        )),
    }
}

pub fn assert_parity(id: &str, code: &str) {
    compare(id, code, KNOWN_DIVERGENCES)
        .unwrap_or_else(|e| panic!("{e}\n--- source ---\n{code}"));
}
