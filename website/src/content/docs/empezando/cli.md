---
title: La CLI de Halo
description: Referencia de los subcomandos run, check, tokens, ast, help y version, con sus flags y códigos de salida.
---

Halo expone un único binario, `halo`, que cubre todas las fases del lenguaje:

```text
halo <SUBCOMANDO> [OPCIONES] <archivo.halo>
halo <archivo.halo>          # atajo para `halo run`
```

## Opciones globales

| Flag | Alias | Descripción |
|---|---|---|
| `--help` | `-h` | Muestra la ayuda completa |
| `--version` | `-V` | Muestra la versión |

También puedes usar `halo help` y `halo version` como subcomandos.

## `halo run` — Intérprete

Ejecuta el archivo con el intérprete tree-walking. Es el comportamiento por
defecto.

```bash
halo run programa.halo

# Atajo equivalente
halo programa.halo

# Ver cada fase del pipeline
halo run --verbose programa.halo
halo run -v programa.halo
```

El intérprete no genera archivos ni necesita herramientas externas. Es la
única forma de ejecutar un programa Halo.

`--verbose` muestra el archivo, el número de tokens, el número de elementos de
nivel superior y el resultado de la ejecución.

## `halo check` — Validar sintaxis

Lexea y parsea el archivo sin ejecutar nada. Útil en CI o como paso previo a un
commit.

```bash
halo check programa.halo
# ✅ programa.halo — OK (2 top-level item(s))
```

El código de salida es `0` si no hay errores y `1` si falla el lexer o el
parser. Los errores se muestran con su posición.

## `halo tokens` — Inspeccionar el lexer

Muestra el flujo de tokens con su posición.

```bash
halo tokens programa.halo
```

Salida abreviada:

```text
╔════════════════════════════════════════╗
║       📋 Token Stream — programa       ║
╚════════════════════════════════════════╝

   0  Identifier      'programa'    1:1
   1  LeftParen       '('           1:9
   2  RightParen      ')'           1:10
   3  LeftBrace       '{'           1:12
   ...
```

Es la forma más directa de entender cómo el lexer convierte caracteres en
tokens y cómo trata los saltos de línea.

## `halo ast` — Inspeccionar el AST

Muestra el árbol que produce el parser.

```bash
halo ast programa.halo
```

Salida abreviada:

```text
╔════════════════════════════════════════╗
║   🌳 Abstract Syntax Tree — programa   ║
╚════════════════════════════════════════╝

[0] Function
    name   : factorial
    params : n
    body   : 2 statement(s)
      [0] if n <= 1 { return 1 }
      [1] return n * factorial(n - 1)
```

Compara esta salida con el código fuente: cada nodo del AST corresponde a una
construcción del lenguaje.

## `halo help` / `halo version`

```bash
halo help       # ayuda completa
halo version    # versión del intérprete
```

`halo` sin argumentos muestra un resumen de uso.

## Subcomandos retirados

Halo es **interpreter-only** desde la versión 0.3.0. Los antiguos `build` y
`llvm` ya no existen, igual que las opciones `-o`, `-O`, `--toolchain`,
`--emit-llvm` y `--run`. Si un script antiguo los usa, la CLI falla con un
mensaje explícito en lugar de intentar abrir un archivo llamado `build`:

```text
❌ The 'build' subcommand was removed: Halo is interpreter-only.
Use 'halo run <file.halo>' to execute the file.
```

## Fases por subcomando

| Subcomando | Lex | Parse | Interpreta |
|---|:---:|:---:|:---:|
| `run` | ✅ | ✅ | ✅ |
| `check` | ✅ | ✅ | — |
| `tokens` | ✅ | — | — |
| `ast` | ✅ | ✅ | — |

## Códigos de salida

| Código | Significado |
|---|---|
| `0` | Éxito |
| `1` | Error de lexer, parser o runtime |

Los errores se escriben en `stderr`; la salida normal del programa va a
`stdout`.

## Flujo típico

```bash
halo check programa.halo      # 1. valida
halo run programa.halo        # 2. prueba la semántica
halo tokens programa.halo     # 3. mira los tokens si algo no cuadra
halo ast programa.halo        # 4. mira el AST
```
