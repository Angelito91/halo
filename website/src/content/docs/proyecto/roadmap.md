---
title: Roadmap
description: Estado actual del lenguaje y próximos hitos de un intérprete educativo pensado para scripts simples.
---

Halo está en desarrollo activo. Esta página describe **qué existe hoy** y qué
viene después, sin fechas prometidas.

:::note
La versión declarada en `Cargo.toml` es **0.3.0**. Desde esa versión Halo es
**interpreter-only**: no hay backend nativo y cualquier backend futuro sería
un proyecto separado, no una dependencia del intérprete.
:::

## Leyenda

| Símbolo | Significado |
|---|---|
| ✅ | Implementado y probado |
| 🚧 | En progreso o parcial |
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

## v0.2.0 — Robustez ✅

El segundo hito convirtió el intérprete en algo usable.

- ✅ Tipo `float` con promoción automática
- ✅ Tipo `string` con concatenación, repetición y comparación
- ✅ `abs` y `type`
- ✅ `break` y `continue`
- ✅ `else if`
- ✅ Frames de función con locales y shadowing
- ✅ Call-before-definition
- ✅ Recursión mutua
- ✅ Límite de profundidad de recursión (500)
- ✅ Límite de iteraciones de bucle (1 000 000)
- ✅ Aritmética comprobada (overflow → error)

## v0.3.0 — Intérprete completo ✅

La versión actual amplía el lenguaje y simplifica el proyecto.

- ✅ Arrays con literales e indexación
- ✅ Slicing `arr[inicio:fin]`
- ✅ `push`, `pop`, `append`, `first`, `last`, `reverse`, `join`, `sort`
- ✅ `contains` e `index_of` para arrays y strings
- ✅ Métodos de string: `upper`, `lower`, `trim`, `starts_with`,
  `ends_with`, `contains`, `index_of`, `replace`, `substr`, `reverse`
- ✅ Strings con comillas simples y dobles
- ✅ `for` sobre arrays y sobre `range()`
- ✅ `print` con varios argumentos y `println`
- ✅ CLI reducida a `run`, `check`, `tokens` y `ast`
- ✅ Errores de runtime con posición y sugerencias
- ✅ Proyecto **interpreter-only**, sin dependencias externas
- ✅ Este sitio de documentación

## v0.4.0 — Experiencia de desarrollo 📅

El siguiente bloque se centra en escribir y depurar programas cómodamente.

- 📅 REPL interactivo (`halo repl`)
- 📅 Diccionarios / maps
- 📅 Más operaciones de colecciones (`map`, `filter`, `reduce`)
- 📅 Comentarios multilínea `/* … */`
- 📅 Mensajes de error con más contexto de expresión
- 📅 Más ejemplos y guías de scripting

## v1.0.0 — Ecosistema 📅

La visión a largo plazo, sin comprometer el tamaño del proyecto.

- 📅 Standard library más amplia
- 📅 Package manager sencillo
- 📅 Documentación avanzada de la stdlib
- 📅 Pipeline de release con binarios por plataforma
- 📅 Más proyectos de ejemplo

## Cómo se prioriza

Las prioridades actuales son:

1. **Mantener el intérprete pequeño y comprensible.**
2. **Mejorar los errores**: posición, contexto y sugerencias.
3. **Ampliar la biblioteca estándar** sin romper la simplicidad.
4. **Documentar** cada comportamiento nuevo.
5. **Añadir ejemplos** que sirvan para aprender y para copiar.

Si quieres proponer algo, abre un issue describiendo el caso de uso. Las
contribuciones pequeñas y bien probadas avanzan más rápido que las grandes.

## Estado de un vistazo

| Área | Estado |
|---|---|
| Lexer / parser | ✅ Estable |
| Intérprete escalar | ✅ Estable |
| Intérprete con arrays | ✅ Funcional |
| Errores con posición | ✅ Funcional |
| Maps | ❌ Pendiente |
| REPL | ❌ Pendiente |
| Package manager | ❌ Pendiente |
| Documentación | ✅ Este sitio |

## Siguiente paso

- [Arquitectura](../arquitectura/).
- [Testing](../testing/).
- [Contribuir](../contribuir/).
