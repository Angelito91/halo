# Programa de paridad intérprete ↔ compilador, con refactor amplio — Design

> **Estado:** aprobado el diseño en conversación (2026-10-17). Este documento es el
> spec del **programa** y, en detalle, del primer sub-proyecto (**R0**).
> Las fases siguientes (P1–P3) tendrán su propio spec antes de implementarse.

**Fecha:** 2026-10-17
**Autor:** Angel A. Portuondo H.
**Ámbito:** Halo (intérprete tree-walking + backend LLVM)

---

## 1. Problema

Halo tiene dos modos de ejecución —intérprete (`halo run`) y binario nativo
(`halo build` + ejecutable)— que **no se comportan igual**. El usuario pidió
literalmente: *"quiero que todo sea igual y tengan lo mismo compilador e
intérprete"*, y además *"en el proceso quiero hacer refactor para mantener
limpio todo"*.

### 1.1 Evidencia medida

Se ejecutaron 33 programas idénticos en ambos modos (probe diferencial,
2026-10-17). Resultados:

**Formato de salida**

| Caso | Intérprete | Compilado |
|---|---|---|
| `print(true)` / `print(false)` | `true` / `false` | `-1` / `0` |
| `print(5.0)`, `print(1.0/4.0)` | `5.0`, `0.25` | `5.000000`, `0.250000` |
| `print("a", "b", 1)` | `a\nb\n1` (¡una línea por argumento!) | `a b 1` |
| `str(<int en runtime>)` | `7` | `?` |
| `str(<float en runtime>)` | `1.5` | `?` |
| `f() { }` → `print(f())` | `null` | `0` |

**Falla de compilación o salida vacía (bugs, no "features que faltan")**

| Caso | Resultado en compilado |
|---|---|
| `b = true` | error: *"Failed to coerce int to slot type"* |
| `print(1 == 1)` | no imprime nada |
| `print("n=" + 5)` | no imprime nada |
| `f(n) { return str(n) }` | error: *"widen_to_i64: value is a pointer"* |
| `int("42")` / `float("1.5")` en runtime | error: *"not yet supported in native compilation"* |

**Ausente por completo en compilado (arrays)**

Literales, indexación, índice negativo, `push`/`pop`/`append`/`first`/`last`/
`reverse`/`sort`/`join`/`index_of`, `len(array)`, `for x in array`,
`range()` como valor.

**Errores que el intérprete detecta y el binario no**

| Caso | Intérprete | Compilado |
|---|---|---|
| overflow entero | error *"Integer overflow in addition"* | envuelve en silencio (`-9223372036854775808`) |
| `1 / 0` | error *"Division by zero"* | valor basura / UB |
| índice negativo | error *"Array index cannot be negative, got -1"* | (no hay arrays) |

**Divergencia en el sentido contrario (existe en compilado, no en intérprete)**

| Caso | Intérprete | Compilado |
|---|---|---|
| `println("hola")` | error *"Undefined function: 'println'"* | `hola` |

`println` se trata en `builtins/mod.rs` del compilador como `print` + salto de
línea, pero el intérprete no lo conoce. La paridad exige decidir en P1: o se
implementa `println` en el intérprete, o se retira del compilador.

### 1.2 Causa raíz

El conocimiento semántico está **duplicado**: el catálogo de builtins y
métodos, las reglas de formato de valores y los mensajes de error viven
escritos a mano por separado en `src/interpreter/` y en `src/compiler/`.
Nada obliga a que ambos lados coincidan, así que han divergido. Además hay
código muerto acumulado (88 marcadores `#[allow(dead_code)]`) y archivos
demasiado grandes (`builtins/strings.rs` 1108 líneas, `parser.rs` 804,
`generator.rs` 784, `evaluator/core.rs` 783, `methods.rs` 770,
`cli/commands.rs` 544).

---

## 2. Objetivo y no-objetivos

### Objetivo

1. **Paridad observable exacta**: un mismo programa Halo válido produce la
   misma salida, los mismos efectos y los mismos errores (mismo mensaje,
   mismo código de salida) en ambos modos. Decisión del usuario: **paridad
   exacta, sin excusas** — el binario inserta comprobaciones de runtime y
   aborta con el mismo mensaje que el intérprete.
