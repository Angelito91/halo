pub mod conversions;
pub mod io;
pub mod math;
pub mod strings;

use crate::compiler::codegen::generator::CodeGenerator;
use crate::parser::ast::Expression;
use inkwell::values::BasicValueEnum;

/// Main entry point for built-in function code generation.
/// Returns Some(value) if the name is a built-in function, None otherwise.
pub fn emit_builtin<'ctx>(
    gen: &mut CodeGenerator<'ctx>,
    name: &str,
    args: &[Expression],
) -> Result<Option<BasicValueEnum<'ctx>>, String> {
    match name {
        "print" | "println" => io::emit_print(gen, args, name == "println").map(Some),
        "str" => conversions::emit_str(gen, args).map(Some),
        "int" => conversions::emit_int(gen, args).map(Some),
        "float" => conversions::emit_float(gen, args).map(Some),
        "len" => conversions::emit_len(gen, args).map(Some),
        "type" => conversions::emit_type(gen, args).map(Some),
        "abs" => math::emit_abs(gen, args).map(Some),
        "upper" => strings::emit_upper(gen, args).map(Some),
        "lower" => strings::emit_lower(gen, args).map(Some),
        "trim" => strings::emit_trim(gen, args).map(Some),
        "starts_with" => strings::emit_starts_with(gen, args).map(Some),
        "ends_with" => strings::emit_ends_with(gen, args).map(Some),
        "contains" => strings::emit_contains(gen, args).map(Some),
        "index_of" => strings::emit_index_of(gen, args).map(Some),
        "reverse" => strings::emit_reverse(gen, args).map(Some),
        "substr" => strings::emit_substr(gen, args).map(Some),
        "replace" => strings::emit_replace(gen, args).map(Some),
        _ => Ok(None),
    }
}
