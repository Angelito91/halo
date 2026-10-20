# P0 — Registro de paridad y cobertura total del corpus — Design

> **Estado:** spec escrito tras el cierre de R0 (2026-10-20). R0 dejó la red
> de congelado en pie; P0 la convierte en una **red de paridad** que cubre
> toda la superficie del lenguaje y whose registro de divergencias no puede
> mentir por deriva.
>
> **Fecha:** 2026-10-20
> **Autor:** Angel A. Portuondo H.
> **Ámbito:** arnés diferencial (`tests/common/parity.rs`), corpus congelado
> (`tests/parity_freeze_tests.rs`), catálogo (`src/lang/catalog.rs`)
>
> **Precedente:** `docs/superpowers/specs/2026-10-17-refactor-amplio-y-paridad-design.md` §3 (tabla de
> sub-proyectos), §3.1 (regla transversal del ledger), §1.1 (divergencias medidas)

---

## 1. Contexto

R0 (el "refactor amplio") quedó cerrado y commiteado (`ff3d13a`). Dejó:

- un **arnés diferencial** (`tests/common/parity.rs`) que ejecuta el mismo
  snippet en el intérprete y en un binario nativo y compara stdout, exit code
  y stderr;
- una **tabla anti-mentira** `KNOWN_DIVERGENCES` con 26 entradas, cada una con
  motivo y fase;
- un **corpus congelado** de 34 casos (`tests/parity_freeze_tests.rs`);
- la **fuente única de verdad** del lenguaje (`src/lang/`): catálogo de
  built-ins y métodos con su matriz de soporte, reglas de formato y mensajes
  de error.

R0 dejó el arnés *correcto* pero *estrecho*. P0 es el sub-proyecto que el
propio spec de R0 define como: *"Ampliar la red a **paridad** con tabla de
divergencias conocidas"* (§3, fila P0).

## 2. Problema

### 2.1 El corpus cubre una fracción de la superficie del lenguaje

La superficie verificable de Halo hoy es:

| Superficie | Elementos | Casos que la ejercitan hoy |
|---|---|---|
| Expresiones | 13 (`Number`, `Float`, `Bool`, `StringLiteral`, `Var`, `Unary`, `Binary`, `Assign`, `Call`, `ArrayLiteral`, `Index`, `MethodCall`, `IndexAssign`) | ~6 |
| Sentencias | 8 (`Expr`, `VarDecl`, `If`, `While`, `For`, `Return`, `Break`, `Continue`) | ~3 |
| Operadores binarios | 14 (`+ - * / % == != < > <= >= && \|\|`) | 6 |
| Operadores unarios | 2 (`-`, `!`) | 0 |
| Built-ins | 26 en `src/lang/catalog.rs` | 9 |
| Métodos | 16 en `src/lang/catalog.rs` | 8 |
| Clases de error | ~9 (variable/función indefinida, aridad, tipo, índice negativo, índice fuera de rango, división por cero, overflow, profundidad de recursión, límite de iteraciones) | 2 |

Un lenguaje puede cambiar de comportamiento en cualquier punto de esa
superficie sin que la red de paridad se entere. Un built-in nuevo puede
existir en un backend y no en el otro sin que ninguna prueba lo detecte: el
catálogo lo declara, pero **nada obliga a que el corpus lo ejercite**.

### 2.2 El registro son dos listas que pueden divergir entre sí

Hoy el registro (`KNOWN_DIVERGENCES`) y el corpus (`case_*`) son dos listas
independientes en dos archivos distintos. La regla transversal §3.1 del spec
de R0 exige que *toda* divergencia conocida:

1. esté listada en `KNOWN_DIVERGENCES` con motivo y fase;
2. **tenga al menos un test que la ejercita**;
3. si su fase ya pasó y sigue divergiendo, el test falla.

El punto 2 **no está verificado mecánicamente**: nadie comprueba que cada
`id` de la tabla tenga un caso que lo ejercite. Hoy se cumple por disciplina
manual, y se rompería en silencio el primer día que alguien añada una
divergencia y olvide su caso.

### 2.3 El corpus y el catálogo no están conectados

`src/lang/catalog.rs` es la fuente única de verdad de *qué* built-ins y
métodos existen y *en qué* backend. El corpus no lo consulta. La consecuencia
es que la cobertura de la red de paridad es una disciplina humana, no una
propiedad verificada.

## 3. Objetivo

> **Una red de paridad que (a) ejercite toda la superficie del lenguaje,
> (b) haga imposible una divergencia sin dueño, y (c) derive su propia
> cobertura del catálogo, de modo que añadir un built-in sin añadir su caso
> rompa la suite.**

Tres propiedades, en este orden:

1. **Cobertura total.** Cada elemento de la superficie (expresiones,
   sentencias, operadores, built-ins, métodos, clases de error) tiene al menos
   un caso en el corpus.
2. **Registro íntegro.** No puede existir una divergencia declarada sin un
   caso que la ejercite, ni un caso divergente sin declarar. La deriva entre
   las dos listas deja de ser representable.
