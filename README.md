<div align="center">
  <img src="halo.png" alt="Halo Compiler Banner">

  # 🌟 Halo

  <p align="center">
    <strong>✨ Un viaje educativo al corazón de los compiladores ✨</strong>
  </p>

  <p align="center">
    <img src="https://img.shields.io/badge/estado-en_desarrollo-blue?style=flat-square" alt="Estado: en desarrollo">
    <img src="https://img.shields.io/badge/versión-0.2.0-orange?style=flat-square" alt="Versión: 0.2.0">
    <img src="https://img.shields.io/badge/licencia-MPL_2.0-green?style=flat-square" alt="Licencia: MPL 2.0">
    <img src="https://img.shields.io/badge/rust-1.70+-b7410e?style=flat-square" alt="Rust 1.70+">
    <img src="https://img.shields.io/badge/contribuciones-bienvenidas-brightgreen?style=flat-square" alt="Contribuciones: bienvenidas">
  </p>

  <p align="center">
    <a href="#-motivación">Motivación</a> •
    <a href="#-características">Características</a> •
    <a href="#-sintaxis-rápida">Sintaxis</a> •
    <a href="#-funciones-built-in">Built-ins</a> •
    <a href="#-instalación">Instalación</a> •
    <a href="#-uso">Uso</a> •
    <a href="#-ejemplos">Ejemplos</a> •
    <a href="#-arquitectura">Arquitectura</a> •
    <a href="#-testing">Testing</a> •
    <a href="#-roadmap">Roadmap</a>
  </p>
</div>

---

## 🎯 Motivación

> *"Lo que no se crea, no se entiende"*

Halo nace como un proyecto de aprendizaje práctico para desmitificar el proceso de construcción de compiladores e intérpretes. Lejos de la complejidad de lenguajes industriales, Halo ofrece un playground minimalista donde:

- 🔍 **Exploras** las fases clásicas: lexing, parsing, AST y ejecución
- 🧪 **Experimentas** con nuevas características de lenguaje sin miedo
- 📚 **Aprendes** haciendo, con código simple y bien documentado
- 🎮 **Juegas** con distintas estrategias de implementación

---

## ⚡ Características

### ✅ Implementado

| Característica | Detalles |
|---|---|
| **Variables dinámicas** | Sin tipos explícitos, tipado inferido en runtime |
| **Tipos de dato** | `number` (i64), `float` (f64), `bool`, `string`, `null` |
| **Aritmética entera y flotante** | `+`, `-`, `*`, `/`, `%` con promoción automática |
| **Concatenación de strings** | `"hola" + " mundo"`, `"x" * 3` → `"xxx"` |
| **Comparaciones** | `<`, `>`, `==`, `!=`, `<=`, `>=` (incluso entre tipos mixtos) |
| **Operadores lógicos** | `&&`, `||`, `!` con cortocircuito |
| **Condicionales** | `if / else if / else` anidables |
| **Bucles** | `while` con `break` y `continue` |
| **Funciones** | Definición sin keyword, parámetros, retorno, recursión |
| **Scope aislado** | Cada llamada a función tiene su propio frame |
| **Call-before-definition** | Puedes llamar una función antes de definirla |
| **Recursión mutua** | `is_even` / `is_odd` y similares funcionan |
| **Límite de recursión** | Guard contra stack overflow (500 niveles en producción) |
| **Límite de iteraciones** | Guard contra bucles infinitos (1 000 000 iteraciones) |
| **Funciones built-in** | `print`, `len`, `str`, `int`, `float`, `abs`, `type` |
| **Protección de overflow** | Aritmética entera con checked operations |
| **Intérprete tree-walking** | Ejecución directa del AST |
| **Compilación nativa** | AST → LLVM IR → binario nativo vía clang o llc+cc |
| **Optimización LLVM** | Niveles O0–O3 configurables |
| **Lexer** | Escaneo lineal O(n), soporte de flotantes, strings con escapes |
| **Parser** | Recursivo descendente, expresiones top-level directas |
| **Suite de tests** | +500 tests de integración y unitarios |

