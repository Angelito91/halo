# P0 — Registro de paridad y cobertura total — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Convertir la red de congelado de R0 en una red de paridad que ejercite toda la superficie del lenguaje y en la que una divergencia sin dueño (sin caso, sin motivo o sin fase) sea representable.

**Architecture:** El registro `KNOWN_DIVERGENCES` y el corpus `case_*` se fusionan en **una sola tabla** de casos (`tests/common/cases.rs`), donde cada entrada lleva su snippet, su divergencia esperada (motivo + fase) y las features del catálogo que ejercita. La cobertura deja de ser una disciplina humana: tres pruebas derivan la cobertura de `src/lang/catalog.rs` y fallan si un built-in o método no tiene caso, o si un caso referencia una feature inexistente.

**Tech Stack:** Rust 2021, tests de integración con `cargo test`, inkwell/LLVM 23 + clang para el modo compilado. Sin dependencias nuevas.

**Spec:** `docs/superpowers/specs/2026-10-20-p0-registro-de-paridad-y-cobertura.md` (el plan argumenta desde el spec; el spec viaja con él)

## Global Constraints

- **Cero cambios de comportamiento.** P0 es una fase de *observación*: describe, congela y registra. No arregla ninguna divergencia. Ningún commit de P0 toca `src/compiler/`, `src/interpreter/` ni `src/parser/`.
- **Toda divergencia tiene dueño.** Motivo + fase válidos, siempre. Sin excepciones: un caso que diverge y no está declarado hace fallar la suite.
- **La cobertura se deriva del catálogo.** `src/lang/catalog.rs` es la fuente única de verdad; la red no mantiene una lista paralela de features.
- **El arnés sigue anti-mentira.** Divergencia no declarada → falla. Entrada obsoleta (ya no diverge) → falla.
- **Cada commit deja `cargo test` verde** y `cargo clippy --all-targets` limpio.
- **Commits lógicos** en `main`, mensajes en inglés (conventional commits); documentación en español.
- **El registro se queda en `tests/`**, no en `src/lang/`: describe el estado del proyecto, no el lenguaje.
- Sin dependencias nuevas.

## Review Focus

Riesgos que el spec implica y que ninguna prueba de tarea cubre por sí sola:

1. **Un caso nuevo del corpus que no se declara.** Al ampliar el corpus, es fácil que un snippet nuevo revele una divergencia real y que el implementador "arregle" el snippet en vez de registrarlo. La prueba: un caso deliberadamente divergente y no declarado hace fallar la suite con `undocumented divergence`.
2. **Una entrada obsoleta que sobrevive.** Cuando una divergencia se arregla en P1–P3, su entrada debe *borrarse*, no quedarse. La prueba: marcar como divergente un caso que hoy coincide hace fallar la suite con `stale entry`.
3. **Cobertura Derivada del catálogo.** Si el catálogo gana un built-in y nadie añade su caso, la red deja de cubrirlo en silencio. La prueba: añadir un built-in al catálogo sin caso hace fallar `parity_coverage_tests`.
4. **Fase inválida.** Una divergencia con `phase: "P4"` o `"p1"` no debe poder existir. La prueba: el tipo `Phase` es un enum, no un string.
5. **Comparación débil.** El arnés compara stdout + exit code + stderr. Un caso donde ambos fallan con mensajes *distintos* no es paridad. La prueba: el harness compara stderr cuando ambos son error.

---

## File Structure

**Creados:**
- `tests/common/cases.rs` — la tabla única de casos: snippet + divergencia esperada + features cubiertas.
- `tests/parity_coverage_tests.rs` — las tres pruebas de cobertura derivada del catálogo.

**Modificados:**
- `tests/common/parity.rs` — `Phase` como enum, `ParityCase`, `compare` con `Option<&KnownDivergence>`, `assert_case`.
- `tests/common/mod.rs` — declarar `pub mod cases;`.
- `tests/parity_freeze_tests.rs` — los 34 casos pasan a una línea cada uno; se añaden los nuevos.
- `docs/superpowers/plans/2026-10-20-p0-registro-de-paridad-y-cobertura.md` — este plan (checkboxes).

**Eliminados:** nada.

---

### Task 1: Fundir el registro y el corpus en una sola tabla (P0.0)

**Files:**
- Create: `tests/common/cases.rs`
- Modify: `tests/common/parity.rs`, `tests/common/mod.rs`, `tests/parity_freeze_tests.rs`

**Interfaces:**
- Consumes: `halo::lang::catalog::{Builtin, Method, Support}` (solo en Task 5).
- Produces:
  - `common::parity::Phase` — enum `P1 | P2 | P3`, con `Display`.
  - `common::parity::KnownDivergence { id: &'static str, reason: &'static str, phase: Phase }`.
  - `common::parity::ParityCase { id: &'static str, code: &'static str, divergence: Option<KnownDivergence>, covers: &'static [&'static str] }`.
  - `common::parity::compare(id: &str, code: &str, declared: Option<&KnownDivergence>) -> Result<(), String>` — **firma nueva**: el tercer parámetro pasa de `&[KnownDivergence]` a `Option<&KnownDivergence>`.
  - `common::parity::compare_case(case: &ParityCase) -> Result<(), String>`.
  - `common::parity::assert_case(id: &str)` — busca el caso en la tabla y lo verifica; entra en pánico si el id no existe.
  - `common::parity::case(id: &str) -> Option<&'static ParityCase>`.
  - `common::cases::CASES: &[ParityCase]` — la tabla.

- [ ] **Step 1: Escribir el test que falla**

`tests/common/parity.rs` — cambiar el enum de fase y la firma de `compare`:

```rust
/// La fase del programa de paridad que debe cerrar una divergencia.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Valores y formato: `true/false`, float mínimo, `null`, `str()` en runtime, `print` multi-argumento.
    P1,
    /// Errores de runtime idénticos: overflow, div/0, índice.
    P2,
    /// Arrays en modo compilado.
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
```

El `use std::fmt;` debe añadirse al principio del archivo.

En `tests/common/mod.rs`, añadir `pub mod cases;` junto a `pub mod parity;`.

`tests/parity_freeze_tests.rs` — reescribir la parte del corpus:

```rust
// Corpus de paridad: una prueba por caso. El snippet vive en
// `common::cases::CASES`; aquí solo se referencia su id.
#[test] fn case_bool_print() { parity::assert_case("bool_print"); }
#[test] fn case_float_print() { parity::assert_case("float_print"); }
```

Las cuatro auto-pruebas del arnés cambian de firma (`compare` ya no recibe una
tabla, recibe la divergencia esperada de ese caso):

```rust
#[test]
fn harness_flags_an_undeclared_divergence() {
    if !parity::clang_available() { return; }
    let err = parity::compare("bool_print", "print(true)", None).unwrap_err();
    assert!(err.contains("undocumented divergence"), "{err}");
}

#[test]
fn harness_accepts_a_declared_divergence() {
    if !parity::clang_available() { return; }
    let d = parity::case("bool_print").unwrap().divergence.as_ref();
    parity::compare("bool_print", "print(true)", d).unwrap();
}

#[test]
fn harness_flags_a_stale_entry() {
    if !parity::clang_available() { return; }
    let d = parity::KnownDivergence {
        id: "int_truncation",
        reason: "test",
        phase: parity::Phase::P1,
    };
    let err = parity::compare("int_truncation", "print(int(3.9))", Some(&d)).unwrap_err();
    assert!(err.contains("stale"), "{err}");
}

#[test]
fn harness_reports_parity_for_an_agreeing_case() {
    if !parity::clang_available() { return; }
    parity::compare("int_truncation", "print(int(3.9))", None).unwrap();
}
```

