---
title: Arquitectura
description: Cómo está construido Halo por dentro, desde el lexer hasta LLVM, y por qué el código está organizado así.
---

Halo es un proyecto pequeño a propósito. La idea es que puedas abrir cualquier
archivo de `src/` y entender qué hace sin un mapa. Esta página es ese mapa.

## Pipeline

```text
Código fuente (.halo)
        │
        ▼
┌───────────────────────────────┐
│ LEXER        src/lexer/       │
│ · Escaneo lineal O(n)         │
│ · Token stream con posición   │
│ · Newlines como tokens        │
│   significativos              │
└───────────────────────────────┘
        │ tokens
        ▼
┌───────────────────────────────┐
│ PARSER       src/parser/      │
│ · Descendente recursivo       │
│ · AST tipado                  │
│ · Errores con posición        │
└───────────────────────────────┘
        │ AST
        ├─────────────────────────────┐
        ▼                             ▼
┌───────────────────────┐   ┌───────────────────────────────┐
│ INTÉRPRETE            │   │ COMPILADOR     src/compiler/  │
│ src/interpreter/      │   │ · AST → LLVM IR               │
│ · Tree-walking        │   │ · Optimización O0–O3          │
│ · Valores dinámicos   │   │ · Toolchain clang o llc+cc    │
│ · Arrays y métodos    │   │ · Strings (sin arrays)        │
└───────────────────────┘   └───────────────────────────────┘
        │                             │
        ▼                             ▼
   Resultado                    Binario nativo
```

Los dos caminos comparten lexer, parser y AST. La diferencia es qué se hace con
el árbol: recorrerlo o convertirlo en IR.

## Estructura del repositorio

```text
halo/
├── src/
│   ├── main.rs              # Punto de entrada del binario
│   ├── lib.rs               # API pública de la librería
│   ├── lexer/
│   │   ├── lexer.rs         # Escáner
│   │   └── token.rs         # TokenKind y Token
│   ├── parser/
│   │   ├── core.rs          # Parser y utilidades
│   │   ├── declarations.rs  # Funciones y globales
│   │   ├── statements.rs    # if, while, for, return…
│   │   ├── expressions.rs   # Precedencia y expresiones
│   │   ├── ast.rs           # Nodos del AST
│   │   └── visitor.rs       # Recorrido genérico
│   ├── interpreter/
│   │   ├── environment.rs   # Tabla plana + frames
│   │   ├── value.rs         # Value y operaciones
│   │   └── evaluator/       # Evaluación del AST
│   │       ├── core.rs
│   │       ├── statements.rs
│   │       ├── expressions.rs
│   │       ├── calls.rs
│   │       ├── builtins.rs
│   │       └── methods/     # Métodos de arrays y strings
│   ├── compiler/
│   │   ├── mod.rs           # Orquestación y toolchains
│   │   ├── builder.rs       # Contexto LLVM
│   │   ├── types.rs         # Tipos del codegen
│   │   └── codegen/         # AST → LLVM IR
│   ├── cli/                 # Un módulo por subcomando
│   ├── diagnostics/         # Códigos y estilo de errores
│   ├── lang/                # Catálogo de builtins y errores
│   └── utils/               # Constantes y helpers
├── tests/                   # Suite de integración
├── examples/                # Programas .halo comentados
├── website/                 # Esta documentación (Astro + Starlight)
├── SYNTAX.md                # Referencia histórica de sintaxis
└── Cargo.toml
```

## Lexer

El lexer convierte caracteres en tokens. Es un escaneo lineal con un pequeño
estado para strings y comentarios.

- Cada token lleva su **posición** (línea y columna) para los diagnósticos.
- El **salto de línea** es un token significativo: separa sentencias en el
  flujo de tokens, aunque el parser actual no exige un separador entre ellas.
- Los comentarios `//` se descartan.
- Los strings aceptan comillas simples y dobles, con escapes.

```rust
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub position: Position,
}
```

## Parser

El parser es **descendente recursivo**. Cada nivel de precedencia tiene su
propia función (`parse_logical_or`, `parse_logical_and`, `parse_equality`, …),
lo que hace que la tabla de precedencia sea fácil de seguir en el código.

```text
parse_expression
  └── parse_logical_or  ||
        └── parse_logical_and  &&
              └── parse_equality   == !=
                    └── parse_comparison  < > <= >=
                          └── parse_term  + -
                                └── parse_factor  * / %
                                      └── parse_unary  - !
                                            └── parse_primary
```

El resultado es un AST tipado:

```rust
pub enum Statement {
    Expr(Expression),
    VarDecl { name: String, init: Option<Expression>, pos: Position },
    If { cond: Expression, then_branch: Block, … },
    While { cond: Expression, body: Block, … },
    For { var: String, iterable: Expression, body: Block, … },
    Return { value: Option<Expression>, … },
    Break { … },
    Continue { … },
}
```

Las funciones se guardan aparte, en una lista de declaraciones. Antes de
ejecutar el programa, el intérprete las registra todas: de ahí viene
*call-before-definition*.

