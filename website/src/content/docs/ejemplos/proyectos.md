---
title: Proyectos prácticos
description: Cinco mini-proyectos completos en Halo para practicar funciones, condicionales, bucles y strings.
---

Estos proyectos están pensados para copiar, ejecutar y modificar. Usan solo
características implementadas en el intérprete.

## 1. Conversor de temperaturas

```halo title="temperaturas.halo"
celsius_to_fahrenheit(c) {
    return c * 9 / 5 + 32
}

fahrenheit_to_celsius(f) {
    return (f - 32) * 5 / 9
}

celsius_to_kelvin(c) {
    return c + 273.15
}

print("0°C = " + str(celsius_to_fahrenheit(0)) + "°F")
print("100°C = " + str(celsius_to_fahrenheit(100)) + "°F")
print("32°F = " + str(fahrenheit_to_celsius(32)) + "°C")
print("212°F = " + str(fahrenheit_to_celsius(212)) + "°C")
print("0°C = " + str(celsius_to_kelvin(0)) + "K")
```

Salida:

```text
0°C = 32°F
100°C = 212°F
32°F = 0°C
212°F = 100°C
0°C = 273.15K
```

**Qué practicas:** funciones, expresiones aritméticas, promoción a float y
concatenación con `str()`.

## 2. Sistema de notas

```halo title="notas.halo"
letra(nota) {
    if nota >= 90 { return "A" }
    if nota >= 80 { return "B" }
    if nota >= 70 { return "C" }
    if nota >= 60 { return "D" }
    return "F"
}

gpa(nota) {
    if nota >= 90 { return 4.0 }
    if nota >= 80 { return 3.0 }
    if nota >= 70 { return 2.0 }
    if nota >= 60 { return 1.0 }
    return 0.0
}

aprobo(nota) {
    return nota >= 60
}

notas = [95, 82, 71, 58, 88]
total = 0.0
i = 0

while i < len(notas) {
    n = notas[i]
    estado = "suspende"
    if aprobo(n) { estado = "aprueba" }

    print(str(n) + " → " + letra(n) + " (" + estado + ")")
    total = total + gpa(n)
    i = i + 1
}

promedio = total / len(notas)
print("GPA medio: " + str(promedio))
```

Salida:

```text
95 → A (aprueba)
82 → B (aprueba)
71 → C (aprueba)
58 → F (suspende)
88 → B (aprueba)
GPA medio: 2.4
```

**Qué practicas:** condicionales encadenados, arrays, `while`, acumuladores y
funciones que devuelven strings y floats.

## 3. Calculadora de IMC

```halo title="imc.halo"
clasificar(imc) {
    if imc < 18.5 { return "bajo peso" }
    if imc < 25.0 { return "normal" }
    if imc < 30.0 { return "sobrepeso" }
    return "obesidad"
}

imc(peso, altura) {
    return peso / (altura * altura)
}

casos = [
    [70.0, 1.75],
    [52.0, 1.68],
    [95.0, 1.72]
]

for caso in casos {
    peso = caso[0]
    altura = caso[1]
    valor = imc(peso, altura)
    print(str(valor) + " → " + clasificar(valor))
}
```

Salida aproximada:

```text
22.857142857142858 → normal
18.424... → bajo peso
32.112... → obesidad
```

**Qué practicas:** arrays de arrays, acceso anidado (`caso[0]`), floats y
clasificación con umbrales.

Para redondear a dos decimales puedes usar `int(valor * 100) / 100.0`.

## 4. Estadísticas de texto

```halo title="texto.halo"
es_vocal(c) {
    return c == "a" || c == "e" || c == "i" || c == "o" || c == "u"
}

estadisticas(texto) {
    limpio = texto.lower().trim()
    total = len(limpio)
    vocales = 0
    espacios = 0
    i = 0

    while i < total {
        c = substr(limpio, i, 1)
        if es_vocal(c) { vocales = vocales + 1 }
        if c == " " { espacios = espacios + 1 }
        i = i + 1
    }

    palabras = espacios + 1
    print("Caracteres: " + str(total))
    print("Vocales: " + str(vocales))
    print("Palabras: " + str(palabras))
    print("Media de letras por palabra: " + str(total / palabras))
}

estadisticas("Halo es un lenguaje simple y divertido")
```

Salida aproximada:

```text
Caracteres: 38
Vocales: 14
Palabras: 7
Media de letras por palabra: 5
```

**Qué practicas:** métodos de string, `substr`, `len`, bucles, lógica
booleana y funciones auxiliares.

:::note
`len()` cuenta bytes UTF-8. Con texto no ASCII, el número de “caracteres” puede
no coincidir con lo que ves. Para estos ejercicios usa texto ASCII.
:::

## 5. Interés compuesto

```halo title="interes.halo"
interes_compuesto(capital, tasa, anios) {
    monto = capital
    i = 0
    while i < anios {
        monto = monto * (1.0 + tasa)
        i = i + 1
    }
    return monto
}

final = interes_compuesto(1000.0, 0.05, 10)
print("Monto final: " + str(final))
print("Ganancia: " + str(final - 1000.0))
```

Salida aproximada:

```text
Monto final: 1628.8946267774422
Ganancia: 628.8946267774422
```

**Qué practicas:** floats, bucles con acumulador y funciones con varios
parámetros.

## Ideas para seguir

- Añade entrada de datos… no hay `input()` todavía, así que prueba con valores
  fijos o arrays de casos.
- Convierte cada proyecto en una función que reciba parámetros y devuelva un
  string.
- Escribe tests manuales con `print()` y compara salidas.
- Inspecciona el AST con `halo ast` y los tokens con `halo tokens`.

## Siguiente paso

- [Guía de arrays](../../guia/arrays/).
- [Guía de strings](../../guia/strings/).
- [Ejemplos del repositorio](https://github.com/Angelito91/halo/tree/main/examples).
