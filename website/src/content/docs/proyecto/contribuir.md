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

Necesitas Rust estable reciente y LLVM 23.1 + Clang. Consulta
[Instalación](../../empezando/instalacion/) si algo falla.

## Ciclo de trabajo

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Repite hasta que todo pase. Antes del PR, ejecuta también la comprobación de
formato sin modificar archivos:

```bash
cargo fmt --all -- --check
```

## Dónde contribuir

| Área | Ideas |
|---|---|
| Tests | Casos límite, errores, paridad intérprete/compilador |
| Documentación | Ejemplos nuevos, aclaraciones, errores corregidos |
| Diagnósticos | Mensajes más claros, línea/columna en runtime |
| Intérprete | Builtins, métodos, semántica de arrays |
| Compilador | Arrays, `for` sobre arrays, optimizaciones |
| CLI | Salida, flags, ayuda |
| Ejemplos | Programas completos y comentados |

Si no sabes por dónde empezar, busca issues etiquetados como
`good first issue` o `documentation`.

## Añadir un builtin

El catálogo es la fuente de verdad. Los pasos típicos:

1. **Registra el builtin** en `src/lang/catalog.rs` con su nombre, aridad y
   `Support` (`Both`, `InterpreterOnly` o `CompilerOnly`).
2. **Implementa la función** en
   `src/interpreter/evaluator/builtins.rs` o en el módulo de métodos que
   corresponda (`methods/arrays.rs`, `methods/strings.rs`).
3. **Añade el método**, si tiene sentido, en
   `src/interpreter/evaluator/methods/mod.rs`.
4. **Implementa el codegen** si el builtin debe funcionar en binarios
   (`src/compiler/codegen/builtins/`). Si no, márcalo como
   `InterpreterOnly`.
5. **Añade tests** de valor, aridad, tipos inválidos y, si aplica, paridad.
6. **Documenta** el builtin en `website/src/content/docs/referencia/builtins.md`
   y, si es una característica del lenguaje, en la guía correspondiente.

Ejemplo de entrada en el catálogo:

```rust
Builtin {
    name: "min",
    arity: 2..=2,
    support: Both,
},
```

No olvides comprobar el mensaje de aridad: los tests lo usan como contrato.

## Añadir una característica del lenguaje

El recorrido completo suele ser:

1. **Lexer**: nuevo `TokenKind` y su regla de escaneo.
2. **Parser**: nodo en `ast.rs`, regla en `declarations.rs` o `statements.rs`,
   y precedencia si es una expresión.
3. **Intérprete**: evaluación en `evaluator/`.
4. **Compilador**: codegen en `compiler/codegen/`.
5. **Tests**: lexer, parser, evaluador, compilador y paridad.
6. **Documentación**: guía, referencia y, si cambia la sintaxis, `SYNTAX.md`.

Si la característica no está lista para el compilador, declárala como
`InterpreterOnly` y documenta la limitación. Es mejor un backend honesto que
uno que falla en silencio.

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
- Distingue lo que hace el intérprete de lo que hace el compilador.
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
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.
- [ ] `cargo test` pasa.
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
