---
title: ¿Qué es Halo?
description: Un lenguaje de programación pequeño, dinámico y educativo, escrito en Rust para aprender compiladores de principio a fin.
---

Halo es un lenguaje de programación **dinámico y minimalista** creado con un
objetivo educativo: que puedas seguir el viaje completo de un programa, desde
el texto fuente hasta el binario nativo, sin perderte en capas de magia.

Está escrito en Rust y cabe en una tarde de lectura. No pretende competir con
lenguajes de producción; pretende ser un laboratorio donde experimentar con
lexers, parsers, intérpretes y generación de código LLVM.

```halo
// Un programa Halo completo
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print(factorial(10))   // 3628800
```

## Por qué existe

Muchos cursos de compiladores terminan en un intérprete de juguete o en un
generador de código incompleto. Halo conecta las dos orillas:

- Tiene un **intérprete tree-walking** que ejecuta el AST directamente, ideal
  para entender la semántica sin esperar a compilar.
- Tiene un **backend LLVM** que produce binarios nativos reales, con niveles de
  optimización `-O0` a `-O3`.
- La suite de tests de paridad vigila las diferencias entre ambos caminos y
  las registra mientras se corrigen. Consulta
  [diferencias entre backends](../../referencia/diferencias/).

El resultado es un proyecto lo bastante pequeño para leerlo entero y lo
bastante real para ver código máquina al final.

## Cómo se ve

```halo
// Variables dinámicas: sin tipos ni palabras clave
nombre = "Halo"
version = 0.2
activo = true

// Control de flujo
if activo {
    print(nombre + " v" + str(version))
}

// Bucles
for i in range(3) {
    print("iteración " + str(i))
}

// Funciones sin `function`, sin `main`, sin `return` obligatorio
saludar(quien) {
    return "Hola, " + quien + "!"
}

print(saludar("mundo"))
```

## Características actuales

| Área | Qué hay |
|---|---|
| Tipos | `number` (entero de 64 bits), `float`, `bool`, `string`, `null` y arrays |
| Aritmética | `+`, `-`, `*`, `/`, `%` con promoción automática y overflow comprobado |
| Strings | Concatenación, repetición, comparación, escapes y métodos |
| Control de flujo | `if / else if / else`, `while`, `for … in`, `break`, `continue` |
| Funciones | Parámetros, retorno, recursión directa y mutua, call-before-definition |
| Colecciones | Arrays con literales, indexación, slicing y métodos |
| Intérprete | Ejecución directa del AST con límites de recursión e iteraciones |
| Compilador | AST → LLVM IR → binario nativo con `clang` o `llc + cc` |
| CLI | `run`, `build`, `check`, `tokens`, `ast`, `llvm` |

Consulta la [guía del lenguaje](../../guia/estructura/) para los detalles y la
[referencia de built-ins](../../referencia/builtins/) para la lista completa.

## Qué no es (todavía)

Halo está en desarrollo activo. Estas son las limitaciones que conviene
conocer antes de escribir tu primer programa:

- **El modo compilado no soporta arrays.** El intérprete sí; los binarios
  nativos todavía no. `for` compilado solo acepta `range()`.
- **No hay diccionarios ni maps.** Están planificados.
- **Los errores de runtime no siempre incluyen línea y columna.** Es una de las
  prioridades del próximo hito.
- **No hay REPL ni gestor de paquetes.** La CLI es pequeña y directa.
- **No hay comentarios multilínea.** Solo `//`.

Puedes seguir el estado en el [roadmap](../../proyecto/roadmap/).

## Para quién es

- **Estudiantes de compiladores** que quieren un código base legible y
  ejecutable, no un esqueleto incompleto.
- **Desarrolladores curiosos** que quieren entender qué pasa entre un archivo
  `.halo` y un ejecutable.
- **Docentes** que buscan un ejemplo real para explicar lexer, parser, AST,
  scoping, evaluación y codegen.
- **Gente que disfruta** de los lenguajes pequeños y de las implementaciones
  honestas.

## Siguiente paso

- [Instala Halo](../instalacion/) en tu máquina.
- [Escribe tu primer programa](../primer-programa/).
- [Conoce la CLI](../cli/).
- [Mira cómo está construido](../../proyecto/arquitectura/).

> Halo se distribuye bajo la Mozilla Public License 2.0. El código está en
> [GitHub](https://github.com/Angelito91/halo).
