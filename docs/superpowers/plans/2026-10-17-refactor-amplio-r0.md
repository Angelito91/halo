# Refactor amplio (R0) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactorizar todo el proyecto Halo (intérprete, compilador, parser, CLI, diagnostics) a una estructura modular limpia y con una fuente única de verdad, **sin cambiar ni una salida observable**, dejando lista la red de congelado que verifica la paridad intérprete ↔ compilador.

**Architecture:** Primero se construye el arnés diferencial (ejecuta un snippet en ambos modos y compara comportamiento, con tabla de divergencias conocidas anti-mentira). Después se introduce `src/lang/` como fuente única descriptiva (catálogo, formato, errores). Luego se parten los archivos gigantes por responsabilidad, un módulo por tarea, con la suite verde y el corpus intacto en cada commit. Cierra la poda de código muerto y la sincronización de docs.

**Tech Stack:** Rust 2021, inkwell (git master, `llvm23-1`) sobre LLVM 23 del sistema, clang para el enlazado, cargo test. Sin dependencias nuevas.

**Spec:** `docs/superpowers/specs/2026-10-17-refactor-amplio-y-paridad-design.md`

## Global Constraints

- **Cero cambios de comportamiento.** Ningún commit de R0 cambia una salida byte a byte. Los bugs conocidos del spec (§1.1) **no se arreglan aquí**; se difieren a P1/P2/P3.
- **Fachada pública estable**: se conservan `halo::lexer`, `halo::parser`, `halo::interpreter`, `halo::compiler`, `halo::diagnostics` (los usan `tests/` y `src/cli/`).
- **Cada commit deja `cargo test` verde** (baseline: `784 passed; 0 failed; 1 ignored`) y `cargo build` **sin warnings**.
- **Sin warnings nuevos de clippy** en el código tocado (`cargo clippy --lib`).
- Trabajo sobre `main`; **commits lógicos**, mensajes en inglés (conventional commits); documentación en español.
- Ninguna divergencia queda "pendiente" sin estar en `KNOWN_DIVERGENCES` (spec §3.1).
- Nada de duplicar fuente de verdad: nombres, aridades, formatos y mensajes usados por ambos modos salen de `src/lang/`.
- El corpus y sus salidas esperadas se copian **verbatim** del spec §1.1 (no inventar casos nuevos en R0).

## Review Focus

Riesgos que el spec implica y que ningún test de las tareas cubre por sí solo:

1. **Salidas que el arnés no compara: `halo tokens` / `halo ast` / `run` / `check` y el orden de los diagnósticos.** Al partir `cli/commands.rs` y `parser.rs` se puede cambiar el *orden* de tokens/items sin cambiar `stdout` del intérprete. Task 7 crea `tests/cli_output_tests.rs` fijando `tokens` y `ast`; Task 8 lo extiende a `run` y `check`. `halo llvm` queda **deliberadamente sin fijar**: el texto del IR no forma parte del contrato congelado (cambia legítimamente con LLVM).
2. **Errores y exit codes.** El arnés compara `is_error`, no el texto. Una tarea no debe cambiar el texto de un mensaje de error existente. Los textos ya están fijados por `tests/error_codes_tests.rs` (Task 9), `tests/style_checker_tests.rs` (Task 9) y `tests/common` (`assert_runtime_error` / `assert_parse_error`, que ejecutan Tasks 3–7). Ninguna tarea añade un test de texto nuevo: su obligación es que esos sigan verdes. Si un refactor obliga a mover un test, se actualiza la ruta, **nunca el texto**.
3. **Catálogo incompleto.** El catálogo puede declarar nombres que el backend no implementa (o al revés). Task 2 añade `catalog_matches_observed_behavior`, que compara cada entrada del catálogo contra el comportamiento observado en ambos modos.
4. **Fachada pública rota.** Un `pub` que se pierde al partir archivos rompe consumidores externos (CLI y tests de integración). Tasks 3–9 terminan cada una con el **tally de `cargo test` completo** (todas las suites de integración), no solo con las suites que la tarea toca.
5. **clang ausente.** En una máquina sin clang, los tests del arnés deben **saltarse**, no fallar. Task 1 lo implementa (`clang_available()` + guardas en cada test) y lo prueba.

---

## File Structure

**Creados:**
- `tests/common/parity.rs` — arnés diferencial + tabla de divergencias.
- `tests/parity_freeze_tests.rs` — corpus de congelado (una prueba por caso).
- `tests/lang_catalog_tests.rs` — catálogo ↔ comportamiento.
- `src/lang/{mod.rs,catalog.rs,format.rs,errors.rs}` — fuente única.
- `src/interpreter/evaluator/{statements.rs,expressions.rs,calls.rs}` y `evaluator/methods/{mod.rs,strings.rs,arrays.rs}`.
- `src/compiler/codegen/builtins/strings/{mod.rs,helpers.rs,trim.rs,replace.rs,substr.rs,search.rs,case.rs}`.
- `src/compiler/codegen/{module.rs,functions.rs}`.
- `src/parser/{core.rs,declarations.rs,statements.rs,expressions.rs}`.
- `src/cli/{run.rs,build.rs,check.rs,tokens.rs,ast.rs,llvm.rs,helpers.rs}`.
- `src/diagnostics/error_codes/{mod.rs,mapping.rs,descriptions.rs,category.rs}`.

**Modificados:** `src/lib.rs`, `src/interpreter/evaluator/{core.rs,mod.rs,methods.rs}`, `src/compiler/codegen/generator.rs`, `src/compiler/codegen/builtins/{mod.rs,strings.rs}`, `src/parser/{parser.rs,mod.rs}`, `src/cli/{mod.rs,commands.rs}`, `src/diagnostics/{mod.rs,error_codes.rs}`, `tests/common/mod.rs`, `COMPILER_REFACTORING.md`, `REFACTORING_SUMMARY.md`, `LSP_ARCHITECTURE.md`, `LSP_IMPLEMENTATION_GUIDE.md`, `README.md`.

**Eliminados:** `src/interpreter/evaluator/methods.rs`, `src/compiler/codegen/builtins/strings.rs`, `src/parser/parser.rs`, `src/cli/commands.rs`, `src/diagnostics/error_codes.rs` (su contenido se reparte; las rutas nuevas son las de arriba).

---

### Task 1: Arnés diferencial y tabla de divergencias (R0.0)

