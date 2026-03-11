<div align="center">
    <img src="halo.png" alt="Halo Compiler Banner">
  
  # 🌟 Halo — Compilador Simple
  
  <p align="center">
    <strong>✨ Un viaje educativo al corazón de los compiladores ✨</strong>
  </p>
  
  <p align="center">
    <img src="https://img.shields.io/badge/estado-en_desarrollo-blue?style=flat-square" alt="Estado: en desarrollo">
    <img src="https://img.shields.io/badge/versión-0.1.0--alpha-orange?style=flat-square" alt="Versión: 0.1.0-alpha">
    <img src="https://img.shields.io/badge/licencia-MPL_2.0-green?style=flat-square" alt="Licencia: MPL">
    <img src="https://img.shields.io/badge/contribuciones-bienvenidas-brightgreen?style=flat-square" alt="Contribuciones: bienvenidas">
  </p>

  <p align="center">
    <a href="#-características">Características</a> •
    <a href="#-instalación">Instalación</a> •
    <a href="#-uso">Uso</a> •
    <a href="#-sintaxis">Sintaxis</a> •
    <a href="#-ejemplos">Ejemplos</a> •
    <a href="#-arquitectura">Arquitectura</a> •
    <a href="#-roadmap">Roadmap</a> •
    <a href="#-contribuir">Contribuir</a>
  </p>
</div>

---

## 🎯 Motivación

> *"Lo que no se crea, no se entiende"*

Halo nace como un proyecto de aprendizaje práctico para desmitificar el proceso de construcción de compiladores e intérpretes. Lejos de la complejidad de lenguajes industriales, Halo ofrece un playground minimalista donde:

- 🔍 **Exploras** las fases clásicas: lexing, parsing, AST, chequeo de tipos y ejecución
- 🧪 **Experimentas** con nuevas características de lenguaje sin miedo
- 📚 **Aprendes** haciendo, con código simple y extensible
- 🎮 **Juegas** con diferentes estrategias de implementación (intérprete, bytecode, transpilación)

## ⚡ Características principales

### ✅ Completado

| Característica | Estado | Detalles |
|---|---|---|
| **Variables dinámicas** | ✅ | Sin tipos explícitos |
| **Expresiones aritméticas** | ✅ | `+`, `-`, `*`, `/`, `%` |
| **Comparaciones** | ✅ | `<`, `>`, `==`, `!=`, `<=`, `>=` |
| **Operadores lógicos** | ✅ | `&&`, `\|\|`, `!` |
| **Condicionales** | ✅ | `if/else` con bloques `{}` |
| **Bucles** | ✅ | `while` |
| **Funciones** | ✅ | Definición y llamada con parámetros |
| **Recursión** | ✅ | Totalmente soportada |
| **Intérprete** | ✅ | Ejecución de AST completa |
| **Lexer** | ✅ | Tokenización O(1) |
| **Parser** | ✅ | Recursivo descendente |
| **AST** | ✅ | Árbol sintáctico funcional |

### 🚧 En Progreso

| Característica | Prioridad | ETA |
|---|---|---|
| Sistema de tipos | Media | Q2 2024 |
| Inferencia de tipos | Media | Q2 2024 |
| Mensajes de error mejorados | Alta | Q1 2024 |
| Standard library | Media | Q2 2024 |

### 📅 Planificado

| Característica | Prioridad |
|---|---|
| Backend a bytecode VM | Baja |
| Backend a C/LLVM | Baja |
| CLI tool | Media |
| Documentación detallada | Media |
| GitHub Actions CI/CD | Baja |

## 📝 Sintaxis

Halo ofrece una sintaxis limpia y minimalista, inspirada en Python pero con bloques obligatorios:

### 📦 Variables (sin tipos explícitos)

```halo
x = 5                    // Las variables se crean con =
contador = 10            // No necesitas declarar el tipo
precio = 99.99           // El tipo se infiere del valor
activo = true            // Soporta números, floats y bools
```

### ➗ Operadores Aritméticos

```halo
a = 10 + 5               // Suma: 15
b = 10 - 5               // Resta: 5
c = 10 * 5               // Multiplicación: 50
d = 10 / 5               // División: 2
e = 10 % 3               // Módulo: 1
```

### 🔀 Comparaciones

```halo
a == b                   // Igual
a != b                   // No igual
a < b                    // Menor que
a > b                    // Mayor que
a <= b                   // Menor o igual
a >= b                   // Mayor o igual
```