## Intérprete

El intérprete recorre el AST y evalúa cada nodo. Es un **tree-walking
interpreter** clásico, con dos decisiones de diseño destacables.

### Valores dinámicos

```rust
pub enum Value {
    Number(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    Null,
}
```

Cada operación comprueba los tipos en runtime. La aritmética entera usa
operaciones comprobadas (`checked_add`, `checked_sub`, …), así que el overflow
es un error, no un wrap silencioso.

### Entorno: tabla plana y frames

En lugar de un `HashMap` por scope, Halo usa:

- un `Vec<Entry>` con todas las variables vivas, en orden de creación;
- un `Vec<usize>` con el índice donde empieza cada frame.

```text
entries:      [ x=10 | base=100 | n=5 | resultado=25 ]
scope_starts: [ 0    | 2         | 3   ]
                global  función    for
```

Ventajas:

- Entrar y salir de un scope es hacer `push`/`pop` de un `usize`.
- La búsqueda recorre el vector hacia atrás: lo más interno está al final.
- No hay allocaciones de `HashMap` por llamada.

`set` escribe en el frame actual; `update` busca desde dentro hacia fuera y
actualiza donde encuentra la variable. El `for` usa `update` para que los
acumuladores funcionen, y por eso las variables nuevas del cuerpo desaparecen
al salir del bucle.

### Llamadas y recursión

Cada llamada a función hace `push_scope`, evalúa el cuerpo y hace `pop_scope`.
Un contador limita la profundidad a **500** llamadas para evitar desbordar el
stack de Rust.

## Compilador

El compilador recorre el AST y emite LLVM IR usando `inkwell`. Después invoca
un toolchain para producir el binario:

| Toolchain | Pasos |
|---|---|
| `clang` (por defecto) | `clang programa.ll -o programa` |
| `llc` | `llc programa.ll` → `cc programa.s -o programa` |

Los niveles `-O0` a `-O3` se aplican con los pases de optimización de LLVM. Por
defecto se usa `-O2`.

### Qué soporta hoy

| Característica | Intérprete | Compilado |
|---|:---:|:---:|
| Números, floats, bools, strings | ✅ | ✅ |
| `if`, `while`, `break`, `continue` | ✅ | ✅ |
| Funciones y recursión | ✅ | ✅ |
| `for i in range(...)` | ✅ | ✅ |
| Arrays | ✅ | ❌ |
| `for` sobre arrays | ✅ | ❌ |
| Métodos de string | ✅ | ✅ |
| `range` como valor | ✅ | ❌ |

Esta tabla resume **disponibilidad**, no paridad exacta. Hay divergencias de
formato y de comprobaciones en el backend nativo; están listadas en
[diferencias entre backends](../../referencia/diferencias/).

El compilador genera una función `main` de C (`i32 main()`) que envuelve las
sentencias globales. Las funciones de usuario se convierten en funciones LLVM
con firmas estáticas: todos los parámetros son `i64` (los strings se pasan como
puntero convertido a entero); el retorno es `i64`, `f64` o un puntero a `i8`
para strings.

## Diagnósticos

`src/diagnostics/` centraliza el estilo de los errores y los códigos. El
objetivo es que los mensajes sean consistentes y apunten a la causa, no solo al
síntoma.

```text
Error: Undefined variable: 'x'
```

Los errores de parser incluyen posición. Los de runtime todavía no siempre
incluyen línea y columna; es una tarea pendiente del roadmap.

## Catálogo de builtins

`src/lang/catalog.rs` es la **fuente de verdad** de las funciones y métodos:

```rust
pub struct Builtin {
    pub name: &'static str,
    pub arity: RangeInclusive<usize>,
    pub support: Support,
}
```

`Support` distingue `Both`, `InterpreterOnly` y `CompilerOnly`. Los tests usan
este catálogo para validar aridad y soporte; el intérprete y el compilador son
quienes implementan el comportamiento. Añadir un builtin empieza ahí.

## Decisiones de diseño

- **Sin dependencias pesadas.** Solo `inkwell`, `serde` y lo mínimo.
- **Un binario.** Todos los subcomandos viven en `halo`.
- **Dos backends, diferencias explícitas.** La suite de paridad compara
  intérprete y binario, y registra las divergencias conocidas hasta corregirlas.
  El intérprete es la referencia semántica actual.
- **Legibilidad antes que microoptimización.** Salvo el entorno, que está
  diseñado para evitar allocaciones en el camino caliente.
- **Errores explícitos.** Mejor fallar con un mensaje claro que devolver un
  valor raro.

## Limitaciones conocidas

- El compilador no soporta arrays ni `for` sobre arrays.
- No hay maps, closures ni funciones de primera clase.
- Los errores de runtime no siempre tienen línea/columna.
- No hay REPL ni package manager.
- La stdlib es el catálogo de builtins, no una biblioteca separada.

## Siguiente paso

- [Testing](../testing/).
- [Roadmap](../roadmap/).
- [Contribuir](../contribuir/).
