# Refactorización del Compilador LLVM - Propuesta Estructural

> **Estado: ejecutado.** Este documento fue la propuesta previa al refactor.
> La estructura del compilador ya está partida como se describe en
> «Estructura real (post-R0)»; el resto del documento se conserva como
> registro de la decisión.
>
> **Cierre P1 (2026-10-23):** el módulo fija el target triple del host y el
> data layout del `TargetMachine`; el parser expone una única API
> `Parser::parse() -> Result<Program, Vec<ParseError>>`; `IRBuilder` mantiene
> privado su `Builder` de inkwell; y el backend compilado imprime booleanos,
> formatea floats y convierte `str`/`int`/`float` en runtime con la misma
> semántica que el intérprete. Los arrays siguen en P3 y están declarados en
> `tests/common/cases.rs`.

## 📋 Problema (estado anterior al refactor)

La estructura anterior del compilador funcionaba, pero tenía limitaciones:

```
src/compiler/
├── mod.rs           # Compilation wrapper
├── builder.rs       # IRBuilder (variable slots, allocation)
├── types.rs         # TypeMapper (LLVM type mapping)
└── codegen/
    ├── mod.rs
    ├── generator.rs # CodeGenerator (TODO: 1600+ líneas, demasiado grande)
    └── emitter.rs   # Emitter (linking, output)
```

**Problemas:**
1. **generator.rs es monolítico** (~1600 líneas) - mezcla responsabilidades
2. **Falta separación de concerns:**
   - Generación de expresiones vs statements
   - Manejo de tipos (primitivos vs complejos)
   - Funciones built-in vs funciones user-defined
3. **Difícil de extender** - agregar arrays requiere cambios en múltiples puntos
4. **Falta abstracción de generación** - operaciones comunes repetidas

---

## 🎯 Estructura real (post-R0)

```
src/
├── lang/                       # Fuente única de verdad (catálogo/formato/errores)
│   ├── catalog.rs
│   ├── format.rs
│   ├── errors.rs
│   └── mod.rs
├── parser/                     # Antes: parser.rs monolítico (804 líneas)
│   ├── ast.rs
│   ├── core.rs                 # Parser: struct, new/parse, cursores
│   ├── declarations.rs         # Nivel superior
│   ├── statements.rs
│   ├── expressions.rs
│   └── mod.rs
├── interpreter/
│   ├── value.rs
│   ├── environment.rs
│   └── evaluator/              # Antes: evaluator.rs monolítico
│       ├── core.rs
│       ├── statements.rs
│       ├── expressions.rs
│       ├── calls.rs
│       ├── builtins.rs
│       ├── methods/{mod,strings,arrays}.rs
│       └── mod.rs
├── compiler/
│   ├── mod.rs                  # Compilation, OptLevel
│   ├── builder.rs              # IRBuilder (allocas, variable slots)
│   ├── types.rs                # TypeMapper (Halo → LLVM)
│   └── codegen/
│       ├── mod.rs
│       ├── generator.rs        # CodeGenerator (orquestación) — 290 líneas
│       ├── module.rs           # libc y helpers de módulo (282)
│       ├── functions.rs        # funciones de usuario (183)
│       ├── emitter.rs          # salida de IR y enlace
│       ├── statements/{mod,expr_stmt,var_decl,while_loop,if_stmt,
│       │              for_loop,return_stmt,break_stmt,continue_stmt}.rs
│       ├── expressions/{mod,literals,arrays,binary_ops,unary_ops,
│       │               variables,function_call,method_call}.rs
│       ├── builtins/{mod,io,math,conversions}.rs
│       └── builtins/strings/{mod,helpers,trim,replace,substr,search,case}.rs
├── diagnostics/
│   ├── mod.rs                  # Diagnostic, localización y estadísticas
│   └── colors.rs
├── cli/                        # Antes: commands.rs monolítico (544 líneas)
│   ├── mod.rs
│   ├── args.rs
│   ├── run.rs build.rs check.rs tokens.rs ast.rs llvm.rs
│   └── helpers.rs              # UI + linkeo (clang/llc)
└── utils/
    ├── constants.rs            # BANNER, VERSION
    ├── similarity.rs
    └── mod.rs
```

Respecto a la propuesta original no se adoptaron `codegen/context.rs`
(`GeneratorContext`), `codegen/optimizer.rs`, `expressions/conditionals.rs`
ni `builtins/{types,arrays,strings}.rs` separados: el reparto actual cubre esas
responsabilidades con menos indirección (`module.rs`/`functions.rs` en vez de
`context.rs`; `builtins/strings/` como subdirectorio).