…y una línea por cada id de `CASES` (los 34 ids actuales, luego los nuevos de Task 3 y Task 4).

Run: `cargo test --test parity_freeze_tests 2>&1 | head -20`
Expected: FALLA — `unresolved import common::cases` / `no function or associated item named assert_case`.

- [ ] **Step 2: Implementar la tabla y el arnés nuevo**

`tests/common/parity.rs`:

```rust
/// Un caso del corpus de paridad: un programa Halo y el resultado que se
/// espera de ambos backends.
pub struct ParityCase {
    /// Identificador estable. Es también el nombre del test.
    pub id: &'static str,
    /// Programa Halo que se ejecuta en el intérprete y en un binario nativo.
    pub code: &'static str,
    /// `None` = ambos modos deben coincidir. `Some(d)` = se espera una
    /// divergencia, con el motivo y la fase que la cierran.
    pub divergence: Option<KnownDivergence>,
    /// Features del catálogo que este caso ejercita (nombres exactos).
    pub covers: &'static [&'static str],
}
```

`KnownDivergence` pasa a llevar `phase: Phase` en vez de `phase: &'static str`.

Y las funciones:

```rust
/// Compara ambos modos. `declared` es la divergencia esperada, si la hay.
pub fn compare(
    id: &str,
    code: &str,
    declared: Option<&KnownDivergence>,
) -> Result<(), String> {
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

/// Busca un caso por su id.
pub fn case(id: &str) -> Option<&'static ParityCase> {
    super::cases::CASES.iter().find(|c| c.id == id)
}

/// Verifica un caso del corpus por su id.
pub fn assert_case(id: &str) {
    let case = case(id).unwrap_or_else(|| panic!("unknown parity case '{id}'"));
    compare_case(case).unwrap_or_else(|e| panic!("{e}\n--- source ---\n{}", case.code));
}
```

`tests/common/cases.rs` — la tabla con los 34 casos actuales, cada uno con su
snippet (idéntico al actual), su divergencia esperada y sus features cubiertas:

```rust
//! Tabla única de casos de paridad: cada entrada es a la vez el caso del
//! corpus y su registro de divergencia esperada.

use super::parity::{KnownDivergence, ParityCase, Phase};

const fn diverges(
    id: &'static str,
    reason: &'static str,
    phase: Phase,
) -> Option<KnownDivergence> {
    Some(KnownDivergence { id, reason, phase })
}

pub const CASES: &[ParityCase] = &[
    ParityCase {
        id: "bool_print",
        code: "print(true)\nprint(false)",
        divergence: diverges("bool_print", "compiled prints -1/0, interpreter true/false", Phase::P1),
        covers: &["print"],
    },
    ParityCase {
        id: "float_print",
        code: "print(5.0)\nprint(3.14)\nprint(1.0 / 4.0)",
        divergence: diverges("float_print", "compiled prints 6 decimals, interpreter minimal", Phase::P1),
        covers: &["print"],
    },
    ParityCase {
        id: "str_runtime_int",
        code: "n = 7\nprint(str(n))\nprint(\"n=\" + str(n))",
        divergence: diverges("str_runtime_int", "str() only formats constants in compiled mode", Phase::P1),
        covers: &["str", "print"],
    },
    ParityCase {
        id: "str_runtime_float",
        code: "x = 1.5\nprint(str(x))",
        divergence: diverges("str_runtime_float", "str() only formats constants in compiled mode", Phase::P1),
        covers: &["str"],
    },
    ParityCase {
        id: "str_runtime_bool",
        code: "b = true\nprint(str(b))",
        divergence: diverges("str_runtime_bool", "bool variable declaration fails to compile", Phase::P1),
        covers: &["str"],
    },
    ParityCase {
        id: "int_from_string",
        code: "print(int(\"42\"))",
        divergence: diverges("int_from_string", "int() from strings unsupported in compiled mode", Phase::P1),
        covers: &["int"],
    },
    ParityCase {
        id: "float_from_string",
        code: "print(float(\"1.5\"))",
        divergence: diverges("float_from_string", "float() from strings unsupported in compiled mode", Phase::P1),
        covers: &["float"],
    },
    ParityCase {
        id: "int_truncation",
        code: "print(int(3.9))\nprint(int(-2.7))",
        divergence: None,
        covers: &["int"],
    },
    ParityCase {
        id: "abs_float",
        code: "print(abs(-3.14))\nprint(abs(-7))",
        divergence: diverges("abs_float", "float formatting differs", Phase::P1),
        covers: &["abs"],
    },
    ParityCase {
        id: "array_literal",
        code: "a = [1, 2, 3]\nprint(len(a))\nprint(a[0])\nprint(a[2])",
        divergence: diverges("array_literal", "arrays unsupported in compiled mode", Phase::P3),
        covers: &["len"],
    },
    ParityCase {
        id: "array_negative_idx",
        code: "a = [1, 2, 3]\nprint(a[-1])",
        divergence: diverges("array_negative_idx", "arrays unsupported in compiled mode", Phase::P3),
        covers: &["len"],
    },
    ParityCase {
        id: "array_methods",
        code: "a = [1, 2, 3]\na.push(4)\nprint(len(a))\nprint(a.last())\na.reverse()\nprint(a[0])",
        divergence: diverges("array_methods", "arrays unsupported in compiled mode", Phase::P3),
        covers: &["push", "last", "reverse", "len"],
    },
    ParityCase {
        id: "array_sort",
        code: "a = [3, 1, 2]\na.sort()\nprint(a[0])",
        divergence: diverges("array_sort", "arrays unsupported in compiled mode", Phase::P3),
        covers: &["sort"],
    },
    ParityCase {
        id: "array_join",
        code: "a = [\"x\", \"y\"]\nprint(a.join(\"-\"))",
        divergence: diverges("array_join", "arrays unsupported in compiled mode", Phase::P3),
        covers: &["join"],
    },
    ParityCase {
        id: "for_over_array",
        code: "for v in [10, 20] { print(v) }",
        divergence: diverges("for_over_array", "arrays unsupported in compiled mode", Phase::P3),
        covers: &["print"],
    },
    ParityCase {
        id: "range_as_value",
        code: "r = range(3)\nprint(len(r))\nprint(r[1])",
        divergence: diverges("range_as_value", "array values unsupported in compiled mode", Phase::P3),
        covers: &["range", "len"],
    },
    ParityCase {
        id: "string_param",
        code: "greet(name) { print(name + \"!\") }\ngreet(\"hola\")",
        divergence: None,
        covers: &["print"],
    },
    ParityCase {
        id: "string_concat_num",
        code: "print(\"n=\" + 5)",
        divergence: diverges("string_concat_num", "string + number prints nothing in compiled mode", Phase::P1),
        covers: &["print"],
    },
    ParityCase {
        id: "string_repeat",
        code: "print(\"ab\" * 3)",
        divergence: None,
        covers: &["print"],
    },
    ParityCase {
        id: "int_overflow",
        code: "print(9223372036854775807 + 1)",
        divergence: diverges("int_overflow", "compiled wraps silently, interpreter errors", Phase::P2),
        covers: &["print"],
    },
    ParityCase {
        id: "div_by_zero",
        code: "print(1 / 0)",
        divergence: diverges("div_by_zero", "compiled has undefined behavior, interpreter errors", Phase::P2),
        covers: &["print"],
    },
    ParityCase {
        id: "mod_negative",
        code: "print(-7 % 3)\nprint(7 % -3)",
        divergence: None,
        covers: &["print"],
    },
    ParityCase {
        id: "mixed_equality",
        code: "print(1 == \"1\")\nprint(true == 1)",
        divergence: diverges("mixed_equality", "printing a comparison result outputs nothing", Phase::P1),
        covers: &["print"],
    },
    ParityCase {
        id: "print_multiarg",
        code: "print(\"a\", \"b\", 1)",
        divergence: diverges("print_multiarg", "interpreter prints one line per arg, compiled joins with spaces", Phase::P1),
        covers: &["print"],
    },
    ParityCase {
        id: "print_noargs",
        code: "print()",
        divergence: diverges("print_noargs", "interpreter prints nothing, compiled prints a newline", Phase::P1),
        covers: &["print"],
    },
    ParityCase {
        id: "len_number",
        code: "print(len(12345))",
        divergence: None,
        covers: &["len"],
    },
    ParityCase {
        id: "type_of_int",
        code: "print(type(42))",
        divergence: None,
        covers: &["type"],
    },
    ParityCase {
        id: "string_methods",
        code: "s = \" Hola \"\nprint(s.trim())\nprint(s.upper())\nprint(\"abc\".contains(\"b\"))",
        divergence: diverges("string_methods", "contains() result printed as 1 instead of true", Phase::P1),
        covers: &["trim", "upper", "contains"],
    },
    ParityCase {
        id: "nested_call_str",
        code: "f(n) { return str(n) }\nprint(\"v=\" + f(3))",
        divergence: diverges("nested_call_str", "function returning a string call fails to compile", Phase::P1),
        covers: &["str"],
    },
    ParityCase {
        id: "global_mutation",
        code: "x = 1\nf() { x = 2 }\nf()\nprint(x)",
        divergence: None,
        covers: &["print"],
    },
    ParityCase {
        id: "return_none",
        code: "f() { }\nprint(f())",
        divergence: diverges("return_none", "void return prints null vs 0", Phase::P1),
        covers: &["print"],
    },
    ParityCase {
        id: "early_return_loop",
        code: "f() {\nfor i in range(5) { if i == 2 { return i } }\nreturn -1\n}\nprint(f())",
        divergence: None,
        covers: &["range"],
    },
    ParityCase {
        id: "bool_var_decl",
        code: "b = true\nprint(b)\nprint(\"ok\")",
        divergence: diverges("bool_var_decl", "bool variable declaration fails to compile", Phase::P1),
        covers: &["print"],
    },
    ParityCase {
        id: "println_builtin",
        code: "println(\"hola\")",
        divergence: diverges("println_builtin", "println exists only in compiled mode", Phase::P1),
        covers: &["println"],
    },
];
```

