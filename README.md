# Halo — Compilador simple

Halo es un proyecto educativo: un compilador/interprete minimalista diseñado para aprender e experimentar con las fases típicas de un compilador (lexing, parsing, AST, chequeo de tipos y backend/interprete). El lenguaje objetivo es intencionalmente pequeño y fácil de analizar; su especificación inicial está en `init.txt`.

Este README explica la sintaxis básica, muestra ejemplos, describe el estado actual y sugiere una hoja de ruta para seguir desarrollando el proyecto.

---

## Tabla de contenido

- [Motivación](#motivación)
- [Características principales](#características-principales)
- [Sintaxis (resumen)](#sintaxis-resumen)
- [Ejemplos](#ejemplos)
- [Estructura del repositorio](#estructura-del-repositorio)
- [Cómo contribuir](#cómo-contribuir)
- [Roadmap / Próximos pasos](#roadmap--próximos-pasos)
- [Licencia](#licencia)

---

## Motivación

El objetivo de `Halo` es servir como un proyecto de aprendizaje para comprender cómo construir un compilador/interprete desde cero. Está pensado para:

- Practicar técnicas de análisis léxico y sintáctico.
- Diseñar un AST simple y realizar chequeo de tipos.
- Explorar estrategias de backend (interprete directo, bytecode, generación a C/LLVM).
- Proveer un playground para experimentar con nuevas características de lenguaje.

---

## Características principales

- Tipos básicos: `int`, `float`, `bool`.
- Declaración e inicialización de variables.
- Expresiones aritméticas y operadores básicos.
- Condicionales `if/else`.
- Bucles `while`.
- Definición de funciones con `fn`.
- Sintaxis simple pensada para facilitar el parsing.

---

## Sintaxis (resumen)

La especificación inicial se encuentra en `init.txt`. Aquí está el contenido original de referencia:

```halo/init.txt#L1-27
Sintaxis

1 - Tipos
int -> Enteros
float -> Flotantes
bool -> Booleanos

1 - Variable
Declaracion -> <tipo> <name>
Inicializacion -> <tipo> <name> = <valor>

2 - Condicionales

if <condicion> {..} else {}

3 - Bucles

while <condicion> {}

4 - Funciones
fn <name>(){

}

5 -Operadores

+ - * / < > = ==
```

Notas sobre la sintaxis:
- Las declaraciones se escriben como `int x` o `int x = 5`.
- No hay aún una sintaxis estándar para comentarios o entrada/salida; se puede definir según convenga.
- Las funciones se declaran con `fn <name>() { ... }`. El soporte de parámetros y retorno puede implementarse en siguientes iteraciones.

---

## Ejemplos

Ejemplo: cálculo factorial (pseudocódigo en la sintaxis del lenguaje):

```/dev/null/example.halo#L1-20
fn main() {
int n = 5
int result = 1

while n > 1 {
result = result * n
n = n - 1
}

# Si existiese, aquí iría la llamada a print o return
}
```

Este ejemplo muestra la estructura general: declaración de variables, bucle `while` y operaciones aritméticas. A medida que el proyecto evolucione se añadirán utilidades de I/O y retornos.

---

## Estructura recomendada del repositorio

Sugerencia de organización para mantener el proyecto claro y extensible:

- `halo/` — raíz del proyecto
  - `src/` — código fuente del compilador
    - `lexer/` — análisis léxico
    - `parser/` — parser y construcción del AST
    - `ast/` — definiciones de nodos del AST
    - `semantics/` — chequeo de tipos y validaciones
    - `backend/` — generación de código / intérprete
  - `examples/` — ejemplos en el lenguaje Halo
  - `tests/` — pruebas unitarias y de integración
  - `docs/` — documentación adicional y especificación extendida
  - `init.txt` — bosquejo inicial de la sintaxis (referencia)

Si prefieres otra estructura (por ejemplo monorepo con múltiples backends), podemos adaptarla.

---

## Cómo contribuir

Algunas formas útiles de contribuir:

- Añadir ejemplos en `examples/` que prueben construcciones del lenguaje.
- Implementar el lexer y el parser (con tests).
- Añadir chequeo de tipos y errores con mensajes claros.
- Crear un intérprete sencillo que ejecute el AST.
- Abrir issues para nuevas características o bugs.
- Enviar pull requests con cambios pequeños y pruebas asociadas.

Buenas prácticas para PRs:
- Una tarea por PR.
- Incluir pruebas que demuestren la corrección.
- Documentar cambios en `docs/` o en `init.txt` según sea necesario.

---

## Roadmap / Próximos pasos (sugeridos)

Prioridad alta:
- [ ] Lexer que genere tokens para palabras clave, identificadores, números y operadores.
- [ ] Parser que produzca un AST para declaraciones, expresiones, condicionales, bucles y funciones.
- [ ] Interprete básico que ejecute programas sencillos.

Prioridad media:
- [ ] Sistema de tipos y mensajes de error legibles.
- [ ] Manejo de parámetros y valores de retorno en funciones.
- [ ] Soporte de expresiones booleanas y operadores lógicos.

Prioridad baja:
- [ ] Backend que genere código C/LLVM o bytecode.
- [ ] Herramienta CLI `halo build/run <archivo>`.
- [ ] Integración con CI y tests automáticos.

---

## Notas sobre implementación

- Lenguajes recomendados para comenzar: Rust (por seguridad y performance), Go o Python (rápido para prototipado).
- Para un parser sencillo se puede empezar con una implementación recursiva descendente.
- Mantener el AST simple y testable facilita añadir passes posteriores (optimización, generación de código).
