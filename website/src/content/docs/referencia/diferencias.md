---
title: Diferencias entre backends
description: Divergencias conocidas entre el intérprete y el compilador de Halo, y cómo evitarlas.
---

Halo comparte lexer, parser y AST entre sus dos backends, pero **todavía no
comparten toda la semántica**. El intérprete (`halo run`) es la referencia:
implementa arrays, comprobación de overflow y la mayoría de métodos. El
compilador (`halo build`) está detrás en varias áreas.

Estas diferencias están congeladas en `tests/common/parity.rs`
(`KNOWN_DIVERGENCES`) y se corrigen por fases. Mientras existan, no asumas que
un programa que funciona con `halo run` producirá la misma salida compilado.

## Resumen

| Área | Intérprete | Compilado | Notas |
|---|---|---|---|
| Arrays | ✅ | ❌ | No existen valores de array en el backend nativo |
| `for` sobre arrays | ✅ | ❌ | Solo `for … in range(...)` |
| `range()` como valor | ✅ | ❌ | Solo se admite directamente en un `for` |
| `print(true)` / bools | `true` / `false` | `-1` / `0` | Divergencia de formato |
| Floats al imprimir | Representación mínima | 6 decimales | Divergencia de formato |
| `print(a, b)` | Una línea por argumento | Argumentos unidos por espacios | |
| `print()` sin argumentos | No imprime nada | Imprime un salto de línea | |
| `str(x)` con variables | ✅ | ❌ | En compilado solo formatea constantes |
| `int(string)` / `float(string)` | ✅ | ❌ | Conversión desde string no soportada |
| `"texto" + 42` | Concatena | No imprime nada | Evita mezclar string y número |
| Imprimir una comparación | `true` / `false` | No imprime nada | |
| `contains()` al imprimir | `true` / `false` | `1` / `0` | |
| Retorno void | `null` | `0` | |
| Declarar una variable bool | ✅ | ❌ | El codegen falla |
| Overflow de enteros | Error | Wrap silencioso | Peligroso en binarios |
| División por cero | Error | Comportamiento indefinido | No lo hagas |
| Llamada anidada que devuelve string | ✅ | ❌ | Falla al compilar |
| `println` | ❌ | ✅ | Solo existe en compilado |

## Qué usar según el objetivo

### Para aprender y probar el lenguaje

Usa `halo run`. Es el backend completo y el que define el comportamiento
documentado en esta guía.

### Para generar un binario nativo

Usa `halo build` con programas **escalares**: números, floats, strings,
booleanos, condicionales, bucles `while`, funciones y `for … in range(...)`.
Evita por ahora:

- arrays y métodos de array;
- `str()` sobre variables;
- concatenar strings con números;
- imprimir comparaciones o booleanos si necesitas el formato exacto;
- overflow intencionado y divisiones por cero;
- funciones que devuelvan strings y se llamen dentro de otra expresión.

## Ejemplo de divergencia

Este programa funciona en el intérprete:

```halo
n = 7
print(str(n))
print("n = " + n)
print(true)
```

Salida con `halo run`:

```text
7
n = 7
true
```

En modo compilado, `str(n)` no formatea variables, `"n = " + n` no imprime el
número y `true` se imprime como `-1`. Por eso la recomendación es no usar estas
construcciones en binarios hasta que la paridad esté completa.

## Seguimiento

La suite `parity_freeze_tests.rs` ejecuta el mismo programa en ambos backends y
falla si aparece una divergencia no declarada o si una entrada de
`KNOWN_DIVERGENCES` ya no diverge. Consulta [testing](../../proyecto/testing/) y el
[roadmap](../../proyecto/roadmap/) para el estado de cada fase.
