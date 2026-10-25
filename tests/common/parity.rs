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
use std::fmt;
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

/// La fase del programa de paridad que debe cerrar una divergencia.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Valores y formato: `true/false`, float mínimo, `null`, `str()` en
    /// runtime, `print` multi-argumento, booleanos y comparaciones impresos.
    P1,
    /// Errores detectados en fases distintas (compilación vs. runtime) y
    /// errores de runtime: overflow, división por cero, índice.
    P2,
    /// Arrays en modo compilado: literales, indexación, métodos, `for`.
    P3,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Phase::P1 => write!(f, "P1"),
            Phase::P2 => write!(f, "P2"),
            Phase::P3 => write!(f, "P3"),
        }
    }
}

/// Una divergencia conocida entre el intérprete y el compilador.
pub struct KnownDivergence {
    pub id: &'static str,
    /// Por qué divergen, en una frase.
    pub reason: &'static str,
    /// La fase que debe cerrar esta divergencia.
    pub phase: Phase,
}

/// Un caso del corpus de paridad: un programa Halo y el resultado que se
/// espera de ambos backends.
///
/// El caso **es** el registro: la divergencia esperada vive dentro del caso,
/// de modo que no puede existir una divergencia declarada sin un caso que la
/// ejercite (spec §3.1).
pub struct ParityCase {
    /// Identificador estable. Es también el nombre del test.
    pub id: &'static str,
    /// Programa Halo que se ejecuta en el intérprete y en un binario nativo.
    pub code: &'static str,
    /// `None` = ambos modos deben coincidir. `Some(d)` = se espera una
    /// divergencia, con el motivo y la fase que la cierran.
    pub divergence: Option<KnownDivergence>,
    /// Features del catálogo (`src/lang/catalog.rs`) que este caso ejercita.
    /// Los nombres son exactos: `parity_coverage_tests` los verifica.
    pub covers: &'static [&'static str],
}

/// Huella del ejecutable de test (mtime + tamaño).
///
/// El binario que `run_compiled` produce lo genera el codegen **enlazado en
/// este ejecutable de test**, no el binario `halo`. Cuando el codegen cambia,
/// este ejecutable se reconstruye, cambia su mtime y la huella cambia: la
/// caché se invalida sola y el caso se vuelve a compilar de verdad.
///
/// Sin esta huella, la clave de la caché sería solo el hash del fuente y un
/// codegen nuevo reutilizaría el binario del viejo: la red "pasaría" sin
/// haber medido nada.
pub fn compiler_fingerprint() -> u64 {
    let mut h = DefaultHasher::new();
    let meta = std::env::current_exe()
        .ok()
        .and_then(|p| std::fs::metadata(&p).ok());
    match meta {
        Some(meta) => {
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0);
            mtime.hash(&mut h);
            meta.len().hash(&mut h);
        }
        None => 0u64.hash(&mut h),
    }
    h.finish()
}

/// Directorio de caché para una huella dada. Dos huellas distintas nunca
/// comparten directorio.
pub fn cache_dir_for(fingerprint: u64) -> PathBuf {
    std::env::temp_dir()
        .join("halo_parity_cache")
        .join(format!("{fingerprint:016x}"))
}

/// Directorio de caché de esta ejecución.
pub fn cache_dir() -> PathBuf {
    cache_dir_for(compiler_fingerprint())
}

fn hash_source(code: &str) -> u64 {
    let mut h = DefaultHasher::new();
    code.hash(&mut h);
    h.finish()
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
                if c2 == 'm' {
                    break;
                }
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
        let program = Parser::new(tokens).parse().map_err(|errs| {
            let joined = errs
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ");
            format!("parse: {joined}")
        })?;
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
        if eof {
            break;
        }
    }
    tokens
}

pub fn both_modes(code: &str) -> BothModes {
    let interpreter = run_interpreter(code);
    let (compiled, note) = match run_compiled(code) {
        Ok(r) => (Some(r), None),
        Err(e) => (None, Some(e)),
    };
    BothModes {
        interpreter,
        compiled,
        compiled_note: note,
    }
}

/// Compara ambos modos. `declared` es la divergencia esperada, si la hay.
/// Devuelve `Err` con un mensaje explicativo si hay una divergencia no
/// declarada o una entrada obsoleta.
pub fn compare(id: &str, code: &str, declared: Option<&KnownDivergence>) -> Result<(), String> {
    if !clang_available() {
        eprintln!("skipping parity test '{id}': clang not available");
        return Ok(());
    }

    let modes = both_modes(code);

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
        (Some(_), false) => Ok(()), // divergencia esperada por ahora
        (Some(d), true) => Err(format!(
            "stale entry for '{id}': it no longer diverges (was declared for {}); remove it from the case",
            d.phase
        )),
    }
}

/// Compara ambos modos para un caso del corpus.
pub fn compare_case(case: &ParityCase) -> Result<(), String> {
    compare(case.id, case.code, case.divergence.as_ref())
}

/// Busca un caso del corpus por su id.
pub fn case(id: &str) -> Option<&'static ParityCase> {
    super::cases::CASES.iter().find(|c| c.id == id)
}

/// Verifica un caso del corpus por su id.
///
/// # Panics
///
/// Entra en pánico si el id no existe en la tabla o si el caso no se
/// comporta como se espera.
pub fn assert_case(id: &str) {
    let case = case(id).unwrap_or_else(|| panic!("unknown parity case '{id}'"));
    compare_case(case).unwrap_or_else(|e| panic!("{e}\n--- source ---\n{}", case.code));
}
