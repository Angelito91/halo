use crate::compiler::codegen::emitter::Emitter;
use crate::compiler::codegen::expressions;
use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;

/// Emit code for return statement
pub fn emit_return<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    value: &Option<Expression>,
    fn_name: &str,
) -> Result<bool, String> {
    let ret_val: BasicValueEnum<'ctx> = match value {
        Some(expr) => {
            let v = expressions::emit_expression(gen, expr)?;
            let ret_ty = gen.fn_return_types.get(fn_name).copied().unwrap_or("i64");
            match ret_ty {
                "string" => {
                    // Return string pointer directly (no widen)
                    if v.is_pointer_value() {
                        v
                    } else {
                        // If it's not a string (e.g. parameter that was passed as i64),
                        // convert via inttoptr (hack for ptr-as-i64 params)
                        let ptr_ty = gen.type_mapper.ptr_type();
                        let iv = Emitter::widen_to_i64(gen, v)?;
                        gen.builder
                            .build_int_to_ptr(iv, ptr_ty, "ret.ptr")
                            .map_err(|_| "return: int_to_ptr failed".to_string())?
                            .into()
                    }
                }
                "f64" => {
                    if v.is_float_value() {
                        v
                    } else {
                        Emitter::promote_to_f64(gen, v)?.into()
                    }
                }
                "bool" => gen.coerce_to_bool(v)?.into(),
                _ => Emitter::widen_to_i64(gen, v)?.into(),
            }
        }
        None => match gen.fn_return_types.get(fn_name).copied().unwrap_or("i64") {
            "string" => gen.type_mapper.ptr_type().const_null().into(),
            "f64" => gen.type_mapper.f64_type().const_zero().into(),
            "bool" => gen.type_mapper.i1_type().const_zero().into(),
            _ => gen.type_mapper.i64_type().const_int(0, false).into(),
        },
    };

    gen.builder
        .build_return(Some(&ret_val))
        .map_err(|_| "Failed to build return")?;
    Ok(true)
}
