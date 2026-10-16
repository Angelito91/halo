# Fase 0: Reparación y puesta en orden — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Dejar el repo a color: build LLVM funcionando, ~500 tests verdes, artefactos limpios y el refactor pendiente commiteado en commits lógicos, README sincronizado y push.

**Architecture:** Trabajo operativo sobre el repo ya existente: limpiar caché de build de llvm-sys (compilada contra LLVM-22 inexistente; el sistema tiene LLVM 21/23 y Cargo.toml apunta a `llvm21-1`), agrupar los cambios sin commitear por subsistema, y actualizar documentación.

**Tech Stack:** Rust 2021, cargo, inkwell 0.8 (`llvm21-1`), llvm-sys 211.0.0, LLVM 21 del sistema.

**Spec:** `docs/superpowers/specs/2026-10-16-halo-roadmap-and-system-scripting.md`

## Global Constraints

- LLVM 21 (inkwell feature `llvm21-1`, llvm-sys 211.0.0). No actualizar llvm-sys.
- La suite de tests debe quedar verde antes de cada commit encadenado.
- El stash `stash@{0}` (WIP on 9c8a1d3) NO se toca: es del propietario.
- No tocar los worktrees ni el de git-llvm del sistema; solo `target/` del proyecto.

## Review Focus

- Builds posteriores que compilen binarios `.halo` en la raíz: con el `.gitignore` actual (`/target`) se colan como untracked — patrón cubierto en Task 3 con test manual de `git status`.
- LLVM del sistema puede volver a cambiar (21→23): si `-lLLVM-2X` no se encuentra, la causa es caché de build-script de llvm-sys — test: `cargo build` limpio tras `cargo clean -p llvm-sys` (Task 1).
- `cargo test` ejecuta ~500 tests de integración; un único failure aísla por nombre con `cargo test --test <suite> <filter>` (Task 2).
- Los diffs sin commitear tocan 4 subsistemas; un grupo mal clasificado corrompe el historial — verificación: `git show --stat <commit>` por commit (Task 4).

---

### Task 1: Arreglar la build LLVM

**Files:**
- Modify: `target/` (solo artefactos; no se commitea)

**Interfaces:**
- Consumes: nada.
- Produces: binario `halo` y lib `halo` compilables y enlazables contra LLVM 21; prerequisito de todos los demás tasks.

- [ ] **Step 1: Limpiar la caché obsoleta de llvm-sys**

```bash
cargo clean -p llvm-sys
```

- [ ] **Step 2: Verificar que llvm-config del sistema coincide con llvm21-1**

Run: `llvm-config --version && llvm-config --link-dynamic --libs`
Expected: `21.1.x` y `-lLLVM-21` (el sistema tiene 21 y 23; llvm-sys 211 debe enlazar la 21).

- [ ] **Step 3: Compilar**

Run: `cargo build 2>&1 | tail -5`
Expected: código 0; a lo sumo warnings (p. ej. `fn_i64_n` never used — se trata en Task 4).

- [ ] **Step 4: Correr la suite completa**

Run: `cargo test 2>&1 | tail -20`
Expected: todos los suites pasan (cero `FAILED`). Si hay fallos, reportar al propietario antes de seguir (los commits del refactor deben partir de una base verde).

### Task 2: Limpiar artefactos de la raíz

**Files:**
- Delete: `01_hello`, `03_factorial`, `test_abs`, `test_string`, `test_string2`, `test_string3` (binarios ELF de la raíz, salidas de `halo build` de pruebas)
- Modify: `.gitignore`

**Interfaces:**
- Consumes: nada.
- Produces: `.gitignore` que evita re-colar binarios compilados en la raíz.

- [ ] **Step 1: Borrar los binarios de prueba de la raíz**

```bash
rm 01_hello 03_factorial test_abs test_string test_string2 test_string3
```

- [ ] **Step 2: Añadir patrones a .gitignore**

```gitignore
/target

# Binarios generados por `halo build` (toman el nombre del archivo .halo)
/01_hello
/03_factorial
/test_*
```

- [ ] **Step 3: Verificar git status limpio de binarios**

Run: `git status --short | grep -E '^\?\?' | grep -vE 'docs/|LSP_|src/'`
Expected: sin salida (los únicos untracked legítimos son docs LSP y los 4 módulos nuevos de src/).

### Task 3: Inspeccionar y clasificar los diffs pendientes

**Files:**
- (solo lectura/ análisis; los cambios ya existen en el working tree)

**Interfaces:**
- Consumes: working tree actual (21 modificados, 2 eliminados, 6 nuevos en src/, 2 docs).
- Produces: mapa diff→commit que usa Task 5.