**Files:**
- Create: `tests/common/parity.rs`
- Modify: `tests/common/mod.rs` (añadir `pub mod parity;` y quitar el `#![allow(dead_code)]` duplicado si molesta)
- Create: `tests/parity_freeze_tests.rs`

**Interfaces:**
- Consumes: `halo::lexer::Lexer`, `halo::lexer::TokenKind`, `halo::parser::Parser`, `halo::compiler::Compilation`.
- Produces (usado por todas las tareas siguientes y por P0–P3):
  - `pub struct RunResult { pub stdout: String, pub stderr: String, pub exit_code: Option<i32> }`
  - `pub struct BothModes { pub interpreter: RunResult, pub compiled: Option<RunResult>, pub compiled_note: Option<String> }`
  - `pub struct KnownDivergence { pub id: &'static str, pub reason: &'static str, pub phase: &'static str }`
  - `pub const KNOWN_DIVERGENCES: &[KnownDivergence]`
  - `pub fn clang_available() -> bool`
  - `pub fn both_modes(code: &str) -> BothModes`
  - `pub fn compare(id: &str, code: &str, table: &[KnownDivergence]) -> Result<(), String>`
  - `pub fn assert_parity(id: &str, code: &str)`

- [x] **Step 1: Escribir los tests que fallan (el arnés aún no existe)**

`tests/common/mod.rs`, añadir al final de las declaraciones de módulo (arriba del archivo, junto a los `use`):

```rust
pub mod parity;
```

`tests/common/parity.rs` (nuevo, esqueleto con la API vacía para que el test compile en rojo):

```rust
//! Arnés diferencial: ejecuta un snippet en el intérprete y en un binario
//! nativo y compara comportamiento observable.
#![allow(dead_code)]

pub struct RunResult { pub stdout: String, pub stderr: String, pub exit_code: Option<i32> }
pub struct BothModes { pub interpreter: RunResult, pub compiled: Option<RunResult>, pub compiled_note: Option<String> }
pub struct KnownDivergence { pub id: &'static str, pub reason: &'static str, pub phase: &'static str }

pub const KNOWN_DIVERGENCES: &[KnownDivergence] = &[];

pub fn clang_available() -> bool { unimplemented!() }
pub fn both_modes(_code: &str) -> BothModes { unimplemented!() }
pub fn compare(_id: &str, _code: &str, _table: &[KnownDivergence]) -> Result<(), String> { unimplemented!() }
pub fn assert_parity(id: &str, code: &str) {
    compare(id, code, KNOWN_DIVERGENCES).unwrap_or_else(|e| panic!("{e}\n--- source ---\n{code}"));
}
```

`tests/parity_freeze_tests.rs` (nuevo):

```rust
mod common;

use common::parity::{self, KnownDivergence};

#[test]
fn harness_flags_an_undeclared_divergence() {
    if !parity::clang_available() { return; }
    let err = parity::compare("bool_print", "print(true)", &[]).unwrap_err();
    assert!(err.contains("undocumented divergence"), "{err}");
}

#[test]
fn harness_accepts_a_declared_divergence() {
    if !parity::clang_available() { return; }
    parity::compare("bool_print", "print(true)", parity::KNOWN_DIVERGENCES).unwrap();
}

#[test]
fn harness_flags_a_stale_entry() {
    if !parity::clang_available() { return; }
    let table = &[KnownDivergence { id: "int_truncation", reason: "test", phase: "P0" }];
    let err = parity::compare("int_truncation", "print(int(3.9))", table).unwrap_err();
    assert!(err.contains("stale"), "{err}");
}

#[test]
fn harness_reports_parity_for_an_agreeing_case() {
    if !parity::clang_available() { return; }
    parity::compare("int_truncation", "print(int(3.9))", &[]).unwrap();
}

// Corpus congelado: una prueba por caso (ver el plan, tabla de corpus).
#[test] fn case_bool_print() { parity::assert_parity("bool_print", "print(true)\nprint(false)"); }
#[test] fn case_float_print() { parity::assert_parity("float_print", "print(5.0)\nprint(3.14)\nprint(1.0 / 4.0)"); }
#[test] fn case_str_runtime_int() { parity::assert_parity("str_runtime_int", "n = 7\nprint(str(n))\nprint(\"n=\" + str(n))"); }
#[test] fn case_str_runtime_float() { parity::assert_parity("str_runtime_float", "x = 1.5\nprint(str(x))"); }
#[test] fn case_str_runtime_bool() { parity::assert_parity("str_runtime_bool", "b = true\nprint(str(b))"); }
#[test] fn case_int_from_string() { parity::assert_parity("int_from_string", "print(int(\"42\"))"); }
#[test] fn case_float_from_string() { parity::assert_parity("float_from_string", "print(float(\"1.5\"))"); }
#[test] fn case_int_truncation() { parity::assert_parity("int_truncation", "print(int(3.9))\nprint(int(-2.7))"); }
#[test] fn case_abs_float() { parity::assert_parity("abs_float", "print(abs(-3.14))\nprint(abs(-7))"); }
#[test] fn case_array_literal() { parity::assert_parity("array_literal", "a = [1, 2, 3]\nprint(len(a))\nprint(a[0])\nprint(a[2])"); }
#[test] fn case_array_negative_idx() { parity::assert_parity("array_negative_idx", "a = [1, 2, 3]\nprint(a[-1])"); }
#[test] fn case_array_methods() { parity::assert_parity("array_methods", "a = [1, 2, 3]\na.push(4)\nprint(len(a))\nprint(a.last())\na.reverse()\nprint(a[0])"); }
#[test] fn case_array_sort() { parity::assert_parity("array_sort", "a = [3, 1, 2]\na.sort()\nprint(a[0])"); }
#[test] fn case_array_join() { parity::assert_parity("array_join", "a = [\"x\", \"y\"]\nprint(a.join(\"-\"))"); }
#[test] fn case_for_over_array() { parity::assert_parity("for_over_array", "for v in [10, 20] { print(v) }"); }
#[test] fn case_range_as_value() { parity::assert_parity("range_as_value", "r = range(3)\nprint(len(r))\nprint(r[1])"); }
#[test] fn case_string_param() { parity::assert_parity("string_param", "greet(name) { print(name + \"!\") }\ngreet(\"hola\")"); }
#[test] fn case_string_concat_num() { parity::assert_parity("string_concat_num", "print(\"n=\" + 5)"); }
#[test] fn case_string_repeat() { parity::assert_parity("string_repeat", "print(\"ab\" * 3)"); }
#[test] fn case_int_overflow() { parity::assert_parity("int_overflow", "print(9223372036854775807 + 1)"); }
#[test] fn case_div_by_zero() { parity::assert_parity("div_by_zero", "print(1 / 0)"); }
#[test] fn case_mod_negative() { parity::assert_parity("mod_negative", "print(-7 % 3)\nprint(7 % -3)"); }
#[test] fn case_mixed_equality() { parity::assert_parity("mixed_equality", "print(1 == \"1\")\nprint(true == 1)"); }
#[test] fn case_print_multiarg() { parity::assert_parity("print_multiarg", "print(\"a\", \"b\", 1)"); }
#[test] fn case_print_noargs() { parity::assert_parity("print_noargs", "print()"); }
#[test] fn case_len_number() { parity::assert_parity("len_number", "print(len(12345))"); }
#[test] fn case_type_of_int() { parity::assert_parity("type_of_int", "print(type(42))"); }
#[test] fn case_string_methods() { parity::assert_parity("string_methods", "s = \" Hola \"\nprint(s.trim())\nprint(s.upper())\nprint(\"abc\".contains(\"b\"))"); }
#[test] fn case_nested_call_str() { parity::assert_parity("nested_call_str", "f(n) { return str(n) }\nprint(\"v=\" + f(3))"); }
#[test] fn case_global_mutation() { parity::assert_parity("global_mutation", "x = 1\nf() { x = 2 }\nf()\nprint(x)"); }
#[test] fn case_return_none() { parity::assert_parity("return_none", "f() { }\nprint(f())"); }
#[test] fn case_early_return_loop() { parity::assert_parity("early_return_loop", "f() {\nfor i in range(5) { if i == 2 { return i } }\nreturn -1\n}\nprint(f())"); }
#[test] fn case_bool_var_decl() { parity::assert_parity("bool_var_decl", "b = true\nprint(b)\nprint(\"ok\")"); }
#[test] fn case_println_builtin() { parity::assert_parity("println_builtin", "println(\"hola\")"); }
```