### 🚧 En Progreso

| Característica | Prioridad |
|---|---|
| Mensajes de error con número de línea | Alta |
| Arrays / Listas | Media |
| Standard library extendida | Media |

### 📅 Planificado

| Característica | Prioridad |
|---|---|
| Diccionarios / Maps | Media |
| Métodos de string | Media |
| Comentarios multi-línea | Baja |
| Backend a bytecode VM | Baja |
| REPL interactivo | Baja |

---

## 📝 Sintaxis Rápida

A continuación se muestra un vistazo rápido a la sintaxis. Para la referencia completa consulta [SYNTAX.md](SYNTAX.md).

### Variables

```halo
x = 42
precio = 99.99
activo = true
nombre = "Halo"
```

### Operadores

```halo
a = 2 + 3 * 4        // 14  (precedencia estándar)
b = (2 + 3) * 4      // 20
c = 10 % 3           // 1
d = "ab" * 3         // "ababab"
e = "valor: " + 42   // "valor: 42"
```

### Condicionales

```halo
if x > 0 {
    print("positivo")
} else if x == 0 {
    print("cero")
} else {
    print("negativo")
}
```

### Bucles

```halo
i = 0
while i < 10 {
    if i % 2 == 0 {
        i = i + 1
        continue
    }
    print(i)
    i = i + 1
}
```

### Funciones

```halo
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print(factorial(10))   // 3628800
```

> 📖 Referencia completa de sintaxis, precedencia de operadores, semántica de scoping y más ejemplos: **[SYNTAX.md](SYNTAX.md)**

---

## 💾 Funciones Built-in

| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `print(x)` | Imprime un valor en stdout | `print("hola")` |
| `len(x)` | Longitud de string o dígitos de un número | `len("hola")` → `4` |
| `str(x)` | Convierte cualquier valor a string | `str(3.14)` → `"3.14"` |
| `int(x)` | Convierte a entero (trunca floats) | `int(3.9)` → `3` |
| `float(x)` | Convierte a flotante | `float(42)` → `42.0` |
| `abs(x)` | Valor absoluto de number o float | `abs(-7)` → `7` |
| `type(x)` | Nombre del tipo como string | `type(true)` → `"bool"` |

Todos los built-ins aceptan exactamente **1 argumento** (salvo `print`). Pasar un número distinto produce un error de runtime.

---

## 📥 Instalación

### Requisitos

