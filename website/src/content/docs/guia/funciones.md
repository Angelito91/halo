---
title: Funciones
description: Declaración, llamada, retorno, recursión y límites de las funciones en Halo.
---

Las funciones son la única forma de abstracción que tiene Halo. No hay
clases, módulos, closures ni funciones anónimas.

## Definición

No se usa ninguna palabra clave:

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

La sintaxis es:

```text
nombre(param1, param2, …) {
    // cuerpo
}
```

Los parámetros se separan por comas. No hay tipos, valores por defecto ni
parámetros variádicos.

## Llamada

```halo
saludar()
resultado = sumar(3, 4)
print(max(10, 20))
```

La aridad debe coincidir **exactamente**:

```halo
add(a, b) { return a + b }

add(1)        // Error: Function 'add' expects 2 argument(s), got 1
add(1, 2, 3)  // Error: Function 'add' expects 2 argument(s), got 3
```

## Retorno

`return` con un valor sale de la función y produce ese valor:

```halo
double(x) {
    return x * 2
}

print(double(21))   // 42
```

`return` sin expresión sale inmediatamente y produce `null`:

```halo
setup(flag) {
    if flag { print("configurado") }
    return
    print("esto nunca se ejecuta")
}
```

Una función sin `return` también produce `null` al terminar:

```halo
nothing() {
    x = 1
}

print(type(nothing()))   // null
```

No hay retorno implícito del último valor. Si quieres devolver algo, escribe
`return`.

## Call-before-definition

Todas las funciones se registran antes de ejecutar las sentencias globales,
así que puedes llamarlas antes de su definición textual:

```halo
resultado = triple(7)   // llamada antes de la definición
triple(n) { return n * 3 }
print(resultado)        // 21
```

Esto también permite recursión mutua sin declaraciones previas.

## Recursión

```halo
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print(factorial(10))   // 3628800
```

```halo
fib(n) {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}

print(fib(10))   // 55
```

### Recursión mutua

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

### Límite de profundidad

El intérprete corta la recursión antes de desbordar el stack nativo de Rust:

```halo
infinite(n) { return infinite(n + 1) }
infinite(0)
// Error: Infinite recursion detected in function 'infinite' — maximum depth 500 exceeded
```

El límite es de **500** llamadas anidadas. Una recursión que necesite más
profundidad debe convertirse en un bucle.

## Expresiones con llamadas

El resultado de una llamada se puede usar como cualquier valor:

```halo
inc(n) { return n + 1 }
square(n) { return n * n }

print(square(inc(3)))                    // 16
print(square(inc(3)) + square(inc(4)))   // 41
```

## Funciones y variables externas

Una función puede **leer** variables globales:

```halo
base = 100

add_base(n) {
    return n + base
}

print(add_base(5))   // 105
```

La lectura no se limita a las globales. Los frames de las llamadas activas
comparten una pila y la búsqueda va de dentro hacia fuera, así que una función
también puede leer un local del llamador si no tiene uno propio con ese nombre:

```halo
f() { return x }

g() {
    x = 42
    return f()
}

print(g())   // 42
```

Esto es **scope dinámico para lectura**, no un closure: la función no captura
el frame del llamador y deja de verlo en cuanto la llamada termina.

Pero una asignación normal dentro de la función crea o actualiza una variable
en su propio frame, aunque exista una global o un local del llamador con el
mismo nombre:

```halo
x = 10

shadow() {
    x = 99    // local, no modifica el global
    return x
}

print(shadow())   // 99
print(x)          // 10
```

La excepción es el cuerpo de un `for`: ahí las asignaciones buscan el binding
existente más cercano y pueden alcanzar una global o un local del llamador.
Consulta [el caso especial del `for`](../scope/#el-caso-especial-del-for).

Si quieres que una función “modifique” un valor global, devuélvelo y asigna el
resultado fuera:

```halo
contador = 0

incrementar(n) {
    return n + 1
}

contador = incrementar(contador)
print(contador)   // 1
```

## Lo que no hay

- **Funciones como valores:** no puedes pasar `sumar` como argumento ni
  guardarla en una variable.
- **Closures:** las funciones no capturan su entorno. Pueden leer locales de
  los frames activos (scope dinámico), pero no los conservan al terminar la
  llamada.
- **Funciones anónimas / lambdas.**
- **Sobrecarga:** no hay resolución por número de argumentos; evita definir dos
  funciones con el mismo nombre.
- **Parámetros por defecto o variádicos.**
- **Métodos definidos por el usuario:** no existen; las operaciones son
  funciones built-in.

## Buenas prácticas

- Funciones pequeñas, con una responsabilidad.
- Nombres en `snake_case` y verbos para las acciones (`calcular_total`,
  `es_valido`).
- Prefiere retornos tempranos para validar.
- Si una función necesita más de 3 o 4 parámetros, quizá estás modelando mal
  el problema (o necesitas arrays).

## Siguiente paso

- [Scope](../scope/).
- [Arrays](../arrays/).
- [Errores de runtime](../../referencia/errores/).
