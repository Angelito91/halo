---
title: Contribuir
description: Cómo preparar el entorno, elegir una tarea, añadir código y abrir un pull request en Halo.
---

Halo es un proyecto educativo y abierto. Las contribuciones no tienen que ser
enormes: un test que falta, una explicación más clara o un mensaje de error
mejor valen tanto como una característica nueva.

## Antes de empezar

- Lee la [arquitectura](../arquitectura/) para hacerte una idea del pipeline.
- Ejecuta `cargo test` para confirmar que tu entorno funciona.
- Busca un issue existente o abre uno para discutir el cambio.
- Para cambios grandes, espera respuesta antes de escribir mucho código.

## Preparar el entorno

```bash
# 1. Haz un fork en GitHub y clona tu fork
git clone https://github.com/TU_USUARIO/halo.git
cd halo

# 2. Añade el remoto original
git remote add upstream https://github.com/Angelito91/halo.git

# 3. Crea una rama
git switch -c feat/mi-caracteristica

# 4. Comprueba que compila y pasa los tests
cargo test
```

Necesitas Rust estable reciente. No hace falta ningún compilador nativo ni
bibliotecas del sistema. Consulta [Instalación](../../empezando/instalacion/)
si algo falla.

## Ciclo de trabajo

```bash
cargo fmt --all
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

Repite hasta que todo pase. Antes del PR, ejecuta también la comprobación de
formato sin modificar archivos:

```bash
cargo fmt --all -- --check
```

## Dónde contribuir

| Área | Ideas |
|---|---|
| Tests | Casos límite, errores, programas completos |
| Documentación | Ejemplos nuevos, aclaraciones, errores corregidos |
| Diagnósticos | Mensajes más claros, posición y sugerencias |
| Intérprete | Builtins, métodos, semántica de arrays |
| Stdlib | Nuevas funciones y métodos sobre valores |
| CLI | Salida, flags, ayuda |
| Ejemplos | Programas completos y comentados |

Si no sabes por dónde empezar, busca issues etiquetados como
`good first issue` o `documentation`.

## Añadir un builtin

Los pasos típicos:

1. **Registra el nombre** en `BUILTIN_NAMES` dentro de
   `src/stdlib/builtins.rs`.
2. **Añade el despacho** en `call_builtin` (`src/stdlib/builtins.rs`).
3. **Implementa la función** en `src/stdlib/arrays.rs` o
   `src/stdlib/strings.rs`, según el tipo sobre el que trabaje, o directamente
   en `src/stdlib/builtins.rs` si es una operación escalar.
4. **Añade el método**, si tiene sentido, en `src/stdlib/mod.rs`
   (`call_method`).
5. **Añade tests** de valor, aridad y tipos inválidos en `tests/stdlib_tests.rs`
   o en la suite del tipo correspondiente.
6. **Documenta** el builtin en
   `website/src/content/docs/referencia/builtins.md` y, si es una
   característica del lenguaje, en la guía correspondiente.

Ejemplo de despacho:

```rust
"min" => super::arrays::builtin_min(args)?,
```

No olvides comprobar el mensaje de aridad: los tests lo usan como contrato.

## Añadir una característica del lenguaje

El recorrido completo suele ser:

1. **Lexer**: nuevo `TokenKind` en `src/lexer/token.rs` y su regla de escaneo
   en `src/lexer/lexer.rs`.
2. **Parser**: nodo en `src/parser/ast.rs`, regla en `declarations.rs`,
   `statements.rs` o `expressions.rs`, y precedencia si es una expresión.
3. **Evaluador**: evaluación en `src/interpreter/evaluator/`.
4. **Stdlib**: si la característica es una operación sobre valores, añádela en
   `src/stdlib/`.
5. **Tests**: lexer, parser, evaluador y, si aplica, CLI y ejemplos.
6. **Documentación**: guía y referencia.

Halo es interpreter-only. No hay una segunda implementación que mantener en
paralelo, así que el comportamiento que documentas es el que ejecuta
`halo run`.

## Documentación

El sitio vive en `website/` y usa Astro + Starlight con Bun.

```bash
cd website
bun install
bun run dev      # servidor local
bun run build    # build de producción
bun run check    # comprobación de tipos y contenido
```

Las páginas están en `website/src/content/docs/`. La barra lateral se define
en `website/astro.config.mjs`. Los enlaces internos son relativos
(`../guia/estructura/`), porque el sitio se publica bajo `/halo`.

### Reglas de estilo de la documentación

- Español claro y directo.
- Ejemplos ejecutables; si algo no funciona, no lo publiques.
- Describe el comportamiento del intérprete, sin comparaciones con otras implementaciones.
- Usa `halo` en los bloques de código para el resaltado.
- Enlaza a la referencia en lugar de repetir tablas enteras.

## Commits

El repositorio usa mensajes descriptivos, a menudo con prefijo:

```text
feat: add min and max builtins
fix: reject negative array indices
docs: explain for-loop scoping
refactor: split CLI commands into modules
test: cover array slice bounds
chore: update dependencies
```

Un commit debe contar **por qué**, no solo qué. Si el cambio es grande,
divídelo en commits revisables.

## Pull request

Antes de abrir el PR:

- [ ] La rama está actualizada con `upstream/main`.
- [ ] `cargo fmt --all -- --check` pasa.
- [ ] `cargo clippy --all-targets --locked -- -D warnings` pasa.
- [ ] `cargo test --locked` pasa.
- [ ] Hay tests para el comportamiento nuevo.
- [ ] La documentación está actualizada.
- [ ] El PR explica el problema y la solución.
- [ ] No incluye cambios ajenos al objetivo.

En la descripción, incluye:

1. **Qué** cambia.
2. **Por qué** es necesario.
3. **Cómo** probarlo.
4. Capturas o salidas si afecta a la CLI.

## Licencia

Halo se distribuye bajo la **Mozilla Public License 2.0**. Al contribuir,
aceptas que tu código se publique bajo la misma licencia.

## Código de conducta

Sé respetuoso y constructivo. Las críticas van dirigidas al código, no a las
personas. Si algo te parece incorrecto, explica el caso con un ejemplo.

## Siguiente paso

- [Arquitectura](../arquitectura/).
- [Testing](../testing/).
- [Roadmap](../roadmap/).
