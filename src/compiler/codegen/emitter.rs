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
            // Format with the same rules the interpreter uses, then print the
            // resulting string. Printing `%f` directly loses trailing-zero
            // trimming and always shows six decimals.
            let fv = Self::promote_to_f64(gen, val)?;
            let fptr = Self::format_float(gen, fv)?;
            let fmt = Self::get_or_create_string(gen, "%s", "fmt_str");
            let fmt_ptr: inkwell::values::BasicMetadataValueEnum = fmt.as_pointer_value().into();
            let sval: inkwell::values::BasicMetadataValueEnum = fptr.into();
            gen.builder
                .build_call(printf, &[fmt_ptr, sval], "printf_f")
                .map_err(|_| "Failed to call printf (float)")?;
        } else if val.is_pointer_value() {
            // Pointer values are assumed to be null-terminated C strings (%s).
            let fmt = Self::get_or_create_string(gen, "%s", "fmt_str");
            let fmt_ptr: inkwell::values::BasicMetadataValueEnum = fmt.as_pointer_value().into();
            let sval: inkwell::values::BasicMetadataValueEnum = val.into_pointer_value().into();
            gen.builder
                .build_call(printf, &[fmt_ptr, sval], "printf_s")
                .map_err(|_| "Failed to call printf (string)")?;
        } else {
            let iv = val.into_int_value();
            if iv.get_type().get_bit_width() == 1 {
                let sptr = Self::bool_string(gen, iv)?;
                let fmt = Self::get_or_create_string(gen, "%s", "fmt_str");
                let fmt_ptr: inkwell::values::BasicMetadataValueEnum =
                    fmt.as_pointer_value().into();
                let sval: inkwell::values::BasicMetadataValueEnum = sptr.into();
                gen.builder
                    .build_call(printf, &[fmt_ptr, sval], "printf_b")
                    .map_err(|_| "Failed to call printf (bool)")?;
            } else {
                // All integer values → widen to i64 and print as %lld.
                let wide = Self::widen_to_i64(gen, val)?;
                let fmt = Self::get_or_create_string(gen, "%lld", "fmt_i64");
                let fmt_ptr: inkwell::values::BasicMetadataValueEnum =
                    fmt.as_pointer_value().into();
                let ival: inkwell::values::BasicMetadataValueEnum = wide.into();
                gen.builder
                    .build_call(printf, &[fmt_ptr, ival], "printf_i")
                    .map_err(|_| "Failed to call printf (int)")?;
            }
        }
        Ok(())
    }

    /// Return a pointer to the static `"true"` or `"false"` string selected
    /// by `cond`.
    pub fn bool_string<'ctx>(
        gen: &mut CodeGenerator<'ctx>,
        cond: inkwell::values::IntValue<'ctx>,
    ) -> Result<inkwell::values::PointerValue<'ctx>, String> {
        let true_ptr = Self::get_or_create_string(gen, "true", "bool.true").as_pointer_value();
        let false_ptr = Self::get_or_create_string(gen, "false", "bool.false").as_pointer_value();
        let true_val: BasicValueEnum = true_ptr.into();
        let false_val: BasicValueEnum = false_ptr.into();
        gen.builder
            .build_select(cond, true_val, false_val, "bool.str")
            .map(|v| v.into_pointer_value())
            .map_err(|_| "Failed to select bool string".to_string())
    }

    /// Convert any scalar value into a null-terminated C string using the
    /// interpreter's formatting rules.
    pub fn value_to_string<'ctx>(
        gen: &mut CodeGenerator<'ctx>,
        val: BasicValueEnum<'ctx>,
    ) -> Result<inkwell::values::PointerValue<'ctx>, String> {
        if val.is_pointer_value() {
            return Ok(val.into_pointer_value());
        }
        if val.is_float_value() {
            let fv = val.into_float_value();
            if let Some((c, _lossless)) = fv.get_constant() {
                return Ok(Self::get_or_create_string(
                    gen,
                    &crate::lang::format::format_float(c),
                    "v2s.float",
                )
                .as_pointer_value());
            }
            return Self::format_float(gen, fv);
        }
        if val.is_int_value() {
            let iv = val.into_int_value();
            if iv.get_type().get_bit_width() == 1 {
                return Self::bool_string(gen, iv);
            }
            if iv.is_const() {
                let v = iv.get_sign_extended_constant().unwrap_or(0);
                return Ok(Self::get_or_create_string(
                    gen,
                    &crate::lang::format::format_number(v),
                    "v2s.int",
                )
                .as_pointer_value());
            }
            return Self::format_int(gen, iv);
        }
        Err("Cannot convert value to string in native compilation".to_string())
    }

    /// Format an integer into a freshly allocated C string.
    pub fn format_int<'ctx>(
        gen: &mut CodeGenerator<'ctx>,
        value: inkwell::values::IntValue<'ctx>,
    ) -> Result<inkwell::values::PointerValue<'ctx>, String> {
        let i64_type = gen.type_mapper.i64_type();
        let malloc = gen
            .module
            .get_function("malloc")
            .ok_or("malloc not declared")?;
        let snprintf = gen
            .module
            .get_function("snprintf")
            .ok_or("snprintf not declared")?;
        let buf = match gen
            .builder
            .build_call(
                malloc,
                &[i64_type.const_int(32, false).into()],
                "str.int.buf",
            )
            .map_err(|_| "malloc failed for str(int)")?
            .try_as_basic_value()
        {
            inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
            _ => return Err("malloc returned an instruction".to_string()),
        };
        let fmt = Self::get_or_create_string(gen, "%lld", "fmt_i64");
        gen.builder
            .build_call(
                snprintf,
                &[
                    buf.into(),
                    i64_type.const_int(32, false).into(),
                    fmt.as_pointer_value().into(),
                    value.into(),
                ],
                "snprintf_int",
            )
            .map_err(|_| "snprintf failed for str(int)")?;
        Ok(buf)
    }

    /// Format a float using the interpreter's rules (shortest round-trip
    /// representation, with `.0` for integral values).
    pub fn format_float<'ctx>(
        gen: &mut CodeGenerator<'ctx>,
        value: inkwell::values::FloatValue<'ctx>,
    ) -> Result<inkwell::values::PointerValue<'ctx>, String> {
        let i32_type = gen.type_mapper.i32_type();
        let i64_type = gen.type_mapper.i64_type();
        let malloc = gen
            .module
            .get_function("malloc")
            .ok_or("malloc not declared")?;
        let snprintf = gen
            .module
            .get_function("snprintf")
            .ok_or("snprintf not declared")?;
        let strchr = gen
            .module
            .get_function("strchr")
            .ok_or("strchr not declared")?;
        let strcat = gen
            .module
            .get_function("strcat")
            .ok_or("strcat not declared")?;

        let buf = match gen
            .builder
            .build_call(malloc, &[i64_type.const_int(64, false).into()], "str.f.buf")
            .map_err(|_| "malloc failed for str(float)")?
            .try_as_basic_value()
        {
            inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
            _ => return Err("malloc returned an instruction".to_string()),
        };
        let f64_type = gen.type_mapper.f64_type();
        let frac = gen
            .builder
            .build_float_rem(value, f64_type.const_float(1.0), "fract")
            .map_err(|_| "frem failed")?;
        let is_integral = gen
            .builder
            .build_float_compare(
                inkwell::FloatPredicate::OEQ,
                frac,
                f64_type.const_zero(),
                "is_integral",
            )
            .map_err(|_| "fcmp failed")?;
        let fmt_integral =
            Self::get_or_create_string(gen, "%.1f", "fmt_f64_integral").as_pointer_value();
        let fmt_fractional = Self::get_or_create_string(gen, "%.16g", "fmt_f64").as_pointer_value();
        let fmt_integral_val: BasicValueEnum = fmt_integral.into();
        let fmt_fractional_val: BasicValueEnum = fmt_fractional.into();
        let fmt = gen
            .builder
            .build_select(
                is_integral,
                fmt_integral_val,
                fmt_fractional_val,
                "fmt_f64_selected",
            )
            .map_err(|_| "select float format failed")?
            .into_pointer_value();
        gen.builder
            .build_call(
                snprintf,
                &[
                    buf.into(),
                    i64_type.const_int(64, false).into(),
                    fmt.into(),
                    value.into(),
                ],
                "snprintf_float",
            )
            .map_err(|_| "snprintf failed for str(float)")?;

        // `%g` omits the decimal point for integral values ("5"), while the
        // interpreter prints "5.0". Append ".0" unless the output already has
        // a decimal point/exponent or is inf/nan.
        let dot = match gen
            .builder
            .build_call(
                strchr,
                &[buf.into(), i32_type.const_int(b'.' as u64, false).into()],
                "strchr_dot",
            )
            .map_err(|_| "strchr failed")?
            .try_as_basic_value()
        {
            inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
            _ => return Err("strchr returned an instruction".to_string()),
        };
        let exp = match gen
            .builder
            .build_call(
                strchr,
                &[buf.into(), i32_type.const_int(b'e' as u64, false).into()],
                "strchr_exp",
            )
            .map_err(|_| "strchr failed")?
            .try_as_basic_value()
        {
            inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
            _ => return Err("strchr returned an instruction".to_string()),
        };
        let upper_exp = match gen
            .builder
            .build_call(
                strchr,
                &[buf.into(), i32_type.const_int(b'E' as u64, false).into()],
                "strchr_exp_upper",
            )
            .map_err(|_| "strchr failed")?
            .try_as_basic_value()
        {
            inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
            _ => return Err("strchr returned an instruction".to_string()),
        };
        let nan = match gen
            .builder
            .build_call(
                strchr,
                &[buf.into(), i32_type.const_int(b'n' as u64, false).into()],
                "strchr_nan",
            )
            .map_err(|_| "strchr failed")?
            .try_as_basic_value()
        {
            inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
            _ => return Err("strchr returned an instruction".to_string()),
        };
        let inf = match gen
            .builder
            .build_call(
                strchr,
                &[buf.into(), i32_type.const_int(b'i' as u64, false).into()],
                "strchr_inf",
            )
            .map_err(|_| "strchr failed")?
            .try_as_basic_value()
        {
            inkwell::values::ValueKind::Basic(v) => v.into_pointer_value(),
            _ => return Err("strchr returned an instruction".to_string()),
        };

        let dot_null = gen
            .builder
            .build_is_null(dot, "dot_null")
            .map_err(|_| "is_null failed")?;
        let exp_null = gen
            .builder
            .build_is_null(exp, "exp_null")
            .map_err(|_| "is_null failed")?;
        let upper_exp_null = gen
            .builder
            .build_is_null(upper_exp, "exp_upper_null")
            .map_err(|_| "is_null failed")?;
        let nan_null = gen
            .builder
            .build_is_null(nan, "nan_null")
            .map_err(|_| "is_null failed")?;
        let inf_null = gen
            .builder
            .build_is_null(inf, "inf_null")
            .map_err(|_| "is_null failed")?;
        let no_dot = gen
            .builder
            .build_and(dot_null, exp_null, "no_dot")
            .map_err(|_| "and failed")?;
        let no_upper_exp = gen
            .builder
            .build_and(no_dot, upper_exp_null, "no_upper_exp")
            .map_err(|_| "and failed")?;
        let not_nan = gen
            .builder
            .build_and(no_upper_exp, nan_null, "not_nan")
            .map_err(|_| "and failed")?;
        let need_suffix = gen
            .builder
            .build_and(not_nan, inf_null, "need_suffix")
            .map_err(|_| "and failed")?;

        let suffix = Self::get_or_create_string(gen, ".0", "fmt_float_suffix").as_pointer_value();
        let empty = Self::get_or_create_string(gen, "", "fmt_empty").as_pointer_value();
        let suffix_val: BasicValueEnum = suffix.into();
        let empty_val: BasicValueEnum = empty.into();
        let selected = gen
            .builder
            .build_select(need_suffix, suffix_val, empty_val, "float_suffix")
            .map_err(|_| "select failed")?
            .into_pointer_value();
        gen.builder
            .build_call(strcat, &[buf.into(), selected.into()], "append_suffix")
            .map_err(|_| "strcat failed")?;
        Ok(buf)
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
        let saved_pos = builder.get_insert_block();

        match entry_bb.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry_bb),
        }

        let alloca = builder
            .build_alloca(ty, name)
            .expect("Failed to build alloca");

        // Restore the builder's original position.
        if let Some(bb) = saved_pos {
            builder.position_at_end(bb);
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
        // Booleans are zero-extended (true == 1), not sign-extended
        // (which would turn true into -1).
        if iv.get_type().get_bit_width() == 1 {
            return gen
                .builder
                .build_int_z_extend(iv, gen.type_mapper.i64_type(), "zext")
                .map_err(|_| "Failed to zero-extend bool to i64".to_string());
        }
        gen.builder
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
                .build_float_ext(fv, gen.type_mapper.f64_type(), "fpext")
                .map_err(|_| "Failed to extend float to f64".to_string());
        }
        let iv = val.into_int_value();
        // Boolean i1 is 0/1, not a signed 1-bit value (-1/0).
        let iv = if iv.get_type().get_bit_width() == 1 {
            gen.builder
                .build_int_z_extend(iv, gen.type_mapper.i64_type(), "bool.to.i64")
                .map_err(|_| "Failed to zero-extend bool".to_string())?
        } else {
            iv
        };
        gen.builder
            .build_signed_int_to_float(iv, gen.type_mapper.f64_type(), "sitofp")
            .map_err(|_| "Failed to convert int to f64".to_string())
    }
}