- [x] **Step 2: Ejecutar y ver el rojo esperado**

Run: `cargo test --test parity_freeze_tests 2>&1 | tail -20`
Expected: los tests fallan con `not implemented` (`unimplemented!()`) — el arnés no existe todavía.

- [x] **Step 3: Implementar el arnés**

Reemplazar el cuerpo de `tests/common/parity.rs`:

```rust
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
use std::path::PathBuf;
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

fn write_source(dir: &PathBuf, hash: u64, code: &str) -> PathBuf {
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
```

- [x] **Step 4: Ejecutar y ver el verde**

Run: `cargo test --test parity_freeze_tests`
Expected: PASS (38 tests: 34 del corpus + 4 del arnés). La primera ejecución compila 33 binarios con clang (~30 s); las siguientes salen de caché.

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: `822 0` (baseline 784 + 38 pruebas nuevas de este archivo).

- [x] **Step 5: Probar que el arnés no miente (mutación temporal)**

Añadir temporalmente al corpus una prueba con una divergencia no declarada:

```rust
#[test] fn tmp_mutation_probe() { parity::assert_parity("tmp_new_divergence", "print(true)"); }
```

Run: `cargo test --test parity_freeze_tests tmp_mutation_probe`
Expected: FALLA con `undocumented divergence for 'tmp_new_divergence'`. **Quitar la prueba temporal** y volver a ejecutar para confirmar verde.

- [x] **Step 6: Commit**

```bash
git add tests/common/mod.rs tests/common/parity.rs tests/parity_freeze_tests.rs
git commit -m "test: freeze interpreter/compiler behavior with a differential harness

Adds a both-modes harness (interpreter subprocess vs compiled binary) with an
anti-lie KNOWN_DIVERGENCES table: an undeclared divergence fails, and a
declared one that stops diverging fails too. Corpus of 34 measured cases."
```

---

### Task 2: `src/lang/` como fuente única descriptiva (R0.1)

**Files:**
- Create: `src/lang/mod.rs`, `src/lang/catalog.rs`, `src/lang/format.rs`, `src/lang/errors.rs`
- Modify: `src/lib.rs` (añadir `pub mod lang;`)
- Create: `tests/lang_catalog_tests.rs`

**Interfaces:**
- Consumes: `tests/common/parity.rs` (Task 1).
- Produces (usado por P1/P2 y por los tests):
  - `lang::catalog::{Builtin, Method, Support, BUILTINS, METHODS, builtin, method}`
  - `lang::format::{format_bool, format_number, format_float, format_null, TYPE_NAMES}` (reglas del intérprete, canónicas)
  - `lang::errors::{DIVISION_BY_ZERO, INTEGER_OVERFLOW, NEGATIVE_INDEX, UNDEFINED_VARIABLE, UNDEFINED_FUNCTION}` (constantes `&str`)

- [x] **Step 1: Escribir el test que falla**

`tests/lang_catalog_tests.rs` (nuevo):

