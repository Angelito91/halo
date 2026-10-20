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
- Consumes: `halo::lang::catalog::{Builtin, Method, Support}` (solo en Task 4).
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

…y una línea por cada id de `CASES` (los 34 ids actuales, luego los nuevos de Task 2 y Task 3).

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

### Task 2: Ampliar el corpus — operadores, literales, control de flujo y funciones (P0.1)

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

### Task 3: Ampliar el corpus — built-ins, métodos, arrays, errores y formato (P0.2)

**Files:**
- Modify: `tests/common/cases.rs`, `tests/parity_freeze_tests.rs`

**Interfaces:**
- Consumes: `common::cases::CASES` (Task 2).
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

Registrar cada una en `CASES` con motivo y fase (tabla de fases del Task 2,
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

### Task 4: Cobertura derivada del catálogo (P0.3)

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

### Task 5: Verificación final de P0 (P0.4)

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
Expected: solo si Task 4 necesitó ajustar el catálogo (y entonces debe ser
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