2. **Suelo limpio**: refactor amplio de todo el proyecto, sin cambios de
   comportamiento, para que el código de paridad se escriba una vez en la
   estructura definitiva.

### No-objetivos

- No se añaden features nuevas de lenguaje (diccionarios, REPL, `/* */`, etc.
  siguen su propio plan del roadmap).
- No se cambia la sintaxis ni la semántica del lenguaje.
- No se reescribe ningún backend desde cero.
- No se toca el LSP en esta fase: no existe código LSP, solo documentación de
  diseño (ver §6).

---

## 3. Programa completo y orden

El trabajo es multi-subsistema, así que se ejecuta como **sub-proyectos con
spec → plan → implementación cada uno**. Orden aprobado por el usuario
(**refactor primero**):

| # | Sub-proyecto | Contenido | Spec |
|---|---|---|---|
| **R0** | **Refactor amplio** | Red de congelado + partir módulos + fuente única del catálogo/formato/errores + poda de código muerto + docs de arquitectura | este documento |
| P0 | Arnés diferencial | Ampliar la red a **paridad** con tabla de divergencias conocidas | propio |
| P1 | Valores y formato | `true/false`, float mínimo, `null`, `str()` en runtime, `print` multi-argumento; bugs de bool/comparación/concatenación; decisión sobre `println` | propio |
| P2 | Errores de runtime idénticos | overflow, div/0, índice; `halo_runtime_error(msg)` con mismo mensaje y mismo exit code | propio |
| P3 | Arrays en compilado | runtime real: literales, indexación con bounds y negativo, métodos, `len`, `for x in array`, `range()` como valor, arrays anidados | propio |

**Por qué R0 primero (decisión del usuario):** evita refactorizar dos veces el
código de paridad; cuando llegue P1–P3, la estructura y la fuente única ya
existen.

### 3.1 Regla transversal del ledger

Ninguna divergencia puede quedar "pendiente" sin dueño. Toda divergencia
conocida:
1. está listada en `KNOWN_DIVERGENCES` (tabla del arnés, §4.2), con motivo y
   fase que la cierra;
2. tiene al menos un test que la ejercita;
3. si su fase ya pasó y sigue divergiendo, el test falla (la tabla no puede
   mentir).

---

## 4. R0 en detalle

### 4.1 Reglas de oro

- **Cero cambios de comportamiento.** Un commit de R0 no cambia ninguna
  salida byte a byte. Si al refactorizar se descubre un bug, se **documenta y
  se difiere** a su fase (P1–P3); no se arregla a escondidas. *(Los bugs de
  §1.1 se arreglan en P1/P2, no en R0.)*
- **Fachada pública estable.** Se conservan las rutas que usan los tests y la
  CLI: `halo::lexer`, `halo::parser`, `halo::interpreter`, `halo::compiler`,
  `halo::diagnostics`. Cambia la organización interna, no la API.
- **Cada commit deja la suite verde** (hoy: 784 tests, 0 fallos) y respeta el
  corpus de congelado.
- **Commits lógicos** en `main`, mensajes en inglés (conventional commits,
  como el historial del repo), documentación en español.
- **Sin duplicar la fuente de verdad**: cualquier nombre de builtin/método,
  regla de formato o mensaje de error usado por los dos backends procede de
  `src/lang/`.

### 4.2 R0.0 — Red de congelado (lo primero)

En `tests/common/mod.rs`:

```rust
/// Resultado de ejecutar un snippet en ambos modos.
pub struct BothModes {
    pub interpreter: String,               // stdout del intérprete
    pub interpreter_error: Option<String>, // mensaje de error del intérprete
    pub compiled: Option<String>,          // stdout del binario; None si no compiló
    pub compiled_error: Option<String>,    // error de compilación o de ejecución
}

/// Ejecuta `code` en el intérprete y (compilándolo con clang) en nativo.
/// Cachea el binario por hash del source para no recompilar en cada test.
pub fn both_modes(code: &str) -> BothModes;

/// Compara ambos modos aplicando la tabla de divergencias conocidas.
/// Falla si aparece una divergencia NO declarada o si una declarada desaparece.
pub fn assert_parity(id: &str, code: &str);

/// Tabla de divergencias conocidas, con motivo y fase que las cierra.
pub const KNOWN_DIVERGENCES: &[KnownDivergence];
```