```rust
mod common;

use common::parity;
use halo::lang::catalog::{builtin, method, Support, BUILTINS, METHODS};

#[test]
fn catalog_has_no_duplicate_names() {
    let mut names: Vec<&str> = BUILTINS.iter().map(|b| b.name).collect();
    names.sort_unstable();
    let mut dedup = names.clone();
    dedup.dedup();
    assert_eq!(names, dedup, "duplicate builtin names in catalog");
}

#[test]
fn catalog_declares_the_measured_support() {
    // Verificado contra el probe del spec §1.1:
    assert_eq!(builtin("print").unwrap().support, Support::Both);
    assert_eq!(builtin("range").unwrap().support, Support::InterpreterOnly);
    assert_eq!(builtin("range").unwrap().arity, 1..=2);
    assert_eq!(builtin("println").unwrap().support, Support::CompilerOnly);
    assert_eq!(builtin("len").unwrap().support, Support::Both);
    assert_eq!(builtin("push").unwrap().support, Support::InterpreterOnly);
    assert_eq!(method("join").unwrap().support, Support::InterpreterOnly);
    assert_eq!(method("upper").unwrap().support, Support::Both);
    assert_eq!(method("substr").unwrap().arity, 2..=2);
    assert!(builtin("nope").is_none());
}

#[test]
fn catalog_matches_observed_behavior() {
    if !parity::clang_available() { return; }
    // Cada builtin declarado Both debe coincidir en los dos modos;
    // cada InterpreterOnly debe funcionar en el intérprete y no compilar.
    let cases: &[(&str, &str)] = &[
        ("print", "print(1)"),
        ("len", "print(len(\"ab\"))"),
        ("str", "print(str(1))"),
        ("int", "print(int(1.9))"),
        ("float", "print(float(1))"),
        ("abs", "print(abs(-1))"),
        ("type", "print(type(1))"),
        ("range", "print(len(range(2)))"),
        ("push", "a = [1]\na.push(2)\nprint(len(a))"),
    ];
    for (name, code) in cases {
        let support = builtin(name).unwrap().support;
        let interpreter_ok = !parity::run_interpreter(code).is_error();
        let compiled_ok = parity::run_compiled(code).map(|r| !r.is_error()).unwrap_or(false);
        match support {
            Support::Both => assert!(interpreter_ok && compiled_ok, "{name}: declared Both"),
            Support::InterpreterOnly => assert!(interpreter_ok && !compiled_ok, "{name}: declared InterpreterOnly"),
            Support::CompilerOnly => assert!(!interpreter_ok && compiled_ok, "{name}: declared CompilerOnly"),
        }
    }
}
```

- [x] **Step 2: Ejecutar y ver el rojo esperado**

Run: `cargo test --test lang_catalog_tests 2>&1 | head -20`
Expected: FAIL — `unresolved import halo::lang` / `unresolved import common::parity` (el módulo no existe).

- [x] **Step 3: Implementar `src/lang/`**

`src/lib.rs`: añadir junto a los otros módulos:

```rust
pub mod lang;
```

`src/lang/mod.rs`:

```rust
//! Single source of truth for language-level metadata shared by the
//! interpreter and the compiler: builtin/method catalog, value formatting
//! rules and runtime error messages.
//!
//! In R0 this module is *descriptive*: it records how each backend behaves
//! today. P1/P2 make the backends consume it, which is when their observable
//! behavior is allowed to change.

pub mod catalog;
pub mod errors;
pub mod format;
```

`src/lang/catalog.rs`:

```rust
//! Builtin and method catalog. `support` mirrors the measured behavior of
//! 2026-10-17 (spec §1.1); flipping a flag is a behavior change and belongs
//! to the parity phases.

use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Support {
    Both,
    InterpreterOnly,
    CompilerOnly,
}

#[derive(Debug, Clone, Copy)]
pub struct Builtin {
    pub name: &'static str,
    pub arity: RangeInclusive<usize>,
    pub support: Support,
}

#[derive(Debug, Clone, Copy)]
pub struct Method {
    pub name: &'static str,
    pub arity: RangeInclusive<usize>,
    pub support: Support,
}

use Support::{Both, CompilerOnly, InterpreterOnly};

pub const BUILTINS: &[Builtin] = &[
    Builtin { name: "print",       arity: 0..=usize::MAX, support: Both },
    Builtin { name: "println",     arity: 0..=usize::MAX, support: CompilerOnly },
    Builtin { name: "len",         arity: 1..=1, support: Both },
    Builtin { name: "str",         arity: 1..=1, support: Both },
    Builtin { name: "int",         arity: 1..=1, support: Both },
    Builtin { name: "float",       arity: 1..=1, support: Both },
    Builtin { name: "abs",         arity: 1..=1, support: Both },
    Builtin { name: "type",        arity: 1..=1, support: Both },
    Builtin { name: "range",       arity: 1..=2, support: InterpreterOnly },
    Builtin { name: "push",        arity: 2..=2, support: InterpreterOnly },
    Builtin { name: "pop",         arity: 1..=1, support: InterpreterOnly },
    Builtin { name: "append",      arity: 2..=2, support: InterpreterOnly },
    Builtin { name: "first",       arity: 1..=1, support: InterpreterOnly },
    Builtin { name: "last",        arity: 1..=1, support: InterpreterOnly },
    Builtin { name: "reverse",     arity: 1..=1, support: Both },
    Builtin { name: "join",        arity: 2..=2, support: InterpreterOnly },
    Builtin { name: "sort",        arity: 1..=1, support: InterpreterOnly },
    Builtin { name: "contains",    arity: 2..=2, support: Both },
    Builtin { name: "index_of",    arity: 2..=2, support: Both },
    Builtin { name: "upper",       arity: 1..=1, support: Both },
    Builtin { name: "lower",       arity: 1..=1, support: Both },
    Builtin { name: "trim",        arity: 1..=1, support: Both },
    Builtin { name: "starts_with", arity: 2..=2, support: Both },
    Builtin { name: "ends_with",   arity: 2..=2, support: Both },
    Builtin { name: "replace",     arity: 3..=3, support: Both },
    Builtin { name: "substr",      arity: 3..=3, support: Both },
];

pub const METHODS: &[Method] = &[
    Method { name: "push",        arity: 1..=1, support: InterpreterOnly },
    Method { name: "pop",         arity: 0..=0, support: InterpreterOnly },
    Method { name: "first",       arity: 0..=0, support: InterpreterOnly },
    Method { name: "last",        arity: 0..=0, support: InterpreterOnly },
    Method { name: "join",        arity: 1..=1, support: InterpreterOnly },
    Method { name: "sort",        arity: 0..=0, support: InterpreterOnly },
    Method { name: "reverse",     arity: 0..=0, support: Both },
    Method { name: "contains",    arity: 1..=1, support: Both },
    Method { name: "index_of",    arity: 1..=1, support: Both },
    Method { name: "upper",       arity: 0..=0, support: Both },
    Method { name: "lower",       arity: 0..=0, support: Both },
    Method { name: "trim",        arity: 0..=0, support: Both },
    Method { name: "starts_with", arity: 1..=1, support: Both },
    Method { name: "ends_with",   arity: 1..=1, support: Both },
    Method { name: "replace",     arity: 2..=2, support: Both },
    Method { name: "substr",      arity: 2..=2, support: Both },
];

pub fn builtin(name: &str) -> Option<&'static Builtin> {
    BUILTINS.iter().find(|b| b.name == name)
}

pub fn method(name: &str) -> Option<&'static Method> {
    METHODS.iter().find(|m| m.name == name)
}
```

