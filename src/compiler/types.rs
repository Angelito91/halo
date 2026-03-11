// The Halo Programming Language
// Type mapping for LLVM IR generation using inkwell
// Version: 0.2.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use inkwell::context::Context;
use inkwell::types::{BasicTypeEnum, FloatType, FunctionType, IntType};
use inkwell::values::BasicValueEnum;

/// Maps Halo types to LLVM types via inkwell
pub struct TypeMapper<'ctx> {
    context: &'ctx Context,
}

impl<'ctx> TypeMapper<'ctx> {
    /// Create a new TypeMapper bound to a given LLVM context
    pub fn new(context: &'ctx Context) -> Self {
        TypeMapper { context }
    }

    // ===== Primitive LLVM types =====

    /// 64-bit signed integer type (default integer type)
    pub fn i64_type(&self) -> IntType<'ctx> {
        self.context.i64_type()
    }

    /// 32-bit signed integer type
    pub fn i32_type(&self) -> IntType<'ctx> {
        self.context.i32_type()
    }

    /// 1-bit integer type (booleans and comparison results)
    pub fn i1_type(&self) -> IntType<'ctx> {
        self.context.bool_type()
    }

    /// 64-bit floating point type
    pub fn f64_type(&self) -> FloatType<'ctx> {
        self.context.f64_type()
    }

    // ===== Function types =====

    /// Function type: (i64 x N) -> i64
    pub fn fn_i64_n(&self, param_count: usize) -> FunctionType<'ctx> {
        let i64_type = self.i64_type();
        let params: Vec<_> = vec![i64_type.into(); param_count];
        i64_type.fn_type(&params, false)
    }

    /// Printf-compatible variadic function type: i32 (ptr, ...)
    pub fn fn_printf(&self) -> FunctionType<'ctx> {
        let i32_type = self.i32_type();
        let i8_ptr = self.context.ptr_type(inkwell::AddressSpace::default());
        i32_type.fn_type(&[i8_ptr.into()], true)
    }

    // ===== Value-level helpers =====

    /// Return the canonical LLVM `BasicTypeEnum` for a runtime value.
    ///
    /// Rules:
    /// - float values  -> f64
    /// - i1 (bool)     -> i1   (kept as-is; callers that need i64 should call `widen`)
    /// - everything else -> i64
    pub fn llvm_type_of(val: BasicValueEnum<'ctx>) -> BasicTypeEnum<'ctx> {
        match val {
            BasicValueEnum::FloatValue(f) => f.get_type().into(),
            BasicValueEnum::IntValue(i) => i.get_type().into(),
            BasicValueEnum::PointerValue(p) => p.get_type().into(),
            BasicValueEnum::ArrayValue(a) => a.get_type().into(),
            BasicValueEnum::StructValue(s) => s.get_type().into(),
            BasicValueEnum::VectorValue(v) => v.get_type().into(),
            BasicValueEnum::ScalableVectorValue(v) => v.get_type().into(),
        }
    }

    /// Return a zero constant of the same type as `val`.
    /// Useful for generating default-initialised allocas.
    pub fn zero_of(&self, ty: BasicTypeEnum<'ctx>) -> BasicValueEnum<'ctx> {
        match ty {
            BasicTypeEnum::IntType(i) => i.const_zero().into(),
            BasicTypeEnum::FloatType(f) => f.const_zero().into(),
            BasicTypeEnum::PointerType(p) => p.const_zero().into(),
            BasicTypeEnum::ArrayType(a) => a.const_zero().into(),
            BasicTypeEnum::StructType(s) => s.const_zero().into(),
            BasicTypeEnum::VectorType(v) => v.const_zero().into(),
            BasicTypeEnum::ScalableVectorType(v) => v.const_zero().into(),
        }
    }

    /// Return the storage `BasicTypeEnum` that should be used when allocating
    /// a variable that holds `val`.  Floats stay as `f64`; everything else
    /// is widened to `i64` so that re-assignments of booleans/i32 are always
    /// stored in a uniform slot.
    pub fn storage_type_of(&self, val: BasicValueEnum<'ctx>) -> BasicTypeEnum<'ctx> {
        if val.is_float_value() {
            self.f64_type().into()
        } else {
            self.i64_type().into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_i64_type() {
        let context = Context::create();
        let mapper = TypeMapper::new(&context);
        assert_eq!(mapper.i64_type().get_bit_width(), 64);
    }

    #[test]
    fn test_i1_type() {
        let context = Context::create();
        let mapper = TypeMapper::new(&context);
        assert_eq!(mapper.i1_type().get_bit_width(), 1);
    }

    #[test]
    fn test_f64_type() {
        let context = Context::create();
        let mapper = TypeMapper::new(&context);
        let _ = mapper.f64_type();
    }

    #[test]
    fn test_fn_i64_n() {
        let context = Context::create();
        let mapper = TypeMapper::new(&context);
        let ft = mapper.fn_i64_n(3);
        assert_eq!(ft.count_param_types(), 3);
    }

    #[test]
    fn test_storage_type_of_int() {
        let context = Context::create();
        let mapper = TypeMapper::new(&context);
        let val: BasicValueEnum = mapper.i1_type().const_int(1, false).into();
        // booleans are stored as i64
        assert_eq!(mapper.storage_type_of(val), mapper.i64_type().into());
    }

    #[test]
    fn test_storage_type_of_float() {
        let context = Context::create();
        let mapper = TypeMapper::new(&context);
        let val: BasicValueEnum = mapper.f64_type().const_float(1.0).into();
        assert_eq!(mapper.storage_type_of(val), mapper.f64_type().into());
    }

    #[test]
    fn test_zero_of_int() {
        let context = Context::create();
        let mapper = TypeMapper::new(&context);
        let zero = mapper.zero_of(mapper.i64_type().into());
        assert!(zero.is_int_value());
    }

    #[test]
    fn test_zero_of_float() {
        let context = Context::create();
        let mapper = TypeMapper::new(&context);
        let zero = mapper.zero_of(mapper.f64_type().into());
        assert!(zero.is_float_value());
    }
}