`covers` de los 34 casos actuales, según la feature que cada uno ejercita:

| id | covers |
|---|---|
| `bool_print` | `print` |
| `float_print` | `print` |
| `str_runtime_int` | `str`, `print` |
| `str_runtime_float` | `str` |
| `str_runtime_bool` | `str` |
| `int_from_string` | `int` |
| `float_from_string` | `float` |
| `int_truncation` | `int` |
| `abs_float` | `abs` |
| `array_literal` | `len` |
| `array_negative_idx` | `len` |
| `array_methods` | `push`, `last`, `reverse`, `len` |
| `array_sort` | `sort` |
| `array_join` | `join` |
| `for_over_array` | `print` |
| `range_as_value` | `range`, `len` |
| `string_param` | `print` |
| `string_concat_num` | `print` |
| `string_repeat` | `print` |
| `int_overflow` | `print` |
| `div_by_zero` | `print` |
| `mod_negative` | `print` |
| `mixed_equality` | `print` |
| `print_multiarg` | `print` |
| `print_noargs` | `print` |
| `len_number` | `len` |
| `type_of_int` | `type` |
| `string_methods` | `trim`, `upper`, `contains` |
| `nested_call_str` | `str` |
| `global_mutation` | `print` |
| `return_none` | `print` |
| `early_return_loop` | `range` |
| `bool_var_decl` | `print` |
| `println_builtin` | `println` |

- [ ] **Step 3: Verificar**

Run: `cargo test --test parity_freeze_tests 2>&1 | tail -5`
Expected: PASS (38 tests: 34 del corpus + 4 del arnés).

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: `828 0` — el mismo total que antes de P0. Los casos se movieron de
fichero, no se añadieron.

Run: `cargo clippy --all-targets 2>&1 | grep -cE "^(warning|error)"`
Expected: `0`.

- [ ] **Step 4: Probar que el arnés no miente (mutación temporal)**

Añadir temporalmente a `tests/parity_freeze_tests.rs`:

```rust
#[test] fn tmp_mutation_probe() { parity::compare_case(parity::case("bool_print").unwrap()).unwrap(); }
```

Run: `cargo test --test parity_freeze_tests tmp_mutation_probe`
Expected: FALLA con `stale entry for 'bool_print'`. **Quitar la prueba temporal**
y volver a ejecutar para confirmar verde.

- [ ] **Step 5: Commit**

```bash
git add tests/common tests/parity_freeze_tests.rs
git commit -m "test: merge the parity registry and the corpus into one case table

Each case now carries its snippet, its expected divergence (reason + phase
as an enum) and the catalog features it exercises, so a divergence cannot
exist without a case that exercises it."
```

---

### Task 2: Blindar el arnés y cerrar los hallazgos de la revisión (P0.0b)

> Esta tarea existe por la revisión final de R0 (2026-10-20), que no encontró
> fallos en el producto pero sí en la **maquinaria de evidencia**: una caché
> que puede servir binarios obsoletos, y tablas que no se autocomprueban. P0 se
> apoya justo en esa maquinaria, así que se arregla antes de ampliar el corpus.

**Files:**
- Modify: `tests/common/parity.rs` (huella del compilador en la clave de caché)
- Modify: `tests/lang_catalog_tests.rs` (sonda derivada del catálogo)
- Modify: `tests/cli_output_tests.rs` (orden de tokens y exit codes)
- Modify: `src/lang/format.rs`, `src/lang/errors.rs` (alinear con el intérprete + tests)
- Modify: `src/compiler/mod.rs` (doc comment obsoleto)
- Modify: `COMPILER_REFACTORING.md`, `REFACTORING_SUMMARY.md` (cifras obsoletas)

**Interfaces:**
- Consumes: nada de tareas anteriores.
- Produces:
  - `common::parity::compiler_fingerprint() -> u64` — huella del ejecutable de test (mtime + tamaño).
  - `common::parity::cache_dir_for(fingerprint: u64) -> PathBuf` — directorio de caché acotado por la huella.
  - `common::parity::cache_dir() -> PathBuf` — `cache_dir_for(compiler_fingerprint())`.
  - `halo::lang::format::format_float` alineado con `Value::Float(..).to_string_value()`.

