# Refactorización del Compilador LLVM - Propuesta Estructural

## 📋 Problema Actual

La estructura actual del compilador funciona, pero tiene limitaciones:

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

## 🎯 Estructura Propuesta (v2)

```
src/compiler/
├── mod.rs                    # Exports públicos
├── compilation.rs            # Compilation (la actual + context)
├── types.rs                  # TypeMapper (sin cambios)
├── builder.rs                # IRBuilder (sin cambios)
│
├── codegen/
│   ├── mod.rs
│   ├── generator.rs          # CodeGenerator (solo orquestación)
│   ├── context.rs            # GeneratorContext (estado compartido)
│   │
│   ├── statements/           # NEW: Generación de statements
│   │   ├── mod.rs
│   │   ├── expr_stmt.rs      # Expression statements
│   │   ├── var_decl.rs       # Variable declarations
│   │   ├── while_loop.rs     # While loops
│   │   ├── if_stmt.rs        # If statements
│   │   └── return_stmt.rs    # Return statements
│   │
│   ├── expressions/          # NEW: Generación de expressions
│   │   ├── mod.rs
│   │   ├── literals.rs       # Numbers, floats, strings, bools
│   │   ├── arrays.rs         # Array literals y indexing
│   │   ├── binary_ops.rs     # Arithmetic, comparison, logic
│   │   ├── unary_ops.rs      # Negation, NOT
│   │   ├── variables.rs      # Variable references
│   │   ├── function_call.rs  # Function calls
│   │   └── conditionals.rs   # Ternary conditionals
│   │
│   ├── builtins/             # NEW: Funciones built-in
│   │   ├── mod.rs
│   │   ├── io.rs             # print()
│   │   ├── types.rs          # type(), str(), int(), float()
│   │   ├── arrays.rs         # len() for arrays, indexing
│   │   ├── strings.rs        # len() for strings
│   │   ├── math.rs           # abs()
│   │   └── conversions.rs    # Type conversions
│   │
│   ├── runtime/              # NEW: Runtime support
│   │   ├── mod.rs
│   │   ├── memory.rs         # Memory management (malloc, free)
│   │   ├── arrays.rs         # Array runtime support
│   │   ├── strings.rs        # String runtime support
│   │   └── helpers.rs        # Helper functions
│   │
│   ├── emitter.rs            # Output (LLVM IR, object, binary)
│   └── optimizer.rs          # Optimization passes
```

---

## 📊 Comparación: Antes vs Después

### **Responsabilidades por módulo**

| Aspecto | Antes | Después |
|---------|-------|---------|
| **generator.rs** | ~1600 líneas (TODO todo) | ~200 líneas (orquestación) |
| **Expresiones** | En generator.rs | `expressions/*.rs` |
| **Statements** | En generator.rs | `statements/*.rs` |
| **Built-ins** | En generator.rs | `builtins/*.rs` |
| **Runtime support** | Disperso | `runtime/*.rs` |
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

### **4. Runtime Support (NEW)**

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

**¿Implementar refactorización antes de arrays?**

### **Opción A: Refactorizar primero** ✅ RECOMENDADO
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
