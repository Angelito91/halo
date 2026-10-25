---
title: Scope y variables
description: Scope global, frames de función, shadowing, paso por valor y el caso especial del for.
---

Halo tiene un modelo de scope sencillo, pero con una excepción que conviene
entender: el `for`.

## Scope global

Las variables asignadas en el nivel superior viven en el scope global y son
visibles desde cualquier función:

```halo
base = 100

add_base(n) {
    return n + base
}

print(add_base(5))   // 105
```

Las funciones se registran antes de ejecutar las sentencias globales, pero las
variables globales se crean en orden. Una función puede leer una global que se
asigna después, siempre que la llamada ocurra después de la asignación:

```halo
saludar() {
    return "Hola, " + nombre
}

nombre = "Halo"
print(saludar())   // Hola, Halo
```

## Scope de función

Cada llamada a función crea un **frame independiente**. Los parámetros y las
variables asignadas dentro son locales:

```halo
set_local() {
    secret = 99
}

set_local()
print(secret)   // Error: Undefined variable: 'secret'
```

Al terminar la función, su frame desaparece. Dos llamadas a la misma función
no comparten locales:

```halo
contar() {
    n = 0
    n = n + 1
    return n
}

print(contar())   // 1
print(contar())   // 1
```

### Lectura dinámica de frames

Los frames no están aislados para **lectura**. La búsqueda recorre todos los
frames activos de dentro hacia fuera, así que una función puede leer un local
del llamador si no tiene una variable propia con ese nombre:

```halo
f() { return x }

g() {
    x = 42
    return f()
}

print(g())   // 42
```

Esto es **scope dinámico para lectura**. No es un closure: la función no captura
el frame del llamador, solo puede verlo mientras la llamada está activa. Las
asignaciones normales no escriben en ese frame; el `for` es la excepción.

## Shadowing

Una variable local puede tener el mismo nombre que una global. Dentro de la
función se usa la local; la global no se modifica:

```halo
x = 10

shadow() {
    x = 99
    return x
}

print(shadow())   // 99
print(x)          // 10
```

Esto es así porque, fuera del cuerpo de un `for`, **toda asignación dentro de
una función crea o actualiza una variable en el frame actual**. No hay una
palabra clave para declarar; la posición decide el scope.

## Leer globales, no mutarlas

Una función puede leer globales y locales de los frames activos, pero una
asignación normal no modifica esos bindings:

```halo
contador = 0

intentar_incrementar() {
    contador = contador + 1   // crea una local `contador`
}

intentar_incrementar()
print(contador)   // 0
```

Para actualizar el valor, devuélvelo:

```halo
contador = 0

incrementar(n) {
    return n + 1
}

contador = incrementar(contador)
print(contador)   // 1
```

Esta restricción hace que las funciones sean más predecibles: fuera del caso
especial del `for`, solo pueden cambiar el mundo a través de su valor de
retorno y de `print`.

:::caution
Dentro de un `for`, las asignaciones buscan el binding existente más cercano.
Si la función no tiene una local con ese nombre, la asignación puede alcanzar
una global o un local de un llamador:

```halo
contador = 0

incrementar_en_bucle() {
    for i in range(1) {
        contador = contador + 1
    }
}

incrementar_en_bucle()
print(contador)   // 1
```

Fuera de un `for`, la misma asignación crearía una local y la global seguiría
siendo `0`.
:::

## Parámetros por valor

Los parámetros se pasan **por valor**. Reasignarlos dentro de la función no
afecta al caller:

```halo
double_in_place(x) {
    x = x * 2
    return x
}

original = 7
resultado = double_in_place(original)
print(original)    // 7
print(resultado)   // 14
```

Los arrays también son valores. Pasar un array a una función pasa una copia
del valor; los métodos no mutan el original. `push` devuelve un array nuevo:

```halo
modificar(arr) {
    arr = arr.push(99)
    return arr
}

datos = [1, 2]
nuevo = modificar(datos)
print(datos)   // [1, 2]
print(nuevo)   // [1, 2, 99]
```

## `if` y `while` no crean scope

A diferencia de otros lenguajes, los bloques de `if` y `while` **no** abren un
scope nuevo:

```halo
if true {
    resultado = 42
}
print(resultado)   // 42
```

```halo
i = 0
while i < 3 {
    ultimo = i
    i = i + 1
}
print(ultimo)   // 2
```

Esto también significa que una variable creada dentro de un `if` puede
“escapar” al resto de la función. Es cómodo, pero úsalo con cuidado.

## El caso especial del `for`

La variable del `for` sí tiene su propio scope:

```halo
i = 99
for i in range(2) {
    print(i)   // 0 1
}
print(i)       // 99
```

Dentro del cuerpo, las asignaciones a variables que ya existen fuera del bucle
**sí** las actualizan (incluidas las globales y los locales de un llamador).
Esto permite acumuladores:

```halo
total = 0
for i in range(4) {
    total = total + i
}
print(total)   // 6
```

Una variable **nueva** creada dentro del cuerpo del `for` vive en el scope del
bucle y desaparece al terminar:

```halo
for i in range(2) {
    temporal = i * 10
}
print(temporal)   // Error: Undefined variable: 'temporal'
```

Resumen:

| Contexto | ¿Nuevo scope? | Asignación a variable existente |
|---|---|---|
| Nivel superior | Global | Actualiza la global |
| Función | Frame propio | Crea/actualiza una local |
| `if` / `while` | No | Actualiza la variable del frame actual |
| `for` (cabecera) | Sí, para la variable del bucle | — |
| `for` (cuerpo) | Sí, para nombres nuevos | Actualiza la variable externa si existe |

La tabla describe **escrituras**. Para **lecturas**, la búsqueda recorre todos
los frames activos, así que una función también puede ver locales de quien la
llamó.

## Recursión y frames

Cada llamada recursiva tiene su propio frame:

```halo
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
```

La `n` de cada llamada es independiente y las asignaciones se quedan en el
frame actual. La lectura, en cambio, busca en todos los frames activos: si una
función no tiene una variable propia con ese nombre, puede encontrar la de un
llamador. Las globales también se leen; el `for` es el único caso en el que una
asignación puede actualizar un binding de otro frame.

## Siguiente paso

- [Arrays](../arrays/).
- [Funciones](../funciones/).
- [Bucles](../bucles/).
