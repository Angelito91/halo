# Refactorización del intérprete: métodos value-level

## Estado actual

El intérprete ya no convierte valores (`Value`) de vuelta a expresiones
(`Expression`) para reutilizar built-ins. Los métodos de string y array
comparten funciones **value-level** en:

- `src/interpreter/evaluator/methods/strings.rs`
- `src/interpreter/evaluator/methods/arrays.rs`
- `src/interpreter/evaluator/methods/mod.rs` (dispatcher)

Cada built-in evalúa sus argumentos y delega en la operación value-level:

```rust
pub fn builtin_upper(evaluator: &mut Evaluator, args: &[Expression]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("upper() expects exactly 1 argument, got {}", args.len()));
    }
    upper_value(evaluator.eval_expr(&args[0])?)
}
```

El dispatcher de métodos hace lo mismo sin fabricar un AST sintético:

```rust
(Value::String(s), "upper") => {
    if !args.is_empty() { /* error de aridad */ }
    upper_value(Value::String(s.clone()))
}
```

Se eliminaron `value_to_expression` y `build_string_call_args`. Esto también
eliminó el `Expression::StringLiteral` con `Position { line: 0, column: 0 }`
que se usaba como puente.

## Cambios relacionados

- `IRBuilder` encapsula su `Builder` de inkwell: el campo es privado y el
  wrapper implementa `Deref<Target = Builder>` para emitir instrucciones.
- `Compilation` fija el **target triple** del host al crear el módulo y
  `CodeGenerator::optimise` fija el **data layout** del `TargetMachine`.
- `Parser::parse()` devuelve `Result<Program, Vec<ParseError>>`; ya no existe
  `parse_diagnostics`.
- El backend compilado trackea variables booleanas (`bool_locals` /
  `bool_globals`) para imprimir y convertir `true`/`false` en vez de `1`/`0`.
- `print` y `println` tienen la misma semántica en ambos backends: todos los
  argumentos en una línea, separados por espacios, con un newline final.
- Los errores de runtime del intérprete se anotan con línea y columna de la
  sentencia que falló.

## Paridad intérprete ↔ compilado

El corpus vive en `tests/common/cases.rs` y el arnés en
`tests/common/parity.rs`. Estado actual:

- **111 casos** de paridad.
- **30 divergencias declaradas**: arrays/`range` como valor (P3), errores de
  runtime detectados en compilación (P2) y el retorno vacío (P1).
- Las divergencias P1 de formato (`bool`, floats, `str` runtime, `int(string)`,
  `float(string)`, `print` multiargumento, predicados de string) están
  cerradas.

## Verificación

Estado del árbol: **783 tests definidos**. El agente no ejecutó la suite
completa; la validación estática usada fue:

```bash
cargo check --all-targets --offline --jobs 1
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --jobs 1 -- -D warnings
```

Para ejecutar la suite completa:

```bash
nice -n 19 cargo test --jobs 1 -- --test-threads=1
```
