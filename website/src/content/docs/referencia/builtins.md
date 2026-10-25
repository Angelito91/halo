---
title: Funciones built-in
description: "Referencia completa de funciones incorporadas: aridad, tipo de retorno y ejemplos."
---

Halo incluye un conjunto pequeño de funciones. Todas son de **aridad fija**
salvo `print`, `println` y `range`.

## Resumen

| Nombre | Aridad | Devuelve |
|---|---|:---:|
| `print(...)` | 0+ | `null` |
| `println(...)` | 0+ | `null` |
| `len(x)` | 1 | `number` |
| `str(x)` | 1 | `string` |
| `int(x)` | 1 | `number` |
| `float(x)` | 1 | `float` |
| `abs(x)` | 1 | `number` / `float` |
| `type(x)` | 1 | `string` |
| `range(end)` / `range(a, b)` | 1–2 | `array` |
| `push(arr, x)` | 2 | `array` |
| `pop(arr)` | 1 | `array` |
| `append(a, b)` | 2 | `array` |
| `first(arr)` | 1 | cualquiera / `null` |
| `last(arr)` | 1 | cualquiera / `null` |
| `reverse(x)` | 1 | `array` / `string` |
| `join(arr, sep)` | 2 | `string` |
| `sort(arr)` | 1 | `array` |
| `contains(x, y)` | 2 | `bool` |
| `index_of(x, y)` | 2 | `number` |
| `upper(s)` | 1 | `string` |
| `lower(s)` | 1 | `string` |
| `trim(s)` | 1 | `string` |
| `starts_with(s, p)` | 2 | `bool` |
| `ends_with(s, p)` | 2 | `bool` |
| `replace(s, a, b)` | 3 | `string` |
| `substr(s, i, n)` | 3 | `string` |

## Entrada / salida

### `print(...)` / `println(...)`

Imprime todos los argumentos en **una sola línea**, separados por un espacio,
y termina con un salto de línea. Acepta cualquier tipo y cualquier número de
argumentos, incluido cero.

```halo
print("hola")
print(42)
print(3.14)
print(true)
print([1, 2, 3])
print("a", 1, true)   // a 1 true
print()               // línea vacía
```

```text
hola
42
3.14
true
[1, 2, 3]
a 1 true

```

`println` es un alias de `print`: ambos añaden el salto de línea final. No hay
una versión que imprima sin salto.

## Tipos y conversión

### `len(x)`

| Tipo | Resultado |
|---|---|
| `string` | Número de **bytes UTF-8** |
| `array` | Número de elementos |
| `number` | Número de dígitos de su representación (incluye el signo) |

```halo
len("hola")     // 4
len("")         // 0
len("ñ")        // 2
len([1, 2, 3])  // 3
len([])         // 0
len(12345)      // 5
len(-99)        // 3
```

`len()` no acepta `bool`, `float` ni `null`.

### `str(x)`

Convierte cualquier valor a string:

```halo
str(42)       // "42"
str(-7)       // "-7"
str(3.14)     // "3.14"
str(2.0)      // "2.0"
str(true)     // "true"
str(false)    // "false"
str("hola")   // "hola"
str([1, 2])   // "[1, 2]"
nada() { return }
str(nada())   // "null"
```

### `int(x)`

| Entrada | Resultado |
|---|---|
| `float` | Trunca hacia cero |
| `bool` | `true` → 1, `false` → 0 |
| `string` | Parsea un entero; error si no es numérico |
| `number` | Identidad |

```halo
int(3.9)     // 3
int(-2.7)    // -2
int(true)    // 1
int(false)   // 0
int("99")    // 99
int("-5")    // -5
int("abc")   // Error: Cannot convert 'abc' to integer
```

### `float(x)`

| Entrada | Resultado |
|---|---|
| `number` | El mismo valor como float |
| `bool` | `true` → 1.0, `false` → 0.0 |
| `string` | Parsea un flotante; error si no es numérico |
| `float` | Identidad |

```halo
float(7)      // 7.0
float(true)   // 1.0
float("2.5")  // 2.5
float("10")   // 10.0
float("xyz")  // Error: Cannot convert 'xyz' to number
```

### `abs(x)`

Valor absoluto de un `number` o `float`:

```halo
abs(7)       // 7
abs(-7)      // 7
abs(0)       // 0
abs(-3.14)   // 3.14
abs(true)    // Error: abs() does not support type 'bool'
```

### `type(x)`

Devuelve el nombre del tipo:

```halo
type(42)      // "number"
type(3.14)    // "float"
type(true)    // "bool"
type("hola")  // "string"
type([1])     // "array"

nada() { return }
type(nada())  // "null"
```

## Rangos

### `range(end)` / `range(start, end)`

Devuelve `[0, …, end-1]` o `[start, …, end-1]`. Ambos límites deben ser
enteros y no hay paso negativo.