- [ ] **Step 1: La caché de binarios, acotada por la huella del compilador**

Hoy la clave de la caché es **solo el hash del fuente**: si cambia el
generador de código, `run_compiled` reutiliza el binario producido por el
código viejo y la red "pasa" sin haber medido nada.

`tests/common/parity.rs`:

```rust
/// Huella del ejecutable de test (mtime + tamaño).
///
/// El binario compilado por `run_compiled` lo produce el codegen **enlazado
/// en este binario de test**, no el binario `halo`. Cuando el codegen
/// cambia, este ejecutable se reconstruye, cambia su mtime y la huella
/// cambia: la caché se invalida sola y el caso se vuelve a compilar de verdad.
pub fn compiler_fingerprint() -> u64 {
    let mut h = DefaultHasher::new();
    match std::env::current_exe().ok().and_then(|p| std::fs::metadata(&p).ok()) {
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
```

Y borrar el `cache_dir()` antiguo (el que no tenía la huella).

Run: `cargo test --test parity_freeze_tests 2>&1 | tail -3`
Expected: PASS (38 tests), y el binario se recompila porque la caché ahora
está en un directorio nuevo.

- [ ] **Step 2: Probar que la huella acota la caché**

Añadir a `tests/parity_freeze_tests.rs`:

```rust
#[test]
fn the_cache_is_scoped_by_the_compiler_fingerprint() {
    // Dos huellas distintas no pueden compartir caché: si compartieran, un
    // codegen nuevo serviría el binario del viejo.
    assert_ne!(
        parity::cache_dir_for(1),
        parity::cache_dir_for(2),
        "distinct fingerprints must not share a cache directory"
    );
    assert_eq!(parity::cache_dir_for(7), parity::cache_dir_for(7));
}

#[test]
fn the_compiler_fingerprint_is_stable_within_a_run() {
    assert_eq!(
        parity::compiler_fingerprint(),
        parity::compiler_fingerprint(),
        "the fingerprint must not change during a run"
    );
    assert_ne!(
        parity::compiler_fingerprint(),
        0,
        "the fingerprint must be computed"
    );
}
```

Run: `cargo test --test parity_freeze_tests 2>&1 | tail -3`
Expected: PASS (40 tests).

- [ ] **Step 3: La sonda del catálogo, derivada del catálogo**

`tests/lang_catalog_tests.rs` solo sondea 9 de 26 built-ins y 0 de 16
métodos: una entrada del catálogo puede estar equivocada sin que nadie lo note.

Sustituir `catalog_matches_observed_behavior` por una versión dirigida por el
catálogo, con una sonda por entrada:

```rust
/// Snippet mínimo que ejercita un built-in.
fn builtin_probe(name: &str) -> Option<&'static str> {
    Some(match name {
        "print" => "print(1)",
        "println" => "println(\"x\")",
        "len" => "print(len(\"ab\"))",
        "str" => "print(str(1))",
        "int" => "print(int(1.9))",
        "float" => "print(float(1))",
        "abs" => "print(abs(-1))",
        "type" => "print(type(1))",
        "range" => "print(len(range(2)))",
        "push" => "a = [1]\na.push(2)\nprint(len(a))",
        "pop" => "a = [1]\na.pop()\nprint(len(a))",
        "append" => "a = [1]\na.append(2)\nprint(len(a))",
        "first" => "a = [1]\nprint(a.first())",
        "last" => "a = [1]\nprint(a.last())",
        "reverse" => "a = [1]\na.reverse()\nprint(a[0])",
        "join" => "a = [\"x\"]\nprint(a.join(\"-\"))",
        "sort" => "a = [1]\na.sort()\nprint(a[0])",
        "contains" => "print(\"ab\".contains(\"a\"))",
        "index_of" => "print(\"ab\".index_of(\"b\"))",
        "upper" => "print(\"a\".upper())",
        "lower" => "print(\"A\".lower())",
        "trim" => "print(\" a \".trim())",
        "starts_with" => "print(\"ab\".starts_with(\"a\"))",
        "ends_with" => "print(\"ab\".ends_with(\"b\"))",
        "replace" => "print(\"ab\".replace(\"a\", \"c\"))",
        "substr" => "print(\"abc\".substr(0, 2))",
        _ => return None,
    })
}

/// Snippet mínimo que ejercita un método (con su receptor).
fn method_probe(name: &str) -> Option<&'static str> {
    Some(match name {
        "push" => "a = [1]\na.push(2)",
        "pop" => "a = [1]\na.pop()",
        "first" => "a = [1]\nprint(a.first())",
        "last" => "a = [1]\nprint(a.last())",
        "join" => "a = [\"x\"]\nprint(a.join(\"-\"))",
        "sort" => "a = [1]\na.sort()",
        "reverse" => "a = [1]\na.reverse()",
        "contains" => "a = [1]\nprint(a.contains(1))",
        "index_of" => "a = [1]\nprint(a.index_of(1))",
        "upper" => "print(\"a\".upper())",
        "lower" => "print(\"A\".lower())",
        "trim" => "print(\" a \".trim())",
        "starts_with" => "print(\"ab\".starts_with(\"a\"))",
        "ends_with" => "print(\"ab\".ends_with(\"b\"))",
        "replace" => "print(\"ab\".replace(\"a\", \"c\"))",
        "substr" => "print(\"abc\".substr(0, 2))",
        _ => return None,
    })
}

#[test]
fn every_catalog_entry_has_a_probe() {
    for b in BUILTINS {
        assert!(
            builtin_probe(b.name).is_some(),
            "builtin '{}' has no probe in the catalog test",
            b.name
        );
    }
    for m in METHODS {
        assert!(
            method_probe(m.name).is_some(),
            "method '{}' has no probe in the catalog test",
            m.name
        );
    }
}

#[test]
fn catalog_matches_observed_behavior() {
    if !parity::clang_available() { return; }
    for b in BUILTINS {
        let code = builtin_probe(b.name).unwrap();
        let interpreter_ok = !parity::run_interpreter(code).is_error();
        let compiled_ok = parity::run_compiled(code).map(|r| !r.is_error()).unwrap_or(false);
        match b.support {
            Support::Both => assert!(interpreter_ok && compiled_ok, "{}: declared Both", b.name),
            Support::InterpreterOnly => assert!(interpreter_ok && !compiled_ok, "{}: declared InterpreterOnly", b.name),
            Support::CompilerOnly => assert!(!interpreter_ok && compiled_ok, "{}: declared CompilerOnly", b.name),
        }
    }
    for m in METHODS {
        let code = method_probe(m.name).unwrap();
        let interpreter_ok = !parity::run_interpreter(code).is_error();
        let compiled_ok = parity::run_compiled(code).map(|r| !r.is_error()).unwrap_or(false);
        match m.support {
            Support::Both => assert!(interpreter_ok && compiled_ok, "method {}: declared Both", m.name),
            Support::InterpreterOnly => assert!(interpreter_ok && !compiled_ok, "method {}: declared InterpreterOnly", m.name),
            Support::CompilerOnly => assert!(!interpreter_ok && compiled_ok, "method {}: declared CompilerOnly", m.name),
        }
    }
}
```

