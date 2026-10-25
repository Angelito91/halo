// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use inkwell::context::Context;
use inkwell::types::{BasicTypeEnum, FloatType, FunctionType, IntType};
use inkwell::values::BasicValueEnum;

/// Maps Halo types to LLVM types via inkwell
pub struct TypeMapper<'ctx> {
    pub context: &'ctx Context,
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

    /// 8-bit integer type (bytes / chars)
    pub fn i8_type(&self) -> IntType<'ctx> {
        self.context.i8_type()
    }

    /// Default pointer type (opaque i8*)
    pub fn ptr_type(&self) -> inkwell::types::PointerType<'ctx> {
        self.context.ptr_type(inkwell::AddressSpace::default())
    }

    // ===== Function types =====

    /// Function type: (i64 x N) -> ret_type where ret_type is "i64", "f64", or "string"
    pub fn fn_type_for(&self, param_count: usize, ret_ty: &str) -> FunctionType<'ctx> {
        let i64_type = self.i64_type();
        let params: Vec<_> = vec![i64_type.into(); param_count];
        match ret_ty {
            "string" => {
                let ptr_ty = self.ptr_type();
                ptr_ty.fn_type(&params, false)
            }
            "f64" => self.f64_type().fn_type(&params, false),
            "bool" => self.i1_type().fn_type(&params, false),
            _ => i64_type.fn_type(&params, false),
        }
    }

    /// Printf-compatible variadic function type: i32 (ptr, ...)
    pub fn fn_printf(&self) -> FunctionType<'ctx> {
        let i32_type = self.i32_type();
        let i8_ptr = self.context.ptr_type(inkwell::AddressSpace::default());
        i32_type.fn_type(&[i8_ptr.into()], true)
    }

    // ===== Value-level helpers =====

    /// Return the storage `BasicTypeEnum` that should be used when allocating
    /// a variable that holds `val`.  Floats stay as `f64`; pointer values
    /// (e.g. strings) are stored as opaque pointers; everything else is
    /// widened to `i64` so that re-assignments of booleans/i32 are always
    /// stored in a uniform slot.
    pub fn storage_type_of(&self, val: BasicValueEnum<'ctx>) -> BasicTypeEnum<'ctx> {
        if val.is_float_value() {
            self.f64_type().into()
        } else if val.is_pointer_value() {
            self.context
                .ptr_type(inkwell::AddressSpace::default())
                .into()
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
    fn test_fn_type_for_i64() {
        let context = Context::create();
        let mapper = TypeMapper::new(&context);
        let ft = mapper.fn_type_for(3, "i64");
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
}
