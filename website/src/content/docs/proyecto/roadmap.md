---
title: Roadmap
description: Estado actual del lenguaje y próximos hitos, desde el intérprete hasta el ecosistema.
---

Halo está en desarrollo activo. Esta página describe **qué existe hoy** y qué
viene después, sin fechas prometidas.

:::note
La versión declarada en `Cargo.toml` es **0.2.0**. El repositorio ya contiene
trabajo de la línea 0.3.0 (arrays y métodos de string), pero todavía no está
liberado como una versión estable. Las listas de abajo reflejan el estado del
código, no solo del último release.
:::

## Leyenda

| Símbolo | Significado |
|---|---|
| ✅ | Implementado y probado |
| 🚧 | En progreso, parcial o solo en un backend |
| 📅 | Planificado, sin implementación todavía |

## v0.1.0 — Intérprete básico ✅

El primer hito fue tener un lenguaje ejecutable de principio a fin.

- ✅ Lexer funcional con posiciones
- ✅ Parser descendente recursivo
- ✅ AST completo
- ✅ Intérprete tree-walking
- ✅ Variables, aritmética, condicionales y bucles
- ✅ Funciones con parámetros y recursión
- ✅ Built-ins básicos: `print`, `len`, `str`, `int`, `float`

## v0.2.0 — Robustez e integración ✅

El segundo hito convirtió el intérprete en algo usable y añadió el backend
nativo.

- ✅ Tipo `float` con promoción automática
- ✅ Tipo `string` con concatenación, repetición y comparación
- ✅ `abs` y `type`
- ✅ `break` y `continue`
- ✅ `else if`
- ✅ Scope aislado por función
- ✅ Call-before-definition
- ✅ Recursión mutua
- ✅ Límite de profundidad de recursión (500)
- ✅ Límite de iteraciones de bucle (1 000 000)
- ✅ Aritmética comprobada (overflow → error)
- ✅ Backend LLVM con binarios nativos
- ✅ CLI unificada: `run`, `build`, `check`, `tokens`, `ast`, `llvm`
- ✅ Toolchains `clang` y `llc + cc`
- ✅ Niveles de optimización `-O0` a `-O3`
- ✅ Más de 800 tests de integración

## v0.3.0 — Estructuras de datos 🚧

La línea actual amplía el lenguaje con colecciones y métodos.

### Implementado

- ✅ Arrays con literales e indexación
- ✅ Slicing `arr[inicio:fin]`
- ✅ `push`, `pop`, `append`, `first`, `last`, `reverse`, `join`, `sort`
- ✅ `contains` e `index_of` para arrays y strings
- ✅ Métodos de string: `upper`, `lower`, `trim`, `starts_with`,
  `ends_with`, `contains`, `index_of`, `replace`, `substr`, `reverse`
- ✅ Strings con comillas simples y dobles
- ✅ `for` sobre arrays en el intérprete
- ✅ `range()` y `for i in range(...)`
- ✅ Métodos de string en binarios compilados

### Pendiente

- 🚧 `for` sobre arrays en modo compilado
- 🚧 Arrays en el backend LLVM
- 📅 Diccionarios / maps
- 📅 Más operaciones de colecciones (`map`, `filter`, `reduce`)

Mientras esto siga así, el intérprete (`halo run`) es el entorno recomendado
para arrays. El compilador (`halo build`) funciona con escalares y strings.

## v0.4.0 — Tooling 📅

El siguiente bloque se centra en la experiencia de desarrollo.

- 📅 Mensajes de error con línea y columna en runtime
- 📅 Comentarios multilínea `/* … */`
- 📅 Standard library extendida
- 📅 REPL interactivo (`halo repl`)
- 📅 Mejoras de diagnóstico y sugerencias

## v1.0.0 — Ecosistema 📅

La visión a largo plazo.

- 📅 Backend de bytecode + VM
- 📅 Package manager
- 📅 Documentación de la stdlib y guías avanzadas
- 📅 Pipeline de release con binarios por plataforma
- 📅 Ecosistema de ejemplos y proyectos

## Cómo se prioriza

Las prioridades actuales son:

1. **Corregir divergencias** entre intérprete y compilador.
2. **Completar arrays** en el backend nativo.
3. **Mejorar los errores**: línea, columna y contexto.
4. **Ampliar la biblioteca estándar** sin romper la simplicidad.
5. **Documentar** cada comportamiento nuevo.

Si quieres proponer algo, abre un issue describiendo el caso de uso. Las
contribuciones pequeñas y bien probadas avanzan más rápido que las grandes.

## Estado de un vistazo

| Área | Estado |
|---|---|
| Lexer / parser | ✅ Estable |
| Intérprete escalar | ✅ Estable |
| Intérprete con arrays | ✅ Funcional |
| Compilador escalar | ✅ Funcional |
| Compilador con strings | ✅ Funcional |
| Compilador con arrays | ❌ Pendiente |
| Maps | ❌ Pendiente |
| REPL | ❌ Pendiente |
| Package manager | ❌ Pendiente |
| Documentación | ✅ Este sitio |

## Siguiente paso

- [Arquitectura](../arquitectura/).
- [Testing](../testing/).
- [Contribuir](../contribuir/).