Run: `cargo test --test lang_catalog_tests 2>&1 | tail -20`
Expected: PASS, o FAIL con el nombre del built-in/método cuya matriz de
soporte no coincide con lo observado. **Si falla, la matriz del catálogo es
incorrecta y hay que corregir `src/lang/catalog.rs`**, no el test.

- [ ] **Step 4: Fijar el orden de tokens y los exit codes de la CLI**

`tests/cli_output_tests.rs` solo comprueba `contains`, así que un reordenamiento
de tokens pasaría; y descarta stderr y exit code, así que un `check` que
fallara no lo detectaría.

Sustituir `run_halo` por una versión que devuelva stdout, stderr y exit code, y
añadir el fixture de error de sintaxis:

```rust
struct Output {
    stdout: String,
    stderr: String,
    code: Option<i32>,
}

fn run_halo_on(source: &str, sub: &str) -> Output {
    let dir = std::env::temp_dir().join("halo_cli_output_tests");
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("prog.halo");
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
```

Y el test del orden de tokens:

```rust
#[test]
fn tokens_subcommand_keeps_the_token_order() {
    let out = run_halo("tokens");
    let ident = out.find("Identifier").unwrap_or_else(|| panic!("no Identifier in {out}"));
    let number = out.find("Number").unwrap_or_else(|| panic!("no Number in {out}"));
    let plus = out.find('+').unwrap_or_else(|| panic!("no '+' in {out}"));
    // `x = 1 + 2`: el identificador va antes que el número, y el `+` entre
    // ambos. Un reordenamiento del lexer o del parser rompe este orden.
    assert!(ident < number, "identifier must come before the number: {out}");
    assert!(number < plus, "the number must come before '+': {out}");
}

#[test]
fn check_subcommand_fails_with_exit_code_1_on_a_syntax_error() {
    let out = run_halo_on("x = = 2\n", "check");
    assert_eq!(out.code, Some(1), "a parse error must exit with 1: {out:?}");
    assert!(!out.stdout.contains("OK"), "a broken program must not report OK: {out:?}");
}
```

Run: `cargo test --test cli_output_tests 2>&1 | tail -5`
Expected: PASS (6 tests).

- [ ] **Step 5: Alinear `src/lang/format.rs` con el intérprete y darle tests**

`format_float` usa `Display` + `.0`, pero el intérprete usa `{:.1}` cuando el
float es entero: para `1e300` o `NaN` los dos discrepan, y `format.rs` no lo
dice.

`src/lang/format.rs`:

```rust
/// How the interpreter prints a float: `{:.1}` when the value is integral
/// (`5.0` → `5.0`), plain `Display` otherwise (`0.25` → `0.25`).
///
/// This mirrors `Value::Float(f).to_string_value()` exactly; the differential
/// test below pins the two together.
pub fn format_float(f: f64) -> String {
    if f.fract() == 0.0 {
        format!("{:.1}", f)
    } else {
        f.to_string()
    }
}
```

Y un test diferencial en el mismo archivo:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Value;

    #[test]
    fn format_float_matches_the_interpreter() {
        for f in [5.0, 0.25, -3.14, 1e300, f64::INFINITY, f64::NAN] {
            assert_eq!(
                format_float(f),
                Value::Float(f).to_string_value(),
                "float formatting diverged for {f}"
            );
        }
    }

    #[test]
    fn format_number_matches_the_interpreter() {
        for n in [0i64, 42, -7, i64::MAX, i64::MIN] {
            assert_eq!(
                format_number(n),
                Value::Number(n).to_string_value(),
                "number formatting diverged for {n}"
            );
        }
    }

    #[test]
    fn format_bool_and_null_match_the_interpreter() {
        assert_eq!(format_bool(true), Value::Bool(true).to_string_value());
        assert_eq!(format_bool(false), Value::Bool(false).to_string_value());
        assert_eq!(format_null(), Value::Null.to_string_value());
    }
}
```

Run: `cargo test --lib 2>&1 | tail -5`
Expected: PASS.

- [ ] **Step 6: Los mensajes de error de `src/lang/errors.rs`, fijados**

`src/lang/errors.rs` declara constantes que ningún backend consume todavía.
Fijarlas contra el intérprete:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Value;

    #[test]
    fn the_error_messages_match_the_interpreter() {
        assert_eq!(DIVISION_BY_ZERO, "Division by zero");
        assert_eq!(
            INTEGER_OVERFLOW,
            Value::Number(i64::MAX).checked_add(1).unwrap_err()
        );
        assert!(UNDEFINED_VARIABLE.contains("Undefined variable"));
        assert!(UNDEFINED_FUNCTION.contains("Undefined function"));
        assert!(NEGATIVE_INDEX.contains("Array index cannot be negative"));
    }
}
```

Run: `cargo test --lib 2>&1 | tail -5`
Expected: PASS. Si `INTEGER_OVERFLOW` no coincide con el mensaje real del
intérprete, corregir la constante (es el intérprete la referencia).

- [ ] **Step 7: Docs y comentarios obsoletos**

- `src/compiler/mod.rs:89` — el doc comment de `optimise` menciona
  `emit_object`, que ya no existe. Dejar: *"Call this after
  `codegen().compile(...)` and before `emit_llvm`."*
- `COMPILER_REFACTORING.md` — `generator.rs` dice 292 líneas; son 290. Y
  `emitter.rs` se describe como "salida (LLVM IR / objeto / binario)", pero ya
  no emite objetos ni binarios. Corregir ambas cosas.
- `REFACTORING_SUMMARY.md` — dice "829 tests"; son 828.

- [ ] **Step 8: Verificar y commitear**

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: `832 0` (828 + 2 de la huella + 1 de sondas + 1 de CLI).

Run: `cargo clippy --all-targets 2>&1 | grep -cE "^(warning|error)"`
Expected: `0`.

Run: `cargo build 2>&1 | grep -cE "^(warning|error)"`
Expected: `0`.

```bash
git add -A
git commit -m "test: harden the parity harness and pin the catalog against observed behavior

Addresses the findings of the R0 final review:

- The parity cache is keyed by the source hash alone, so a changed code
  generator could be served binaries produced by the old one. The cache
  directory is now scoped by a fingerprint of the test executable, so a
  rebuild invalidates it automatically.
- The catalog test probed 9 of 26 builtins and 0 of 16 methods; it is now
  driven by the catalog itself and fails on any entry without a probe.
- The CLI tests only checked for substrings and discarded the exit code;
  token order and the exit code of a syntax error are now pinned.
- src/lang/format.rs diverged from the interpreter on large and non-finite
  floats and had no tests; it is now aligned and pinned differentially.
- Stale doc comments and counts in COMPILER_REFACTORING, REFACTORING_SUMMARY
  and src/compiler/mod.rs."
```

---

### Task 3: Ampliar el corpus — operadores, literales, control de flujo y funciones (P0.1)

**Files:**
- Modify: `tests/common/cases.rs`, `tests/parity_freeze_tests.rs`

**Interfaces:**
- Consumes: `common::cases::CASES`, `common::parity::assert_case` (Task 1).
- Produces: 40 casos nuevos con ids estables.

