---
title: Estructura de un programa
description: Cómo se organiza un archivo Halo, cómo funcionan los saltos de línea y por qué no hay main ni semicolons.
---

Un programa Halo es una secuencia de **declaraciones de función** y
**sentencias globales**. Por convención, cada sentencia va en su propia línea.
No hay función `main` obligatoria, no hay imports y no hay punto y coma.

## El archivo es el programa

```halo
// Funciones
double(x) {
    return x * 2
}

saludar() {
    print("Hola")
}

// Sentencias globales
saludar()
resultado = double(21)
print(resultado)   // 42
```

Las sentencias globales se ejecutan de arriba abajo, en orden, cuando arranca
el programa. Las funciones no se ejecutan hasta que las llamas.

## Las funciones se registran primero

Aunque el intérprete ejecute las sentencias en orden, **todas las funciones se
registran en un primer pase**. Por eso puedes llamar una función antes de que
aparezca en el archivo:

```halo
print(square(7))   // 49 — llamada antes de la definición

square(n) {
    return n * n
}
```

Esto vale para el nivel superior. Una función puede llamar a otra definida más
abajo, y la recursión mutua funciona sin declaraciones previas.

## Saltos de línea y separación de sentencias

Halo no usa `;`. Por convención, cada sentencia va en su propia línea:

```halo
// ✅ Dos sentencias, dos líneas
x = 1
y = 2
```

El parser actual no exige un `;` ni un salto de línea entre sentencias, así que
podría aceptar `x = 1 y = 2`; no lo recomendamos porque rompe el estilo del
proyecto y la legibilidad. Los bloques `{ … }` delimitan el cuerpo de
funciones, condicionales y bucles. Puedes escribir un bloque en una línea si
cabe y sigue siendo legible:

```halo
if n <= 1 { return 1 }
```

## Comentarios

Solo hay comentarios de línea:

```halo
// Comentario completo
x = 5   // Comentario al final de la sentencia
```

Los comentarios multilínea `/* … */` todavía no están implementados.

## Indentación

La indentación **no es significativa** para el parser: el lenguaje no usa
Python-style blocks. Aun así, el proyecto usa **4 espacios** por nivel y lo
recomendamos para que el código se lea igual que el de los ejemplos.

```halo
factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
```

## Orden recomendado

No es obligatorio, pero un archivo se lee mejor así:

1. Constantes y configuración global.
2. Funciones auxiliares.
3. Funciones principales.
4. Sentencias de arranque al final.

```halo
MAX_INTENTOS = 3

leer_config() {
    return "producción"
}

main_logic() {
    config = leer_config()
    print("Entorno: " + config)
}

main_logic()
```

## Sin `main`

No necesitas envolver todo en una función. Las sentencias del nivel superior
son el punto de entrada:

```halo
print("El programa empieza aquí")
```

Si vienes de Java, C# o Rust, el cambio es liberador: no hay ceremonia de
arranque. Por costumbre puedes crear una función `main` y llamarla al final,
pero evita ese nombre si vas a compilar: el backend ya genera un `main` de C.

```halo
principal() {
    print("Hola")
}

principal()
```

## Siguiente paso

- [Variables y tipos](../variables-y-tipos/).
- [Operadores](../operadores/).
- [Estilo](../estilo/).
