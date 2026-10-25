---
title: Guía de estilo
description: Convenciones de formato y nombres usadas en el proyecto Halo.
---

Estas son las convenciones que usa Halo en sus ejemplos y en su propio código.
No las impone el intérprete, pero ayudan a que todos los programas se lean
igual.

## Indentación

**4 espacios** por nivel. No uses tabs.

```halo
factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
```

## Nombres

- Variables y funciones: `snake_case`.
- Constantes globales: `UPPER_SNAKE_CASE` (convención, no impuesta).

```halo
MAX_INTENTOS = 1000
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

## Una sentencia por línea

Halo no usa `;`; la convención es una sentencia por línea:

```halo
// ✅
x = 1
y = 2

// ❌ No recomendado
x = 1 y = 2
```

El parser actual puede aceptar dos sentencias en la misma línea, pero rompe el
estilo y la legibilidad.

La excepción son los bloques de una línea:

```halo
if n <= 1 { return 1 }
```

## Llaves en la misma línea

El `{` va al final de la línea de la declaración:

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

```halo
// ❌ El parser no lo acepta
if x > 0
{
    print(x)
}
```

## Espacios

- Un espacio alrededor de los operadores binarios: `a + b`, no `a+b`.
- Sin espacio antes de `(` en llamadas: `sumar(1, 2)`.
- Un espacio después de las comas: `[1, 2, 3]`.
- Sin espacio dentro de los corchetes: `arr[0]`, no `arr[ 0 ]`.

```halo
resultado = (a + b) * 2
valores = [1, 2, 3]
print(valores[0])
```

## Comentarios

Comenta el **por qué**, no el qué. El código ya dice qué hace.

```halo
// Gauss: suma de 1..n = n*(n+1)/2
sum_to(n) {
    return n * (n + 1) / 2
}
```

Evita comentarios obvios:

```halo
// ❌
i = i + 1   // incrementa i

// ✅
i = i + 1   // avanza al siguiente índice
```

## Evita el anidamiento profundo

Si tienes más de tres niveles, extrae funciones o usa retornos tempranos:

```halo
in_range(n, lo, hi) {
    return n >= lo && n <= hi
}

process(n) {
    if !in_range(n, 1, 100) { return 0 }
    return n * 2
}
```

## Funciones pequeñas

Una función debería hacer una cosa. Si necesitas un comentario para separar
secciones, probablemente son dos funciones.

```halo
// ✅
es_par(n) {
    return n % 2 == 0
}

saludar_si_par(n) {
    if es_par(n) { print("par") }
}
```

## Constantes

Halo no tiene `const`, pero puedes usar una convención:

```halo
PI = 3.14159
MAX_REINTENTOS = 3
```

No las reasignes. Si necesitas un valor calculado, usa una función.

## Formato de strings largos

Como no hay interpolación, divide las concatenaciones en varias líneas o usa
variables intermedias:

```halo
nombre = "Halo"
version = "0.3"
mensaje = "Lenguaje: " + nombre + " v" + version
print(mensaje)
```

## Orden de un archivo

1. Constantes.
2. Funciones auxiliares.
3. Funciones principales.
4. Sentencias de arranque.

## Siguiente paso

- [Estructura](../estructura/).
- [Funciones](../funciones/).
- [Referencia](../../referencia/builtins/).