- [ ] **Step 1: Añadir los 40 casos a `CASES`**

Añadir estas entradas a `tests/common/cases.rs`. `divergence: None` significa
"ambos modos deben coincidir"; si al ejecutar divergen, se corrige en el
mismo paso (Step 3).

```rust
    // ── Operadores y literales ────────────────────────────────────────────
    ParityCase { id: "add_int",        code: "print(2 + 3)",                          divergence: None, covers: &["print"] },
    ParityCase { id: "sub_int",        code: "print(7 - 2)",                          divergence: None, covers: &["print"] },
    ParityCase { id: "mul_int",        code: "print(6 * 7)",                          divergence: None, covers: &["print"] },
    ParityCase { id: "div_int",        code: "print(7 / 2)",                          divergence: None, covers: &["print"] },
    ParityCase { id: "mod_int",        code: "print(7 % 2)",                          divergence: None, covers: &["print"] },
    ParityCase { id: "add_float",      code: "print(1.5 + 2.5)",                      divergence: None, covers: &["print"] },
    ParityCase { id: "div_float",      code: "print(7.0 / 2.0)",                      divergence: None, covers: &["print"] },
    ParityCase { id: "mixed_promotion", code: "print(1 + 2.5)\nprint(3.0 * 2)",       divergence: None, covers: &["print"] },
    ParityCase { id: "cmp_lt",         code: "print(1 < 2)\nprint(2 < 1)",              divergence: None, covers: &["print"] },
    ParityCase { id: "cmp_gt",         code: "print(1 > 2)\nprint(2 > 1)",              divergence: None, covers: &["print"] },
    ParityCase { id: "cmp_le",         code: "print(1 <= 1)\nprint(2 <= 1)",            divergence: None, covers: &["print"] },
    ParityCase { id: "cmp_ge",         code: "print(1 >= 1)\nprint(1 >= 2)",            divergence: None, covers: &["print"] },
    ParityCase { id: "cmp_neq",        code: "print(1 != 2)\nprint(2 != 2)",            divergence: None, covers: &["print"] },
    ParityCase { id: "logical_and",    code: "print(true && false)\nprint(true && true)", divergence: None, covers: &["print"] },
    ParityCase { id: "logical_or",     code: "print(false || true)\nprint(false || false)", divergence: None, covers: &["print"] },
    ParityCase { id: "short_circuit",  code: "f() { return false }\nprint(true && f())\nprint(false || true)", divergence: None, covers: &["print"] },
    ParityCase { id: "unary_not",      code: "print(!true)\nprint(!false)",             divergence: None, covers: &["print"] },
    ParityCase { id: "unary_minus",    code: "print(-5)\nprint(-2.5)",                  divergence: None, covers: &["print"] },
    ParityCase { id: "literal_int",    code: "print(42)",                              divergence: None, covers: &["print"] },
    ParityCase { id: "literal_float",  code: "print(42.0)",                            divergence: None, covers: &["print"] },
    ParityCase { id: "literal_bool",   code: "print(true)\nprint(false)",               divergence: None, covers: &["print"] },
    ParityCase { id: "literal_string", code: "print(\"hola\")",                        divergence: None, covers: &["print"] },
    ParityCase { id: "literal_string_single_quote", code: "print('hola')",              divergence: None, covers: &["print"] },
    ParityCase { id: "literal_string_escape", code: "print(\"a\\nb\")",                 divergence: None, covers: &["print"] },

    // ── Control de flujo ──────────────────────────────────────────────────
    ParityCase { id: "if_true",        code: "if 1 > 0 { print(\"si\") }",              divergence: None, covers: &["print"] },
    ParityCase { id: "if_false",       code: "if 1 > 2 { print(\"si\") } else { print(\"no\") }", divergence: None, covers: &["print"] },
    ParityCase { id: "if_else_if",     code: "x = 2\nif x == 1 { print(\"uno\") } else if x == 2 { print(\"dos\") } else { print(\"otro\") }", divergence: None, covers: &["print"] },
    ParityCase { id: "if_nested",      code: "if 1 > 0 { if 2 > 1 { print(\"dentro\") } }", divergence: None, covers: &["print"] },
    ParityCase { id: "while_loop",     code: "i = 0\nwhile i < 3 { print(i)\ni = i + 1 }", divergence: None, covers: &["print"] },
    ParityCase { id: "while_break",    code: "i = 0\nwhile true { print(i)\ni = i + 1\nif i == 2 { break } }", divergence: None, covers: &["print"] },
    ParityCase { id: "while_continue", code: "i = 0\nwhile i < 4 { i = i + 1\nif i % 2 == 0 { continue }\nprint(i) }", divergence: None, covers: &["print"] },
    ParityCase { id: "for_range",      code: "for i in range(3) { print(i) }",          divergence: None, covers: &["range", "print"] },
    ParityCase { id: "for_range_two",  code: "for i in range(2, 5) { print(i) }",        divergence: None, covers: &["range", "print"] },

    // ── Funciones ─────────────────────────────────────────────────────────
    ParityCase { id: "fn_call_before_def", code: "greet() { print(\"hola\") }\ngreet()", divergence: None, covers: &["print"] },
    ParityCase { id: "fn_recursion",   code: "fact(n) { if n <= 1 { return 1 }\nreturn n * fact(n - 1) }\nprint(fact(5))", divergence: None, covers: &["print"] },
    ParityCase { id: "fn_mutual_recursion", code: "is_even(n) { if n == 0 { return true }\nreturn is_odd(n - 1) }\nis_odd(n) { if n == 0 { return false }\nreturn is_even(n - 1) }\nprint(is_even(10))", divergence: None, covers: &["print"] },
    ParityCase { id: "fn_nested_call", code: "double(n) { return n * 2 }\nadd(a, b) { return a + b }\nprint(add(double(2), double(3)))", divergence: None, covers: &["print"] },
    ParityCase { id: "fn_returns_value", code: "one() { return 1 }\nprint(one())",        divergence: None, covers: &["print"] },
    ParityCase { id: "fn_void",        code: "nothing() { }\nnothing()\nprint(\"ok\")",   divergence: None, covers: &["print"] },
    ParityCase { id: "fn_arity_error", code: "f(a) { return a }\nf(1, 2)",               divergence: None, covers: &[] },
    ParityCase { id: "fn_undefined",   code: "missing()",                               divergence: None, covers: &[] },
```

- [ ] **Step 2: Añadir el test por caso**

En `tests/parity_freeze_tests.rs`, añadir una línea por id nuevo:

```rust
#[test] fn case_add_int() { parity::assert_case("add_int"); }
```

…para los 40 ids de la tabla anterior.

- [ ] **Step 3: Ejecutar y registrar las divergencias que aparezcan**

Run: `cargo test --test parity_freeze_tests 2>&1 | grep -E "^test .* FAILED|undocumented divergence|stale entry" | head -40`
Expected: los casos que **de verdad** difieren fallan con `undocumented divergence`.

Para cada fallo: **no se toca el snippet**. Se registra la divergencia en la
misma entrada de `CASES`, con el motivo observado en el mensaje de error y la
fase que la cerrará:

```rust
divergence: both("add_int", "motivo observado literalmente en el fallo", Phase::P1),
```

Regla de fases (del spec de R0 §3):