---

## 📊 Comparación: Antes vs Después

### **Responsabilidades por módulo**

| Aspecto | Antes | Después |
|---------|-------|---------|
| **generator.rs** | monolítico (~1600 líneas) | 290 líneas (orquestación) + `module.rs` + `functions.rs` |
| **Expresiones** | En generator.rs | `expressions/*.rs` (865 líneas) |
| **Statements** | En generator.rs | `statements/*.rs` (637 líneas) |
| **Built-ins** | En generator.rs | `builtins/*.rs` + `builtins/strings/*` (1507 líneas) |
| **Runtime support** | Disperso | Retirado: no tenía consumidores; arrays siguen en P3 |
| **Testing** | Difícil | Fácil (módulos pequeños) |

---

## 🔧 Detalles de Implementación

### **1. GeneratorContext (NEW)**

Centraliza estado compartido:

```rust
// codegen/context.rs
pub struct GeneratorContext<'ctx> {
    pub context: &'ctx Context,
    pub module: &'ctx Module<'ctx>,
    pub builder: &'ctx mut IRBuilder<'ctx>,
    pub type_mapper: &'ctx TypeMapper<'ctx>,
    pub functions: &'ctx HashMap<String, FunctionValue<'ctx>>,
    pub globals: &'ctx HashMap<String, GlobalValue<'ctx>>,
    pub loop_stack: &'ctx mut Vec<(BasicBlock<'ctx>, BasicBlock<'ctx>)>,
}
```

**Beneficio:** Evita pasar múltiples parámetros, reducimos `&mut self` en métodos.

---

### **2. Separación de Expresiones**

**Antes (generator.rs):**
```rust
fn eval_expr(&mut self, expr: &Expression) -> Result<BasicValueEnum, String> {
    match expr {
        Expression::Number(n) => { /* 5 líneas */ },
        Expression::Float(f) => { /* 5 líneas */ },
        Expression::String(s) => { /* 20 líneas */ },
        Expression::Array(items) => { /* ERROR: Not yet supported */ },
        Expression::BinOp(l, op, r) => { /* 50 líneas */ },
        // ... 20+ más cases
    }
}
```

**Después (expressions/*.rs):**
```rust
// expressions/mod.rs
pub mod literals;
pub mod arrays;
pub mod binary_ops;
// ...

impl<'ctx> ExpressionGenerator<'ctx> {
    pub fn eval_expr(&mut self, expr: &Expression) -> Result<BasicValueEnum, String> {
        match expr {
            Expression::Number(n) => literals::emit_number(self, *n),
            Expression::Float(f) => literals::emit_float(self, *f),
            Expression::String(s) => literals::emit_string(self, s),
            Expression::Array(items) => arrays::emit_array(self, items),
            Expression::BinOp(l, op, r) => binary_ops::emit_binop(self, l, op, r),
            // ... delegación clara
        }
    }
}

// expressions/arrays.rs
pub fn emit_array(gen: &mut ExpressionGenerator, items: &[Expression]) 
    -> Result<BasicValueEnum, String> 
{
    // Lógica de arrays aquí - fácil de extender
}
```

**Beneficio:** Cada módulo es testeable, mantenible, extensible.

---

### **3. Built-ins Organizados**

**Antes:**
```rust
// En generator.rs
fn builtin_len(&mut self, args: &[Expression]) { /* 30 líneas */ }
fn builtin_print(&mut self, args: &[Expression]) { /* 40 líneas */ }
fn builtin_type(&mut self, args: &[Expression]) { /* 35 líneas */ }
// ... etc
```

**Después:**
```rust
// builtins/io.rs
pub fn emit_print(gen: &mut ExpressionGenerator, args: &[Expression]) 
    -> Result<BasicValueEnum, String> { /* 40 líneas */ }

// builtins/arrays.rs
pub fn emit_len_array(gen: &mut ExpressionGenerator, arg: &Expression) 
    -> Result<BasicValueEnum, String> { /* 15 líneas */ }

