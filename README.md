<div align="center">
    <img src="halo.png" alt="Halo Compiler Banner">
  
  # 🌟 Halo — Compilador simple
  
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
    <a href="#-motivación">Motivación</a> •
    <a href="#-características">Características</a> •
    <a href="#-sintaxis">Sintaxis</a> •
    <a href="#-ejemplos">Ejemplos</a> •
    <a href="#-estructura">Estructura</a> •
    <a href="#-roadmap">Roadmap</a> •
    <a href="#-contribuir">Contribuir</a>
  </p>
</div>

---

## 🎯 Motivación

> *"Lo que no se crea, no se entiende"*

Halo nace como un proyecto de aprendizaje práctico para desmitificar el proceso de construcción de compiladores e intérpretes. Lejos de la complejidad de lenguajes industriales, Halo ofrece un playground minimalista donde:

- 🔍 **Exploras** las fases clásicas: lexing, parsing, AST, chequeo de tipos y backend
- 🧪 **Experimentas** con nuevas características de lenguaje sin miedo
- 📚 **Aprendes** haciendo, con código simple y extensible
- 🎮 **Juegas** con diferentes estrategias de implementación (intérprete, bytecode, transpilación)

## ⚡ Características principales

<div align="center">
  
| | | |
|---|---|---|
| ✅ Variables dinámicas | ✅ Expresiones aritméticas |
| ✅ Condicionales `if/else` | ✅ Bucles `while` | ✅ Funciones simples |
| 🚧 Sistema de tipos | 🚧 Parámetros | 🚧 Retorno de funciones |
| 📅 I/O básico | 📅 Backend alternativo | 📅 CLI tool |

</div>

## 📝 Sintaxis

La especificación inicial (ver [`init.txt`](init.txt)) define una sintaxis limpia y fácil de parsear:

### 📦 Variables (sin tipos explícitos)
```
x = 5                // Las variables se crean con =
contador = 10        // No necesitas declarar el tipo
precio = 99.99       // El tipo se infiere del valor
activo = true        // Soporta números, floats y bools
```

### 🔀 Condicionales (con {} obligatorios)
```
if edad >= 18 {
    // Código para adultos
} else {
    // Código para menores
}
```

### 🔁 Bucles (while con {})
```
while intentos < 3 {
    intentos = intentos + 1
}
```

### 🧩 Funciones (sin keyword fn)
```
saludar() {
    // Simplemente: nombre() { cuerpo }
    // Sin tipos de parámetros
}

mi_funcion(x, y) {
    return x + y
}
```

### ➗ Operadores
| Aritméticos | Comparación | Control |
|------------|-------------|---------|
| `+` `-` `*` `/` | `<` `>` `==` `!=` `<=` `>=` | `if` `else` `while` `return` |

## 💡 Ejemplos

### Factorial
```rust
main() {
    n = 5
    resultado = 1
    
    while n > 1 {
        resultado = resultado * n
        n = n - 1
    }
    
    // Aquí iría print cuando lo implementemos
    // print(resultado) -> 120
}
```

### Números pares
```rust
main() {
    numero = 1
    
    while numero <= 10 {
        esPar = (numero % 2) == 0
        
        if esPar == true {
            // print(numero) -> 2, 4, 6, 8, 10
        }
        
        numero = numero + 1
    }
}
```

## 📁 Estructura del proyecto

```
halo/
├── 📦 src/                    # Código fuente
│   ├── 🎯 lexer/              # Análisis léxico
│   │   ├── lexer.rs
│   │   ├── token.rs
│   │   └── mod.rs
│   └── 🌳 parser/             # Análisis sintáctico y AST
│       ├── ast.rs             # Definiciones de nodos
│       ├── parser.rs          # Parser recursivo descendente
│       ├── visitor.rs         # Visitor pattern para AST
│       └── mod.rs
│   └── ⚙️  backend/           # Intérprete / generación de código
│
├── 📚 examples/               # Programas de ejemplo
│   ├── factorial.halo         # Ejemplo: Factorial recursivo
│   ├── fibonacci.halo         # Ejemplo: Fibonacci
│   └── even_numbers.halo      # Ejemplo: Números pares
│
├── 🧪 tests/                  # Pruebas
│   ├── unit/
│   └── integration/
│
├── 📖 docs/                   # Documentación
├── 📄 init.txt                # Especificación inicial de sintaxis
├── 🖼️  halo.png               # Banner del proyecto
├── Cargo.toml                 # Manifest de Rust
└── README.md                  # Este archivo
```

## 🗺️ Roadmap

### 🚀 Prioridad alta (Q1 2024)
- [x] Definición inicial de sintaxis (sin tipos, sin fn)
- [x] Lexer completo con tokens
- [x] Parser recursivo descendente
- [x] AST funcional con Block y Visitor
- [ ] Intérprete básico

### 📈 Prioridad media (Q2 2024)
- [ ] Inferencia de tipos dinámicos
- [ ] Mensajes de error legibles con posiciones
- [x] Parámetros en funciones (sintaxis lista)
- [x] Valores de retorno (sintaxis lista)
- [ ] Operadores lógicos (&&, ||, !)
- [ ] Semantic analysis (type checking)

### 🌟 Prioridad baja (Q3 2024)
- [ ] Backend a bytecode VM
- [ ] Backend a C/LLVM
- [ ] CLI (`halo run`, `halo build`, `halo check`)
- [ ] Standard library mínima (print, len, etc.)
- [ ] CI/CD automatizado con GitHub Actions
- [ ] Documentación detallada y tutoriales

## 🤝 Contribuir

¡Toda ayuda es bienvenida! Halo es el proyecto perfecto para:

- 🎓 Estudiantes aprendiendo compiladores
- 👨‍💻 Desarrolladores curiosos
- 🧙‍♂️ Expertos que quieran compartir conocimiento

### Formas de contribuir

| Área | Ideas |
|------|-------|
| 📝 **Documentación** | Mejorar ejemplos, escribir tutoriales |
| 🎯 **Lexer/Parser** | Implementar desde cero o con herramientas |
| ✅ **Semántica** | Añadir chequeo de tipos, optimizaciones |
| ⚡ **Backend** | Crear intérprete, bytecode VM, transpilador |
| 🧪 **Testing** | Añadir casos de prueba, encontrar bugs |
| 💡 **Ideas** | Proponer nuevas características |

### Buenas prácticas para PRs

✅ Una característica por PR  
✅ Incluir pruebas  
✅ Documentar cambios  
✅ Mantener el estilo de código  

## 📜 Licencia

MLP 2.0 © Angel A. Portuondo H.

---

<div align="center">
  
**[⭐ Star](https://github.com/Angelito91/halo)** • **[🐛 Reportar bug](https://github.com/Angelito91/halo/issues)** • **[📬 Contacto](mailto:portuondoangel@gmail.com)**
  
**Hecho con 💙 para la educación y el aprendizaje**

</div>