Reglas del arnés (anti-mentira):

- **Normalización:** se comparan `(stdout, ¿hubo error?)` del intérprete contra
  los del binario. Una diferencia (incluido "éxito vs error") es divergencia.
- `interpreter != compiled` y `id` **no** está en `KNOWN_DIVERGENCES` → **falla**
  (divergencia no declarada).
- `interpreter == compiled` y `id` **sí** está en `KNOWN_DIVERGENCES` → **falla**
  (hay que quitar la entrada: ya no diverge).
- `compiled == None` (no compiló) cuenta como divergencia declarada mientras
  esté en la tabla.

`KNOWN_DIVERGENCES` inicial: una entrada por fila de las tablas de §1.1, con
`reason` y `phase` (`P1`, `P2`, `P3`). El corpus de congelado son esos 33
programas más el caso `println` (ampliable) y su salida de referencia es la del
intérprete.

**Criterio de aceptación:** la suite queda verde; añadir una divergencia nueva
sin declararla hace fallar el arnés (se demuestra con un test temporal).

### 4.3 R0.1 — `src/lang/` (fuente única, descriptiva)

Módulo nuevo, sin dependencias de LLVM ni del intérprete:

- `lang/catalog.rs`: catálogo de **builtins** y **métodos** con, por entrada,
  `name`, `arity` (o rango de aridad) y
  `support: Both | InterpreterOnly | CompilerOnly`, tal **como está hoy**
  (hoy hay al menos un `CompilerOnly`: `println`). Nace descriptivo para no
  cambiar comportamiento: cuando una fase añade o retira soporte, se cambia el
  flag.
- `lang/format.rs`: reglas de formato de valores (cómo se imprime/stringifica
  cada tipo). En R0 documenta el formato **del intérprete** como referencia
  canónica; P1 hará que el compilador lo consuma.
- `lang/errors.rs`: tabla única de mensajes de runtime (hoy dispersos);
  P2 hará que ambos backends la usen.

**Criterio de aceptación:** existe el módulo y el **arnés de R0.0 y los tests
lo consumen como fuente única de verdad** (comparan el catálogo contra el
comportamiento observado de ambos modos). Los backends lo consumirán de verdad
en P1/P2, que es cuando su comportamiento *debe* cambiar; en R0 solo se
consume desde tests y documentación, para no alterar ninguna salida.

### 4.4 R0.2 — Partir el intérprete

- `interpreter/evaluator/core.rs` (783) → `evaluator/mod.rs` (estructura +
  estado) + `evaluator/statements.rs` + `evaluator/expressions.rs` +
  `evaluator/calls.rs`.
- `interpreter/evaluator/methods.rs` (770) → `evaluator/methods/mod.rs` +
  `methods/strings.rs` + `methods/arrays.rs`.
- Sin cambios de comportamiento; solo movimiento y `pub(crate)`.

### 4.5 R0.3 — Partir el compilador

- `compiler/codegen/builtins/strings.rs` (1108) →
  `builtins/strings/mod.rs`, `helpers.rs` (macro `b!`, helpers malloc/strlen),
  `trim.rs`, `replace.rs`, `substr.rs`, `search.rs` (`index_of`, `contains`,
  `starts_with`, `ends_with`), `case.rs` (`upper`, `lower`, `reverse`),
  `join.rs`, `sort.rs`.
- `compiler/codegen/generator.rs` (784) → `codegen/module.rs` (emisión de
  módulo/funciones) + `codegen/functions.rs` (firmas y **inferencia de tipo de
  retorno**, hoy en los helpers sueltos `stmt_has_string` /
  `return_type_of_stmt`) + `emitter.rs` sin cambios.

### 4.6 R0.4 — Partir el parser