```halo
range(4)       // [0, 1, 2, 3]
range(2, 5)    // [2, 3, 4]
range(3, 3)    // []
range(5, 2)    // []
range(1.5)     // Error: range() expects integer arguments, got 'float'
```

El tamaño máximo es 1 000 000 de elementos.

## Arrays

### `push(arr, x)`

Devuelve un array **nuevo** con `x` al final.

```halo
arr = [1, 2]
arr = push(arr, 3)    // [1, 2, 3]
arr = push(arr, 4)    // [1, 2, 3, 4]
```

### `pop(arr)`

Devuelve un array **nuevo** sin su último elemento. No muta el original.
`pop([])` devuelve `[]`.

```halo
arr = [10, 20, 30]
print(pop(arr))     // [10, 20]
print(arr)          // [10, 20, 30]
print(pop([]))      // []
```

### `append(a, b)`

Concatena dos arrays y devuelve uno nuevo.

```halo
print(append([1, 2], [3, 4]))   // [1, 2, 3, 4]
```

### `first(arr)`

Primer elemento, o `null` si está vacío.

```halo
print(first([10, 20]))   // 10
print(first([]))         // null
```

### `last(arr)`

Último elemento, o `null` si está vacío. Es una lectura: no modifica el array.

```halo
print(last([10, 20]))   // 20
print(last([]))         // null
```

### `reverse(x)`

Invierte un array o un string y devuelve un valor nuevo.

```halo
print(reverse([1, 2, 3]))   // [3, 2, 1]
print(reverse("abc"))       // cba
```

### `join(arr, sep)`

Convierte cada elemento a string y los une con `sep`.

```halo
print(join([1, 2, 3], "-"))     // 1-2-3
print(join(["a", "b"], ", "))   // a, b
```

### `sort(arr)`

Ordena números (enteros y floats) o strings. No mezcla tipos.

```halo
print(sort([3, 1, 2]))            // [1, 2, 3]
print(sort(["pera", "manzana"]))  // [manzana, pera]
sort([1, "dos"])                  // Error
```

## Búsqueda

### `contains(x, y)`

Para strings, comprueba si `y` es un substring. Para arrays, comprueba si hay
un elemento igual a `y` usando `==`. Como los arrays no tienen igualdad
estructural, no encuentra arrays anidados por su contenido.

```halo
print(contains("hola", "ol"))    // true
print(contains([1, 2, 3], 2))    // true
```

### `index_of(x, y)`

Para strings, devuelve el offset de bytes de la primera aparición o `-1`. Para
arrays, devuelve el índice del elemento o `-1`; al igual que `contains`, usa
`==` y no encuentra arrays anidados por su contenido.

```halo
print(index_of("hola", "l"))     // 2
print(index_of("hola", "z"))     // -1
print(index_of([10, 20], 20))    // 1
```

## Strings

### `upper(s)`

```halo
print(upper("Hola"))   // HOLA
```

### `lower(s)`

```halo
print(lower("HOLA"))   // hola
```

### `trim(s)`

Quita espacios, tabs y saltos de línea al inicio y al final.

```halo
print(trim("  hola  "))   // hola
```

### `starts_with(s, p)`

```halo
print(starts_with("programa.halo", "programa"))   // true
```

### `ends_with(s, p)`

```halo
print(ends_with("programa.halo", ".halo"))   // true
```

### `replace(s, a, b)`

Reemplaza **todas** las apariciones de `a` por `b`.

```halo
print(replace("a-b-a", "a", "x"))   // x-b-x
```

### `substr(s, i, n)`

Extrae `n` caracteres a partir de `i`. Trabaja con caracteres, no bytes.

```halo
print(substr("hola mundo", 0, 4))   // hola
print(substr("hola", 2, 99))        // la
print(substr("hola", 99, 1))        // (vacío)
```

Un `i` o `n` negativo es un error.

## Pureza y mutación

Todas las funciones son puras: no mutan sus argumentos. Los arrays se pasan por
valor, así que `push`, `pop`, `reverse` y `sort` devuelven arrays nuevos.

```halo
s = "hola"
print(upper(s))   // HOLA
print(s)          // hola

arr = [1, 2]
arr = push(arr, 3)
print(arr)        // [1, 2, 3]
```

La única mutación de un array es la asignación por índice:

```halo
arr = [1, 2, 3]
arr[0] = 99
print(arr)        // [99, 2, 3]
```

No hay métodos con punto: `s.upper()` o `arr.push(3)` no son válidos. Usa
`upper(s)` y `push(arr, 3)`.

## Errores de aridad

Pasar un número incorrecto de argumentos es un error de runtime:

```halo
len()          // len() expects exactly 1 argument, got 0
len("a", "b")  // len() expects exactly 1 argument, got 2
range()        // range() expects 1 or 2 arguments, got 0
```

Consulta [errores de runtime](../errores/) para la lista completa.
