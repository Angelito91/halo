---
title: Condicionales
description: if, else if y else en Halo, con truthiness, anidamiento y ejemplos prácticos.
---

Halo tiene una única construcción condicional: `if`. No hay `switch`,
`match` ni operador ternario.

## `if`

```halo
if condicion {
    // se ejecuta si la condición es truthy
}
```

La condición puede ser cualquier valor; se aplican las reglas de
[truthiness](../variables-y-tipos/#truthiness). No hace falta que sea `bool`.

```halo
if 42       { print("truthy") }   // se ejecuta
if 0        { print("no") }       // no se ejecuta
if nombre   { print("hay nombre") }
if lista    { print("hay elementos") }
```

## `if / else`

```halo
if x > 0 {
    print("positivo")
} else {
    print("no positivo")
}
```

## `else if`

Se escribe `else if`, no `elif`:

```halo
if nota >= 90 {
    print("A")
} else if nota >= 80 {
    print("B")
} else if nota >= 70 {
    print("C")
} else {
    print("F")
}
```

Las ramas se evalúan en orden. La primera condición verdadera gana; el resto
no se evalúa.

## Anidamiento

```halo
if usuario {
    if admin {
        print("panel de administración")
    } else {
        print("panel de usuario")
    }
} else {
    print("inicia sesión")
}
```

Puedes anidar tanto como quieras, pero si llegas a tres niveles suele ser
mejor extraer una función o usar retornos tempranos.

## Retornos tempranos

En una función, el patrón más legible para validar es salir pronto:

```halo
procesar(pedido) {
    if !pedido {
        return "sin pedido"
    }
    if pedido == "cancelado" {
        return "cancelado"
    }
    return "procesado"
}
```

## Bloques de una línea

Puedes escribir el cuerpo en la misma línea si cabe:

```halo
if n <= 1 { return 1 }
if debug { print("modo debug") }
```

No pongas la llave en una línea distinta:

```halo
// ❌ Error de parse
if x > 0
{
    print("positivo")
}

// ✅
if x > 0 {
    print("positivo")
}
```

## Condicionales dentro de expresiones

`if` es una sentencia, no una expresión. No puedes escribir:

```halo
// ❌ No existe el operador ternario
estado = if activo { "on" } else { "off" }
```

Usa una función:

```halo
estado(activo) {
    if activo { return "on" }
    return "off"
}

print(estado(true))   // on
```

## Ejemplo: FizzBuzz

```halo
i = 1
while i <= 20 {
    if i % 15 == 0 {
        print("FizzBuzz")
    } else if i % 3 == 0 {
        print("Fizz")
    } else if i % 5 == 0 {
        print("Buzz")
    } else {
        print(i)
    }
    i = i + 1
}
```

## Errores comunes

### Olvidar las llaves

```halo
if x > 0
    print("positivo")   // ❌ el parser espera `{`
```

### Usar `=` en lugar de `==`

```halo
if x = 5 { ... }   // ❌ no es una comparación
```

Halo no permite asignaciones dentro de condiciones; el parser espera una
expresión y `=` no forma parte de ella.

### Confundir `else if` con `elif`

```halo
// ❌ `elif` no existe
if a { ... } elif b { ... }

// ✅
if a { ... } else if b { ... }
```

## Siguiente paso

- [Bucles](../bucles/).
- [Funciones](../funciones/).
- [Precedencia](../../referencia/precedencia/).
