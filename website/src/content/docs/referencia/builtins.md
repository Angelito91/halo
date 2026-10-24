---
title: Funciones built-in
description: "Referencia completa de funciones y métodos incorporados: aridad, tipo de retorno, soporte en cada backend y ejemplos."
---

Halo incluye un conjunto pequeño de funciones y métodos. Todas las funciones
son de **aridad fija** salvo `print` y `range`.

:::note
✅ significa que la función existe en ese backend y coincide en lo esencial;
⚠️ indica que existe pero tiene divergencias conocidas; ❌ significa que no
está disponible. `str()` sobre variables, `int(string)`, `float(string)`, los
formatos de impresión y algunos métodos todavía divergen. Consulta
[diferencias entre backends](../diferencias/).
:::

## Resumen

| Nombre | Aridad | Devuelve | Intérprete | Compilado |
|---|---|:---:|:---:|:---:|
| `print(...)` | 0+ | `null` | ✅ | ⚠️ |
| `len(x)` | 1 | `number` | ✅ | ✅ |
| `str(x)` | 1 | `string` | ✅ | ⚠️ |
| `int(x)` | 1 | `number` | ✅ | ⚠️ |
| `float(x)` | 1 | `float` | ✅ | ⚠️ |
| `abs(x)` | 1 | `number` / `float` | ✅ | ⚠️ |
| `type(x)` | 1 | `string` | ✅ | ✅ |
| `range(end)` / `range(a, b)` | 1–2 | `array` | ✅ | ❌ |
| `push(arr, x)` | 2 | `array` | ✅ | ❌ |
| `pop(arr)` | 1 | cualquiera / `null` | ✅ | ❌ |
| `append(a, b)` | 2 | `array` | ✅ | ❌ |
| `first(arr)` | 1 | cualquiera / `null` | ✅ | ❌ |
| `last(arr)` | 1 | cualquiera / `null` | ✅ | ❌ |
| `reverse(x)` | 1 | `array` / `string` | ✅ | ✅ (solo string) |
| `join(arr, sep)` | 2 | `string` | ✅ | ❌ |
| `sort(arr)` | 1 | `array` | ✅ | ❌ |
| `contains(x, y)` | 2 | `bool` | ✅ | ⚠️ (solo string) |
| `index_of(x, y)` | 2 | `number` | ✅ | ✅ (solo string) |
| `upper(s)` | 1 | `string` | ✅ | ✅ |
| `lower(s)` | 1 | `string` | ✅ | ✅ |
| `trim(s)` | 1 | `string` | ✅ | ✅ |
| `starts_with(s, p)` | 2 | `bool` | ✅ | ✅ |
| `ends_with(s, p)` | 2 | `bool` | ✅ | ✅ |
| `replace(s, a, b)` | 3 | `string` | ✅ | ✅ |
| `substr(s, i, n)` | 3 | `string` | ✅ | ✅ |

Las funciones que dependen de arrays no funcionan en modo compilado porque los
binarios nativos todavía no tienen arrays. `range()` es la excepción parcial:
el compilador la admite directamente en un `for`.

## Entrada / salida

### `print(...)`

Imprime cada argumento en **su propia línea**. Acepta cualquier tipo y
cualquier número de argumentos, incluido cero.

```halo
print("hola")
print(42)
print(3.14)
print(true)
print([1, 2, 3])
print()              // línea vacía
```

```text
hola
42
3.14
true
[1, 2, 3]

```

`print(a, b)` imprime `a` y luego `b`, cada uno con salto de línea; no añade
separadores. En modo compilado, los argumentos se unen con espacios y `print()`
sin argumentos imprime una línea vacía; consulta
[diferencias entre backends](../diferencias/).

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

### `push(arr, x)` / `arr.push(x)`

Devuelve un array **nuevo** con `x` al final.

```halo
arr = [1, 2]
arr = push(arr, 3)    // [1, 2, 3]
arr = arr.push(4)     // [1, 2, 3, 4]
```

### `pop(arr)` / `arr.pop()`

Devuelve el último elemento, o `null` si el array está vacío. No devuelve un
array nuevo.