- `parser/parser.rs` (804) → `parser/core.rs` (cursor, `advance`/`check`/
  `consume`, manejo de errores) + `parser/declarations.rs` (top level:
  funciones/globales) + `parser/statements.rs`
  (`if`/`while`/`for`/`return`/`break`/`continue`) + `parser/expressions.rs`
  (expresiones y precedencia).

### 4.7 R0.5 — CLI y diagnostics

- `cli/commands.rs` (544) → `cli/run.rs`, `cli/build.rs`, `cli/check.rs`,
  `cli/tokens.rs`, `cli/ast.rs`, `cli/llvm.rs` + `cli/helpers.rs`
  (linkeo con clang/llc).
- `diagnostics/error_codes.rs` (480) → `diagnostics/error_codes/{mod,lexer,
  parser,type,runtime}.rs` por categoría.

### 4.8 R0.6 — Poda y documentación

- Eliminar los `#[allow(dead_code)]` cuyo motivo ya caducó (p. ej.
  "PLANNED FOR v0.3.0": v0.3 ya está hecha) tras comprobar que no los usan
  tests ni CLI.
- Actualizar `COMPILER_REFACTORING.md` y `REFACTORING_SUMMARY.md` al estado
  real; retirar afirmaciones obsoletas.
- Marcar `LSP_ARCHITECTURE.md` / `LSP_IMPLEMENTATION_GUIDE.md` como **diseño
  futuro, no implementado** (no existe `src/lsp`).

### 4.9 Estructura objetivo

```
src/lang/{mod,catalog,format,errors}.rs                    ← fuente única (nueva)
src/parser/{mod,core,declarations,statements,expressions}.rs
src/interpreter/evaluator/{mod,statements,expressions,calls}.rs
src/interpreter/evaluator/methods/{mod,strings,arrays}.rs
src/compiler/codegen/{module,functions,generator,emitter}.rs
src/compiler/codegen/builtins/strings/{mod,helpers,trim,replace,substr,
                                      search,case,join,sort}.rs
src/cli/{mod,args,run,build,check,tokens,ast,llvm,helpers}.rs
src/diagnostics/error_codes/{mod,lexer,parser,type,runtime}.rs
```

### 4.10 Orden de ejecución y riesgo

`R0.0 → R0.1 → R0.2/R0.3/R0.4 (mayor a menor riesgo) → R0.5 → R0.6`.
R0.1 va antes para que los demás ya consuman la fuente única. Riesgo principal:
los `pub` cruzados entre módulos al partir archivos; mitigado por commits
pequeños, suite verde y el corpus.

### 4.11 Verificación

- `cargo test` verde en cada commit (hoy 784 passed / 0 failed).
- `cargo build` sin warnings.
- Corpus de congelado: salidas del intérprete idénticas antes y después de
  cada slice (y las divergencias declaradas, sin cambios).
- Sin warnings nuevos de `clippy` en el código tocado.

---

## 5. Decisiones abiertas (para las fases siguientes)

- **P2 / mensajes de error:** el intérprete imprime errores con color y
  prefijo (`error: Runtime error: …`). Antes de P2 hay que fijar el **formato
  canónico** (qué va a stdout vs stderr, y el **exit code**), y su relación con
  la fase "errores con línea/columna" del roadmap. Se decide en el spec de P2.
- **P3 / representación de valores en compilado:** los arrays de Halo son
  **heterogéneos** (`Value::Array(Vec<Value>)`), lo que choca con el tipado
  estático actual (`i64`/`f64`/`ptr` por variable). Opciones: (a) unión
  etiquetada uniforme `HaloValue` para todo; (b) cajear solo los elementos de
  arrays. Es la decisión arquitectónica pivotal de P3 y se presenta con
  trade-offs en su spec.

---

## 6. Estado del LSP

No existe `src/lsp` ni dependencias LSP en `Cargo.toml`. Los documentos
`LSP_ARCHITECTURE.md` y `LSP_IMPLEMENTATION_GUIDE.md` son **diseño**, no código.
R0.6 los marca como tal; implementarlos (o retirarlos) es una decisión futura
del usuario, fuera de este programa.