`src/lang/format.rs`:

```rust
//! Canonical value-formatting rules. The interpreter's output is the
//! reference; P1 makes compiled binaries match it.

/// How the interpreter prints a boolean.
pub fn format_bool(b: bool) -> &'static str {
    if b { "true" } else { "false" }
}

/// How the interpreter prints an integer.
pub fn format_number(n: i64) -> String {
    n.to_string()
}

/// How the interpreter prints a float: minimal representation that always
/// keeps a decimal part (`5.0`, `0.25`).
pub fn format_float(f: f64) -> String {
    let s = format!("{f}");
    if s.contains('.') || s.contains('e') { s } else { format!("{s}.0") }
}

/// How the interpreter prints the null value.
pub fn format_null() -> &'static str {
    "null"
}

/// `type()` names by Halo type, as the interpreter reports them.
pub const TYPE_NAMES: &[(&str, &str)] = &[
    ("number", "number"),
    ("float", "float"),
    ("bool", "bool"),
    ("string", "string"),
    ("array", "array"),
    ("null", "null"),
];
```

`src/lang/errors.rs`:

```rust
//! Single table of runtime error messages. The interpreter produces these
//! today; P2 makes compiled binaries emit the same text.

pub const DIVISION_BY_ZERO: &str = "Division by zero";
pub const INTEGER_OVERFLOW: &str = "Integer overflow in";
pub const NEGATIVE_INDEX: &str = "Array index cannot be negative, got";
pub const UNDEFINED_VARIABLE: &str = "Undefined variable:";
pub const UNDEFINED_FUNCTION: &str = "Undefined function:";
```

- [x] **Step 4: Ejecutar y ver el verde**

Run: `cargo test --test lang_catalog_tests`
Expected: PASS (3 tests).

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: sin fallos; el total sube en 3 (de 822 a 825).

- [x] **Step 5: Verificar que no cambió nada observable**

Run: `cargo test 2>&1 | grep -cE "^test result: ok"`
Expected: todas las suites `ok` (ninguna `FAILED`).

- [x] **Step 6: Commit**

```bash
git add src/lib.rs src/lang tests/lang_catalog_tests.rs
git commit -m "refactor(lang): add a single descriptive source of truth for catalog, formatting and errors

Introduces src/lang/{catalog,format,errors}.rs recording today's support
matrix (Both / InterpreterOnly / CompilerOnly), the interpreter's canonical
value formatting and the runtime error strings, with tests asserting the
catalog against observed behavior in both modes."
```

---

### Task 3: Partir el evaluador del intérprete (R0.2a)

**Files:**
- Create: `src/interpreter/evaluator/statements.rs`, `expressions.rs`, `calls.rs`
- Modify: `src/interpreter/evaluator/core.rs`, `src/interpreter/evaluator/mod.rs`
- Test: `tests/control_flow_tests.rs`, `tests/scope_and_variable_tests.rs`, `tests/function_tests.rs` (sin cambios; son la red)

**Interfaces:**
- Consumes: nada nuevo.
- Produces (visible para Task 4): `Evaluator` conserva sus campos `pub` y los métodos siguen siendo accesibles desde los submódulos (`pub(crate)` como mínimo: `eval_stmt`, `eval_expr`, `eval_call`, `eval_args`).

- [x] **Step 1: Línea base**

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: `825 0` (el número exacto tras Task 2). Anotarlo.

- [x] **Step 2: Mover funciones a los submódulos**

`src/interpreter/evaluator/statements.rs` (nuevo) recibe, como `impl Evaluator { … }`:
`eval_program`, `eval_block`, `eval_stmt`, y la constante `MAX_LOOP_ITERATIONS` (hoy privada en `core.rs`; pasa a `pub(crate)`).

`src/interpreter/evaluator/expressions.rs` (nuevo) recibe:
`eval_expr`, `eval_binary`, `value_to_expression`, `undefined_variable_error`.

`src/interpreter/evaluator/calls.rs` (nuevo) recibe:
`eval_call`, `eval_args`, `undefined_function_error`, `get_call_stack`, `format_call_stack`.

Cada archivo empieza con:

```rust
use super::core::Evaluator;   // o `use super::Evaluator;` si mod.rs reexporta
use crate::parser::ast::{Block, Expression, Statement};
use crate::interpreter::value::Value;
```

`core.rs` conserva: el `struct Evaluator` con sus campos, `impl Default`, `Evaluator::new`, y las declaraciones `mod`/`pub use` que enlazan los submódulos.

- [x] **Step 3: Ajustar el árbol de módulos**

`src/interpreter/evaluator/mod.rs`: declarar los submódulos y reexportar lo que era público:

```rust
mod calls;
mod core;
mod expressions;
mod statements;

pub use core::Evaluator;
pub use statements::MAX_LOOP_ITERATIONS;  // solo si hace falta fuera
```

- [x] **Step 4: Compilar y arreglar visibilidad**

Run: `cargo build 2>&1 | grep -E "^(error|warning)" | head -30`
Expected: 0 errores, 0 warnings. Si aparece `private` en un método llamado desde otro submódulo, subirlo a `pub(crate)` **sin cambiar la lógica**.

- [x] **Step 5: Verificar que la red sigue verde y el total es idéntico**

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: exactamente el mismo total que Step 1, con `0` fallos.

Run: `cargo test --test parity_freeze_tests`
Expected: PASS (el corpus no cambia).

- [x] **Step 6: Commit**

```bash
git add src/interpreter/evaluator
git commit -m "refactor(interpreter): split the evaluator core into statement, expression and call modules

Pure move: eval_program/eval_block/eval_stmt, eval_expr/eval_binary and
eval_call/eval_args now live in statements.rs, expressions.rs and calls.rs.
No behavior change."
```

---

### Task 4: Partir los métodos del intérprete (R0.2b)

**Files:**
- Create: `src/interpreter/evaluator/methods/{mod.rs,strings.rs,arrays.rs}`
- Delete: `src/interpreter/evaluator/methods.rs` (tras mover su contenido)
- Modify: `src/interpreter/evaluator/mod.rs`

**Interfaces:**
- Consumes: `Evaluator` (Task 3).
- Produces: `methods::try_eval_method_call` y los `builtin_*` siguen en la misma ruta pública (`super::methods::builtin_range` etc. ya usados por `builtins.rs`).

