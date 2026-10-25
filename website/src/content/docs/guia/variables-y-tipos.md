---
title: Variables y tipos
description: Tipado dinámico, tipos de dato, literales, truthiness y conversiones en Halo.
---

Halo es de **tipado dinámico**. Las variables se crean con una asignación y su
tipo se decide en runtime a partir del valor. No hay `var`, `let`, `const` ni
anotaciones de tipo.

## Crear y reasignar

```halo
x = 42
precio = 99.99
activo = true
nombre = "Halo"
```

Una variable puede cambiar de valor **y de tipo** en cualquier momento:

```halo
n = 1
n = 3.14        // ahora es float
n = "hola"      // ahora es string
n = [1, 2, 3]   // ahora es array
```

Si lees una variable que no existe, el programa falla:

```halo
print(x)   // Error: Undefined variable: 'x'
```

No hay declaraciones sin inicializar. Una variable existe desde la primera
asignación.

## Tipos

| Tipo | Descripción | Ejemplos |
|---|---|---|
| `number` | Entero de 64 bits con signo (`i64`) | `0`, `42`, `-10` |
| `float` | Flotante IEEE 754 de 64 bits (`f64`) | `3.14`, `-0.5`, `1.0` |
| `bool` | Booleano | `true`, `false` |
| `string` | Cadena UTF-8 | `"hola"`, `'mundo'`, `""` |
| `array` | Lista ordenada y heterogénea | `[1, "dos", true]`, `[]` |
| `null` | Ausencia de valor (retorno de funciones void) | — |

Puedes consultar el tipo de un valor con `type()`:

```halo
print(type(42))       // number
print(type(3.14))     // float
print(type(true))     // bool
print(type("hola"))   // string
print(type([1, 2]))   // array
```

`null` no tiene literal. Aparece cuando una función no devuelve nada:

```halo
nada() {
    x = 1
}

print(type(nada()))   // null
```

## Literales numéricos

```halo
n = 42        // number
n = -10       // number negativo
f = 3.14      // float
f = -0.5      // float negativo
f = 1.0       // float, aunque tenga parte decimal cero
```

No hay notación científica, sufijos (`42i64`, `1.5f`) ni literales
hexadecimales en la sintaxis actual.

La aritmética entera usa operaciones comprobadas: si un resultado se sale del
rango de `i64`, el programa falla en vez de dar la vuelta silenciosamente.

```halo
x = 9223372036854775807 + 1   // Error: Integer overflow in addition
```

## Literales de string

Puedes usar comillas dobles o simples:

```halo
a = "hola"
b = 'hola'
c = 'puedo usar "dobles" dentro'
d = "puedo usar 'simples' dentro"
```

Secuencias de escape:

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

Los strings son UTF-8. Puedes usar acentos y emojis directamente:

```halo
print("Hola, ñandú 🦄")
```

## Truthiness

Cualquier valor puede usarse como condición. Estas son las reglas:

| Valor | ¿Verdadero? |
|---|---|
| `true` | Sí |
| `false` | No |
| `0`, `0.0` | No |
| Cualquier otro número | Sí |
| `""` | No |
| Cualquier otro string | Sí |
| `[]` (array vacío) | No |
| Cualquier array con elementos | Sí |
| `null` | No |

```halo
if 42       { print("truthy") }   // se ejecuta
if 0        { print("falsy") }    // no se ejecuta
if "hola"   { print("truthy") }   // se ejecuta
if ""       { print("falsy") }    // no se ejecuta
if [1]      { print("truthy") }   // se ejecuta
if []       { print("falsy") }    // no se ejecuta
```

Esto permite escribir condiciones como `if nombre { … }` para comprobar que un
string no está vacío, o `if lista { … }` para comprobar que un array tiene
elementos.

## Conversiones

Las conversiones explícitas son funciones built-in:

```halo
print(str(42))       // "42"
print(int(3.9))      // 3
print(int("99"))     // 99
print(float(7))      // 7.0
print(float("2.5"))  // 2.5
```

`int(float)` trunca hacia cero; no redondea:

```halo
int(3.9)    // 3
int(-2.7)   // -2
```

`int(string)` y `float(string)` fallan si el texto no es numérico:

```halo
int("abc")    // Error: Cannot convert 'abc' to integer
float("xyz")  // Error: Cannot convert 'xyz' to number
```

Para la lista completa, consulta [built-ins](../../referencia/builtins/).

## Igualdad y comparación entre tipos

`==` y `!=` comparan valores del mismo tipo, y permiten mezclar `number` con
`float`:

```halo
5 == 5        // true
5 == 5.0      // true
5.0 == 5      // true
"a" == "a"    // true
true == false // false
```

Los operadores de orden (`<`, `>`, `<=`, `>=`) solo funcionan con números
(incluyendo mezclas `number`/`float`) y strings:

```halo
"abc" < "abd"   // true (orden lexicográfico)
5 < 5.5         // true
```

Comparar un número con un string es un error de runtime.

## Siguiente paso

- [Operadores](../operadores/).
- [Strings](../strings/).
- [Arrays](../arrays/).
