---
title: Testing
description: Cómo ejecutar la suite de Halo, qué cubre cada archivo y cómo añadir tests nuevos.
---

Halo tiene **más de 800 tests** de integración, además de los tests unitarios
de cada módulo. La suite es la red de seguridad del proyecto: si un cambio
rompe el intérprete o el compilador, `cargo test` debería decirlo.

:::note
Compilar los tests requiere LLVM 23.1, igual que compilar el proyecto. Los tests
de paridad que invocan `clang` se saltan automáticamente si el binario no está
disponible.
:::

## Ejecutar todo

```bash
cargo test
```

Para ver la salida de los programas durante los tests:

```bash
cargo test -- --nocapture
```

## Filtrar por suite

Cada archivo de `tests/` es un crate de integración independiente:

```bash
cargo test --test arithmetic_tests
cargo test --test control_flow_tests
cargo test --test function_tests
cargo test --test scope_and_variable_tests
cargo test --test string_and_types_tests
cargo test --test array_functions_tests
cargo test --test lexer_tests
cargo test --test parser_tests
cargo test --test interpreter_tests
```

## Ejecutar un test concreto

```bash
cargo test test_factorial_10
cargo test test_fibonacci_10
cargo test test_break_inside_function
```

El filtro es una subcadena del nombre, así que `cargo test factorial` ejecuta
todos los tests que lo contengan.

## Qué cubre cada suite

| Archivo | Qué prueba |
|---|---|
| `lexer_tests.rs` | Tokens, posiciones, strings, comentarios, errores léxicos |
| `parser_tests.rs` | Precedencia, AST, declaraciones, errores de sintaxis |
| `evaluator_tests.rs` | Evaluación de expresiones y sentencias |
| `environment_tests.rs` | Scopes, `set`, `update`, `get`, frames |
| `value_tests.rs` | Operaciones, comparaciones y conversiones de `Value` |
| `arithmetic_tests.rs` | Aritmética entera y flotante, overflow, división |
| `control_flow_tests.rs` | `if`, `while`, `for`, `break`, `continue` |
| `function_tests.rs` | Llamadas, retorno, recursión, aridad |
| `scope_and_variable_tests.rs` | Shadowing, globales, frames, `for` |
| `string_and_types_tests.rs` | Strings, métodos, truthiness, conversiones |
| `array_functions_tests.rs` | Arrays, indexación, slicing, métodos |
| `interpreter_tests.rs` | Programas completos de extremo a extremo |
| `compiler_output_tests.rs` | Salida del backend LLVM |
| `parity_freeze_tests.rs` | Intérprete vs binario compilado |
| `cli_args_tests.rs` | Parseo de argumentos de la CLI |
| `cli_output_tests.rs` | Formato de salida de la CLI |
| `lang_catalog_tests.rs` | Catálogo de builtins y aridades |
| `error_codes_tests.rs` | Códigos y mensajes de diagnóstico |
| `style_checker_tests.rs` | Reglas de estilo |
| `similarity_tests.rs` | Sugerencias de nombres parecidos |

## Tests de paridad

`parity_freeze_tests.rs` es especial: ejecuta el mismo programa en el
intérprete y en un binario compilado, y compara la salida. El corpus está
“congelado” para que un cambio de comportamiento no pase desapercibido.

```rust
#[test]
fn case_int_truncation() {
    parity::assert_parity(
        "int_truncation",
        "print(int(3.9))\nprint(int(-2.7))",
    );
}
```

Si una diferencia es intencionada, se declara en `KNOWN_DIVERGENCES` con una
razón y la fase en la que se resolverá. El harness falla si encuentra una
divergencia no declarada o una entrada obsoleta. La lista visible para usuarios
está en [diferencias entre backends](../../referencia/diferencias/).

## Añadir un test

Los helpers viven en `tests/common/mod.rs`. El patrón habitual:

```rust
mod common;

use common::eval_number;

#[test]
fn suma_basica() {
    assert_eq!(eval_number("x = 2 + 3", "x"), 5);
}
```

Para comprobar errores:

```rust
mod common;

#[test]
fn division_por_cero() {
    let err = common::eval_code("x = 1 / 0").unwrap_err();
    assert!(err.contains("Division by zero"), "error inesperado: {err}");
}
```

Para strings:

```rust
mod common;

use common::eval_var;
use halo::interpreter::Value;

#[test]
fn trim_quita_espacios() {
    let value = eval_var(r#"x = "  hola  ".trim()"#, "x");
    assert_eq!(value, Value::String("hola".to_string()));
}
```

### Convenciones

- Un test, un comportamiento.
- El nombre describe qué se espera, no cómo se implementa.
- Comprueba el mensaje de error cuando el error sea parte del contrato.
- Evita depender de detalles de formato que no estén estabilizados.
- Si el test necesita LLVM o `clang`, sáltalo cuando no estén disponibles,
  como hacen los tests de paridad.

## Calidad de código

Antes de abrir un PR, ejecuta lo mismo que la CI:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

Para aplicar el formato automáticamente:

```bash
cargo fmt --all
```

## Cobertura

Si quieres medir cobertura:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

El informe queda en `tarpaulin-report.html`. No es un requisito para
contribuir, pero ayuda a encontrar ramas sin probar.

## CI

El workflow de GitHub Actions ejecuta tres bloques:

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all -- --nocapture`

La documentación tiene su propio workflow para construir y desplegar el sitio
en GitHub Pages.

## Siguiente paso

- [Arquitectura](../arquitectura/).
- [Contribuir](../contribuir/).
- [Roadmap](../roadmap/).
