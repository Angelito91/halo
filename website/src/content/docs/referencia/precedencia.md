---
title: Precedencia de operadores
description: Orden de evaluación de los operadores de Halo, con ejemplos y reglas de asociatividad.
---

De mayor a menor prioridad:

| Nivel | Operadores | Asociatividad |
|---|---|---|
| 7 (más alto) | `-x`, `!x` (unarios) | Derecha |
| 6 | `*`, `/`, `%` | Izquierda |
| 5 | `+`, `-` | Izquierda |
| 4 | `<`, `>`, `<=`, `>=` | Izquierda |
| 3 | `==`, `!=` | Izquierda |
| 2 | `&&` / `and` | Izquierda (cortocircuito) |
| 1 (más bajo) | `\|\|` / `or` | Izquierda (cortocircuito) |

La asignación (`=`) no es un operador de expresión: es una sentencia. No
puedes escribir `a = b = c` ni usar una asignación dentro de una condición.

## Ejemplos

```halo
2 + 3 * 4           // 14  (* antes que +)
(2 + 3) * 4         // 20
10 - 2 * 3          // 4
2 + 3 * 4 - 6 / 2   // 11  (2 + 12 - 3)
10 + 7 % 3          // 11  (% antes que +)
```

```halo
true || false && false   // true  (&& antes que ||)
!true && false           // false (!true = false, false && false = false)
```

## Comparaciones

Los operadores relacionales tienen más prioridad que la igualdad:

```halo
1 < 2 == true    // (1 < 2) == true → true
```

Si quieres comparar el resultado de una igualdad, usa paréntesis:

```halo
(1 == 1) == true   // true
```

## Operadores unarios

El menos unario y el `!` se aplican al operando inmediato:

```halo
-3 * 2       // (-3) * 2 = -6
-(3 * 2)     // -6
!true || true    // (!true) || true → true
```

No hay `++` ni `--`.

## Cortocircuito

`&&` y `||` evalúan de izquierda a derecha y se detienen en cuanto el
resultado está decidido:

```halo
false && peligro()   // peligro() no se llama
true  || peligro()   // peligro() no se llama
```

Recuerda que el resultado es el operando que decide, no necesariamente un
booleano:

```halo
0 || "fallback"    // "fallback"
"hola" && 42       // 42
```

## Paréntesis

Los paréntesis anulan la precedencia y también sirven para documentar la
intención:

```halo
a + b * c       // a + (b * c)
(a + b) * c     // otra cosa

(x > 0) && (y > 0)
```

## Errores comunes

### `!` con comparaciones

```halo
!a == b    // (!a) == b
!(a == b)  // lo que probablemente quieres
```

### Mezclar `&&` y `||` sin paréntesis

```halo
a || b && c    // a || (b && c)
(a || b) && c  // otra cosa
```

### Asignación dentro de una condición

```halo
if x = 5 { }   // ❌ `=` no es una expresión
if x == 5 { }  // ✅
```

## Siguiente paso

- [Operadores](../../guia/operadores/).
- [Condicionales](../../guia/condicionales/).
- [Errores](../errores/).
