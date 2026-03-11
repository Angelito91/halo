# Halo — Referencia de Sintaxis (v0.2.0)

Halo es un lenguaje dinámico y minimalista diseñado con fines educativos.
Esta página describe todo lo que el lenguaje soporta actualmente.

---

## Tabla de Contenidos

1. [Estructura de un programa](#1-estructura-de-un-programa)
2. [Comentarios](#2-comentarios)
3. [Variables](#3-variables)
4. [Tipos de Dato](#4-tipos-de-dato)
5. [Operadores](#5-operadores)
6. [Condicionales](#6-condicionales)
7. [Bucles](#7-bucles)
8. [Funciones](#8-funciones)
9. [Scope y Variables](#9-scope-y-variables)
10. [Funciones Built-in](#10-funciones-built-in)
11. [Errores de Runtime](#11-errores-de-runtime)
12. [Precedencia de Operadores](#12-precedencia-de-operadores)
13. [Guía de Estilo](#13-guía-de-estilo)

---

## 1. Estructura de un Programa

Un programa Halo es una secuencia de **declaraciones de función** y
**sentencias globales** separadas por saltos de línea.
No hay semicolons, no hay función `main` obligatoria.

```halo
// Declaraciones de función (pueden aparecer en cualquier orden)
double(x) {
    return x * 2
}

// Sentencias globales
result = double(21)
print(result)           // 42
```

Las funciones se registran antes de ejecutar cualquier sentencia, así que
puedes llamar una función antes de que aparezca textualmente en el archivo:

```halo
print(square(7))        // 49  — llamada ANTES de la definición

square(n) {
    return n * n
}
```

---

## 2. Comentarios

Solo se admiten comentarios de línea. Empiezan con `//` y abarcan
hasta el final de la línea.

```halo
// Esto es un comentario completo
x = 5        // Comentario al final de una sentencia
```

> Los comentarios multi-línea (`/* … */`) están planificados para v0.4.0.

---

## 3. Variables

### Declaración y asignación

Las variables se crean con una simple asignación. No hay palabra clave
(`var`, `let`, `const`). El tipo se infiere del valor.

```halo
x = 42
precio = 99.99
activo = true
nombre = "Halo"
```

### Reasignación

Una variable puede cambiar de valor y de tipo en cualquier momento:

```halo
n = 1
n = 3.14        // ahora es float
n = "hola"      // ahora es string
```

### Variables no definidas

Leer una variable que no existe es un error de runtime:

```halo
print(x)        // Error: Undefined variable: 'x'
```

---

## 4. Tipos de Dato

| Tipo | Descripción | Ejemplos |
|------|-------------|---------|
| `number` | Entero de 64 bits con signo | `0`, `42`, `-10` |
| `float` | Flotante IEEE 754 de 64 bits | `3.14`, `-0.5`, `1.0` |
| `bool` | Booleano | `true`, `false` |
| `string` | Cadena UTF-8 | `"hola"`, `""` |
| `null` | Ausencia de valor (retorno de funciones void) | — |

### Veracidad (truthiness)

Cualquier valor puede usarse como condición:

| Valor | Truthy / Falsy |
|-------|---------------|
| `true` | truthy |
| `false` | falsy |
| `0` (number) | falsy |
| cualquier otro `number` | truthy |
| `0.0` (float) | falsy |
| cualquier otro `float` | truthy |
| `""` (string vacío) | falsy |
| cualquier otro `string` | truthy |
| `null` | falsy |

```halo
if 42       { print("truthy") }     // se ejecuta
if 0        { print("truthy") }     // no se ejecuta
if "hola"   { print("truthy") }     // se ejecuta
if ""       { print("truthy") }     // no se ejecuta
```

### Literales numéricos

```halo
n  = 42        // number (i64)
n  = -10       // number negativo
f  = 3.14      // float  (f64)
f  = -0.5      // float negativo
f  = 1.0       // float entero
```

### Literales de string

Las cadenas van entre comillas dobles y soportan las siguientes
secuencias de escape:

| Escape | Carácter |
|--------|---------|
| `\\` | Barra invertida `\` |
| `\"` | Comilla doble `"` |
| `\n` | Salto de línea |
| `\t` | Tabulador horizontal |
| `\r` | Retorno de carro |

```halo
s = "hola mundo"
s = "línea1\nlínea2"
s = "col1\tcol2"
s = "comilla: \""
s = "barra: \\"
```

---

## 5. Operadores

### 5.1 Aritméticos

| Operador | Descripción | Ejemplo |
|----------|-------------|---------|
| `+` | Suma / concatenación de strings | `3 + 4` → `7` |
| `-` | Resta / negación unaria | `10 - 3` → `7` |
| `*` | Multiplicación / repetición de string | `"ab" * 3` → `"ababab"` |
| `/` | División | `10 / 4` → `2` (entera) |
| `%` | Módulo | `10 % 3` → `1` |

#### División entera vs flotante

Cuando ambos operandos son `number`, la división es entera (trunca hacia cero):

```halo
x = 7 / 2       // 3  (no 3.5)
y = -7 / 2      // -3 (no -4)
```

Si al menos un operando es `float`, el resultado es `float`:

```halo
x = 7 / 2.0     // 3.5
y = 7.0 / 2     // 3.5
```

#### Promoción automática int → float

Cualquier operación aritmética entre `number` y `float` produce `float`:

```halo
a = 5 + 0.5     // 5.5  (float)
b = 3 * 1.5     // 4.5  (float)
c = 10 - 0.1    // 9.9  (float)
```

#### Concatenación de strings con `+`

Si uno de los operandos es un `string`, el otro se convierte a su
representación textual y se concatenan:

```halo
s = "valor: " + 42       // "valor: 42"
s = 42 + " items"        // "42 items"
s = "ok: " + true        // "ok: true"
s = "hola" + " " + "mundo"  // "hola mundo"
```

#### Repetición de string con `*`

```halo
s = "ab" * 3    // "ababab"
s = 3 * "ab"    // "ababab"  (el orden no importa)
s = "x" * 0     // ""
```

Multiplicar por un número negativo es un error de runtime.

#### Negación unaria

```halo
x = -7          // -7
y = -(3 + 4)    // -7
z = - -5        // 5
```

#### Protección contra overflow

La aritmética entera usa operaciones verificadas. Desbordarse es un error
de runtime, no un wrap silencioso:

```halo
x = 9223372036854775807 + 1   // Error: overflow
```

#### División por cero

```halo
x = 5 / 0      // Error: Division by zero
x = 5 % 0      // Error: Division by zero
x = 5.0 / 0.0  // Error: Division by zero
```

---

### 5.2 Comparación

Devuelven `bool`. Pueden comparar valores del mismo tipo o `number` con `float`.

| Operador | Descripción |
|----------|-------------|
| `==` | Igual |
| `!=` | Distinto |
| `<` | Menor que |
| `>` | Mayor que |
| `<=` | Menor o igual |
| `>=` | Mayor o igual |

```halo
5 == 5          // true
5 == 5.0        // true  (number y float se comparan por valor)
"abc" < "abd"   // true  (comparación lexicográfica de strings)
true == true    // true
```

---

### 5.3 Lógicos

| Operador | Descripción |
|----------|-------------|
| `&&` | AND con cortocircuito |
| `\|\|` | OR con cortocircuito |
| `!` | NOT unario |

#### Cortocircuito

- `false && expr` — `expr` **no** se evalúa.
- `true || expr` — `expr` **no** se evalúa.

```halo
// Si x < 0 es falso, y > 10 no se evalúa
if x >= 0 && y > 10 {
    print("ambos")
}

// Si a es truthy, b no se evalúa
result = true || undefined_var   // no lanza error
```

#### NOT

`!` invierte la veracidad del valor:

```halo
!true           // false
!false          // true
!0              // true
!1              // false
!"hola"         // false
!""             // true
```

---

## 6. Condicionales

### if / else

Los bloques `{ }` son obligatorios.

```halo
if condicion {
    // rama verdadera
}

if condicion {
    // si verdadero
} else {
    // si falso
}
```

### else if

Se pueden encadenar tantas ramas `else if` como necesites.
Solo se ejecuta la **primera** rama cuya condición sea verdadera.

```halo
if x < 0 {
    print("negativo")
} else if x == 0 {
    print("cero")
} else if x < 10 {
    print("pequeño positivo")
} else {
    print("grande positivo")
}
```

### Condicionales anidados

```halo
if a {
    if b {
        print("a y b")
    } else {
        print("solo a")
    }
} else {
    print("ni a")
}
```

### Condicional como expresión en funciones

```halo
sign(n) {
    if n > 0  { return 1  }
    if n == 0 { return 0  }
    return -1
}
```

---

## 7. Bucles

### while

```halo
while condicion {
    // cuerpo
}
```

El cuerpo se ejecuta mientras `condicion` sea truthy. Si la condición
es falsa desde el principio, el cuerpo no se ejecuta nunca.

```halo
i = 0
while i < 5 {
    print(i)
    i = i + 1
}
// 0 1 2 3 4
```

### break

Sale del bucle más cercano inmediatamente.

```halo
i = 0
while i < 100 {
    if i == 5 { break }
    i = i + 1
}
print(i)    // 5
```

`break` solo sale del bucle **más interno**:

```halo
outer = 0
while outer < 3 {
    i = 0
    while i < 10 {
        if i == 2 { break }   // sale del while interno
        i = i + 1
    }
    outer = outer + 1
}
```

### continue

Salta el resto del cuerpo y vuelve a evaluar la condición.

```halo
i = 0
while i < 10 {
    i = i + 1
    if i % 2 == 0 { continue }   // salta los pares
    print(i)                      // 1 3 5 7 9
}
```

### Límite de iteraciones

Para proteger contra bucles infinitos, el intérprete lanza un error
después de **1 000 000** iteraciones:

```halo
while true { }   // Error: Loop iteration limit exceeded (1000000)
```

### Bucles anidados

```halo
i = 0
while i < 3 {
    j = 0
    while j < 3 {
        print(str(i) + "," + str(j))
        j = j + 1
    }
    i = i + 1
}
```

---

## 8. Funciones

### Definición

No se usa ninguna palabra clave. La sintaxis es:

```
nombre(param1, param2, …) {
    // cuerpo
}
```

```halo
saludar() {
    print("Hola!")
}

sumar(a, b) {
    return a + b
}

max(a, b) {
    if a >= b { return a }
    return b
}
```

### Llamada

```halo
saludar()
resultado = sumar(3, 4)
print(max(10, 20))
```

### Retorno de valor

`return` con un valor sale de la función y produce ese valor:

```halo
double(x) {
    return x * 2
}
print(double(21))   // 42
```

### Retorno sin valor (`return` vacío)

`return` sin expresión sale de la función inmediatamente y produce `null`:

```halo
setup(flag) {
    if flag { print("configurado") }
    return
    print("esto nunca se ejecuta")
}
```

### Funciones void

Una función sin `return` explícito produce `null` al terminar:

```halo
nothing() {
    x = 1   // variable local, no visible fuera
}
result = nothing()
print(result)   // null
```

### Aridad

El número de argumentos debe coincidir exactamente con el número de parámetros.
Pasar más o menos es un error de runtime:

```halo
add(a, b) { return a + b }
add(1)          // Error: Function 'add' expects 2 argument(s), got 1
add(1, 2, 3)    // Error: Function 'add' expects 2 argument(s), got 3
```

### Call-before-definition

Puedes llamar una función antes de su definición textual porque el intérprete
registra todas las funciones en un primer pase antes de ejecutar:

```halo
result = triple(7)          // llamada antes de la definición
triple(n) { return n * 3 }
print(result)               // 21
```

### Recursión

Halo soporta recursión directa y mutua:

```halo
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
print(factorial(10))   // 3628800

fib(n) {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}
print(fib(10))         // 55
```

#### Recursión mutua

```halo
is_even(n) {
    if n == 0 { return true }
    return is_odd(n - 1)
}
is_odd(n) {
    if n == 0 { return false }
    return is_even(n - 1)
}
print(is_even(8))   // true
print(is_odd(7))    // true
```

#### Límite de profundidad de recursión

El intérprete lanza un error antes de que el stack nativo de Rust se desborde:

```halo
infinite(n) { return infinite(n + 1) }
infinite(0)   // Error: Maximum recursion depth exceeded (500)
```

### Funciones como argumentos (resultados)

El resultado de una llamada puede usarse directamente en expresiones:

```halo
inc(n) { return n + 1 }
square(n) { return n * n }

print(square(inc(3)))           // square(4) = 16
print(square(inc(3)) + square(inc(4)))   // 16 + 25 = 41
```

---

## 9. Scope y Variables

### Scope global

Las variables asignadas en el nivel superior del programa viven en el
scope global y son visibles desde cualquier función:

```halo
base = 100

add_base(n) {
    return n + base    // lee `base` del scope global
}
print(add_base(5))     // 105
```

### Scope de función

Cada llamada a función crea un **frame independiente**. Las variables
asignadas dentro de la función son locales y **no son visibles** en el
scope del caller una vez que la función retorna.

```halo
set_local() {
    secret = 99    // variable local
}
set_local()
print(secret)      // Error: Undefined variable: 'secret'
```

### Shadowing

Un parámetro o variable local puede tener el mismo nombre que una
variable global. Dentro de la función se usa la local; el global no
se modifica:

```halo
x = 10

shadow() {
    x = 99      // local — no toca el global
    return x
}

print(shadow())  // 99
print(x)         // 10  ← global intacto
```

### Parámetros como copia

Los parámetros se pasan **por valor**. Modificarlos dentro de la función
no afecta a la variable del caller:

```halo
double_in_place(x) {
    x = x * 2
    return x
}
original = 7
result = double_in_place(original)
print(original)  // 7  ← sin cambios
print(result)    // 14
```

### Variables en if y while

Los bloques `if` y `while` **no crean un scope nuevo**. Las variables
asignadas dentro son visibles después del bloque:

```halo
if true {
    resultado = 42
}
print(resultado)    // 42

i = 0
while i < 3 { i = i + 1 }
print(i)            // 3  — i sigue visible
```

---

## 10. Funciones Built-in

Todas las funciones built-in son de **aridad fija 1** (salvo `print`).
Pasar un número distinto de argumentos es un error de runtime.

### `print(x)`

Imprime el valor en stdout seguido de un salto de línea. Acepta cualquier
tipo y cualquier número de argumentos (incluyendo cero).

```halo
print(42)           // 42
print(3.14)         // 3.14
print(true)         // true
print("hola")       // hola
print()             // (línea vacía)
```

### `len(x)`

Devuelve la longitud como `number`:
- Para `string`: número de caracteres.
- Para `number` o `float`: número de dígitos en su representación textual.

```halo
len("hola")         // 4
len("")             // 0
len("hola mundo")   // 10
len(12345)          // 5
len(-99)            // 3  (incluye el signo)
```

### `str(x)`

Convierte cualquier valor a su representación como `string`:

```halo
str(42)             // "42"
str(-7)             // "-7"
str(3.14)           // "3.14"
str(2.0)            // "2.0"
str(true)           // "true"
str(false)          // "false"
str("hola")         // "hola"  (identidad)
```

### `int(x)`

Convierte a `number` (entero):
- `float` → trunca hacia cero.
- `bool` → `true` = 1, `false` = 0.
- `string` → parsea como entero; falla si no es numérico.
- `number` → identidad.

```halo
int(3.9)            // 3
int(-2.7)           // -2
int(true)           // 1
int(false)          // 0
int("99")           // 99
int("-5")           // -5
int("abc")          // Error: Cannot convert "abc" to integer
```

### `float(x)`

Convierte a `float`:
- `number` → el mismo valor como float.
- `bool` → `true` = 1.0, `false` = 0.0.
- `string` → parsea como flotante; falla si no es numérico.
- `float` → identidad.

```halo
float(7)            // 7.0
float(true)         // 1.0
float("2.5")        // 2.5
float("10")         // 10.0
float("xyz")        // Error: Cannot convert "xyz" to float
```

### `abs(x)`

Valor absoluto. Acepta `number` y `float`:

```halo
abs(7)              // 7
abs(-7)             // 7
abs(0)              // 0
abs(-3.14)          // 3.14
abs(true)           // Error: abs does not support type 'bool'
```

### `type(x)`

Devuelve el nombre del tipo como `string`:

```halo
type(42)            // "number"
type(3.14)          // "float"
type(true)          // "bool"
type("hola")        // "string"
type(0)             // "number"
```

---

## 11. Errores de Runtime

El intérprete lanza errores descriptivos en lugar de comportamientos
indefinidos. A continuación se listan los principales:

| Error | Causa |
|-------|-------|
| `Undefined variable: 'x'` | Leer una variable no declarada |
| `Undefined function: 'f'` | Llamar a una función no definida |
| `Function 'f' expects N argument(s), got M` | Aridad incorrecta |
| `Division by zero` | División o módulo por cero |
| `overflow` | Desbordamiento de aritmética entera |
| `Cannot negate a value of type 'T'` | Negación unaria sobre tipo inválido |
| `Maximum recursion depth exceeded (N)` | Recursión demasiado profunda |
| `Loop iteration limit exceeded (N)` | Bucle con demasiadas iteraciones |
| `Cannot convert "x" to integer` | `int()` sobre string no numérico |
| `Cannot convert "x" to float` | `float()` sobre string no numérico |
| `abs does not support type 'T'` | `abs()` sobre tipo no numérico |
| `negative` | Repetición de string con entero negativo |

---

## 12. Precedencia de Operadores

De mayor a menor prioridad (los de mayor prioridad se evalúan primero):

| Nivel | Operadores | Asociatividad |
|-------|-----------|---------------|
| 7 (más alto) | `-x`, `!x` (unarios) | Derecha |
| 6 | `*`, `/`, `%` | Izquierda |
| 5 | `+`, `-` | Izquierda |
| 4 | `<`, `>`, `<=`, `>=` | Izquierda |
| 3 | `==`, `!=` | Izquierda |
| 2 | `&&` | Izquierda (cortocircuito) |
| 1 (más bajo) | `\|\|` | Izquierda (cortocircuito) |

Los paréntesis anulan la precedencia:

```halo
2 + 3 * 4           // 14  (* antes que +)
(2 + 3) * 4         // 20
10 - 2 * 3          // 4
2 + 3 * 4 - 6 / 2   // 11  (2 + 12 - 3)
10 + 7 % 3          // 11  (% antes que +)

true || false && false   // true  (&& antes que ||)
!true && false           // false (!true = false, false && false = false)
```

---

## 13. Guía de Estilo

Estas son las convenciones que usa el proyecto y que se recomiendan:

### Indentación

Usa **4 espacios** por nivel de indentación. No uses tabs.

```halo
factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
```

### Nombres

- **Variables y funciones:** `snake_case`
- **Constantes globales:** `UPPER_SNAKE_CASE` (convención, no impuesta)

```halo
MAX_ITER = 1000
total_sum = 0

compute_sum(n) {
    result = 0
    i = 1
    while i <= n {
        result = result + i
        i = i + 1
    }
    return result
}
```

### Un statement por línea

Halo usa el salto de línea como terminador de sentencia. Pon siempre
una sola sentencia por línea.

```halo
// ✅ Correcto
x = 1
y = 2

// ❌ No soportado (causa error de parse)
x = 1 y = 2
```

### Llaves en la misma línea

El `{` de apertura va siempre al final de la misma línea que la
declaración:

```halo
// ✅
if x > 0 {
    print(x)
}

// ✅
compute(n) {
    return n * 2
}
```

### Comentarios

Comenta el *por qué*, no el *qué*:

```halo
// Gauss: suma de 1..n = n*(n+1)/2
sum_to(n) {
    return n * (n + 1) / 2
}
```

### Evita el anidamiento profundo

Si tienes más de 3 niveles de anidamiento, considera extraer funciones:

```halo
// ✅ Más legible
in_range(n, lo, hi) {
    return n >= lo && n <= hi
}

process(n) {
    if !in_range(n, 1, 100) { return 0 }
    // ...
    return n
}
```

---

> Para la referencia del proyecto completo ve a [README.md](README.md).