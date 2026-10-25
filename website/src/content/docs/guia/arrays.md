---
title: Arrays
description: Creación, indexación, slicing, métodos y limitaciones de los arrays en Halo.
---

Los arrays son colecciones ordenadas y **heterogéneas**: pueden mezclar
números, strings, booleanos y otros arrays en la misma lista.

## Crear arrays

```halo
numeros = [1, 2, 3, 4, 5]
nombres = ["Ana", "Luis", "Marta"]
flags = [true, false, true]
mixto = [1, "hola", 3.14, true]
vacio = []
anidado = [[1, 2], [3, 4]]
```

El literal puede contener expresiones:

```halo
a = 10
b = 20
valores = [a, b, a + b]   // [10, 20, 30]
```

## Indexación

Los índices empiezan en **0**. Se accede con corchetes:

```halo
frutas = ["manzana", "pera", "uva"]

print(frutas[0])   // manzana
print(frutas[2])   // uva
```

Un índice negativo o fuera de rango es un error:

```halo
frutas[-1]   // Error: Array index cannot be negative, got -1
frutas[3]    // Error: Array index 3 out of bounds (length 3)
```

No hay acceso circular ni `at()`.

## Slicing

Puedes extraer una porción con `[inicio:fin]`. El índice `fin` es **exclusivo**:

```halo
letras = ["a", "b", "c", "d", "e"]

print(letras[1:4])   // [b, c, d]
print(letras[:2])    // [a, b]
print(letras[3:])    // [d, e]
print(letras[:])     // [a, b, c, d, e]
```

El slicing devuelve un array nuevo. Los límites deben estar dentro del array y
`inicio <= fin`:

```halo
letras[2:1]   // Error: Slice start index 2 cannot be greater than end index 1
letras[0:9]   // Error: Slice end index 9 out of bounds for array of length 5
```

## Longitud

```halo
print(len([1, 2, 3]))   // 3
print(len([]))          // 0
```

No hay propiedad `.length`; usa `len()`.

## Métodos y built-ins

Los arrays tienen dos formas equivalentes: **función** (el array es el primer
argumento) y **método** (el array va antes del punto).

| Operación | Función | Método | Devuelve |
|---|---|---|---|
| Añadir al final | `push(arr, x)` | `arr.push(x)` | Array nuevo |
| Concatenar | `append(a, b)` | — | Array nuevo |
| Quitar el último | `pop(arr)` | `arr.pop()` | Último elemento |
| Primero | `first(arr)` | `arr.first()` | Elemento o `null` |
| Último | `last(arr)` | `arr.last()` | Elemento o `null` |
| Invertir | `reverse(arr)` | `arr.reverse()` | Array nuevo |
| Unir en string | `join(arr, sep)` | `arr.join(sep)` | String |
| Ordenar | `sort(arr)` | `arr.sort()` | Array nuevo |
| Contiene | `contains(arr, x)` | `arr.contains(x)` | `bool` |
| Índice de | `index_of(arr, x)` | `arr.index_of(x)` | `number` (`-1` si no está) |

### Los métodos no mutan

Esta es la trampa más importante: **los métodos devuelven un valor nuevo y no
modifican el receptor**. Si escribes:

```halo
arr = [1, 2]
arr.push(3)
print(arr)   // [1, 2]  ← no cambió
```

El array original sigue igual. Tienes que quedarte con el resultado:

```halo
arr = [1, 2]
arr = arr.push(3)
print(arr)   // [1, 2, 3]
```

O usar la forma funcional:

```halo
arr = [1, 2]
arr = push(arr, 3)
print(arr)   // [1, 2, 3]
```

### Ejemplos

```halo
arr = [1, 2, 3]

arr = arr.push(4)          // [1, 2, 3, 4]
arr = append(arr, [5, 6])  // [1, 2, 3, 4, 5, 6]

print(arr.pop())           // 6
print(arr.first())         // 1
print(arr.last())          // 5
print(arr.reverse())       // [5, 4, 3, 2, 1]
print(arr.join("-"))       // 1-2-3-4-5
```

`pop()` devuelve el último elemento, no un array nuevo. Si el array está
vacío, devuelve `null`.

`first()` y `last()` también devuelven `null` en arrays vacíos.

### Ordenar

`sort()` ordena números (enteros y floats) o strings, pero no mezclas:

```halo
print(sort([3, 1, 2]))             // [1, 2, 3]
print(sort(["pera", "manzana"]))   // [manzana, pera]

sort([1, "dos"])   // Error: sort() cannot mix strings and numbers in the same array
```

### Buscar

```halo
nums = [10, 20, 30]

print(nums.contains(20))    // true
print(nums.contains(99))    // false
print(nums.index_of(30))    // 2
print(nums.index_of(99))    // -1
```

## Recorrer arrays

Con `for`:

```halo
nombres = ["Ana", "Luis", "Marta"]
for nombre in nombres {
    print("Hola, " + nombre)
}
```

Con índice:

```halo
for i in range(len(nombres)) {
    print(str(i) + ": " + nombres[i])
}
```

Con `while`:

```halo
i = 0
while i < len(nombres) {
    print(nombres[i])
    i = i + 1
}
```

## Arrays como valores

Los arrays se pasan por valor, como el resto de tipos. Una función recibe una
copia y no puede mutar el array del caller:

```halo
agregar(arr, x) {
    return arr.push(x)
}

original = [1, 2]
nuevo = agregar(original, 3)
print(original)   // [1, 2]
print(nuevo)      // [1, 2, 3]
```

## `range`

`range()` es la forma estándar de generar secuencias de enteros:

```halo
range(4)       // [0, 1, 2, 3]
range(2, 5)    // [2, 3, 4]
```

Consulta [bucles](../bucles/#range) para los límites y el tamaño máximo.

## Limitaciones

- No hay `map`, `filter`, `reduce` ni `forEach`.
- No hay arrays asociativos ni diccionarios.
- No hay `arr.length`; usa `len(arr)`.
- No hay indexación negativa.
- No hay asignación por índice (`arr[0] = x`).
- No hay igualdad estructural: `[1, 2] == [1, 2]` devuelve `false`, y
  `contains`/`index_of` no encuentran arrays anidados por su contenido.
- Los literales de array deben ir en una sola línea; el parser no acepta saltos
  de línea dentro de `[...]`.
- No hay métodos que muten el array in-place.

## Siguiente paso

- [Strings](../strings/).
- [Built-ins](../../referencia/builtins/).
- [Ejemplos](../../ejemplos/).
