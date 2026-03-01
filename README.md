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
| ✅ Tipos básicos | ✅ Variables | ✅ Expresiones aritméticas |
| ✅ Condicionales `if/else` | ✅ Bucles `while` | ✅ Funciones con `fn` |
| 🚧 Sistema de tipos | 🚧 Parámetros | 🚧 Retorno de funciones |
| 📅 I/O básico | 📅 Backend alternativo | 📅 CLI tool |

</div>

## 📝 Sintaxis

La especificación inicial (ver [`init.txt`](init.txt)) define una sintaxis limpia y fácil de parsear:

### 📦 Tipos y variables
```
int           // Números enteros
float         // Números decimales  
bool          // Booleanos (true/false)

int x                 // Declaración
int contador = 10     // Inicialización
float precio = 99.99
bool activo = true
```

### 🔀 Condicionales
```rust
if edad >= 18 {
    // Código para adultos
} else {
    // Código para menores
}
```

### 🔁 Bucles
```rust
while intentos < 3 {
    intentos = intentos + 1
}
```

### 🧩 Funciones
```rust
fn saludar() {
    // Cuerpo de la función
}
```

### ➗ Operadores
| Aritméticos | Comparación |
|------------|-------------|
| `+` `-` `*` `/` | `<` `>` `=` `==` |

## 💡 Ejemplos

### Factorial
```rust
fn main() {
    int n = 5
    int resultado = 1
    
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
fn main() {
    int numero = 1
    
    while numero <= 10 {
        bool esPar = (numero % 2) == 0
        
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
│   ├── 🌳 parser/             # Análisis sintáctico y AST
│   ├── 🏗️  ast/               # Definiciones de nodos
│   ├── ✅ semantics/          # Chequeo de tipos
│   └── ⚙️  backend/           # Intérprete / generación de código
│
├── 📚 examples/               # Programas de ejemplo
│   ├── factorial.halo
│   ├── fibonacci.halo
│   └── ...
│
├── 🧪 tests/                   # Pruebas
│   ├── unit/
│   └── integration/
│
├── 📖 docs/                    # Documentación
├── 📄 init.txt                 # Semilla de la sintaxis
└── 🖼️  halo.png                # Banner del proyecto
```

## 🗺️ Roadmap

### 🚀 Prioridad alta (Q1 2024)
- [x] Definición inicial de sintaxis
- [ ] Lexer completo con tokens
- [ ] Parser recursivo descendente
- [ ] AST funcional
- [ ] Intérprete básico

### 📈 Prioridad media (Q2 2024)
- [ ] Sistema de tipos robusto
- [ ] Mensajes de error legibles
- [ ] Parámetros en funciones
- [ ] Valores de retorno
- [ ] Operadores lógicos (&&, ||, !)

### 🌟 Prioridad baja (Q3 2024)
- [ ] Backend a C/LLVM
- [ ] CLI (`halo run`, `halo build`)
- [ ] Standard library mínima
- [ ] CI/CD automatizado

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
