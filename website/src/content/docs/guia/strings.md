---
title: Strings
description: Literales, escapes, concatenación, comparación y métodos de string en Halo.
---

Los strings son secuencias UTF-8 inmutables. Se pueden crear con comillas
dobles o simples y tienen un conjunto de métodos para transformarlos.

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

## Métodos de string

Igual que con los arrays, hay dos formas: función y método. Los métodos no
mutan el string; devuelven uno nuevo.

| Operación | Función | Método | Ejemplo |
|---|---|---|---|
| Mayúsculas | `upper(s)` | `s.upper()` | `"hola".upper()` → `HOLA` |
| Minúsculas | `lower(s)` | `s.lower()` | `"HOLA".lower()` → `hola` |
| Quitar espacios | `trim(s)` | `s.trim()` | `"  hola  ".trim()` → `hola` |
| Prefijo | `starts_with(s, p)` | `s.starts_with(p)` | `"hola".starts_with("ho")` → `true` |
| Sufijo | `ends_with(s, p)` | `s.ends_with(p)` | `"hola".ends_with("la")` → `true` |
| Contiene | `contains(s, p)` | `s.contains(p)` | `"hola".contains("ol")` → `true` |
| Índice | `index_of(s, p)` | `s.index_of(p)` | `"hola".index_of("l")` → `2` |
| Reemplazar | `replace(s, a, b)` | `s.replace(a, b)` | `"a-b".replace("-", "_")` → `a_b` |
| Substring | `substr(s, i, n)` | `s.substr(i, n)` | `"hola".substr(1, 2)` → `ol` |
| Invertir | `reverse(s)` | `s.reverse()` | `"abc".reverse()` → `cba` |

### `upper` / `lower`

```halo
print("Hola".upper())   // HOLA
print("HOLA".lower())   // hola
```

### `trim`

Elimina espacios, tabs y saltos de línea al principio y al final:

```halo
print("  hola  ".trim())   // hola
```

### `starts_with` / `ends_with`

```halo
print("programa.halo".ends_with(".halo"))   // true
print("https://".starts_with("http"))       // true
```

### `contains`

```halo
print("hola mundo".contains("mundo"))   // true
print("hola".contains(""))              // true
```

### `index_of`

Devuelve el índice de la primera aparición o `-1`:

```halo
print("hola".index_of("l"))    // 2
print("hola".index_of("z"))    // -1
```

El índice es un **offset de bytes**. Con texto no ASCII puede no coincidir con
la posición visual:

```halo
print("añb".index_of("b"))   // 3
```

### `replace`

Reemplaza **todas** las apariciones:

```halo
print("a-b-a".replace("a", "x"))   // x-b-x
```

### `substr`

`substr(texto, inicio, longitud)` trabaja con caracteres:

```halo
print("hola mundo".substr(0, 4))   // hola
print("hola mundo".substr(5, 5))   // mundo
print("hola".substr(1, 2))         // ol
```

Si `inicio` está fuera del string, devuelve `""`. Si la longitud se pasa del
final, se recorta:

```halo
print("hola".substr(99, 3))   // (vacío)
print("hola".substr(2, 99))   // la
```

Un inicio o una longitud negativos son errores.

### `reverse`

```halo
print("abc".reverse())   // cba
print("añb".reverse())   // bña
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
- No hay `split()`, `format()`, `char_at()` ni expresiones regulares.
- No hay interpolación (`"Hola, ${nombre}"`).
- No hay recorrido directo con `for`; usa `substr()` y `range()`.
- `len()` cuenta bytes, no caracteres.

## Siguiente paso

- [Arrays](../arrays/).
- [Built-ins](../../referencia/builtins/).
- [Estilo](../estilo/).