- [x] **Step 1: Mover**

`methods/arrays.rs` recibe: `builtin_range`, `builtin_push`, `builtin_pop`, `builtin_append`, `builtin_first`, `builtin_last`, `builtin_reverse`, `builtin_join`, `builtin_sort`.

`methods/strings.rs` recibe: `builtin_contains`, `builtin_index_of`, `builtin_upper`, `builtin_lower`, `builtin_trim`, `builtin_starts_with`, `builtin_ends_with`, `builtin_replace`, `builtin_substr`, y los helpers `extract_string`, `extract_number`, `apply_string_transformer`, `apply_string_predicate`, `apply_multi_string_transformer`, `build_string_call_args`.

`methods/mod.rs` conserva el dispatcher `try_eval_method_call` y declara:

```rust
mod arrays;
mod strings;

pub use arrays::*;
pub use strings::*;
```

- [x] **Step 2: Verificar**

Run: `cargo build 2>&1 | grep -cE "^(error|warning)"`
Expected: `0`.

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: mismo total que Task 3, `0` fallos.

- [x] **Step 3: Commit**

```bash
git add -A src/interpreter/evaluator
git commit -m "refactor(interpreter): split array and string method implementations

methods.rs becomes methods/{mod,arrays,strings}.rs. The dispatcher stays in
mod.rs; no behavior change."
```

---

### Task 5: Partir los builtins de string del compilador (R0.3a)

**Files:**
- Create: `src/compiler/codegen/builtins/strings/{mod.rs,helpers.rs,trim.rs,replace.rs,substr.rs,search.rs,case.rs}`
- Delete: `src/compiler/codegen/builtins/strings.rs`
- Modify: `src/compiler/codegen/builtins/mod.rs` (las rutas `strings::emit_upper` etc. deben seguir resolviendo)

**Interfaces:**
- Consumes: nada nuevo.
- Produces: `builtins::strings::{emit_upper, emit_lower, emit_trim, emit_starts_with, emit_ends_with, emit_contains, emit_index_of, emit_reverse, emit_substr, emit_replace}` con firmas idénticas (`&mut CodeGenerator<'ctx>`, `&[Expression]`) → `Result<BasicValueEnum<'ctx>, String>`.

- [x] **Step 1: Mover**

`strings/helpers.rs` recibe: la macro `b!` (hoy `strings.rs:8`), `get_fn`, `get_strlen`, `get_malloc`, `get_memcpy`, `get_strcpy`, `get_toupper`, `get_tolower`, `get_isspace`, `get_strstr`, `get_strcmp`, `get_strncmp`, `expect_string`, `ptr_off`, `load_u8`, `store_u8`, `call_strlen`, `call_malloc`, `call_libc1`. Como la macro se usa en otros submódulos, declararla con `macro_rules!` y `pub(crate) use`:

```rust
macro_rules! b { ($gen:expr) => { &$gen.builder.builder }; }
pub(crate) use b;
```

`strings/case.rs`: `emit_upper`, `emit_lower`, `emit_reverse`.
`strings/trim.rs`: `emit_trim`.
`strings/search.rs`: `emit_starts_with`, `emit_ends_with`, `emit_contains`, `emit_index_of`.
`strings/substr.rs`: `emit_substr`.
`strings/replace.rs`: `emit_replace`.
`strings/mod.rs`:

```rust
mod case;
mod helpers;
mod replace;
mod search;
mod substr;
mod trim;

pub use case::*;
pub use replace::*;
pub use search::*;
pub use substr::*;
pub use trim::*;
```

- [x] **Step 2: Verificar que las salidas no cambian**

Run: `cargo build 2>&1 | grep -cE "^(error|warning)"`
Expected: `0`.

Run: `cargo test --test compiler_output_tests --test parity_freeze_tests`
Expected: PASS en ambos (los string methods compilados siguen imprimiendo igual).

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: mismo total que Task 4, `0` fallos (esto compila **todas** las suites de integración, no solo las dos de arriba).

- [x] **Step 3: Commit**

```bash
git add -A src/compiler/codegen/builtins
git commit -m "refactor(compiler): split the string builtins module by responsibility

strings.rs (1108 lines) becomes strings/{mod,helpers,trim,replace,substr,
search,case}.rs; the b! macro and libc helpers move to helpers.rs. No
behavior change."
```

---

### Task 6: Partir el generador del compilador (R0.3b)

**Files:**
- Create: `src/compiler/codegen/{module.rs,functions.rs}`
- Modify: `src/compiler/codegen/generator.rs`, `src/compiler/codegen/mod.rs`

**Interfaces:**
- Consumes: nada nuevo.
- Produces: `CodeGenerator` conserva su API pública (`new`, `compile`, `optimise`, `emit_llvm`, `emit_object`, `print_ir`, `get_module`, `generate_block`, `coerce_to_storage`, `coerce_to_bool`).

- [x] **Step 1: Mover**

`codegen/module.rs` recibe, todas como métodos de `impl CodeGenerator`: `generate_main`, `declare_printf`, `declare_string_runtime`, `declare_function`, `generate_function`, `generate_global_var`, `generate_global_var_typed`, `generate_const_expr`, `generate_block`, `generate_statement`.

`codegen/functions.rs` recibe las **siete funciones libres** de inferencia (hoy al final de `generator.rs`, sin `impl`): `determine_return_type`, `contains_string_op`, `stmt_has_string`, `expr_has_string`, `return_type_of_stmt`, `return_type_of_expr`, `is_constant_initializer`, todas `pub(crate)`.

`generator.rs` conserva: `struct CodeGenerator`, campos, `new`, `compile`, `optimise`, `emit_llvm`, `emit_object`, `print_ir`, `get_module`, `coerce_to_storage`, `coerce_to_bool`.

- [x] **Step 2: Verificar**

Run: `cargo build 2>&1 | grep -cE "^(error|warning)"`
Expected: `0`.

Run: `cargo test --test compiler_output_tests --test parity_freeze_tests`
Expected: PASS.

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: mismo total que Task 5, `0` fallos (compila todas las suites de integración).

- [x] **Step 3: Commit**

```bash
git add src/compiler/codegen
git commit -m "refactor(compiler): split code generation into module and function-signature units

Module/function emission moves to codegen/module.rs and the return-type
inference helpers (formerly loose functions in generator.rs) to
codegen/functions.rs. No behavior change."
```

