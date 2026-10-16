use crate::compiler::codegen::expressions;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;

pub fn emit_abs<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    args: &[Expression],
) -> Result<BasicValueEnum<'ctx>, String> {
    if args.len() != 1 {
        return Err(format!(
            "abs() expects exactly 1 argument, got {}",
            args.len()
        ));
    }
    let val = expressions::emit_expression(gen, &args[0])?;

    if val.is_float_value() {
        let fv = val.into_float_value();
        let abs_f = gen
            .module
            .get_function("llvm.fabs.f64")
            .or_else(|| {
                let f64_ty = gen.type_mapper.f64_type();
                let fn_ty = f64_ty.fn_type(&[f64_ty.into()], false);
                gen.module.add_function("llvm.fabs.f64", fn_ty, None).into()
            })
            .ok_or("Failed to get/create llvm.fabs.f64")?;
        let call_site = gen
            .builder
            .builder
            .build_call(abs_f, &[fv.into()], "abs.f64")
            .map_err(|_| "Failed to call llvm.fabs.f64")?;
        return match call_site.try_as_basic_value() {
            inkwell::values::ValueKind::Basic(v) => Ok(v),
            _ => Err("llvm.fabs.f64 returned instruction".to_string()),
        };
    }

    if val.is_pointer_value() {
        return Err("abs() does not support strings".to_string());
    }

    let iv = val.into_int_value();
    let zero = iv.get_type().const_zero();
    let neg = gen
        .builder
        .builder
        .build_int_sub(zero, iv, "neg")
        .map_err(|_| "Failed to negate for abs")?;
    let cond = gen
        .builder
        .builder
        .build_int_compare(inkwell::IntPredicate::SLT, iv, zero, "abs.cond")
        .map_err(|_| "Failed to compare for abs")?;
    gen.builder
        .builder
        .build_select(cond, neg, iv, "abs.result")
        .map(|v| v.into())
        .map_err(|_| "Failed to build select for abs".to_string())
}