- [ ] **Step 1: Revisar qué cambió por archivo**

```bash
git diff --stat HEAD
git diff HEAD -- src/lexer/lexer.rs src/utils/ src/diagnostics/ src/interpreter/ | head -200
```

Clasificar cada archivo en uno de estos subsistemas:
- `compiler-codegen`: src/compiler/** + tests/array_functions_tests.rs (si corresponde)
- `interpreter-strings`: src/interpreter/evaluator/methods.rs (+ builtins.rs si es del mismo tema)
- `misc`: lexer, utils/similarity, utils/statistics, diagnostics/error_codes (agrupar según temática real visible en el diff)

- [ ] **Step 2: Decidir destino de los módulos nuevos**

`builtins/{conversions,math,strings}.rs` y `expressions/method_call.rs` van con `compiler-codegen`, salvo que el diff demuestre lo contrario.

### Task 4: Warning residual y estado "verde"

**Files:**
- Modify: `src/compiler/types.rs:57` (solo si `fn_i64_n` está realmente sin uso)

**Interfaces:**
- Consumes: build verde de Task 1.
- Produces: build sin warnings.

- [ ] **Step 1: Confirmar que fn_i64_n no tiene referencias**

```bash
grep -rn "fn_i64_n" src/ tests/ | grep -v "src/compiler/types.rs"
```

- [ ] **Step 2: Eliminar el método o silenciar con criterio**

Si no hay referencias: eliminar `fn_i64_n` por completo (es código muerto en un mapper de tipos). Si `cmp` demuestra que debería usarse en algún call site obvio, apuntarlo para la Fase 1 y por ahora dejarlo con nota en el commit del codegen.

- [ ] **Step 3: Build limpia**

Run: `cargo build 2>&1 | grep -c warning`
Expected: `0`.

### Task 5: Commits lógicos encadenados

**Files:**
- (working tree → índice, por subsistema según el mapa de Task 3)

**Interfaces:**
- Consumes: clasificación de Task 3, tests verdes de Task 1/4, limpieza de Task 2.
- Produces: historial limpio; prerequisito del push (Task 6) y del roadmap (Task 7).

- [ ] **Step 1: Commit de refactor del codegen (incluye módulos nuevos)**

```bash
git add src/compiler/ tests/array_functions_tests.rs
git commit -m "refactor(compiler): modularizar codegen (builtins, methods) y eliminar restos del contexto monolítico"
```

- [ ] **Step 2: Commit del refactor del intérprete**

```bash
git add src/interpreter/
git commit -m "refactor(interpreter): deduplicar evaluación de métodos de string (~250 líneas)"
```

- [ ] **Step 3: Commit del grupo misc (según clasificación real de Task 3)**

```bash
git add src/lexer/ src/utils/ src/diagnostics/
git commit -m "chore: ajustes en lexer, utilidades y códigos de diagnóstico"
```

(Si el diff de Task 3 muestra que lexer/utils/diagnostics pertenecen a otro tema, usar el mensaje correcto, no forzar.)

- [ ] **Step 4: Commit de docs LSP**

```bash
git add LSP_ARCHITECTURE.md LSP_IMPLEMENTATION_GUIDE.md .gitignore
git commit -m "docs(lsp): guías de arquitectura e implementación + gitignore de binarios"
```

- [ ] **Step 5: Verificar estado final limpio**

```bash
git status --short
```
Expected: solo `?? docs/superpowers/` (spec + plan de esta fase, commiteados en Task 7) y `M README.md` (Task 6).

### Task 6: Sincronizar README con la realidad

**Files:**
- Modify: `README.md` (sección 🗺️ Roadmap y tabla ✅ Implementado)

**Interfaces:**
- Consumes: features reales verificadas en Task 1 (arrays, métodos de string).
- Produces: README sin features fantasma; base para el rebranding de la Etapa 2.

- [ ] **Step 1: Actualizar checklists**

Marcar como hecho en v0.3.0: `Arrays / Listas con indexación`, `Métodos de string`. Quedan pendientes: `Diccionarios / Maps`, `Iteración con for`.

- [ ] **Step 2: Verificar que no quedan invenciones**

Comparar la tabla "✅ Implementado" contra `cargo test` real (los suites nombran las features).

- [ ] **Step 3: Commit**

```bash
git add README.md docs/superpowers/
git commit -m "docs: sincronizar roadmap con el estado real y registrar spec/plan de redirección"
```

### Task 6b: Push

- [ ] **Step 1: Push a origin/main**

```bash
git push origin main
```

- [ ] **Step 2: Verificación**

```bash
git status -sb | head -2
```
Expected: `## main...origin/main` sin "ahead".
