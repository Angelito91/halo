// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, GlobalValue};

use super::generator::CodeGenerator;
use crate::compiler::builder::IRBuilder;

/// Helper struct containing static functions for emission operations.
/// These functions handle printf calls, string management, type conversions,
/// and memory allocation operations.
pub struct Emitter;

impl Emitter {
    /// Emit a `printf` call for a literal string (e.g. `"\n"`, `" "`).
    pub fn emit_printf_str<'ctx>(gen: &mut CodeGenerator<'ctx>, s: &str) -> Result<(), String> {
        let printf = gen
            .module
            .get_function("printf")
            .ok_or("printf not declared")?;
        let fmt = Self::get_or_create_string(gen, s, &format!("str_{}", s.len()));
        let fmt_ptr: inkwell::values::BasicMetadataValueEnum = fmt.as_pointer_value().into();
        gen.builder
            .builder
            .build_call(printf, &[fmt_ptr], "printf_str")
            .map_err(|_| format!("Failed to call printf for {:?}", s))?;
        Ok(())
    }

    /// Emit a `printf` call that prints `val` with an appropriate format string.
    pub fn emit_printf_for_value<'ctx>(
        gen: &mut CodeGenerator<'ctx>,
        val: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let printf = gen
            .module
            .get_function("printf")
            .ok_or("printf not declared")?;

        if val.is_float_value() {
            // Ensure the value is f64 (promote f32 if necessary).
            let fv = Self::promote_to_f64(gen, val)?;
            let fmt = Self::get_or_create_string(gen, "%f", "fmt_f64");
            let fmt_ptr: inkwell::values::BasicMetadataValueEnum = fmt.as_pointer_value().into();
            let fval: inkwell::values::BasicMetadataValueEnum = fv.into();
            gen.builder
                .builder
                .build_call(printf, &[fmt_ptr, fval], "printf_f")
                .map_err(|_| "Failed to call printf (float)")?;
        } else if val.is_pointer_value() {
            // Pointer values are assumed to be null-terminated C strings (%s).
            let fmt = Self::get_or_create_string(gen, "%s", "fmt_str");
            let fmt_ptr: inkwell::values::BasicMetadataValueEnum = fmt.as_pointer_value().into();
            let sval: inkwell::values::BasicMetadataValueEnum = val.into_pointer_value().into();
            gen.builder
                .builder
                .build_call(printf, &[fmt_ptr, sval], "printf_s")
                .map_err(|_| "Failed to call printf (string)")?;
        } else {
            // All integer/bool values → widen to i64 and print as %lld.
            let wide = Self::widen_to_i64(gen, val)?;
            let fmt = Self::get_or_create_string(gen, "%lld", "fmt_i64");
            let fmt_ptr: inkwell::values::BasicMetadataValueEnum = fmt.as_pointer_value().into();
            let ival: inkwell::values::BasicMetadataValueEnum = wide.into();
            gen.builder
                .builder
                .build_call(printf, &[fmt_ptr, ival], "printf_i")
                .map_err(|_| "Failed to call printf (int)")?;
        }
        Ok(())
    }

    /// Get (or lazily create) a global null-terminated string constant.
    pub fn get_or_create_string<'ctx>(
        gen: &mut CodeGenerator<'ctx>,
        s: &str,
        name: &str,
    ) -> GlobalValue<'ctx> {
        if let Some(&gv) = gen.string_globals.get(s) {
            return gv;
        }
        let gv = gen
            .builder
            .builder
            .build_global_string_ptr(s, name)
            .expect("Failed to create global string");
        gen.string_globals.insert(s.to_string(), gv);
        gv
    }

    /// Emit an `alloca` instruction at the very start of the function's entry
    /// block so that mem2reg can promote it to a register.
    pub fn emit_entry_alloca<'ctx>(
        builder: &IRBuilder<'ctx>,
        function: FunctionValue<'ctx>,
        name: &str,
        ty: BasicTypeEnum<'ctx>,
    ) -> inkwell::values::PointerValue<'ctx> {
        // Temporarily reposition to the start of the entry block.
        let entry_bb = function
            .get_first_basic_block()
            .expect("function has no entry block");
        let saved_pos = builder.builder.get_insert_block();

        match entry_bb.get_first_instruction() {
            Some(first_instr) => builder.builder.position_before(&first_instr),
            None => builder.builder.position_at_end(entry_bb),
        }

        let alloca = builder
            .builder
            .build_alloca(ty, name)
            .expect("Failed to build alloca");

        // Restore the builder's original position.
        if let Some(bb) = saved_pos {
            builder.builder.position_at_end(bb);
        }

        alloca
    }

    /// Sign-extend any integer value to `i64`. No-op if already `i64`.
    /// Returns an error if called with a float (use `promote_to_f64` instead).
    pub fn widen_to_i64<'ctx>(
        gen: &mut CodeGenerator<'ctx>,
        val: BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::IntValue<'ctx>, String> {
        if val.is_float_value() {
            return Err("widen_to_i64: value is a float — use promote_to_f64 instead".to_string());
        }
        if val.is_pointer_value() {
            return Err("widen_to_i64: value is a pointer — likely a string. Use emit_printf_for_value for string values".to_string());
        }
        let iv = val.into_int_value();
        if iv.get_type().get_bit_width() == 64 {
            return Ok(iv);
        }
        gen.builder
            .builder
            .build_int_s_extend(iv, gen.type_mapper.i64_type(), "sext")
            .map_err(|_| "Failed to sign-extend to i64".to_string())
    }

    /// Promote any numeric value to `f64`.
    /// - `f64` → no-op
    /// - `f32` → `fpext`
    /// - integer → `sitofp`
    pub fn promote_to_f64<'ctx>(
        gen: &mut CodeGenerator<'ctx>,
        val: BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::FloatValue<'ctx>, String> {
        if val.is_float_value() {
            let fv = val.into_float_value();
            if fv.get_type().get_bit_width() == 64 {
                return Ok(fv);
            }
            return gen
                .builder
                .builder
                .build_float_ext(fv, gen.type_mapper.f64_type(), "fpext")
                .map_err(|_| "Failed to extend float to f64".to_string());
        }
        let iv = val.into_int_value();
        gen.builder
            .builder
            .build_signed_int_to_float(iv, gen.type_mapper.f64_type(), "sitofp")
            .map_err(|_| "Failed to convert int to f64".to_string())
    }
}