3. **Cobertura derivada.** La lista de features que la red debe cubrir se
   calcula desde `src/lang/catalog.rs`, no desde una lista escrita a mano.

**No cambia el comportamiento de ningún backend.** P0 sigue siendo una fase
de *observación*: Describe y congela, no arregla. Las divergencias que
aparezcan se registran con su fase; las arreglan P1–P3.

## 4. Decisiones de diseño

### D1 — Una sola lista: el caso **es** el registro

El registro y el corpus se fusionan en una única tabla de casos. Cada entrada
declara su snippet **y** su resultado esperado:

```rust
pub struct ParityCase {
    /// Identificador estable del caso. Es el nombre del test.
    pub id: &'static str,
    /// Programa Halo que se ejecuta en ambos modos.
    pub code: &'static str,
    /// `None` = ambos modos deben coincidir byte a byte.
    /// `Some(d)` = se espera una divergencia, con motivo y fase que la cierra.
    pub divergence: Option<KnownDivergence>,
    /// Features del catálogo que este caso ejercita (nombres exactos).
    pub covers: &'static [&'static str],
}
```

Consecuencias directas:

- **No puede haber divergencia sin caso**: la divergencia vive *dentro* del
  caso. §3.1 punto 2 pasa a ser una propiedad estructural, no una disciplina.
- **No puede haber entrada obsoleta**: un caso marcado como divergente que
  hoy coincide → el arnés falla (`stale entry`). Un caso sin marcar que
  diverge → el arnés falla (`undocumented divergence`). Las dos propiedades
  anti-mentira del arnés de R0 se conservan intactas.
- **El snippet vive en un solo sitio**: el caso es la definición; el test es
  una referencia a él.

### D2 — La cobertura se deriva del catálogo

Cada caso declara `covers: &[&str]` con los nombres exactos de las features
del catálogo que ejercita. Tres pruebas mecánicas:

| Prueba | Qué garantiza |
|---|---|
| `every_builtin_with_both_support_is_covered` | Todo built-in con `support == Both` aparece en el `covers` de al menos un caso. |
| `every_method_with_both_support_is_covered` | Igual para los métodos. |
| `every_covered_feature_exists` | Todo nombre en `covers` existe en el catálogo (detecta typos y referencias obsoletas). |

Añadir un built-in a `src/lang/catalog.rs` sin añadir su caso de paridad
**rompe la suite**. Ese es el cierre del bucle que R0 abrió.

> **Decisión de alcance:** la exigencia de cobertura se aplica a las features
> con `support == Both`, que es donde la paridad es exigible. Las
> `InterpreterOnly` y `CompilerOnly` se ejercitan igualmente, pero su
> cobertura es **recomendada, no obligatoria** — su divergencia *esperada*
> está ya en la tabla.

### D3 — El registro se queda en `tests/`, no en `src/lang/`

`src/lang/catalog.rs` describe el **lenguaje**: qué built-ins existen, cómo se
formatean los valores, qué mensajes de error existen. El registro de
divergencias describe el **estado del proyecto**: qué backends difieren hoy.
No es metalinguaje: es estado de trabajo que muere con P1–P3.

Ponerlo en `src/lang/` la convertiría en deuda permanente en la librería
pública. Se queda en `tests/common/`, que es donde vive el arnés.

### D4 — El `phase` se valida contra un enum, no contra un string

Hoy `phase: &'static str` admite cualquier texto; un typo ("P4", "p1") pasaría
desapercibido hasta que esa fase nunca llegara y la divergencia quedara
colgada. P0 lo cambia a un enum:

```rust
pub enum Phase { P1, P2, P3 }
```

con `Display` para el mensaje de error. Una divergencia sin fase válida no
compila.

### D5 — Los snippets van en una línea, con `\n` escapados

La tabla de casos es una entrada por línea. Los snippets se escriben como
strings de Rust con `\n` escapados, no como `r#"…"#` multilínea: así la tabla
se lee de un vistazo, un `git diff` de la tabla muestra solo las líneas que
cambian, y la entrada de un caso ocupa una línea. El formato del lenguaje
Halo no tiene saltos de línea obligatorios dentro de una expresión, así que
`\n` es el separador de sentencias.

## 5. La matriz de cobertura

Objetivo: pasar de **34** a **≥ 90** casos, sin dejar ningún hueco de la
superficie.

### 5.1 Operadores y literales (≈16 casos)

`+` `-` `*` `/` `%` entre enteros; los mismos entre flotantes; la promoción
int↔float; `==` `!=` entre tipos mixtos; `<` `>` `<=` `>=`; `&&` `||` con
cortocircuito (que el segundo operando no se evalúa); `!`; `-` unario; literales
entero, flotante, booleano, string con comillas simples, string con comillas
dobles, string con escapes.

### 5.2 Sentencias y control de flujo (≈12 casos)

