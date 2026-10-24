---
title: La CLI de Halo
description: Referencia de los subcomandos run, build, check, tokens, ast y llvm, con sus flags y códigos de salida.
---

Halo expone un único binario, `halo`, que cubre todas las fases del lenguaje:

```text
halo <SUBCOMANDO> [OPCIONES] <archivo.halo>
halo <archivo.halo>          # atajo para `halo run`
```

## Opciones globales

| Flag | Alias | Descripción |
|---|---|---|
| `--help` | `-h` | Muestra la ayuda |
| `--version` | `-V` | Muestra la versión |

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

El intérprete no necesita LLVM ni genera archivos. Es la forma más rápida de
probar ideas y la única que hoy soporta arrays y la mayoría de métodos.

## `halo build` — Compilar a binario nativo

Genera un ejecutable pasando por LLVM IR.

```bash
# Compila a ./programa (toolchain clang por defecto)
halo build programa.halo

# Nombre de salida
halo build -o mi_programa programa.halo

# Optimización
halo build -O0 programa.halo
halo build -O3 programa.halo

# Compila y ejecuta
halo build --run programa.halo
halo build -r programa.halo

# Conserva el .ll intermedio
halo build --emit-llvm programa.halo

# Toolchain alternativo
halo build --toolchain llc programa.halo

# Verbose
halo build --verbose programa.halo
```

| Flag | Alias | Descripción |
|---|---|---|
| `-o, --output <ruta>` | | Ruta del binario de salida. Por defecto usa el nombre del fuente |
| `-O0` / `-O1` / `-O2` / `-O3` | `--opt <N>` | Nivel de optimización LLVM. Por defecto `-O2` |
| `--toolchain <nombre>` | | `clang` (por defecto) o `llc` |
| `--emit-llvm` | | Conserva el archivo `.ll` intermedio |
| `--run` | `-r` | Ejecuta el binario al terminar |
| `--verbose` | `-v` | Muestra cada fase del pipeline |

### Toolchains

| Toolchain | Comando interno | Cuándo usarlo |
|---|---|---|
| `clang` | `clang <archivo.ll> -o <salida>` | Entorno habitual con Clang |
| `llc` | `llc <archivo.ll>` → `cc <archivo.s> -o <salida>` | Solo hay `llc` y un compilador C |

:::caution
El modo compilado todavía no soporta arrays. `for` solo acepta `range(...)`
directamente; iterar sobre un array o una variable produce un error de
“not yet supported”. También hay divergencias de formato y de comprobaciones
en el backend nativo: consulta
[diferencias entre backends](../../referencia/diferencias/).
:::

## `halo check` — Validar sintaxis

Lexea y parsea el archivo sin ejecutar ni generar código. Útil en CI o como
paso previo a un commit.

```bash
halo check programa.halo
# ✅ programa.halo — OK (2 top-level item(s))
```

El código de salida es `0` si no hay errores y `1` si el parse falla.

## `halo tokens` — Inspeccionar el lexer

Muestra el flujo de tokens con su posición.

```bash
halo tokens programa.halo
```

Salida abreviada:

```text
📋 Token Stream — programa

   0  Identifier   'programa'    1:1
   1  LParen       '('           1:9
   2  RParen       ')'           1:10
   3  LBrace       '{'           1:12
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
🌳 Abstract Syntax Tree — programa

[0] Function
    name   : factorial
    params : n
    body   : 2 statement(s)
      [0] if n <= 1 { return 1 }
      [1] return n * factorial(n - 1)
```

Compara esta salida con el código fuente: cada nodo del AST corresponde a una
construcción del lenguaje.

## `halo llvm` — Emitir LLVM IR

Genera el archivo `.ll` sin enlazar ni ejecutar.

```bash
# Emite programa.ll
halo llvm programa.halo

# Nombre de salida
halo llvm -o salida.ll programa.halo

# Optimización
halo llvm -O3 programa.halo

# Verbose
halo llvm --verbose programa.halo
```

| Flag | Alias | Descripción |
|---|---|---|
| `-o, --output <ruta>` | | Ruta del `.ll`. Por defecto `<stem>.ll` |
| `-O0` / `-O1` / `-O2` / `-O3` | `--opt <N>` | Nivel de optimización. Por defecto `-O2` |
| `--verbose` | `-v` | Muestra el progreso del codegen |

## Fases por subcomando

| Subcomando | Lex | Parse | Interpreta | Codegen | Enlaza |
|---|:---:|:---:|:---:|:---:|:---:|
| `run` | ✅ | ✅ | ✅ | — | — |
| `build` | ✅ | ✅ | — | ✅ | ✅ |
| `check` | ✅ | ✅ | — | — | — |
| `tokens` | ✅ | — | — | — | — |
| `ast` | ✅ | ✅ | — | — | — |
| `llvm` | ✅ | ✅ | — | ✅ | — |

## Códigos de salida

| Código | Significado |
|---|---|
| `0` | Éxito |
| `1` | Error de lexer, parser, runtime o compilación |

Los errores se escriben en `stderr`; la salida normal del programa va a
`stdout`.

## Flujo típico

```bash
halo check programa.halo      # 1. valida
halo run programa.halo        # 2. prueba la semántica
halo ast programa.halo        # 3. inspecciona si algo no cuadra
halo llvm programa.halo       # 4. mira el IR
halo build -O3 programa.halo  # 5. genera el binario
```