| Tipo de divergencia | Fase |
|---|---|
| Formato de valores, `str()` en runtime, `print` multi-arg, `bool` impreso, comparaciones impresas | `P1` |
| Errores de runtime: overflow, división por cero, índice | `P2` |
| Arrays, `range()` como valor, `for` sobre array | `P3` |

Run: `cargo test --test parity_freeze_tests 2>&1 | tail -5`
Expected: PASS.

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: `868 0` (828 + 40 nuevos).

- [ ] **Step 4: Commit**

```bash
git add tests/common/cases.rs tests/parity_freeze_tests.rs
git commit -m "test: extend the parity corpus with operators, control flow and functions

Adds 40 cases covering binary and unary operators, literals, every
control-flow form and the function contract, registering each measured
divergence with its reason and closing phase."
```

---

### Task 4: Ampliar el corpus — built-ins, métodos, arrays, errores y formato (P0.2)

**Files:**
- Modify: `tests/common/cases.rs`, `tests/parity_freeze_tests.rs`

**Interfaces:**
- Consumes: `common::cases::CASES` (Task 1).
- Produces: 32 casos nuevos.

- [ ] **Step 1: Añadir los 32 casos a `CASES`**

```rust
    // ── Built-ins ─────────────────────────────────────────────────────────
    ParityCase { id: "len_string",     code: "print(len(\"hola\"))",                   divergence: None, covers: &["len"] },
    ParityCase { id: "len_array",      code: "a = [1, 2]\nprint(len(a))",              divergence: None, covers: &["len"] },
    ParityCase { id: "str_number",     code: "print(str(42))",                         divergence: None, covers: &["str"] },
    ParityCase { id: "str_float",      code: "print(str(1.5))",                        divergence: None, covers: &["str"] },
    ParityCase { id: "str_bool",       code: "print(str(true))",                       divergence: None, covers: &["str"] },
    ParityCase { id: "int_from_float", code: "print(int(3.9))",                        divergence: None, covers: &["int"] },
    ParityCase { id: "float_from_int", code: "print(float(42))",                       divergence: None, covers: &["float"] },
    ParityCase { id: "abs_negative",   code: "print(abs(-7))",                         divergence: None, covers: &["abs"] },
    ParityCase { id: "type_float",     code: "print(type(1.5))",                       divergence: None, covers: &["type"] },
    ParityCase { id: "type_bool",      code: "print(type(true))",                      divergence: None, covers: &["type"] },
    ParityCase { id: "type_string",    code: "print(type(\"s\"))",                     divergence: None, covers: &["type"] },
    ParityCase { id: "type_null",      code: "print(type(null))",                      divergence: None, covers: &["type"] },
    ParityCase { id: "type_array",     code: "print(type([1]))",                       divergence: None, covers: &["type"] },

    // ── Métodos de string ─────────────────────────────────────────────────
    ParityCase { id: "str_upper",      code: "print(\"hola\".upper())",                divergence: None, covers: &["upper"] },
    ParityCase { id: "str_lower",      code: "print(\"HOLA\".lower())",                divergence: None, covers: &["lower"] },
    ParityCase { id: "str_trim",       code: "print(\"  hola  \".trim())",             divergence: None, covers: &["trim"] },
    ParityCase { id: "str_starts_with",code: "print(\"hola\".starts_with(\"ho\"))",    divergence: None, covers: &["starts_with"] },
    ParityCase { id: "str_ends_with",  code: "print(\"hola\".ends_with(\"la\"))",      divergence: None, covers: &["ends_with"] },
    ParityCase { id: "str_contains",   code: "print(\"hola\".contains(\"ol\"))",       divergence: None, covers: &["contains"] },
    ParityCase { id: "str_index_of",   code: "print(\"hola\".index_of(\"l\"))",        divergence: None, covers: &["index_of"] },
    ParityCase { id: "str_replace",    code: "print(\"hola\".replace(\"o\", \"0\"))",   divergence: None, covers: &["replace"] },
    ParityCase { id: "str_substr",     code: "print(\"hola\".substr(1, 3))",           divergence: None, covers: &["substr"] },

    // ── Métodos de array ──────────────────────────────────────────────────
    ParityCase { id: "arr_push",       code: "a = [1]\na.push(2)\nprint(len(a))",      divergence: None, covers: &["push", "len"] },
    ParityCase { id: "arr_pop",        code: "a = [1, 2]\na.pop()\nprint(len(a))",      divergence: None, covers: &["pop", "len"] },
    ParityCase { id: "arr_append",     code: "a = [1]\na.append(2)\nprint(len(a))",     divergence: None, covers: &["append", "len"] },
    ParityCase { id: "arr_first",      code: "a = [7, 8]\nprint(a.first())",           divergence: None, covers: &["first"] },
    ParityCase { id: "arr_last",       code: "a = [7, 8]\nprint(a.last())",            divergence: None, covers: &["last"] },
    ParityCase { id: "arr_reverse",    code: "a = [1, 2]\na.reverse()\nprint(a[0])",    divergence: None, covers: &["reverse"] },
    ParityCase { id: "arr_index_of",   code: "a = [1, 2]\nprint(a.index_of(2))",       divergence: None, covers: &["index_of"] },
    ParityCase { id: "arr_contains",   code: "a = [1, 2]\nprint(a.contains(2))",       divergence: None, covers: &["contains"] },

    // ── Arrays y errores ──────────────────────────────────────────────────
    ParityCase { id: "arr_empty",      code: "a = []\nprint(len(a))",                  divergence: None, covers: &["len"] },
    ParityCase { id: "arr_single",     code: "a = [1]\nprint(a[0])",                   divergence: None, covers: &[] },
    ParityCase { id: "arr_nested",     code: "a = [[1, 2], [3]]\nprint(len(a))",        divergence: None, covers: &["len"] },
    ParityCase { id: "arr_as_argument",code: "f(a) { return len(a) }\nprint(f([1, 2]))",  divergence: None, covers: &["len"] },
    ParityCase { id: "err_undefined_variable", code: "print(missing_var)",             divergence: None, covers: &["print"] },
    ParityCase { id: "err_index_out_of_range", code: "a = [1]\nprint(a[5])",           divergence: None, covers: &[] },
    ParityCase { id: "err_type_mismatch", code: "print(abs(\"x\"))",                  divergence: None, covers: &["abs"] },
```

- [ ] **Step 2: Añadir el test por caso**

Una línea por id nuevo en `tests/parity_freeze_tests.rs`.

- [ ] **Step 3: Ejecutar y registrar las divergencias que aparezcan**

Run: `cargo test --test parity_freeze_tests 2>&1 | grep -E "^test .* FAILED" | head -40`
Expected: los casos que difieren de verdad fallan.

Registrar cada una en `CASES` con motivo y fase (tabla de fases del Task 3,
Step 3). **No se modifica el snippet.**

Run: `cargo test --test parity_freeze_tests 2>&1 | tail -5`
Expected: PASS.

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print p, f}'`
Expected: `900 0` (868 + 32).

- [ ] **Step 4: Commit**

```bash
git add tests/common/cases.rs tests/parity_freeze_tests.rs
git commit -m "test: extend the parity corpus with builtins, methods, arrays and errors

Adds 32 cases covering every builtin and method with Both support, the
array surface and the runtime error classes, registering each measured
divergence with its reason and closing phase."
```

