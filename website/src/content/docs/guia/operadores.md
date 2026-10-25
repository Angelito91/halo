---
title: Operadores
description: Aritmética, strings, comparaciones y lógica con cortocircuito en Halo.
---

Halo tiene un conjunto pequeño de operadores. Todos funcionan con valores
dinámicos y comprueban los tipos en runtime.

## Aritméticos

| Operador | Descripción | Ejemplo |
|---|---|---|
| `+` | Suma o concatenación | `3 + 4` → `7` |
| `-` | Resta o negación unaria | `10 - 3` → `7` |
| `*` | Multiplicación o repetición de string | `"ab" * 3` → `"ababab"` |
| `/` | División | `10 / 4` → `2` |
| `%` | Módulo (solo enteros) | `10 % 3` → `1` |

### División entera y flotante

Si ambos operandos son `number`, la división es entera y trunca hacia cero:

```halo
7 / 2       // 3
-7 / 2      // -3
```

Si al menos uno es `float`, el resultado es `float`:

```halo
7 / 2.0     // 3.5
7.0 / 2     // 3.5
```

### Promoción automática

Cualquier operación aritmética entre `number` y `float` produce `float`:

```halo
5 + 0.5     // 5.5
3 * 1.5     // 4.5
10 - 0.1    // 9.9
```

### Concatenación

Si uno de los operandos es un string, el otro se convierte a texto:

```halo
"valor: " + 42       // "valor: 42"
42 + " items"        // "42 items"
"ok: " + true        // "ok: true"
"hola" + " " + "mundo"   // "hola mundo"
```

El orden importa: `+` es conmutativo para números, pero no para strings.

### Repetición

```halo
"ab" * 3    // "ababab"
3 * "ab"    // "ababab"
"x" * 0     // ""
```

Repetir un número negativo de veces es un error de runtime.

### Negación unaria

```halo
-7          // -7
-(3 + 4)    // -7
- -5        // 5
```

### Overflow y división por cero

La aritmética entera es comprobada. Un desbordamiento no da la vuelta; falla:

```halo
9223372036854775807 + 1   // Error: Integer overflow in addition
```

La división y el módulo por cero son errores:

```halo
5 / 0       // Error: Division by zero
5 % 0       // Error: Division by zero
5.0 / 0.0   // Error: Division by zero
```

`%` solo acepta enteros; usarlo con floats es un error.

## Comparación

| Operador | Descripción |
|---|---|
| `==` | Igual |
| `!=` | Distinto |
| `<` | Menor que |
| `>` | Mayor que |
| `<=` | Menor o igual |
| `>=` | Mayor o igual |

`==` y `!=` devuelven `bool` y permiten comparar `number` con `float`:

```halo
5 == 5          // true
5 == 5.0        // true
5 != 6          // true
"abc" == "abc"  // true
true == true    // true
```

Comparar valores de tipos incompatibles con `==` devuelve `false` (no es un
error):

```halo
1 == "1"        // false
```

Los operadores de orden solo funcionan con números (incluyendo mezclas) y
strings:

```halo
5 < 5.5         // true
"abc" < "abd"   // true
"hola" >= "hola" // true
```

```halo
1 < "2"   // Error: Cannot compare number < string
```

La comparación de strings es lexicográfica por bytes UTF-8.

## Lógicos

| Operador | Alias | Descripción |
|---|---|---|
| `&&` | `and` | AND con cortocircuito |
| `\|\|` | `or` | OR con cortocircuito |
| `!` | `not` | NOT unario |

### Cortocircuito

`&&` evalúa el lado derecho solo si el izquierdo es truthy. `||` lo evalúa
solo si el izquierdo es falsy:

```halo
false && undefined_var   // false, sin error
true  || undefined_var   // true, sin error
```

### El resultado no siempre es bool

`&&` y `||` devuelven **el operando** que decide el resultado, como en
JavaScript o Python:

```halo
0 || "fallback"     // "fallback"
"hola" && 42        // 42
false && 99         // false
true || 99          // true
```

Esto es útil para valores por defecto:

```halo
nombre = "" || "anónimo"
print(nombre)   // anónimo
```

Si necesitas un booleano estricto, envuelve la expresión en `!` dos veces o
usa una comparación explícita.

### NOT

`!` siempre devuelve `bool` e invierte la truthiness:

```halo
!true       // false
!false      // true
!0          // true
!1          // false
!"hola"     // false
!""         // true
```

### Palabras clave alternativas

Puedes escribir `and`, `or` y `not` en lugar de `&&`, `||` y `!`. El lexer los
trata como los mismos operadores:

```halo
if activo and not bloqueado {
    print("adelante")
}
```

## Asignación compuesta

Halo **no** tiene `+=`, `-=`, `++` ni `--`. Escribe la asignación completa:

```halo
i = i + 1
total = total + valor
```

## Precedencia

La precedencia es la habitual: unarios, multiplicación/división/módulo,
suma/resta, comparaciones, igualdad, `&&`, `||`. Consulta la
[tabla completa](../../referencia/precedencia/) y usa paréntesis cuando dudes.

```halo
2 + 3 * 4           // 14
(2 + 3) * 4         // 20
true || false && false   // true
```