// builtins/mod.rs - centraliza decisiones
pub fn emit_builtin(
    gen: &mut ExpressionGenerator,
    name: &str,
    args: &[Expression]
) -> Result<Option<BasicValueEnum>, String> {
    match name {
        "print" => io::emit_print(gen, args).map(Some),
        "len" => arrays::emit_len_array(gen, &args[0]).map(Some),
        // ...
        _ => Ok(None), // No es built-in
    }
}
```

**Beneficio:** Fácil localizar y modificar built-ins.

---

### **4. Runtime Support (histórico)**

> El módulo `codegen/runtime/` se eliminó en la limpieza posterior: estaba
> vacío y no tenía consumidores. La sección se conserva como registro de la
> propuesta original; si arrays entra en el compilado (P3), se decidirá entonces
> si conviene reintroducirlo.

Para arrays necesitamos funciones helper:

```rust
// runtime/arrays.rs
pub fn create_array_type(tm: &TypeMapper, elem_type: BasicTypeEnum) 
    -> StructTypeEnum {
    // Struct: { i64* data, i64 length, i64 capacity }
}

pub fn emit_array_malloc(gen: &ExpressionGenerator, count: i64) 
    -> Result<PointerValue, String> {
    // Aloca memoria para array
}

pub fn emit_array_push(gen: &ExpressionGenerator, array: PointerValue, value: BasicValueEnum) 
    -> Result<(), String> {
    // Push a array
}

// runtime/helpers.rs
pub fn emit_malloc(gen: &ExpressionGenerator, size: BasicValueEnum) 
    -> Result<PointerValue, String> {
    // Emite llamada a malloc
}
```

---

## 🛠️ Plan de Migración

> **Ejecutado** (fases 1–3) durante el refactor R0; la fase 4 (runtime de
> arrays en el compilado) queda para la fase **P3** del programa de paridad.

### **Fase 1: Preparación** (1-2 horas)
1. Crear `codegen/context.rs` con `GeneratorContext`
2. Crear estructura de directorios vacíos
3. Crear archivos `mod.rs` para cada submódulo

### **Fase 2: Extracción** (3-4 horas)
1. Extraer `eval_expr()` → `expressions/mod.rs`
2. Extraer handlers de statements → `statements/*.rs`
3. Extraer built-ins → `builtins/*.rs`
4. Actualizar `generator.rs` para usar módulos

### **Fase 3: Testing** (1-2 horas)
1. Verificar compilación sin errores
2. Ejecutar tests existentes
3. Verificar salida LLVM es idéntica

### **Fase 4: Arrays** (2-3 horas)
1. Implementar `runtime/arrays.rs`
2. Implementar `expressions/arrays.rs`
3. Agregar support en `builtins/arrays.rs`
4. Probar compilación con arrays

---

## 📈 Beneficios

| Aspecto | Antes | Después |
|---------|-------|---------|
| **Tamaño de generator.rs** | ~1600 líneas | ~200 líneas |
| **Testabilidad** | Difícil (monolítico) | Fácil (módulos) |
| **Extensibilidad** | Lenta (cambios globales) | Rápida (módulos) |
| **Tiempo para agregar feature** | ~3-4 horas | ~1-2 horas |
| **Curva de aprendizaje** | Alta | Media |
| **Reusabilidad de código** | Baja | Alta |

---

## 🎓 Ejemplo: Agregar Feature "Diccionarios"

### **Con estructura actual:**
1. Cambiar `eval_expr()` en generator.rs
2. Cambiar `eval_stmt()` en generator.rs
3. Modificar `TypeMapper` en types.rs
4. Agregar built-ins en generator.rs
5. Riesgo: romper algo existente

### **Con nueva estructura:**
1. Crear `expressions/dictionaries.rs`
2. Crear `runtime/dictionaries.rs`
3. Crear `builtins/dictionaries.rs`
4. Actualizar `expressions/mod.rs`
5. Actualizar `builtins/mod.rs`
6. Seguro: cambios localizados

---

## 📌 Decision Point

**¿Implementar refactorización antes de arrays?** → **Resuelto:** se eligió la
Opción A y se ejecutó (el compilador ya está partido). El soporte de arrays en
el backend compilado es ahora la fase P3 del programa de paridad.

### **Opción A: Refactorizar primero** ✅ ELEGIDA Y EJECUTADA
- Pro: Arrays quedan en estructura limpia
- Pro: Futuras features más fáciles
- Con: Extra 3-4 horas de trabajo
- Resultado: Código mantenible a largo plazo

### **Opción B: Arrays ahora, refactorizar después** ⚠️
- Pro: Arrays funcionan rápido
- Con: Código duplicado/confuso en generator.rs
- Con: Refactorización después es 2x más trabajo
- Resultado: Deuda técnica

**Recomendación:** Opción A - la refactorización se paga sola con la facilidad de mantener arrays.
