---
title: ¿Qué es Halo?
description: Un lenguaje interpretado, dinámico y educativo, escrito en Rust para aprender cómo funciona un lenguaje y escribir scripts simples.
---

Halo es un lenguaje de programación **dinámico, interpretado y minimalista**
creado con un objetivo educativo: que puedas seguir el viaje completo de un
programa, desde el texto fuente hasta su ejecución, sin perderte en capas de
magia.

Está escrito en Rust y cabe en una tarde de lectura. No pretende competir con
lenguajes de producción; pretende ser un laboratorio donde experimentar con
lexers, parsers, AST y evaluación, y a la vez un lenguaje cómodo para escribir
scripts cortos.

```halo
// Un programa Halo completo
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print(factorial(10))   // 3628800
```

## Por qué existe

Muchos proyectos educativos terminan en un intérprete de juguete o en un
conjunto de piezas que nunca llegan a ejecutarse. Halo conecta las dos orillas:

- Tiene un **lexer** y un **parser** que puedes inspeccionar por separado con
  `halo tokens` y `halo ast`.
- Tiene un **intérprete tree-walking** que ejecuta el AST directamente, sin
  binarios intermedios ni pasos ocultos.
- Tiene una **suite de tests** que fija el comportamiento del lenguaje y sirve
  de red de seguridad al modificarlo.
- Está pensado para **scripts y programas simples**: no necesitas un proyecto,
  un gestor de paquetes ni una fase de compilación.

El resultado es un proyecto lo bastante pequeño para leerlo entero y lo
bastante real para ejecutar programas útiles.

## Cómo se ve

```halo
// Variables dinámicas: sin tipos ni palabras clave
nombre = "Halo"
version = 0.3
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
| Strings | Concatenación, repetición, comparación, escapes y funciones |
| Control de flujo | `if / else if / else`, `while`, `for … in`, `break`, `continue` |
| Funciones | Parámetros, retorno, recursión directa y mutua, call-before-definition |
| Colecciones | Arrays con literales, indexación, asignación por índice, slicing y funciones |
| Intérprete | Ejecución directa del AST con límites de recursión e iteraciones |
| CLI | `run`, `check`, `tokens`, `ast`, `help` y `version` |

Consulta la [guía del lenguaje](../../guia/estructura/) para los detalles y la
[referencia de built-ins](../../referencia/builtins/) para la lista completa.

## Qué no es (todavía)

Halo está en desarrollo activo. Estas son las limitaciones que conviene
conocer antes de escribir tu primer programa:

- **No genera binarios nativos.** Es un intérprete: ejecutas el archivo con
  `halo run` y nada más.
- **No hay diccionarios ni maps.** Están planificados.
- **No hay closures ni funciones de primera clase.**
- **No hay REPL ni gestor de paquetes.** La CLI es pequeña y directa.
- **No hay comentarios multilínea.** Solo `//`.
- **No hay métodos con punto.** Todas las operaciones son funciones:
  `upper(texto)`, `push(arr, x)`, etc.
- **La única mutación de arrays es la asignación por índice**
  (`arr[i] = value`). `push`, `pop`, `append`, `reverse` y `sort` devuelven
  arrays nuevos; `first` y `last` solo leen.

Puedes seguir el estado en el [roadmap](../../proyecto/roadmap/).

## Para quién es

- **Estudiantes de lenguajes e intérpretes** que quieren un código base
  legible y ejecutable, no un esqueleto incompleto.
- **Desarrolladores curiosos** que quieren entender qué pasa entre un archivo
  `.halo` y su salida.
- **Docentes** que buscan un ejemplo real para explicar lexer, parser, AST,
  scoping y evaluación.
- **Gente que quiere escribir scripts cortos** y prefiere un lenguaje pequeño,
  sin configuración y sin dependencias.

## Siguiente paso

- [Instala Halo](../instalacion/) en tu máquina.
- [Escribe tu primer programa](../primer-programa/).
- [Conoce la CLI](../cli/).
- [Mira cómo está construido](../../proyecto/arquitectura/).

> Halo se distribuye bajo la Mozilla Public License 2.0. El código está en
> [GitHub](https://github.com/Angelito91/halo).
