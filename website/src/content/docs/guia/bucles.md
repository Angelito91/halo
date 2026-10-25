---
title: Bucles
description: while, for … in, range, break y continue, con sus límites y ejemplos.
---

Halo tiene dos bucles: `while` y `for … in`. No hay `do … while`, `loop` ni
iteradores personalizados.

## `while`

```halo
while condicion {
    // cuerpo
}
```

El cuerpo se repite mientras la condición sea truthy. Si es falsa desde el
principio, no se ejecuta nunca.

```halo
i = 0
while i < 5 {
    print(i)
    i = i + 1
}
// 0 1 2 3 4
```

La condición se evalúa **antes** de cada iteración, así que acuérdate de
modificar algo para que el bucle termine.

## `for … in`

```halo
for variable in iterable {
    // cuerpo
}
```

El iterable se evalúa una sola vez, antes de la primera iteración. Hoy puede
ser un array o el resultado de `range()`:

```halo
for i in range(5) {
    print(i)
}
// 0 1 2 3 4
```

```halo
nombres = ["Ana", "Luis", "Marta"]
for nombre in nombres {
    print("Hola, " + nombre)
}
```

## `range`

```halo
range(end)          // [0, 1, …, end-1]
range(start, end)   // [start, …, end-1]
```

```halo
range(4)       // [0, 1, 2, 3]
range(2, 5)    // [2, 3, 4]
range(3, 3)    // []
range(5, 2)    // []
```

`range()` devuelve un array de enteros. No hay paso negativo; si `end <= start`
el resultado es un array vacío. Los límites deben ser enteros:

```halo
range(1.5)   // Error: range() expects integer arguments, got 'float'
```

Para evitar quedarte sin memoria, `range()` rechaza rangos de más de
1 000 000 de elementos.

## `break`

Sale inmediatamente del bucle más interno:

```halo
i = 0
while i < 100 {
    if i == 5 { break }
    i = i + 1
}
print(i)   // 5
```

```halo
for i in range(10) {
    if i == 2 { break }
    print(i)   // 0 1
}
```

En bucles anidados, `break` solo afecta al bucle en el que aparece:

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

## `continue`

Salta el resto del cuerpo y pasa a la siguiente iteración:

```halo
i = 0
while i < 10 {
    i = i + 1
    if i % 2 == 0 { continue }
    print(i)   // 1 3 5 7 9
}
```

En un `for`, `continue` pasa al siguiente elemento del iterable.

## Scope del `for`

La variable del bucle vive en un scope propio. Oculta a una variable externa
con el mismo nombre y desaparece al terminar:

```halo
i = 99
for i in range(2) {
    print(i)   // 0 1
}
print(i)       // 99
```

Las asignaciones a variables ya existentes **sí** modifican la de fuera, así
que los acumuladores funcionan:

```halo
total = 0
for i in range(4) {
    total = total + i
}
print(total)   // 6
```

Consulta [scope](../scope/) para los detalles.

## Límite de iteraciones

El intérprete protege contra bucles infinitos. Tras **1 000 000** de
iteraciones, lanza un error:

```halo
while true {
    // ...
}
// Error: Loop iteration limit exceeded (1000000)
```

Es un límite por ejecución del programa: todos los bucles comparten el mismo
contador, que se reinicia al volver a llamar a `eval_program`. No es una
característica del lenguaje que debas usar en producción.

## Anidamiento

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

Salida:

```text
0,0
0,1
0,2
1,0
...
```

## Patrones útiles

### Contar hacia atrás

```halo
i = 10
while i > 0 {
    print(i)
    i = i - 1
}
print("despegue")
```

### Acumular

```halo
suma = 0
for n in range(1, 101) {
    suma = suma + n
}
print(suma)   // 5050
```

### Buscar

```halo
encontrado = false
i = 0
while i < len(lista) && !encontrado {
    if lista[i] == objetivo {
        encontrado = true
    }
    i = i + 1
}
```

### Iterar con índice

```halo
for i in range(len(items)) {
    print(str(i) + ": " + items[i])
}
```

## Errores comunes

### Bucle infinito

Si la condición nunca cambia, el intérprete corta a las 1 000 000 de
iteraciones. Revisa que actualizas la variable de control.

### `for` sobre un string

Los strings no son iterables:

```halo
for c in "hola" { print(c) }   // Error: for expects an array to iterate, got 'string'
```

Si necesitas recorrer caracteres, usa `substr(texto, i, 1)` dentro de un
`range(len(texto))`. Ten en cuenta que `len` cuenta bytes UTF-8 y `substr`
cuenta caracteres; para texto no ASCII no hay todavía un iterador por
caracteres.

## Siguiente paso

- [Funciones](../funciones/).
- [Arrays](../arrays/).
- [Scope](../scope/).
