---
title: Errores de runtime
description: Catálogo de errores del intérprete, sus causas y cómo depurarlos.
---

Halo intenta fallar con mensajes explícitos en lugar de producir
comportamientos indefinidos. La mayoría de los errores son de **runtime**: el
programa parsea bien, pero una operación no es válida con los valores que
recibe.

:::note
Los errores de runtime incluyen la posición de la sentencia que falló y, cuando
aplica, una sugerencia de nombre parecido. Para localizar el problema también
puedes combinar `print()` con `halo check`, `halo tokens` y `halo ast`.
:::

## Errores de sintaxis

El lexer y el parser fallan antes de ejecutar nada. Algunos mensajes típicos:

| Mensaje | Causa |
|---|---|
| `Expected '}' after if body` | Falta cerrar una llave de un `if` |
| `Expected ')' after arguments` | Falta cerrar un paréntesis |
| `Expected ']' after array index` | Falta cerrar un corchete |
| `Expected '{' after if condition` | El `{` no está en la misma línea o falta |
| `Expected '{' before function body` | Falta la llave de una función |
| `Expected '=' in global variable declaration` | Asignación incompleta en el nivel superior |
| `Expected an expression` | Operador sin operando o salto de línea dentro de `[...]` |
| `Expected a function definition, global variable, or statement` | Token inesperado al inicio de una declaración |

Ejemplos:

```halo
if true
{
    print("hola")
}
// Error: el parser espera `{` en la misma línea
```

```halo
if true {
    print("hola")
// Error: Expected '}' after if body
```

Ejecuta `halo check archivo.halo` para validar sin ejecutar.

## Variables y funciones

| Error | Causa |
|---|---|
| `Undefined variable: 'x'` | Leer una variable que no existe |
| `Undefined function: 'f'` | Llamar a una función no definida |
| `Function 'f' expects N argument(s), got M` | Aridad incorrecta |
| `Infinite recursion detected in function 'f' — maximum depth 500 exceeded` | Recursión demasiado profunda |

```halo
print(x)          // Undefined variable: 'x'
foo()             // Undefined function: 'foo'

add(a, b) { return a + b }
add(1)            // Function 'add' expects 2 argument(s), got 1
```

## Aritmética

| Error | Causa |
|---|---|
| `Division by zero` | `/` o `%` con divisor cero |
| `Integer overflow in addition` | Suma fuera del rango de `i64` |
| `Integer overflow in subtraction` | Resta fuera del rango de `i64` |
| `Integer overflow in multiplication` | Multiplicación fuera del rango de `i64` |
| `Cannot negate a value of type 'T'` | `-` sobre algo que no es número |
| `Cannot repeat string negative times` | `"x" * -1` |
| `Modulo only works with integers` | `%` con floats |

```halo
5 / 0        // Division by zero
5 % 0        // Division by zero
9223372036854775807 + 1
// Integer overflow in addition
"x" * -1     // Cannot repeat string negative times
```

## Conversión de tipos

| Error | Causa |
|---|---|
| `Cannot convert 'x' to integer` | `int(string)` no numérico |
| `Cannot convert 'x' to number` | `float(string)` no numérico |
| `abs() does not support type 'T'` | `abs()` sobre un tipo no numérico |
| `len() does not support type 'T'` | `len()` sobre bool, float o null |

```halo
int("abc")   // Cannot convert 'abc' to integer
float("x")   // Cannot convert 'x' to number
abs(true)    // abs() does not support type 'bool'
len(3.14)    // len() does not support type 'float'
```

## Comparaciones y operaciones inválidas

| Error | Causa |
|---|---|
| `Cannot compare number < string` | Orden entre tipos incompatibles |
| `Cannot add bool and number` | `+` entre tipos incompatibles |
| `Cannot subtract ...` | Resta con tipos no numéricos |
| `Cannot multiply ...` | Multiplicación inválida |

```halo
1 < "2"      // Cannot compare number < string
true + 1     // Cannot add bool and number
true - 1     // Cannot subtract number from bool
```

Recuerda que `==` y `!=` entre tipos incompatibles **no** fallan: devuelven
`false`.

## Bucles

| Error | Causa |
|---|---|
| `for expects an array to iterate, got 'T'` | `for` sobre algo que no es array |
| `Loop iteration limit exceeded (1000000)` | Bucle infinito o demasiado largo |
| `range() expects 1 or 2 arguments, got N` | Aridad incorrecta |
| `range() expects integer arguments, got 'T'` | Límites no enteros |
| `range(a, b) produces N elements, above the limit of 1000000` | Rango demasiado grande |

```halo
for c in "hola" { }   // for expects an array to iterate, got 'string'
while true { }        // Loop iteration limit exceeded (1000000)
range(1.5)            // range() expects integer arguments, got 'float'
```

## Arrays

| Error | Causa |
|---|---|
| `Array index cannot be negative, got N` | Índice negativo |
| `Array index N out of bounds (length M)` | Índice fuera del array |
| `Array index must be a number, got 'T'` | Índice no numérico |
| `Cannot index a value of type 'T'` | Indexar algo que no es array |
| `Slice start index N out of bounds for array of length M` | Inicio de slice inválido |
| `Slice end index N out of bounds for array of length M` | Fin de slice inválido |
| `Slice start index N cannot be greater than end index M` | Rango invertido |
| `sort() cannot mix strings and numbers in the same array` | Mezcla en `sort()` |

```halo
arr = [1, 2, 3]
arr[-1]        // Array index cannot be negative, got -1
arr[10]        // Array index 10 out of bounds (length 3)
arr[0:9]       // Slice end index 9 out of bounds for array of length 3
sort([1, "a"]) // sort() cannot mix strings and numbers in the same array
```

## Strings

| Error | Causa |
|---|---|
| `substr() start index cannot be negative, got N` | Inicio negativo |
| `substr() length cannot be negative, got N` | Longitud negativa |
| `join() expects second argument to be a string, got 'T'` | Separador no string |

`substr()` con un inicio mayor que la longitud **no** falla: devuelve `""`.

## Métodos

| Error | Causa |
|---|---|
| `Array method push() expects 1 argument, got N` | Aridad del método |
| `String method upper() expects 0 arguments, got N` | Aridad del método |
| `Unknown method 'push' for type 'number'` | Método de array sobre otro tipo |
| `Unknown method 'upper' for type 'number'` | Método de string sobre otro tipo |

```halo
print([1, 2].push())        // Array method push() expects 1 argument, got 0
print("hola".upper("x"))    // String method upper() expects 0 arguments, got 1
```

## Cómo depurar

1. **Aísla**: reduce el programa a la expresión mínima que falla.
2. **Imprime tipos**: `print(type(x))` antes de la operación sospechosa.
3. **Valida sintaxis**: `halo check archivo.halo`.
4. **Inspecciona tokens**: `halo tokens archivo.halo`.
5. **Inspecciona el AST**: `halo ast archivo.halo`.
6. **Reproduce el fallo**: ejecuta el caso mínimo con `halo run`.

```halo
valor = "42"
print(type(valor))   // string
print(int(valor))    // 42
```

## Códigos de salida

| Código | Significado |
|---|---|
| `0` | Éxito |
| `1` | Error de lexer, parser o runtime |

Los errores se escriben en `stderr`, así que puedes separar la salida normal:

```bash
halo run programa.halo > salida.txt 2> errores.txt
```

## Siguiente paso

- [Built-ins](../builtins/).
- [Precedencia](../precedencia/).
- [CLI](../../empezando/cli/).