```halo
arr = [10, 20, 30]
print(arr.pop())     // 30
print([].pop())      // null
```

### `append(a, b)`

Concatena dos arrays y devuelve uno nuevo.

```halo
print(append([1, 2], [3, 4]))   // [1, 2, 3, 4]
```

### `first(arr)` / `arr.first()`

Primer elemento, o `null` si está vacío.

```halo
print([10, 20].first())   // 10
print([].first())         // null
```

### `last(arr)` / `arr.last()`

Último elemento, o `null` si está vacío.

```halo
print([10, 20].last())   // 20
```

### `reverse(x)` / `x.reverse()`

Invierte un array o un string y devuelve un valor nuevo.

```halo
print([1, 2, 3].reverse())   // [3, 2, 1]
print("abc".reverse())       // cba
```

En modo compilado solo está disponible para strings.

### `join(arr, sep)` / `arr.join(sep)`

Convierte cada elemento a string y los une con `sep`.

```halo
print([1, 2, 3].join("-"))     // 1-2-3
print(["a", "b"].join(", "))   // a, b
```

### `sort(arr)` / `arr.sort()`

Ordena números (enteros y floats) o strings. No mezcla tipos.

```halo
print([3, 1, 2].sort())            // [1, 2, 3]
print(["pera", "manzana"].sort())  // [manzana, pera]
sort([1, "dos"])                   // Error
```

## Búsqueda

### `contains(x, y)` / `x.contains(y)`

Para strings, comprueba si `y` es un substring. Para arrays, comprueba si hay
un elemento igual a `y`.

```halo
print("hola".contains("ol"))    // true
print([1, 2, 3].contains(2))    // true
```

### `index_of(x, y)` / `x.index_of(y)`

Para strings, devuelve el offset de bytes de la primera aparición o `-1`. Para
arrays, devuelve el índice del elemento o `-1`.

```halo
print("hola".index_of("l"))     // 2
print("hola".index_of("z"))     // -1
print([10, 20].index_of(20))    // 1
```

## Strings

### `upper(s)` / `s.upper()`

```halo
print("Hola".upper())   // HOLA
```

### `lower(s)` / `s.lower()`

```halo
print("HOLA".lower())   // hola
```

### `trim(s)` / `s.trim()`

Quita espacios, tabs y saltos de línea al inicio y al final.

```halo
print("  hola  ".trim())   // hola
```

### `starts_with(s, p)` / `s.starts_with(p)`

```halo
print("programa.halo".starts_with("programa"))   // true
```

### `ends_with(s, p)` / `s.ends_with(p)`

```halo
print("programa.halo".ends_with(".halo"))   // true
```

### `replace(s, a, b)` / `s.replace(a, b)`

Reemplaza **todas** las apariciones de `a` por `b`.

```halo
print("a-b-a".replace("a", "x"))   // x-b-x
```

### `substr(s, i, n)` / `s.substr(i, n)`

Extrae `n` caracteres a partir de `i`. Trabaja con caracteres, no bytes.

```halo
print("hola mundo".substr(0, 4))   // hola
print("hola".substr(2, 99))        // la
print("hola".substr(99, 1))        // (vacío)
```

Un `i` o `n` negativo es un error.

## Métodos y pureza

Los métodos son azúcar sintáctico sobre las funciones. **No mutan el
receptor**; devuelven un valor nuevo:

```halo
s = "hola"
s.upper()
print(s)          // hola (sin cambios)

s = s.upper()
print(s)          // HOLA
```

Lo mismo aplica a arrays:

```halo
arr = [1, 2]
arr.push(3)
print(arr)        // [1, 2]

arr = arr.push(3)
print(arr)        // [1, 2, 3]
```

## Errores de aridad

Pasar un número incorrecto de argumentos es un error de runtime:

```halo
len()          // len() expects exactly 1 argument, got 0
len("a", "b")  // len() expects exactly 1 argument, got 2
range()        // range() expects 1 or 2 arguments, got 0
```

Consulta [errores de runtime](../errores/) para la lista completa.