`if` / `else if` / `else` con sus tres ramas; anidamiento; `while` con
`break`; `while` con `continue`; `for` sobre `range()`; `for` con `return`
temprano; `return` sin valor; `return` con valor; asignación a variable ya
declarada; shadowing de variable global desde una función.

### 5.3 Funciones (≈12 casos)

Llamada antes de la definición; recursión simple; recursión mutua
(`is_even`/`is_odd`); llamada anidada; función que devuelve string; función
void; aridad incorrecta (faltan y sobran argumentos); llamada a función
inexistente; el mismo nombre en scope local y global.

### 5.4 Built-ins y métodos (≈30 casos)

Un caso por built-in con `support == Both` (`print`, `len`, `str`, `int`,
`float`, `abs`, `type`, `reverse`, `contains`, `index_of`, `upper`, `lower`,
`trim`, `starts_with`, `ends_with`, `replace`, `substr`), y los de soporte
mixto (`range`, `push`, `pop`, `append`, `first`, `last`, `join`, `sort`,
`println`). Casos de borde: `len` de string, de array y de número; `type` de
cada tipo; `int()` que trunca; `abs` de negativo.

### 5.5 Errores (≈10 casos)

Variable indefinida; función indefinida; aridad incorrecta; tipo de argumento
incorrecto; índice fuera de rango; índice negativo; división por cero;
overflow de entero; límite de profundidad de recursión; límite de iteraciones
de bucle.

### 5.6 Arrays (≈8 casos)

Literal vacío; literal con un elemento; indexación válida; indexación negativa;
`len` de array; `for` sobre array; arrays anidados; array como argumento de
función.

### 5.7 Formato de salida (≈4 casos)

`print` con un argumento; con varios argumentos; sin argumentos; `println`
(solo compilado).

## 6. Criterios de aceptación

P0 está terminado cuando:

1. `cargo test` está verde y el **tally sube** de forma verificable (los
   nuevos casos del corpus son los que suben el número).
2. `cargo test --test parity_freeze_tests` pasa y su número de casos es ≥ 90.
3. `cargo test --test parity_coverage_tests` pasa: las tres pruebas de D2
   verdes.
4. **La red detecta una divergencia real**: al añadir un caso nuevo que
   diverge y **no** declararlo, la suite falla con `undocumented divergence`.
   (Sonda de mutación obligatoria, como la que se hizo en R0 Task 1.)
5. **La red detecta una entrada obsoleta**: al cambiar el `phase` de una
   divergencia ya corregida, la suite falla con `stale entry`.
6. **La cobertura se deriva del catálogo**: al añadir un built-in a
   `src/lang/catalog.rs` sin añadir su caso, `parity_coverage_tests` falla.
   (Sonda de mutación obligatoria sobre el catálogo.)
7. `cargo clippy --all-targets` sigue limpio.
8. Ningún backend cambia de comportamiento: el diff de P0 no toca
   `src/compiler/`, `src/interpreter/` ni `src/parser/` salvo para leer el
   catálogo.

## 7. Fuera de alcance

- **Arreglar divergencias.** Eso es P1 (valores y formato), P2 (errores de
  runtime) y P3 (arrays en compilado). P0 solo *las registra*.
- **Cambiar la superficie del lenguaje.** P0 no añade ni quita sintaxis.
- **REPL, LSP, package manager.** Nada de eso existe todavía.
- **Mover el arnés a la librería.** El arnés es una herramienta de desarrollo;
  vive en `tests/`.
- **Tocar el compilador.** Salvo para leer `src/lang/catalog.rs`.

## 8. Riesgos

| Riesgo | Mitigación |
|---|---|
| El corpus crece y `cargo test` se vuelve lento (cada caso compila un binario con clang) | La caché de binarios ya existe (`/tmp/halo_parity_cache`); 90 casos ≈ 15 s en la primera pasada, ~0 s después. Si aun así molesta, se mide un corpus "smoke" aparte. |
| El `covers` explícito se olvida al añadir un caso | Las tres pruebas de D2 lo detectan: o falta la cobertura, o el nombre no existe. |
| Una divergencia nueva se encuentra a mitad de la ampliación y rompe la suite | Es el objetivo: el arnés falla hasta que se declare con fase y motivo. Se registra en el mismo commit. |
| Acoplar la red al catálogo hace que P1–P3 fallen al cambiar el soporte de un built-in | **Deseado**: P1–P3 cambian el soporte de varios built-ins, y ese cambio debe reescribir la tabla de divergencias, no dejarla obsoleta. El arnés ya falla ante entradas obsoletas. |

## 9. Relación con P1–P3

P0 deja el arnés listo para que P1–P3 sean *mecánicos*: cuando P1 corrija
`bool_print`, el caso correspondiente dejará de divergir, el arnés fallará con
`stale entry`, y P1 tendrá que **borrar la entrada** del registro. La fase de
cada divergencia es así un contrato con trabajo pendiente, no una etiqueta
decorativa.

Ese es exactamente el mecanismo que el spec de R0 (§3.1) llama *"la tabla no
puede mentir"*. P0 se lo entrega al proyecto entero.
