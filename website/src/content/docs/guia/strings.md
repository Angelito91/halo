---
title: Strings
description: Literales, escapes, concatenación, comparación y funciones de string en Halo.
---

Los strings son secuencias UTF-8 inmutables. Se pueden crear con comillas
dobles o simples y tienen un conjunto de funciones para transformarlos.

## Literales y escapes

```halo
a = "hola"
b = 'hola'
c = 'puedo usar "dobles" dentro'
d = "puedo usar 'simples' dentro"
```

| Escape | Resultado |
|---|---|
| `\\` | Barra invertida |
| `\"` | Comilla doble |
| `\'` | Comilla simple |
| `\n` | Salto de línea |
| `\t` | Tabulador |
| `\r` | Retorno de carro |

```halo
print("línea 1\nlínea 2")
print("columna 1\tcolumna 2")
print("comilla: \"")
print("ruta: C:\\Users\\halo")
```

Los strings son UTF-8; puedes usar acentos y emojis directamente.

## Operaciones básicas

### Concatenar

```halo
saludo = "Hola, " + "mundo"
print(saludo)   // Hola, mundo

print("valor: " + 42)   // valor: 42
print(42 + " items")    // 42 items
```

Si uno de los operandos es string, el otro se convierte a texto.

### Repetir

```halo
print("ab" * 3)    // ababab
print(3 * "ab")    // ababab
print("x" * 0)     // (vacío)
```

Un número negativo es un error.

### Comparar

```halo
"abc" == "abc"   // true
"abc" != "abd"   // true
"abc" < "abd"    // true (lexicográfico)
"hola" >= "hola" // true
```

La comparación es lexicográfica por bytes UTF-8.

## Longitud: bytes, no caracteres

`len(string)` cuenta **bytes UTF-8**, no caracteres visibles:

```halo
len("hola")   // 4
len("")       // 0
len("ñ")      // 2  (la ñ ocupa dos bytes)
len("🦄")     // 4
```

Esto es importante si trabajas con texto no ASCII. `substr()` sí trabaja con
caracteres, así que hay una pequeña asimetría que conviene tener presente.

## Indexación y slicing

Los strings **no** se pueden indexar con `texto[0]` ni recorrer con `for`. Para
extraer partes, usa `substr()`.

## Funciones de string

No hay métodos con punto. Todas las operaciones son funciones que reciben el
string como primer argumento:

| Operación | Función | Ejemplo |
|---|---|---|
| Mayúsculas | `upper(s)` | `upper("hola")` → `HOLA` |
| Minúsculas | `lower(s)` | `lower("HOLA")` → `hola` |
| Quitar espacios | `trim(s)` | `trim("  hola  ")` → `hola` |
| Prefijo | `starts_with(s, p)` | `starts_with("hola", "ho")` → `true` |
| Sufijo | `ends_with(s, p)` | `ends_with("hola", "la")` → `true` |
| Contiene | `contains(s, p)` | `contains("hola", "ol")` → `true` |
| Índice | `index_of(s, p)` | `index_of("hola", "l")` → `2` |
| Reemplazar | `replace(s, a, b)` | `replace("a-b", "-", "_")` → `a_b` |
| Substring | `substr(s, i, n)` | `substr("hola", 1, 2)` → `ol` |
| Invertir | `reverse(s)` | `reverse("abc")` → `cba` |

`"hola".upper()` no es válido: el parser rechaza los métodos con punto y
sugiere `upper("hola")`.

### `upper` / `lower`

```halo
print(upper("Hola"))   // HOLA
print(lower("HOLA"))   // hola
```

### `trim`

Elimina espacios, tabs y saltos de línea al principio y al final:

```halo
print(trim("  hola  "))   // hola
```

### `starts_with` / `ends_with`

```halo
print(ends_with("programa.halo", ".halo"))   // true
print(starts_with("https://", "http"))       // true
```

### `contains`

```halo
print(contains("hola mundo", "mundo"))   // true
print(contains("hola", ""))              // true
```

### `index_of`

Devuelve el índice de la primera aparición o `-1`:

```halo
print(index_of("hola", "l"))    // 2
print(index_of("hola", "z"))    // -1
```

El índice es un **offset de bytes**. Con texto no ASCII puede no coincidir con
la posición visual:

```halo
print(index_of("añb", "b"))   // 3
```

### `replace`

Reemplaza **todas** las apariciones:

```halo
print(replace("a-b-a", "a", "x"))   // x-b-x
```

### `substr`

`substr(texto, inicio, longitud)` trabaja con caracteres:

```halo
print(substr("hola mundo", 0, 4))   // hola
print(substr("hola mundo", 5, 5))   // mundo
print(substr("hola", 1, 2))         // ol
```

Si `inicio` está fuera del string, devuelve `""`. Si la longitud se pasa del
final, se recorta:

```halo
print(substr("hola", 99, 3))   // (vacío)
print(substr("hola", 2, 99))   // la
```

Un inicio o una longitud negativos son errores.

### `reverse`

```halo
print(reverse("abc"))   // cba
print(reverse("añb"))   // bña
```

## Convertir a string

`str()` convierte cualquier valor a su representación textual:

```halo
print(str(42))       // "42"
print(str(3.14))     // "3.14"
print(str(true))     // "true"
print(str([1, 2]))   // "[1, 2]"
```

`str()` sobre un string devuelve el mismo string.

## Formatear salida

No hay interpolación de strings ni `printf`. Concatena con `+` y convierte con
`str()`:

```halo
nombre = "Halo"
version = 0.3
print("Lenguaje: " + nombre + " v" + str(version))
```

## Limitaciones

- No hay indexación (`texto[0]`).
- No hay métodos con punto (`texto.upper()` no es válido); usa funciones.
- No hay `split()`, `format()`, `char_at()` ni expresiones regulares.
- No hay interpolación (`"Hola, ${nombre}"`).
- No hay recorrido directo con `for`; usa `substr()` y `range()`.
- `len()` cuenta bytes, no caracteres.

## Siguiente paso

- [Arrays](../arrays/).
- [Built-ins](../../referencia/builtins/).
- [Estilo](../estilo/).