---

### Task 7: Partir el parser (R0.4)

**Files:**
- Create: `src/parser/{core.rs,declarations.rs,statements.rs,expressions.rs}`
- Delete: `src/parser/parser.rs`
- Modify: `src/parser/mod.rs`

**Interfaces:**
- Consumes: nada nuevo.
- Produces: `Parser` conserva `new` y `parse`; el resto pasan a ser `pub(crate)`.

- [x] **Step 1: Mover**

`parser/core.rs`: `struct Parser`, `new`, `parse`, `parse_program`, y los helpers de cursor: `consume`, `advance_if`, `check`, `advance`, `skip_newlines`, `is_at_end`, `peek`, `previous`, `error`.

`parser/declarations.rs`: `parse_toplevel`, `parse_toplevel_identifier`, `parse_function`, `parse_global_var`, `parse_toplevel_expression`, `parse_toplevel_statement`.

`parser/statements.rs`: `parse_block`, `parse_statement`, `parse_if_statement`, `parse_while_statement`, `parse_for_statement`, `parse_return_statement`, `parse_var_decl`, `parse_expression_statement`.

`parser/expressions.rs`: `parse_expression`, `parse_logical_or`, `parse_logical_and`, `parse_equality`, `parse_comparison`, `parse_term`, `parse_factor`, `parse_unary`, `parse_postfix`, `parse_index_or_slice`, `parse_primary`.

`src/parser/mod.rs`:

```rust
mod core;
mod declarations;
mod expressions;
mod statements;

pub use core::Parser;
```

Los submódulos se importan entre sí con `use super::Parser;` (los campos de `Parser` ya son privados al módulo padre: ponerlos `pub(crate)` si hace falta).