### ✅ Operadores Lógicos

```halo
a && b                   // AND lógico
a || b                   // OR lógico
!a                       // NOT lógico
```

### 🔀 Condicionales (con {} obligatorios)

```halo
if edad >= 18 {
    // Código para adultos
} else {
    // Código para menores
}
```

### 🔁 Bucles (while con {})

```halo
while intentos < 3 {
    intentos = intentos + 1
}
```

### 🧩 Funciones (sin keyword fn)

```halo
saludar() {
    print("Hola!")
}

sumar(a, b) {
    return a + b
}

factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
```

## 💾 Funciones Built-in

Halo proporciona las siguientes funciones integradas:

| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `print(x)` | Imprime un valor | `print(42)` |
| `len(x)` | Longitud de string/número | `len("hola")` |
| `str(x)` | Convierte a string | `str(123)` |
| `int(x)` | Convierte a entero | `int(3.14)` → `3` |
| `float(x)` | Convierte a float | `float(42)` → `42.0` |
| `abs(x)` | Valor absoluto | `abs(-5)` → `5` |
| `type(x)` | Tipo de valor | `type(42)` → `"number"` |
| `bool(x)` | Convierte a booleano | `bool(0)` → `false` |

## 📥 Instalación

### Requisitos

- Rust 1.56+ ([Instalar Rust](https://www.rust-lang.org/tools/install))
- Cargo (incluido con Rust)

### Compilar desde fuente

```bash
# Clonar el repositorio
git clone https://github.com/Angelito91/halo.git
cd halo

# Compilar en modo debug
cargo build

# Compilar en modo release (optimizado)
cargo build --release
```

## 🚀 Uso

### Ejecutar el compilador

```bash
# Ejecutar con el ejemplo por defecto
cargo run

# Con optimizaciones
cargo run --release
```

### Ejecutar un archivo .halo

Para ejecutar un programa Halo, primero necesitas compilar el proyecto. La CLI mejorada está en progreso, por ahora puedes:

```bash
# Ejecutar el intérprete con un archivo (próximamente)
# cargo run examples/factorial.halo
```

### Ejecutar las pruebas

```bash
# Todas las pruebas
cargo test

# Pruebas verbosas
cargo test -- --nocapture

# Prueba específica
cargo test test_factorial

# Pruebas de integración solo
cargo test --test interpreter_tests
```

## 💡 Ejemplos

### Factorial (Recursión)

```halo
factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

result = factorial(5)
print(result)              // Salida: 120
```

### Fibonacci (Recursión)

```halo
fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

i = 0
while i < 10 {
    print(fibonacci(i))
    i = i + 1
}
```

### Números Pares

```halo
numero = 1
while numero <= 10 {
    esPar = (numero % 2) == 0
    
    if esPar == true {
        print(numero)        // Salida: 2, 4, 6, 8, 10
    }
    
    numero = numero + 1
}
```

### Operadores Lógicos

```halo
a = true
b = false

if a && !b {
    print("a es true y b es false")
}

if a || b {
    print("Al menos uno es true")
}
```

### Type Conversions

```halo
num = 42
float_num = float(num)
str_num = str(num)
back_to_int = int(float_num)

print(float_num)           // 42.0
print(str_num)             // "42"
print(back_to_int)         // 42
```

## 📁 Estructura del Proyecto

```
halo/
├── 📦 src/                          # Código fuente
│   ├── 🎯 lexer/                    # Análisis léxico
│   │   ├── lexer.rs                 # Tokenizador
│   │   ├── token.rs                 # Definiciones de tokens
│   │   └── mod.rs                   # Módulo
│   ├── 🌳 parser/                   # Análisis sintáctico
│   │   ├── ast.rs                   # Definiciones AST
│   │   ├── parser.rs                # Parser recursivo descendente
│   │   ├── visitor.rs               # Patrón Visitor para AST
│   │   └── mod.rs                   # Módulo
│   ├── ⚙️  interpreter/             # Intérprete
│   │   ├── value.rs                 # Valores en runtime
│   │   ├── environment.rs           # Gestión de variables
│   │   ├── evaluator.rs             # Evaluador de AST
│   │   └── mod.rs                   # Módulo
│   ├── main.rs                      # Punto de entrada
│   └── lib.rs                       # Raíz de biblioteca
│
├── 📚 examples/                     # Programas de ejemplo
│   ├── factorial.halo
│   ├── fibonacci.halo
│   └── even_numbers.halo
│
├── 🧪 tests/                        # Pruebas integrales
│   └── interpreter_tests.rs
│
├── 📖 Documentación
│   ├── README.md                    # Este archivo
│   ├── SYNTAX.md                    # Especificación de sintaxis
│   └── COMMITS_SIMPLE.txt           # Guía de commits
│
├── Cargo.toml                       # Manifest de Rust
├── Cargo.lock                       # Lock de dependencias
├── LICENSE                          # Licencia MPL 2.0
└── halo.png                         # Banner del proyecto
```

## 🏗️ Arquitectura

### Fases del Compilador

```
Código Fuente
    ↓
┌─────────────────────────────────────┐
│  LEXER (Análisis Léxico)            │
│  • Escanea caracteres               │
│  • Genera tokens                    │
│  • Tiempo: O(1) por carácter        │
└─────────────────────────────────────┘
    ↓
Token Stream
    ↓
┌─────────────────────────────────────┐
│  PARSER (Análisis Sintáctico)       │
│  • Lee tokens                       │
│  • Construye AST                    │
│  • Recursivo descendente            │
└─────────────────────────────────────┘
    ↓
Abstract Syntax Tree (AST)
    ↓
┌─────────────────────────────────────┐
│  INTERPRETER (Evaluador)            │
│  • Recorre el AST                   │
│  • Ejecuta instrucciones            │
│  • Maneja variables (Environment)   │
│  • Maneja funciones y scopes        │
└─────────────────────────────────────┘
    ↓
Output / Resultado
```

### Componentes Principales

#### 🎯 Lexer (`src/lexer/`)
- **Entrada:** Código fuente (String)
- **Salida:** Stream de tokens
- **Algoritmo:** Escaneo lineal O(n)
- **Características:** Manejo de palabras clave, operadores, números, strings

#### 🌳 Parser (`src/parser/`)
- **Entrada:** Stream de tokens
- **Salida:** AST (Abstract Syntax Tree)
- **Algoritmo:** Recursive Descent Parser
- **Características:** Construcción de árbol sintáctico, error recovery

#### ⚙️ Interpreter (`src/interpreter/`)
- **Entrada:** AST
- **Salida:** Valores ejecutados
- **Componentes:**
  - **Value:** Enum con tipos en runtime (Number, Float, Bool, String, Null)
  - **Environment:** Gestión de variables con scoping
  - **Evaluator:** Ejecución del AST

### Tipos de Datos

```rust
enum Value {
    Number(i64),      // Enteros
    Float(f64),       // Números decimales
    Bool(bool),       // Booleanos
    String(String),   // Strings (planeado)
    Null,             // Valor nulo
}
```

### Scoping

Halo soporta múltiples niveles de scope:

```halo
x = 10                  // Scope global

test(y) {
    x = 20              // Shadowing local
    z = 30              // Variable local
    return x + y + z
}

print(x)                // 10 (scope global sin cambios)
print(test(5))          // 55
```

## 🗺️ Roadmap

### 🚀 Fase 1: Intérprete Básico (v0.1.0) ✅ COMPLETADA

- [x] Lexer funcional
- [x] Parser recursivo descendente
- [x] AST completo
- [x] Intérprete con ejecución
- [x] Variables y asignación
- [x] Operadores aritméticos
- [x] Control de flujo (if/else, while)
- [x] Funciones con parámetros
- [x] Recursión
- [x] Funciones built-in básicas

### 📈 Fase 2: Sistema de Tipos (v0.2.0)

- [ ] Inferencia de tipos
- [ ] Type checking en tiempo de compilación
- [ ] Mensajes de error con posiciones
- [ ] Mejor diagnosticación de errores
- [ ] Operadores lógicos (!= &&, ||)

### 🌟 Fase 3: Features Avanzadas (v0.3.0)

- [ ] Arrays/Listas
- [ ] Diccionarios/Maps
- [ ] Métodos de strings
- [ ] Breakpoints y debugging
- [ ] Comments multi-línea

### 🔥 Fase 4: Backend Alternativo (v0.4.0)

- [ ] Compilador a bytecode
- [ ] Virtual machine
- [ ] Optimizaciones
- [ ] Compilador a C

### 🎉 Fase 5: Ecosistema (v1.0.0)

- [ ] CLI tool completo
- [ ] Package manager
- [ ] Standard library
- [ ] Documentation site

## 🧪 Testing

Halo incluye un suite de pruebas completo:

### Pruebas Unitarias

```bash
# En los módulos individuales
cargo test lexer::
cargo test parser::
cargo test interpreter::
```

### Pruebas de Integración

```bash
# Suite completo de integración
cargo test --test interpreter_tests

# Con output
cargo test --test interpreter_tests -- --nocapture
```

### Coverage

```bash
# Requiere tarpaulin
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## 📊 Métricas

### Tamaño de Código

```
src/lexer/         ~400 LOC
src/parser/        ~600 LOC
src/interpreter/   ~700 LOC
Total:            ~1700 LOC
```

### Rendimiento

- **Lexing:** ~1M tokens/segundo
- **Parsing:** ~100K programas/segundo
- **Recursión:** Hasta 10,000 niveles

## 🤝 Contribuir

¡Toda ayuda es bienvenida! Halo es el proyecto perfecto para:

- 🎓 Estudiantes aprendiendo compiladores
- 👨‍💻 Desarrolladores curiosos
- 🧙‍♂️ Expertos que quieran compartir conocimiento

### Guía de Contribución

1. **Clonar el repositorio**
   ```bash
   git clone https://github.com/Angelito91/halo.git
   cd halo
   ```

2. **Crear una rama**
   ```bash
   git checkout -b feature/nueva-caracteristica
   ```

3. **Hacer cambios y committear**
   ```bash
   git add .
   git commit -m "feat: descripción clara"
   ```

4. **Push y PR**
   ```bash
   git push origin feature/nueva-caracteristica
   ```

### Buenas Prácticas

✅ Una característica por PR  
✅ Incluir pruebas  
✅ Documentar cambios  
✅ Mantener el estilo de código  
✅ Usar commits atómicos y descriptivos  

### Estándares de Código

- **Lenguaje:** Rust
- **Formatter:** `rustfmt`
- **Linter:** `clippy`
- **Tests:** `cargo test`

```bash
# Antes de hacer PR
cargo fmt
cargo clippy
cargo test
```

## 📚 Recursos

### Documentación Interna

- [SYNTAX.md](SYNTAX.md) - Especificación completa de sintaxis
- [COMMITS_SIMPLE.txt](COMMITS_SIMPLE.txt) - Guía de commits (inglés)
- Código fuente con comentarios extensos

### Recursos Externos

- [Crafting Interpreters](https://craftinginterpreters.com/) - Libro excelente sobre compiladores
- [The Rust Book](https://doc.rust-lang.org/book/) - Documentación oficial de Rust
- [Compiler Design](https://www.geeksforgeeks.org/compiler-design/) - Conceptos fundamentales

## 📜 Licencia

Halo está licenciado bajo **MPL 2.0** (Mozilla Public License 2.0)

Esto significa:
- ✅ Uso comercial permitido
- ✅ Distribución permitida
- ✅ Modificación permitida
- ⚠️ Cambios deben ser públicos
- ⚠️ Debe incluir notice de copyright

Ver [LICENSE](LICENSE) para detalles completos.

## 👤 Autor

**Angel A. Portuondo H.**
- GitHub: [@Angelito91](https://github.com/Angelito91)
- Email: portuondoangel@gmail.com

## 🙏 Agradecimientos

Agradecemos a:
- La comunidad de Rust por las herramientas fantásticas
- Todos los que contribuyen con ideas, bugs, y código
- Los educadores que inspiraron este proyecto

## 📞 Soporte

### Reportar Bugs

Abre un issue con:
- Descripción clara del problema
- Pasos para reproducirlo
- Código de ejemplo mínimo
- Versión de Rust y SO

### Solicitar Características

Abre un issue con:
- Descripción de la característica
- Caso de uso
- Ejemplos de código

### Preguntas

Abre un issue etiquetado con `question` o contáctame directamente.

---

<div align="center">

**[⭐ Star](https://github.com/Angelito91/halo)** • **[🐛 Reportar bug](https://github.com/Angelito91/halo/issues)** • **[💬 Discusiones](https://github.com/Angelito91/halo/discussions)** • **[📧 Contacto](mailto:portuondoangel@gmail.com)**

**Hecho con 💙 para la educación y el aprendizaje**

**v0.1.0-alpha** | MPL 2.0 © 2024 Angel A. Portuondo H.

</div>