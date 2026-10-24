---
title: Tu primer programa
description: Crea, ejecuta y compila tu primer programa en Halo paso a paso.
---

En cinco minutos vas a tener un programa Halo ejecutándose en el intérprete y
compilado a un binario nativo.

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
version = 0.2
anio = 2024

print("Lenguaje: " + nombre)
print("Versión: " + str(version))
print("Año: " + str(anio))
```

Salida:

```text
Lenguaje: Halo
Versión: 0.2
Año: 2024
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

## 5. Compila a binario

El backend nativo todavía tiene divergencias conocidas: `str()` sobre variables,
la concatenación de string con número y los arrays no se comportan igual que en
el intérprete. Para probar `halo build`, usa un programa escalar sencillo:

```halo title="saludo.halo"
nombre = "Halo"
print("Hola, " + nombre)
```

Si tienes LLVM 23.1 y Clang instalados:

```bash
halo build saludo.halo
./saludo
# Hola, Halo
```

En Windows:

```powershell
halo build saludo.halo
.\saludo.exe
# Hola, Halo
```

El binario es independiente del intérprete de Halo. Por defecto se genera con
optimización `-O2`; puedes cambiarla:

```bash
halo build -O3 -o saludo saludo.halo
halo build --run saludo.halo    # compila y ejecuta
```

Consulta [diferencias entre backends](../../referencia/diferencias/) antes de
compilar programas más grandes.

## 6. Inspecciona el proceso

Una de las ventajas de Halo es que puedes ver cada fase:

```bash
halo tokens hola.halo    # tokens del lexer
halo ast hola.halo       # árbol de sintaxis
halo check hola.halo     # solo valida sintaxis
halo llvm hola.halo      # emite hola.ll (LLVM IR)
```

Prueba `halo ast` con una función y observa cómo se representa el cuerpo.

## Errores comunes al empezar

### `Undefined variable: 'x'`

Leíste una variable antes de asignarla. Halo no tiene `undefined`; toda
variable debe existir antes de usarse.

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
