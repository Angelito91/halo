---
title: Arquitectura
description: Cómo está construido Halo por dentro, desde el lexer hasta el evaluador, y por qué el código está organizado así.
---

Halo es un proyecto pequeño a propósito. La idea es que puedas abrir cualquier
archivo de `src/` y entender qué hace sin un mapa. Esta página es ese mapa.

## Pipeline

Halo tiene un solo motor: un intérprete tree-walking.

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
        ▼
┌───────────────────────────────┐
│ EVALUADOR    src/interpreter/ │
│ · Tree-walking                │
│ · Valores dinámicos           │
│ · Entorno plano con frames    │
│ · Built-ins en src/stdlib/    │
└───────────────────────────────┘
        │
        ▼
   stdout / error
```

No hay backend nativo, bytecode, VM ni artefactos intermedios. Esa decisión
mantiene el proyecto legible y hace que el comportamiento documentado sea el
mismo que se ejecuta.

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
│   │   └── ast.rs           # Nodos del AST
│   ├── interpreter/
│   │   ├── environment.rs   # Tabla plana + frames
│   │   ├── value.rs         # Value y operaciones
│   │   ├── errors.rs        # Mensajes de runtime
│   │   ├── format.rs        # Formato canónico de valores
│   │   └── evaluator/       # Evaluación del AST
│   │       ├── core.rs
│   │       ├── statements.rs
│   │       ├── expressions.rs
│   │       └── calls.rs
│   ├── stdlib/
│   │   ├── builtins.rs      # Despacho de funciones globales
│   │   ├── arrays.rs        # Operaciones de arrays
│   │   ├── strings.rs       # Operaciones de strings
│   │   └── mod.rs           # Despacho de métodos
│   ├── cli/                 # Un módulo por subcomando
│   └── diagnostics/         # Errores, colores y sugerencias
├── tests/                   # Suite de integración
├── examples/                # Programas .halo comentados
├── website/                 # Esta documentación (Astro + Starlight)
└── Cargo.toml
```

## Lexer

El lexer convierte caracteres en tokens. Es un escaneo lineal con un pequeño
estado para strings y comentarios.

- Cada token lleva su **posición** (línea y columna) para los diagnósticos.
- El **salto de línea** es un token significativo en el flujo de tokens,
  aunque el parser no lo exige entre sentencias: también acepta varias
  sentencias en la misma línea.
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
                                            └── parse_postfix  llamadas, índices, métodos
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
ejecutar el programa, el evaluador las registra todas: de ahí viene
*call-before-definition*.

## Evaluador

El evaluador recorre el AST y evalúa cada nodo. Es un **tree-walking
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
actualiza donde encuentra la variable. El `for` crea un frame propio para su
variable, así que esta no pisa una variable externa. Las asignaciones dentro
del cuerpo siguen actualizando la variable que encuentran, y las variables
nuevas del cuerpo desaparecen al salir del bucle.

Como `get` recorre todo el vector, la lectura no se limita al frame actual: una
función puede leer un local de un frame llamador si no tiene una variable
propia con ese nombre. Es **scope dinámico de lectura**. Las asignaciones
normales siguen confinadas al frame actual; solo el `for` usa `update` para
alcanzar un binding externo.

### Llamadas y recursión

Cada llamada a función hace `push_scope`, evalúa el cuerpo y hace `pop_scope`.
Un contador limita la profundidad a **500** llamadas fuera de los tests para
evitar desbordar el stack de Rust. Los tests usan un límite menor para que el
fallo llegue antes que el desbordamiento nativo.

Los bucles tienen además un límite de **1 000 000 de iteraciones** por
programa, pensado para cortar bucles infinitos sin bloquear la máquina.

### Built-ins y métodos

`src/stdlib/` implementa las funciones globales y los métodos. Reciben
**valores ya evaluados**, no nodos del AST, así que no conocen el evaluador ni
el parser. Añadir una función empieza en `builtins.rs` y termina en `arrays.rs`
o `strings.rs`.

En los built-ins, los argumentos se evalúan antes del despacho. Por eso una
llamada a un builtin puede reportar primero el error de un argumento (por
ejemplo, una variable inexistente) y después la aridad o el método
desconocido. En funciones de usuario, la aridad se comprueba antes de evaluar
los argumentos.

## Diagnósticos

`src/diagnostics/` centraliza el estilo de los errores, los colores y las
sugerencias. El objetivo es que los mensajes sean consistentes y apunten a la
causa, no solo al síntoma.

```text
error: Undefined variable: 'nombra'
  → script.halo:3:7
  Did you mean 'nombre'?
```

Los errores del parser incluyen posición. Los errores de runtime se anotan con
la posición de la sentencia que falló y, cuando aplica, con una sugerencia de
nombre parecido.

## Decisiones de diseño

- **Sin dependencias externas.** El `Cargo.toml` no declara ninguna.
- **Un binario.** Todos los subcomandos viven en `halo`.
- **Interpreter-only.** No hay backend nativo. Cualquier backend futuro sería
  un proyecto separado, no una dependencia del intérprete.
- **Legibilidad antes que microoptimización.** Salvo el entorno, que está
  diseñado para evitar allocaciones en el camino caliente.
- **Errores explícitos.** Mejor fallar con un mensaje claro que devolver un
  valor raro.

## Nota histórica

Hasta la versión 0.2.0 el proyecto incluía un backend LLVM que generaba
binarios nativos. Se eliminó en 0.3.0 para simplificar el código y centrar el
proyecto en el intérprete. El backend anterior está preservado en el tag
`llvm-backend-final`; no forma parte de la versión actual.

## Limitaciones conocidas

- No hay maps, closures ni funciones de primera clase.
- No hay asignación por índice (`arr[0] = x`); los métodos no mutan el array
  original. `push`, `append`, `reverse` y `sort` devuelven uno nuevo.
- No hay REPL ni package manager.
- No hay comentarios multilínea `/* … */`.
- La posición de un error de runtime apunta a la sentencia, no siempre a la
  expresión exacta.

## Siguiente paso

- [Testing](../testing/).
- [Roadmap](../roadmap/).
- [Contribuir](../contribuir/).