- **Rust 1.70+** — [Instalar Rust](https://www.rust-lang.org/tools/install)
- **Cargo** — incluido con Rust
- **LLVM 21 + Clang** — *solo* necesarios para compilar a binario nativo (`halo build`)

### Compilar desde fuente

```bash
# Clonar el repositorio
git clone https://github.com/Angelito91/halo.git
cd halo

# Compilar en modo debug
cargo build

# Compilar en modo release (recomendado para producción)
cargo build --release

# Verificar errores sin compilar
cargo check
```

Tras compilar, el único binario generado es **`halo`**.

---

## 🚀 Uso

Halo expone un **único binario** (`halo`) que cubre todas las fases del lenguaje: interpretación, compilación, inspección de tokens, AST y emisión de IR.

```
halo <SUBCOMANDO> [OPCIONES] <archivo.halo>
halo <archivo.halo>          ← atajo para 'halo run'
```

---

### `halo run` — Intérprete

Ejecuta el archivo con el intérprete tree-walking (comportamiento por defecto).

```bash
halo run examples/factorial.halo

# Atajo equivalente (omitir subcomando):
halo examples/factorial.halo

# Con salida verbose de cada fase del pipeline:
halo run --verbose examples/factorial.halo
halo run -v examples/factorial.halo
```

---

### `halo build` — Compilar a binario nativo

Genera un ejecutable nativo pasando por LLVM IR.

```bash
# Compilar → ./factorial  (toolchain clang, por defecto)
halo build examples/factorial.halo

# Especificar nombre de salida
halo build -o mi_programa examples/factorial.halo

# Nivel de optimización
halo build -O3 examples/factorial.halo

# Compilar y ejecutar inmediatamente
halo build --run examples/factorial.halo
halo build -r examples/factorial.halo

# Mantener el archivo .ll intermedio en disco
halo build --emit-llvm examples/factorial.halo

# Toolchain alternativo: llc + cc (útil si solo dispones de llc y un compilador C)
halo build --toolchain llc examples/factorial.halo

# Verbose: muestra cada paso del pipeline
halo build --verbose examples/factorial.halo
```

#### Referencia de flags — `halo build`

| Flag | Alias | Descripción |
|------|-------|-------------|
| `-o, --output <ruta>` | | Ruta del binario de salida (por defecto: nombre del archivo fuente) |
| `-O0` / `-O1` / `-O2` / `-O3` | `--opt <N>` | Nivel de optimización LLVM (por defecto: `-O2`) |
| `--toolchain <nombre>` | | Toolchain de enlazado: `clang` (por defecto) o `llc` |
| `--emit-llvm` | | Conserva el archivo `.ll` intermedio en disco |
| `--run` | `-r` | Ejecuta el binario inmediatamente después de compilar |
| `--verbose` | `-v` | Muestra cada fase del pipeline de compilación |

##### Toolchains disponibles

| Toolchain | Comando interno | Cuándo usarlo |
|-----------|----------------|---------------|
| `clang` *(por defecto)* | `clang <file.ll> -o <out>` | Entorno habitual con clang instalado |
| `llc` | `llc <file.ll>` → `cc <file.s> -o <out>` | Entornos donde solo hay `llc` y un compilador C (`cc`) |

---

### `halo check` — Validar sintaxis

Lexea y parsea el archivo sin ejecutar nada. Útil en pipelines CI.

```bash
halo check examples/factorial.halo
# ✅ examples/factorial.halo — OK (2 top-level item(s))

# Código de salida: 0 = sin errores, 1 = errores de parse
```

---

### `halo tokens` — Inspeccionar tokens

Muestra el flujo de tokens que produce el lexer.

```bash
halo tokens examples/factorial.halo
```

Salida de ejemplo:

```
╔════════════════════════════════════════╗
║       📋 Token Stream — factorial     ║
╚════════════════════════════════════════╝

   0  Ident                'factorial'    1:1
   1  LParen               '('            1:10
   2  Ident                'n'            1:11
   3  RParen               ')'            1:12
   ...
```

---

### `halo ast` — Inspeccionar el AST

Muestra el Árbol de Sintaxis Abstracta producido por el parser.

```bash
halo ast examples/factorial.halo
```

Salida de ejemplo:

```
╔════════════════════════════════════════╗
║   🌳 Abstract Syntax Tree — factorial ║
╚════════════════════════════════════════╝

[0] Function
    name   : factorial
    params : n
    body   : 2 statement(s)
      [0] if n <= 1 { return 1 }
      [1] return n * factorial(n - 1)
```

---

### `halo llvm` — Emitir LLVM IR

Genera el archivo `.ll` con el IR de LLVM sin enlazar ni ejecutar.

```bash
# Emite factorial.ll
halo llvm examples/factorial.halo

# Nombre de salida personalizado
halo llvm -o salida.ll examples/factorial.halo

# Con optimizaciones
halo llvm -O3 examples/factorial.halo

# Verbose
halo llvm --verbose examples/factorial.halo
```

#### Referencia de flags — `halo llvm`

| Flag | Alias | Descripción |
|------|-------|-------------|
| `-o, --output <ruta>` | | Ruta del archivo `.ll` de salida (por defecto: `<stem>.ll`) |
| `-O0` / `-O1` / `-O2` / `-O3` | `--opt <N>` | Nivel de optimización LLVM (por defecto: `-O2`) |
| `--verbose` | `-v` | Muestra el progreso de la generación de código |

---

### Opciones globales

| Flag | Alias | Descripción |
|------|-------|-------------|
| `--help` | `-h` | Muestra la ayuda |
| `--version` | `-V` | Muestra la versión |

---

### Resumen de subcomandos

| Subcomando | Descripción |
|---|---|
| `run` | Interpreta el archivo (por defecto) |
| `build` | Compila a binario nativo vía LLVM |
| `check` | Valida sintaxis y reporta errores |
| `tokens` | Muestra el flujo de tokens del lexer |
| `ast` | Muestra el Árbol de Sintaxis Abstracta |
| `llvm` | Emite LLVM IR a un archivo `.ll` |

---

## 💡 Ejemplos

### Factorial (recursión)

```halo
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print(factorial(10))   // 3628800
```

### Fibonacci (recursión mutua-compatible)

```halo
fib(n) {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}

i = 0
while i <= 10 {
    print(fib(i))
    i = i + 1
}
```

### FizzBuzz

```halo
i = 1
while i <= 20 {
    if i % 15 == 0      { print("FizzBuzz") }
    else if i % 3 == 0  { print("Fizz") }
    else if i % 5 == 0  { print("Buzz") }
    else                 { print(i) }
    i = i + 1
}
```

### Operadores lógicos y cortocircuito

```halo
in_range(n) {
    if n >= 1 && n <= 100 { return true }
    return false
}

print(in_range(50))    // true
print(in_range(150))   // false
```

### Conversión de tipos

```halo
n = 42
print(float(n))           // 42.0
print(str(n))             // "42"
print(type(str(n)))       // "string"
print(int("99"))          // 99
print(abs(-3.14))         // 3.14
```

### Manipulación de strings

```halo
sep = "-" * 20
greeting(name) {
    return "Hola, " + name + "!"
}

print(sep)
print(greeting("Halo"))
print(sep)
// --------------------
// Hola, Halo!
// --------------------
```

---

## 📁 Estructura del Proyecto

```
halo/
├── src/
│   ├── lexer/
│   │   ├── lexer.rs          # Tokenizador carácter a carácter
│   │   ├── token.rs          # TokenKind + Token
│   │   └── mod.rs
│   ├── parser/
│   │   ├── ast.rs            # Nodos del AST (Expression, Statement, TopLevel…)
│   │   ├── parser.rs         # Parser recursivo descendente
│   │   ├── visitor.rs        # Patrón Visitor
│   │   └── mod.rs
│   ├── interpreter/
│   │   ├── value.rs          # Enum Value (Number, Float, Bool, String, Null)
│   │   ├── environment.rs    # Scoping con tabla plana + pila de frames
│   │   ├── evaluator.rs      # Evaluador del AST + built-ins + guards
│   │   └── mod.rs
│   ├── compiler/
│   │   ├── codegen.rs        # AST → LLVM IR
│   │   ├── builder.rs        # IRBuilder y gestión de allocas
│   │   ├── types.rs          # TypeMapper: tipos Halo → tipos LLVM
│   │   └── mod.rs            # Compilation, OptLevel
│   ├── main.rs               # Binario halo — CLI unificada
│   └── lib.rs                # Exports públicos de la librería
│
├── tests/
│   ├── common/
│   │   └── mod.rs            # Helpers compartidos (eval_*, assert_*)
│   ├── arithmetic_tests.rs   # Aritmética, floats, overflow, comparaciones
│   ├── control_flow_tests.rs # if/else-if/else, while, break, continue, return
│   ├── function_tests.rs     # Definición, llamada, recursión, scope, aridad
│   ├── interpreter_tests.rs  # Tests end-to-end del intérprete
│   ├── lexer_tests.rs        # Tokenización
│   ├── parser_tests.rs       # Análisis sintáctico y errores de parse
│   ├── scope_and_variable_tests.rs  # Scoping, shadowing, variables globales
│   └── string_and_types_tests.rs    # Strings, built-ins, conversiones
│
├── examples/
│   ├── factorial.halo
│   ├── fibonacci.halo
│   ├── even_numbers.halo
│   ├── logical_and_modulo.halo
│   ├── test_func_call.halo
│   ├── test_return.halo
│   └── comments.halo
│
├── README.md                 # Este archivo
├── SYNTAX.md                 # Referencia completa de sintaxis
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── halo.png
```

---

## 🏗️ Arquitectura

### Pipeline

```
Código Fuente (.halo)
        │
        ▼
┌───────────────────────────────────┐
│  LEXER  (src/lexer/)              │
│  · Escaneo lineal O(n)            │
│  · Produce Token stream           │
│  · Newline como terminador        │
└───────────────────────────────────┘
        │  Token stream
        ▼
┌───────────────────────────────────┐
│  PARSER  (src/parser/)            │
│  · Recursivo descendente          │
│  · Produce AST tipado             │
│  · Reporte de errores con pos.    │
└───────────────────────────────────┘
        │  AST
        ▼
   ┌────┴────┐
   │         │
   ▼         ▼
┌────────┐  ┌──────────────────────────────────┐
│INTÉR-  │  │  COMPILER  (src/compiler/)        │
│PRETE   │  │  · AST → LLVM IR                  │
│(halo   │  │  · Optimización O0–O3             │
│ run)   │  │  · Toolchain clang  → binario     │
│        │  │  · Toolchain llc+cc → binario     │
└────────┘  └──────────────────────────────────┘
   │
   ▼
Resultado / Valor
```

### Subcomandos y fases ejecutadas

| Subcomando | Lex | Parse | Interpret | Codegen | Link |
|---|:---:|:---:|:---:|:---:|:---:|
| `run` | ✅ | ✅ | ✅ | — | — |
| `build` | ✅ | ✅ | — | ✅ | ✅ |
| `check` | ✅ | ✅ | — | — | — |
| `tokens` | ✅ | — | — | — | — |
| `ast` | ✅ | ✅ | — | — | — |
| `llvm` | ✅ | ✅ | — | ✅ | — |

### Tipos de Dato en Runtime

```rust
enum Value {
    Number(i64),    // entero con checked arithmetic
    Float(f64),     // flotante IEEE 754
    Bool(bool),     // true / false
    String(String), // UTF-8
    Null,           // retorno de funciones void
}
```

### Scoping

Halo usa una **tabla plana de entradas** más una **pila de offsets de frame**, lo que evita allocaciones de `HashMap` en cada llamada de función:

- `env.set(name, val)` — crea una binding en el frame actual.
- `env.update(name, val)` — busca desde el frame más interno hacia afuera y actualiza donde encuentre la variable; si no existe, la crea en el frame actual.
- `env.get(name)` — lectura desde el frame más interno hacia afuera.

Las variables locales a una función **no son visibles** en el scope del caller una vez que la función retorna.

```halo
x = 10               // global

shadow() {
    x = 99           // local — no modifica el global
    return x
}

print(shadow())      // 99
print(x)             // 10  ← global intacto
```

---

## 🧪 Testing

### Ejecutar todos los tests

```bash
cargo test
```

### Filtrar por suite

```bash
# Tests del intérprete (evaluator, environment, value)
cargo test interpreter

# Tests de la suite de integración completa
cargo test --test arithmetic_tests
cargo test --test control_flow_tests
cargo test --test function_tests
cargo test --test scope_and_variable_tests
cargo test --test string_and_types_tests
cargo test --test lexer_tests
cargo test --test parser_tests
cargo test --test interpreter_tests

# Un test específico por nombre
cargo test test_factorial_10
cargo test test_fibonacci_10
cargo test test_break_inside_function
```

### Herramientas de calidad

```bash
# Verificar sin compilar (rápido)
cargo check

# Linter
cargo clippy

# Formatear código
cargo fmt

# Coverage (requiere cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

---

## 🗺️ Roadmap

### ✅ v0.1.0 — Intérprete Básico

- [x] Lexer funcional
- [x] Parser recursivo descendente
- [x] AST completo
- [x] Intérprete con ejecución
- [x] Variables, aritmética, condicionales, bucles
- [x] Funciones con parámetros y recursión
- [x] Built-ins básicos (`print`, `len`, `str`, `int`, `float`)

### ✅ v0.2.0 — Robustez e Integración

- [x] Tipo `float` con promoción automática int↔float
- [x] Tipo `string` con concatenación, repetición y comparación
- [x] Built-ins `abs`, `type`
- [x] `break` y `continue` en bucles
- [x] `else if` en condicionales
- [x] Scope aislado por función (frame independiente)
- [x] Call-before-definition en el top level
- [x] Recursión mutua
- [x] Guard de profundidad de recursión
- [x] Guard de iteraciones de bucle
- [x] Checked arithmetic (overflow → error de runtime)
- [x] Soporte de flotantes en el parser (literals `1.5`, `-3.14`)
- [x] Expresiones top-level directas
- [x] Backend LLVM con compilación a binario nativo
- [x] CLI unificada con subcomandos `run`, `build`, `check`, `tokens`, `ast`, `llvm`
- [x] Toolchain doble: `clang` y `llc+cc`
- [x] Niveles de optimización O0–O3
- [x] Suite de +500 tests de integración

### 📋 v0.3.0 — Estructuras de Datos

- [ ] Arrays / Listas con indexación
- [ ] Diccionarios / Maps
- [ ] Métodos de string (`upper`, `lower`, `split`, `trim`…)
- [ ] Iteración con `for`

### 🔮 v0.4.0 — Tooling

- [ ] Mensajes de error con número de línea y columna
- [ ] Comentarios multi-línea `/* … */`
- [ ] Standard library extendida
- [ ] REPL interactivo (`halo repl`)

### 🚀 v1.0.0 — Ecosistema

- [ ] Backend a bytecode + VM
- [ ] Package manager
- [ ] Documentación en sitio web
- [ ] GitHub Actions CI/CD

---

## 🤝 Contribuir

¡Toda ayuda es bienvenida! Halo es el proyecto perfecto para:

- 🎓 Estudiantes aprendiendo compiladores
- 👨‍💻 Desarrolladores curiosos
- 🧙‍♂️ Expertos que quieran compartir conocimiento

### Guía rápida

```bash
# 1. Clonar
git clone https://github.com/Angelito91/halo.git
cd halo

# 2. Crear rama
git checkout -b feature/mi-caracteristica

# 3. Desarrollar, testear y formatear
cargo test
cargo clippy
cargo fmt

# 4. Commit y PR
git add .
git commit -m "feat: descripción clara"
git push origin feature/mi-caracteristica
```

**Buenas prácticas:**
- ✅ Una característica por PR
- ✅ Incluir tests para los cambios
- ✅ Documentar el comportamiento nuevo
- ✅ Pasar `cargo clippy` sin warnings

---

## 📚 Recursos

- [SYNTAX.md](SYNTAX.md) — Referencia completa de sintaxis
- [Crafting Interpreters](https://craftinginterpreters.com/) — Libro de referencia sobre compiladores
- [The Rust Book](https://doc.rust-lang.org/book/) — Documentación oficial de Rust

---

## 📜 Licencia

Halo está licenciado bajo **MPL 2.0** (Mozilla Public License 2.0).

- ✅ Uso comercial permitido
- ✅ Distribución permitida
- ✅ Modificación permitida
- ⚠️ Los cambios a archivos existentes deben ser públicos

Ver [LICENSE](LICENSE) para detalles completos.

---

## 👤 Autor

**Angel A. Portuondo H.**
- GitHub: [@Angelito91](https://github.com/Angelito91)
- Email: portuondoangel@gmail.com

---

<div align="center">

**[⭐ Star](https://github.com/Angelito91/halo)** • **[🐛 Reportar bug](https://github.com/Angelito91/halo/issues)** • **[💬 Discusiones](https://github.com/Angelito91/halo/discussions)**

**Hecho con 💙 para la educación y el aprendizaje**

**v0.2.0** | MPL 2.0 © 2024 Angel A. Portuondo H.

</div>