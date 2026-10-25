---
title: Arrays
description: Creación, indexación, asignación, slicing, funciones y limitaciones de los arrays en Halo.
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

## Asignación por índice

`arr[i] = valor` muta el array que vive en la variable y devuelve el valor
asignado:

```halo
nums = [10, 20, 30]
nums[0] = 99
print(nums)   // [99, 20, 30]

resultado = nums[1] = 7
print(resultado)   // 7
print(nums)        // [99, 7, 30]
```

También funciona con índices anidados:

```halo
matrix = [[1, 2], [3, 4]]
matrix[1][0] = 42
print(matrix)   // [[1, 2], [42, 4]]
```

El destino debe ser una variable o una cadena de índices simples. No se puede
asignar a un slice, a un literal ni al resultado de una llamada:

```halo
nums[0:1] = [9]   // Error: Cannot assign to a slice; index a single element
```

Los índices negativos o fuera de rango producen los mismos errores que en la
lectura.

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

## Funciones de array

No hay métodos con punto. Todas las operaciones son funciones que reciben el
array como primer argumento:

| Operación | Función | Devuelve |
|---|---|---|
| Añadir al final | `push(arr, x)` | Array nuevo |
| Concatenar | `append(a, b)` | Array nuevo |
| Quitar el último | `pop(arr)` | Array nuevo sin el último elemento |
| Primero | `first(arr)` | Elemento o `null` |
| Último | `last(arr)` | Elemento o `null` |
| Invertir | `reverse(arr)` | Array nuevo |
| Unir en string | `join(arr, sep)` | String |
| Ordenar | `sort(arr)` | Array nuevo |
| Contiene | `contains(arr, x)` | `bool` |
| Índice de | `index_of(arr, x)` | `number` (`-1` si no está) |

### Las funciones no mutan

Todas las funciones de array devuelven un valor nuevo y no modifican el
original. La única forma de mutar un array es la asignación por índice:

```halo
arr = [1, 2]
arr = push(arr, 3)
print(arr)   // [1, 2, 3]

arr[0] = 9
print(arr)   // [9, 2, 3]
```

`arr.push(3)` no es válido: el parser rechaza los métodos con punto. La forma
correcta es `arr = push(arr, 3)`.

`pop` devuelve un array nuevo sin el último elemento; `last` lee el último
elemento sin modificar el array:

```halo
arr = [1, 2, 3]
print(pop(arr))    // [1, 2]
print(arr)         // [1, 2, 3]
print(last(arr))   // 3
```

`pop([])` devuelve `[]`. `first` y `last` devuelven `null` en arrays vacíos.

### Ejemplos

```halo
arr = [1, 2, 3]

arr = push(arr, 4)          // [1, 2, 3, 4]
arr = append(arr, [5, 6])   // [1, 2, 3, 4, 5, 6]

print(pop(arr))       // [1, 2, 3, 4, 5]
print(first(arr))     // 1
print(last(arr))      // 6
print(reverse(arr))   // [6, 5, 4, 3, 2, 1]
print(join(arr, "-")) // 1-2-3-4-5-6
```

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

print(contains(nums, 20))    // true
print(contains(nums, 99))    // false
print(index_of(nums, 30))    // 2
print(index_of(nums, 99))    // -1
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
    return push(arr, x)
}

original = [1, 2]
nuevo = agregar(original, 3)
print(original)   // [1, 2]
print(nuevo)      // [1, 2, 3]
```

La asignación por índice dentro de una función modifica solo la copia local:

```halo
mutar(arr) {
    arr[0] = 99
    return arr
}

original = [1, 2]
nuevo = mutar(original)
print(original)   // [1, 2]
print(nuevo)      // [99, 2]
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
- No hay métodos con punto: usa `push(arr, x)`, `upper(text)`, etc.
- No se puede asignar a un slice (`arr[0:2] = ...`).
- No hay igualdad estructural: `[1, 2] == [1, 2]` devuelve `false`, y
  `contains`/`index_of` no encuentran arrays anidados por su contenido.
- Los literales de array deben ir en una sola línea; el parser no acepta saltos
  de línea dentro de `[...]`.
- `push`, `pop`, `reverse` y `sort` no mutan el array: devuelven uno nuevo.
  La única mutación es `arr[i] = value`.

## Siguiente paso

- [Strings](../strings/).
- [Built-ins](../../referencia/builtins/).
- [Ejemplos](../../ejemplos/).