- [x] **Step 2: Verificar (incluye orden de tokens/items, Review Focus #1)**

Crear primero el test que fija la salida de los subcomandos que dependen del parser, `tests/cli_output_tests.rs`:

```rust
use std::process::Command;

fn run_halo(sub: &str) -> String {
    let dir = std::env::temp_dir().join("halo_cli_output_tests");
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("prog.halo");
    std::fs::write(&src, "x = 1 + 2\nprint(x)\n").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_halo"))
        .arg(sub).arg(&src)
        .output().expect("spawn halo");
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
    assert!(out.contains("VarDecl"), "unexpected ast output: {out}");
    assert!(out.contains("print"), "unexpected ast output: {out}");
}
```

Run: `cargo test --test cli_output_tests` (antes de mover) → Expected: PASS.

Run: `cargo build 2>&1 | grep -cE "^(error|warning)"`
Expected: `0`.

Run: `cargo test --test parser_tests --test lexer_tests`
Expected: PASS sin cambios en `tests/`.

Run: `cargo test --test cli_output_tests` (después de mover) → Expected: PASS con el mismo texto.

Run: `cargo test --test parity_freeze_tests`
Expected: PASS.

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: mismo total que Task 6 **más 2** (los dos tests nuevos de la CLI), `0` fallos.

- [x] **Step 3: Commit**

```bash
git add -A src/parser tests/cli_output_tests.rs
git commit -m "refactor(parser): split the recursive-descent parser into core, declarations, statements and expressions

Pure move; Parser keeps its public new/parse entry points. Adds a CLI output
test pinning tokens/ast output so the split cannot change token or item order."
```

---

### Task 8: Partir la CLI (R0.5a)

**Files:**
- Create: `src/cli/{run.rs,build.rs,check.rs,tokens.rs,ast.rs,llvm.rs,helpers.rs}`
- Delete: `src/cli/commands.rs`
- Modify: `src/cli/mod.rs`, `src/cli/args.rs` (solo si hay que ajustar imports)

**Interfaces:**
- Consumes: nada nuevo.
- Produces: `cli::{run::cmd_run, build::cmd_build, check::cmd_check, tokens::cmd_tokens, ast::cmd_ast, llvm::cmd_llvm}` con las mismas firmas `fn(args: &Args) -> i32`; `cli::helpers::{link_with_clang, link_with_llc, toplevel_kind}`.

- [x] **Step 1: Mover y comprobar la salida de los subcomandos**

Mover `cmd_run`→`run.rs`, `cmd_build`+`link_with_clang`+`link_with_llc`→`build.rs` (los dos `link_*` van a `helpers.rs`, `build.rs` los importa), `cmd_check`→`check.rs`, `cmd_tokens`→`tokens.rs`, `cmd_ast`+`toplevel_kind`→`ast.rs`, `cmd_llvm`→`llvm.rs`.

`cli/mod.rs`:

```rust
pub mod args;
pub mod ast;
pub mod build;
pub mod check;
pub mod helpers;
pub mod llvm;
pub mod run;
pub mod tokens;
```

Antes de mover, los tests `tokens_subcommand_is_stable` / `ast_subcommand_is_stable` de `tests/cli_output_tests.rs` (creados en Task 7) son la red. Añadir dos más al mismo archivo, que cubren `run` y `check`:

```rust
#[test]
fn run_subcommand_is_stable() {
    let out = run_halo("run");
    assert_eq!(out, "3\n");
}

#[test]
fn check_subcommand_is_stable() {
    let out = run_halo("check");
    assert!(out.contains("No issues") || out.contains("ok") || out.is_empty(),
        "unexpected check output: {out}");
}
```

Run: `cargo test --test cli_output_tests` (antes de mover) → Expected: PASS. Ejecutarlo otra vez **después** de mover: mismo resultado.

- [x] **Step 2: Verificar**

Run: `cargo build 2>&1 | grep -cE "^(error|warning)"`
Expected: `0`.

Run: `cargo test --test cli_output_tests --test parity_freeze_tests`
Expected: PASS.

- [x] **Step 3: Commit**

```bash
git add -A src/cli tests/cli_output_tests.rs
git commit -m "refactor(cli): split commands into one module per subcommand

commands.rs becomes run/build/check/tokens/ast/llvm plus helpers; extends the
CLI output tests to run/check so the split cannot change their output."
```

---

### Task 9: Partir los error codes de diagnostics (R0.5b)

**Files:**
- Create: `src/diagnostics/error_codes/{mod.rs,mapping.rs,descriptions.rs,category.rs}`
- Delete: `src/diagnostics/error_codes.rs`
- Modify: `src/diagnostics/mod.rs` (la ruta `error_codes::ErrorCode` debe seguir resolviendo)

**Interfaces:**
- Consumes: nada nuevo.
- Produces: `diagnostics::error_codes::{ErrorCode, ErrorCategory, ErrorSeverity}` exactamente como hoy (+ los métodos `code`, `description`, `suggestions`, `category`, `severity`, `is_warning`, `is_error`).

- [x] **Step 1: Mover respetando los límites de Rust**

Un `enum` no se puede repartir entre archivos, así que el corte es **por método** (cada `impl ErrorCode` va en su archivo):

`error_codes/mod.rs`: `ErrorCategory`, `ErrorSeverity`, el `enum ErrorCode` completo, sus `Display`, y:

```rust
mod category;
mod descriptions;
mod mapping;

pub use category::*;
```

(Nota: `impl ErrorCode` en submódulos es válido porque el tipo vive en la misma crate.)

`error_codes/mapping.rs`: `impl ErrorCode { pub fn code(&self) -> &'static str { … } }`.
`error_codes/descriptions.rs`: `impl ErrorCode { pub fn description(&self) -> &'static str { … } pub fn suggestions(&self) -> Vec<&'static str> { … } }`.
`error_codes/category.rs`: `impl ErrorCode { pub fn category(&self) -> ErrorCategory { … } pub fn severity(…) … pub fn is_warning(…) … pub fn is_error(…) … }`.

Mover cada `match` **verbatim** (las armas son exhaustivas y no deben tocarse).

- [x] **Step 2: Verificar**

Run: `cargo build 2>&1 | grep -cE "^(error|warning)"`
Expected: `0`.

Run: `cargo test --test error_codes_tests --test style_checker_tests`
Expected: PASS con los mismos textos de diagnóstico.

- [x] **Step 3: Commit**

```bash
git add -A src/diagnostics
git commit -m "refactor(diagnostics): split the error-code catalog by method

error_codes.rs becomes error_codes/{mod,mapping,descriptions,category}.rs.
Each match moves verbatim; no diagnostic text changes."
```

---

### Task 10: Poda de código muerto y sincronización de docs (R0.6)

**Files:**
- Modify: `src/compiler/codegen/generator.rs`, `src/compiler/builder.rs`, `src/compiler/mod.rs`, `src/compiler/types.rs`, `src/interpreter/evaluator/core.rs`, `src/diagnostics/error_codes/*.rs` (retirar `#[allow(dead_code)]` obsoletos)
- Modify: `COMPILER_REFACTORING.md`, `REFACTORING_SUMMARY.md`, `LSP_ARCHITECTURE.md`, `LSP_IMPLEMENTATION_GUIDE.md`, `README.md`

**Interfaces:**
- Consumes: todo lo anterior.
- Produces: nada nuevo.

- [x] **Step 1: Inventariar el código muerto con evidencia**

Run:
```bash
grep -rn "PLANNED FOR v0.3.0" src/ | wc -l
grep -rn "allow(dead_code)" src/ | wc -l
```
Para **cada** item marcado "PLANNED FOR v0.3.0" (v0.3 está terminada) hay que
decidir: o (a) se usa → **quitar el atributo y el comentario**; o (b) no se usa
en `src/`, `tests/` ni `src/cli/` → **borrar la función/campo**. Repetir por
item, sustituyendo `IDENT` por el identificador real:

```bash
# 1) ¿lo usa alguien fuera de su propia definición?
grep -rn "IDENT" src/ tests/ | grep -v "fn IDENT" | grep -v "PLANNED FOR"
# 2) si sale vacío → borrar la definición; si sale algo → quitar solo el atributo
```

- [x] **Step 2: Verificar tras la poda**

Run: `cargo build 2>&1 | grep -cE "^(error|warning)"`
Expected: `0` (si borrar algo rompe un consumidor, ese item se **restaura** y se deja marcado: forma parte de la fachada).

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: mismo total que Task 9, `0` fallos.

Run: `cargo test --test parity_freeze_tests`
Expected: PASS.

- [x] **Step 3: Sincronizar documentación**

- `COMPILER_REFACTORING.md` y `REFACTORING_SUMMARY.md`: actualizar el árbol de módulos al estado real tras R0 (rutas de `src/lang`, `strings/`, `module.rs`, `functions.rs`, parser partido, CLI partida).
- `LSP_ARCHITECTURE.md` y `LSP_IMPLEMENTATION_GUIDE.md`: añadir al inicio una nota destacada: **"Diseño futuro — no implementado. No existe `src/lsp` ni dependencias LSP."**
- `README.md`: actualizar el árbol de archivos de la sección de tests si menciona módulos movidos (p. ej. describe `control_flow_tests.rs`), y añadir `tests/parity_freeze_tests.rs` a la lista.

- [x] **Step 4: Verificación final de R0**

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print "passed", p, "failed", f}'`
Expected: `failed 0`.

Run: `cargo clippy --lib 2>&1 | grep -c "^warning"`
Expected: **no más** que el baseline (2: `math.rs` useless conversion y `generator.rs` collapsible if; si se pueden arreglar sin cambiar comportamiento, se arreglan y baja a 0).

Run: `git status --short`
Expected: solo los archivos de esta tarea.

- [x] **Step 5: Commit**

```bash
git add -A
git commit -m "chore: prune dead code and sync architecture docs after the refactor

Removes dead_code allowances whose v0.3 rationale expired, updates the module
trees in COMPILER_REFACTORING/REFACTORING_SUMMARY and marks the LSP docs as
design-only (no src/lsp exists)."
```

---

## Verificación de R0 (Definition of Done)

1. `cargo test` → **0 fallos**, mismo conjunto de comportamiento que el baseline (ninguna salida cambió).
2. `cargo build` → 0 warnings; `cargo clippy --lib` → sin warnings nuevos.
3. `tests/parity_freeze_tests.rs` verde: cada divergencia conocida está declarada con su fase; ninguna divergencia nueva.
4. Estructura de `src/` igual a la del spec §4.9.
5. `docs/` sincronizado (spec + plan + docs de arquitectura).
6. Historial: un commit por tarea, en `main`, mensajes en inglés.

Tras R0, el siguiente ciclo es **P0** (ampliar el corpus y convertir `KNOWN_DIVERGENCES` en el registro de paridad) y luego **P1** (formato de valores), cada uno con su propio spec + plan.
