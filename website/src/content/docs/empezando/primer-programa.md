---
title: Tu primer programa
description: Crea, ejecuta e inspecciona tu primer programa en Halo paso a paso.
---

En cinco minutos vas a tener un programa Halo ejecutándose y vas a poder mirar
por dentro de cada fase del intérprete.

## 1. Crea el archivo

Crea una carpeta para tus pruebas y un archivo llamado `hola.halo`:

```halo title="hola.halo"
// Mi primer programa
print("Hello, Halo!")
print("Esto es un lenguaje nuevo")
```

No hay función `main`, no hay `package`, no hay imports. Un archivo `.halo`
es el programa completo.

## 2. Ejecútalo

```bash
halo run hola.halo
```

Salida:

```text
Hello, Halo!
Esto es un lenguaje nuevo
```

También puedes omitir `run`; el intérprete es el subcomando por defecto:

```bash
halo hola.halo
```

## 3. Añade variables y expresiones

```halo title="hola.halo"
nombre = "Halo"
version = 0.3
anio = 2026

print("Lenguaje: " + nombre)
print("Versión: " + str(version))
print("Año: " + str(anio))
```

Salida:

```text
Lenguaje: Halo
Versión: 0.3
Año: 2026
```

La concatenación con `+` convierte el otro operando a string automáticamente.
`str()` hace la conversión explícita cuando quieres dejar clara la intención.

## 4. Añade una función

```halo title="hola.halo"
saludar(quien) {
    return "Hola, " + quien + "!"
}

print(saludar("mundo"))
print(saludar("Halo"))
```

Salida:

```text
Hola, mundo!
Hola, Halo!
```

Las funciones se definen con `nombre(parámetros) { … }`, sin palabra clave.
Puedes llamarlas antes de que aparezcan en el archivo: el intérprete registra
todas las definiciones en un primer pase.

## 5. Inspecciona el proceso

Una de las ventajas de Halo es que puedes ver cada fase:

```bash
halo tokens hola.halo    # tokens del lexer, con línea y columna
halo ast hola.halo       # árbol de sintaxis
halo check hola.halo     # solo valida sintaxis
```

`halo ast` es especialmente útil para entender cómo se representa una función
o un `if`. Prueba a ejecutarlo con el archivo que acabas de escribir.

También puedes pedir información de progreso mientras ejecutas:

```bash
halo run --verbose hola.halo
```

## 6. Escribe un script corto

Los arrays y los bucles permiten resolver tareas pequeñas sin salir del
lenguaje:

```halo title="notas.halo"
notas = [7, 9, 6, 10]

total = 0
for nota in notas {
    total = total + nota
}

print("Notas:", notas)
print("Media:", total / len(notas))
```

```bash
halo run notas.halo
# Notas: [7, 9, 6, 10]
# Media: 8
```

## Errores comunes al empezar

### `Undefined variable: 'x'`

Leíste una variable antes de asignarla. Halo no tiene un valor `undefined`;
toda variable debe existir antes de usarse y `null` representa la ausencia de
valor. El error incluye la línea y la columna:

```halo
print(x)   // Error
x = 1
```

### `Expected '{' after ...` / `Expected '}' after ...`

Falta una llave de apertura o de cierre, o el `{` no está en la misma línea que
la declaración. La llave de apertura debe ir inmediatamente después de la
condición; el resto del formato es cuestión de estilo.

```halo
// ✅
if true {
    print("sí")
}

// ❌
if true
{
    print("sí")
}
```

### `Function 'f' expects 2 argument(s), got 1`

La aridad debe coincidir exactamente. No hay argumentos por defecto ni
variádicos para funciones de usuario.

## Siguiente paso

- [Domina la CLI](../cli/).
- [Recorre la guía del lenguaje](../../guia/estructura/).
- [Mira ejemplos completos](../../ejemplos/).
