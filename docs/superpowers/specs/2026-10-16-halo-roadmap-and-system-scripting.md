# Halo — Roadmap y redirección a scripting de sistema

**Fecha:** 2026-10-16
**Estado:** Aprobado en conversación con el propietario del proyecto

## 1. Visión

Halo es un lenguaje educativo en Rust (intérprete tree-walking + backend LLVM vía
inkwell/llvm-sys 211 = LLVM 21) que evoluciona en dos etapas:

1. **Terminar el roadmap v0.3/v0.4 original** (lenguaje genérico).
2. **Redirigir hacia scripting de sistema**: la moderna alternativa a bash —
   scripts pequeños para gestionar Linux y apps sencillas de terminal.

## 2. Etapa 1 — terminar el roadmap original (orden aprobado)

| # | Fase | Contenido | Notas |
|---|---|---|---|
| 0 | Reparación | Build LLVM, limpieza, commits del refactor pendiente, push | Ver plan 2026-10-16-fase-0 |
| 1 | `for` | `for x in range(n)` / `for x in array {}`, builtin `range()` 1–2 args, scope de la variable en el cuerpo, break/continue reutilizados, codegen baja a la misma estructura IR de while (`statements/for_loop.rs`) | Sin iterar strings en v1 |
| 2 | Diccionarios/Maps | Literal `{"k": v}`, claves string en v1, HashMap en intérprete, runtime struct + helpers en `codegen/runtime/` | Requiere mini-spec propio antes de implementar |
| 3 | Errores con posición | Número de línea y columna en los diagnostics | Ya existe `src/diagnostics/` |
| 4 | Comentarios `/* ... */` | Multi-línea, en lexer | |
| 5 | REPL | `halo repl` interactivo | Al final, con stdlib rica |

Los **métodos de string** de v0.3.0 ya están implementados (intérprete deduplicado
+ código compilado vía `desugaring`); se verifican y commitean en la Fase 0.

La "stdlib extendida" genérica de v0.4.0 se absorbe más adelante con el toolkit
de sistema de la Etapa 2 (evita duplicar trabajo).

## 3. Etapa 2 — redirección a scripting de sistema

Decisiones aprobadas:

- **Ejecución de comandos**: biblioteca primero (`sh("cmd")` → `{ok, exit, stdout, stderr}`);
  azúcar de pipes nativos queda para una v2 si la comunidad la pide.
- **Modelo de errores**: Result explícito + variante estricta (`sh!` aborta con
  diagnóstico claro si `exit != 0`).
- **Toolkit de sistema v1** (los 4 dominios):
  1. Procesos y entorno: `sh/sh!`, `env()`, `ARGS`, `exit(code)`
  2. Archivos y rutas: `read_file/write_file/append/exists/remove/mkdir/list_dir/glob`,
     `path.join/abs/expand("~")`
  3. Interacción de usuario: `prompt()`, `read_password()`, `confirm()`,
     `parse_args() -> dict`
  4. `parse_json/to_json` + `glob()` (pares con los diccionarios)

Detalles de diseño:

- **Dos modos de ejecución**: script vía intérprete (`#!/usr/bin/env halo`,
  el lexer ignora la primera línea si empieza con `#!`) y distribución vía
  `halo build` a binario nativo.
- **Interpolación de strings**: `"texto ${var}"` y expresiones `"${i + 1}"`.
- **For/Maps/arrays** son las estructuras que procesan salidas de comando.
- **Arquitectura del stdlib**: funciones nativas en Rust registrables en ambos
  backends (el intérprete las llama directamente; el codegen emite llamadas a
  una mini runtime library enlazada al binario compilado).
- **Rebranding README** cuando arranque la Etapa 2.

## 4. Orden de ejecución de la Etapa 2 (fases)

| Fase | Contenido |
|---|---|
| 6.1 | Fundación: shebang, `ARGS`/`env`/`exit`, interpolación `${}`, rebranding README |
| 6.2 | Procesos: `sh`/`sh!`, captura stdout/stderr, cwd |
| 6.3 | Archivos: fs + rutas + glob |
| 6.4 | Interacción: prompt, password, confirm, `parse_args` (necesita dicts |
| 6.5 | Dicts + JSON (con spec propio) — habilita `parse_json`/`to_json` |

El REPL queda para después de que la stdlib sea rica.

## 5. Restricciones globales

- Rust stable, edition 2021; mantener la suite de ~500 tests siempre verde.
- Cada fase termina con tests pasando y commit antes de empezar la siguiente.
- No romper la sintaxis existente salvo lo diseñado (interpolación a partir
  de la 6.1, comentarios multi-línea en su fase).
- LLVM 21 (inkwell feature `llvm21-1`); no actualizar llvm-sys sin decisión explícita.