---

### Task 5: Cobertura derivada del catálogo (P0.3)

**Files:**
- Create: `tests/parity_coverage_tests.rs`
- Modify: `tests/common/mod.rs` (si `cases` no está ya declarado)

**Interfaces:**
- Consumes: `common::cases::CASES`, `halo::lang::catalog::{BUILTINS, METHODS, Support}`.
- Produces: la suite `parity_coverage_tests` con 3 pruebas.

- [ ] **Step 1: Escribir el test que falla**

`tests/parity_coverage_tests.rs` (nuevo):

```rust
//! Cobertura de la red de paridad derivada del catálogo.
//!
//! El catálogo (`src/lang/catalog.rs`) es la fuente única de verdad de qué
//! built-ins y métodos existen. Estas pruebas comprueban que la red de
//! paridad los ejercita a todos: añadir un built-in sin añadir su caso
//! rompe la suite.

mod common;

use common::cases::CASES;
use halo::lang::catalog::{BUILTINS, METHODS, Support};

fn covered_features() -> Vec<&'static str> {
    CASES.iter().flat_map(|c| c.covers.iter().copied()).collect()
}

#[test]
fn every_builtin_with_both_support_is_covered() {
    let covered = covered_features();
    for b in BUILTINS {
        if b.support != Support::Both {
            continue;
        }
        assert!(
            covered.contains(&b.name),
            "builtin '{}' has no parity case covering it",
            b.name
        );
    }
}

#[test]
fn every_method_with_both_support_is_covered() {
    let covered = covered_features();
    for m in METHODS {
        if m.support != Support::Both {
            continue;
        }
        assert!(
            covered.contains(&m.name),
            "method '{}' has no parity case covering it",
            m.name
        );
    }
}

#[test]
fn every_covered_feature_exists_in_the_catalog() {
    let covered = covered_features();
    for name in covered {
        let known = BUILTINS.iter().any(|b| b.name == name)
            || METHODS.iter().any(|m| m.name == name);
        assert!(known, "a parity case covers '{name}', which is not in the catalog");
    }
}
```

Run: `cargo test --test parity_coverage_tests 2>&1 | tail -20`
Expected: FALLA — `unresolved import common::cases` o features sin cubrir
faltantes.

- [ ] **Step 2: Ejecutar y corregir lo que falte**

Run: `cargo test --test parity_coverage_tests 2>&1 | grep -E "has no parity case|not in the catalog" | head -20`
Expected: lista de features sin cubrir y de referencias inválidas.

Para cada feature sin cubrir: añadir un caso a `CASES` que la ejercite (o
ampliar el `covers` de un caso existente si ya la ejercita), y su test en
`tests/parity_freeze_tests.rs`.

Para cada referencia inválida: corregir el `covers` al nombre real.

Run: `cargo test --test parity_coverage_tests 2>&1 | tail -5`
Expected: PASS (3 tests).

- [ ] **Step 3: Probar que la cobertura muerde (sonda de mutación)**

Añadir temporalmente un built-in al catálogo, en `src/lang/catalog.rs`:

```rust
    Builtin { name: "zzz_probe",   arity: 1..=1, support: Both },
```

Run: `cargo test --test parity_coverage_tests every_builtin_with_both_support_is_covered`
Expected: FALLA con `builtin 'zzz_probe' has no parity case covering it`.
**Quitar el built-in temporal** y volver a ejecutar para confirmar verde.

- [ ] **Step 4: Commit**

```bash
git add tests/parity_coverage_tests.rs tests/common/cases.rs tests/parity_freeze_tests.rs src/lang/catalog.rs
git commit -m "test: derive the parity corpus coverage from the language catalog

Three tests assert that every builtin and method with Both support is
exercised by a parity case and that every covered feature name exists in
the catalog, so a new builtin without a case breaks the suite."
```

---

### Task 6: Verificación final de P0 (P0.4)

**Files:**
- Modify: `docs/superpowers/plans/2026-10-20-p0-registro-de-paridad-y-cobertura.md` (checkboxes)

**Interfaces:**
- Consumes: todo lo anterior.
- Produces: nada nuevo.

- [ ] **Step 1: Verificar la suite completa**

Run: `cargo test 2>&1 | grep -E "^test result" | awk '{p+=$4; f+=$6} END {print "passed", p, "failed", f}'`
Expected: `failed 0`, y `passed` ≥ 900.

Run: `cargo test --test parity_freeze_tests 2>&1 | tail -3`
Expected: PASS con ≥ 106 casos (34 originales + 40 + 32).

Run: `cargo test --test parity_coverage_tests 2>&1 | tail -3`
Expected: PASS (3 tests).

Run: `cargo clippy --all-targets 2>&1 | grep -cE "^(warning|error)"`
Expected: `0`.

Run: `cargo build 2>&1 | grep -cE "^(warning|error)"`
Expected: `0`.

- [ ] **Step 2: Verificar que ningún backend cambió de comportamiento**

Run: `git diff --stat 0f91d25..HEAD -- src/compiler src/interpreter src/parser src/lexer`
Expected: **vacío**. P0 no toca los backends.

Run: `git diff --stat 0f91d25..HEAD -- src/lang`
Expected: solo si Task 5 necesitó ajustar el catálogo (y entonces debe ser
justificado en el mensaje de commit).

- [ ] **Step 3: Actualizar la documentación**

En `README.md`:
- La fila de la tabla "✅ Implementado" de la suite de tests: actualizar el
  número al total real.
- Añadir `tests/parity_coverage_tests.rs` al árbol de `tests/`.

En `docs/superpowers/specs/2026-10-17-refactor-amplio-y-paridad-design.md`:
- Actualizar la fila P0 de la tabla §3 con el estado "hecho" y el enlace al
  spec de P0.

- [ ] **Step 4: Marcar los checkboxes del plan**

Run: `sed -i 's/^- \[ \] /- [x] /' docs/superpowers/plans/2026-10-20-p0-registro-de-paridad-y-cobertura.md`
Expected: todos los pasos marcados.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "docs: record the P0 parity registry and the corpus coverage

Updates the test tally and the docs to the state after P0: the parity
registry is now a single case table and its coverage is derived from the
language catalog."
```

---

## Verificación de P0 (Definition of Done)

1. `cargo test` → **0 fallos**, ≥ 900 tests.
2. `cargo test --test parity_freeze_tests` → ≥ 106 casos, verde.
3. `cargo test --test parity_coverage_tests` → 3/3 verde.
4. Una divergencia no declarada hace fallar la suite (sonda de mutación).
5. Una entrada obsoleta hace fallar la suite (sonda de mutación).
6. Un built-in nuevo sin caso hace fallar la suite (sonda de mutación).
7. `cargo clippy --all-targets` y `cargo build` → 0 warnings.
8. `git diff --stat <base>..HEAD -- src/compiler src/interpreter src/parser src/lexer` → vacío: P0 no cambió ningún backend.
9. Toda divergencia de la tabla tiene motivo y una fase válida del enum `Phase`.
10. Historial: un commit por tarea, en `main`, mensajes en inglés.

Tras P0, el siguiente ciclo es **P1** (valores y formato), con su propio
spec + plan.
