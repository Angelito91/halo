//! Congelado de comportamiento: cada caso se ejecuta en el intérprete y en un
//! binario compilado, y debe comportarse igual — o estar declarado como
//! divergencia conocida en el propio caso (ver `common::cases::CASES`).

mod common;

use common::parity::Phase;
use common::parity::{self, KnownDivergence};

// ── El arnés no puede mentir ────────────────────────────────────────────────

#[test]
fn harness_flags_an_undeclared_divergence() {
    if !parity::clang_available() {
        return;
    }
    let err = parity::compare("bool_print", "print(true)", None).unwrap_err();
    assert!(err.contains("undocumented divergence"), "{err}");
}

#[test]
fn harness_accepts_a_declared_divergence() {
    if !parity::clang_available() {
        return;
    }
    let d = parity::case("bool_print").unwrap().divergence.as_ref();
    parity::compare("bool_print", "print(true)", d).unwrap();
}

#[test]
fn harness_flags_a_stale_entry() {
    if !parity::clang_available() {
        return;
    }
    let d = KnownDivergence {
        id: "int_truncation",
        reason: "test",
        phase: Phase::P1,
    };
    let err = parity::compare("int_truncation", "print(int(3.9))", Some(&d)).unwrap_err();
    assert!(err.contains("stale"), "{err}");
}

#[test]
fn harness_reports_parity_for_an_agreeing_case() {
    if !parity::clang_available() {
        return;
    }
    parity::compare("int_truncation", "print(int(3.9))", None).unwrap();
}

// ── La caché está acotada por el compilador ─────────────────────────────────

#[test]
fn the_cache_is_scoped_by_the_compiler_fingerprint() {
    // Dos huellas distintas no pueden compartir caché: si compartieran, un
    // codegen nuevo serviría el binario del viejo.
    assert_ne!(
        parity::cache_dir_for(1),
        parity::cache_dir_for(2),
        "distinct fingerprints must not share a cache directory"
    );
    assert_eq!(parity::cache_dir_for(7), parity::cache_dir_for(7));
}

#[test]
fn the_compiler_fingerprint_is_stable_within_a_run() {
    assert_eq!(
        parity::compiler_fingerprint(),
        parity::compiler_fingerprint(),
        "the fingerprint must not change during a run"
    );
    assert_ne!(
        parity::compiler_fingerprint(),
        0,
        "the fingerprint must be computed"
    );
}

// ── Corpus de paridad ───────────────────────────────────────────────────────
//
// El snippet de cada caso vive en `common::cases::CASES`; aquí solo se
// referencia su id, de modo que el registro y el corpus no pueden divergir.

#[test]
fn case_bool_print() {
    parity::assert_case("bool_print");
}
#[test]
fn case_float_print() {
    parity::assert_case("float_print");
}
#[test]
fn case_str_runtime_int() {
    parity::assert_case("str_runtime_int");
}
#[test]
fn case_str_runtime_float() {
    parity::assert_case("str_runtime_float");
}
#[test]
fn case_str_runtime_bool() {
    parity::assert_case("str_runtime_bool");
}
#[test]
fn case_int_from_string() {
    parity::assert_case("int_from_string");
}
#[test]
fn case_float_from_string() {
    parity::assert_case("float_from_string");
}
#[test]
fn case_int_truncation() {
    parity::assert_case("int_truncation");
}
#[test]
fn case_abs_float() {
    parity::assert_case("abs_float");
}
#[test]
fn case_array_literal() {
    parity::assert_case("array_literal");
}
#[test]
fn case_array_negative_idx() {
    parity::assert_case("array_negative_idx");
}
#[test]
fn case_array_methods() {
    parity::assert_case("array_methods");
}
#[test]
fn case_array_sort() {
    parity::assert_case("array_sort");
}
#[test]
fn case_array_join() {
    parity::assert_case("array_join");
}
#[test]
fn case_for_over_array() {
    parity::assert_case("for_over_array");
}
#[test]
fn case_range_as_value() {
    parity::assert_case("range_as_value");
}
#[test]
fn case_string_param() {
    parity::assert_case("string_param");
}
#[test]
fn case_string_concat_num() {
    parity::assert_case("string_concat_num");
}
#[test]
fn case_string_repeat() {
    parity::assert_case("string_repeat");
}
#[test]
fn case_int_overflow() {
    parity::assert_case("int_overflow");
}
#[test]
fn case_div_by_zero() {
    parity::assert_case("div_by_zero");
}
#[test]
fn case_mod_negative() {
    parity::assert_case("mod_negative");
}
#[test]
fn case_mixed_equality() {
    parity::assert_case("mixed_equality");
}
#[test]
fn case_print_multiarg() {
    parity::assert_case("print_multiarg");
}
#[test]
fn case_print_noargs() {
    parity::assert_case("print_noargs");
}
#[test]
fn case_len_number() {
    parity::assert_case("len_number");
}
#[test]
fn case_type_of_int() {
    parity::assert_case("type_of_int");
}
#[test]
fn case_string_methods() {
    parity::assert_case("string_methods");
}
#[test]
fn case_nested_call_str() {
    parity::assert_case("nested_call_str");
}
#[test]
fn case_global_mutation() {
    parity::assert_case("global_mutation");
}
#[test]
fn case_return_none() {
    parity::assert_case("return_none");
}
#[test]
fn case_early_return_loop() {
    parity::assert_case("early_return_loop");
}
#[test]
fn case_bool_var_decl() {
    parity::assert_case("bool_var_decl");
}
#[test]
fn case_println_builtin() {
    parity::assert_case("println_builtin");
}
#[test]
fn case_add_int() {
    parity::assert_case("add_int");
}
#[test]
fn case_sub_int() {
    parity::assert_case("sub_int");
}
#[test]
fn case_mul_int() {
    parity::assert_case("mul_int");
}
#[test]
fn case_div_int() {
    parity::assert_case("div_int");
}
#[test]
fn case_mod_int() {
    parity::assert_case("mod_int");
}
#[test]
fn case_add_float() {
    parity::assert_case("add_float");
}
#[test]
fn case_div_float() {
    parity::assert_case("div_float");
}
#[test]
fn case_mixed_promotion() {
    parity::assert_case("mixed_promotion");
}
#[test]
fn case_cmp_lt() {
    parity::assert_case("cmp_lt");
}
#[test]
fn case_cmp_gt() {
    parity::assert_case("cmp_gt");
}
#[test]
fn case_cmp_le() {
    parity::assert_case("cmp_le");
}
#[test]
fn case_cmp_ge() {
    parity::assert_case("cmp_ge");
}
#[test]
fn case_cmp_neq() {
    parity::assert_case("cmp_neq");
}
#[test]
fn case_logical_and() {
    parity::assert_case("logical_and");
}
#[test]
fn case_logical_or() {
    parity::assert_case("logical_or");
}
#[test]
fn case_short_circuit() {
    parity::assert_case("short_circuit");
}
#[test]
fn case_unary_not() {
    parity::assert_case("unary_not");
}
#[test]
fn case_unary_minus() {
    parity::assert_case("unary_minus");
}
#[test]
fn case_literal_int() {
    parity::assert_case("literal_int");
}
#[test]
fn case_literal_float() {
    parity::assert_case("literal_float");
}
#[test]
fn case_literal_string() {
    parity::assert_case("literal_string");
}
#[test]
fn case_literal_string_single_quote() {
    parity::assert_case("literal_string_single_quote");
}
#[test]
fn case_literal_string_escape() {
    parity::assert_case("literal_string_escape");
}
#[test]
fn case_if_true() {
    parity::assert_case("if_true");
}
#[test]
fn case_if_false() {
    parity::assert_case("if_false");
}
#[test]
fn case_if_else_if() {
    parity::assert_case("if_else_if");
}
#[test]
fn case_if_nested() {
    parity::assert_case("if_nested");
}
#[test]
fn case_while_loop() {
    parity::assert_case("while_loop");
}
#[test]
fn case_while_break() {
    parity::assert_case("while_break");
}
#[test]
fn case_while_continue() {
    parity::assert_case("while_continue");
}
#[test]
fn case_for_range() {
    parity::assert_case("for_range");
}
#[test]
fn case_for_range_two() {
    parity::assert_case("for_range_two");
}
#[test]
fn case_fn_call_before_def() {
    parity::assert_case("fn_call_before_def");
}
#[test]
fn case_fn_recursion() {
    parity::assert_case("fn_recursion");
}
#[test]
fn case_fn_mutual_recursion() {
    parity::assert_case("fn_mutual_recursion");
}
#[test]
fn case_fn_nested_call() {
    parity::assert_case("fn_nested_call");
}
#[test]
fn case_fn_returns_value() {
    parity::assert_case("fn_returns_value");
}
#[test]
fn case_fn_void() {
    parity::assert_case("fn_void");
}
#[test]
fn case_fn_arity_error() {
    parity::assert_case("fn_arity_error");
}
#[test]
fn case_fn_undefined() {
    parity::assert_case("fn_undefined");
}
#[test]
fn case_len_string() {
    parity::assert_case("len_string");
}
#[test]
fn case_len_array() {
    parity::assert_case("len_array");
}
#[test]
fn case_str_number() {
    parity::assert_case("str_number");
}
#[test]
fn case_str_float() {
    parity::assert_case("str_float");
}
#[test]
fn case_str_bool() {
    parity::assert_case("str_bool");
}
#[test]
fn case_int_from_float() {
    parity::assert_case("int_from_float");
}
#[test]
fn case_float_from_int() {
    parity::assert_case("float_from_int");
}
#[test]
fn case_abs_negative() {
    parity::assert_case("abs_negative");
}
#[test]
fn case_type_float() {
    parity::assert_case("type_float");
}
#[test]
fn case_type_bool() {
    parity::assert_case("type_bool");
}
#[test]
fn case_type_string() {
    parity::assert_case("type_string");
}
#[test]
fn case_type_null() {
    parity::assert_case("type_null");
}
#[test]
fn case_type_array() {
    parity::assert_case("type_array");
}
#[test]
fn case_str_upper() {
    parity::assert_case("str_upper");
}
#[test]
fn case_str_lower() {
    parity::assert_case("str_lower");
}
#[test]
fn case_str_trim() {
    parity::assert_case("str_trim");
}
#[test]
fn case_str_starts_with() {
    parity::assert_case("str_starts_with");
}
#[test]
fn case_str_ends_with() {
    parity::assert_case("str_ends_with");
}
#[test]
fn case_str_contains() {
    parity::assert_case("str_contains");
}
#[test]
fn case_str_index_of() {
    parity::assert_case("str_index_of");
}
#[test]
fn case_str_replace() {
    parity::assert_case("str_replace");
}
#[test]
fn case_str_substr() {
    parity::assert_case("str_substr");
}
#[test]
fn case_arr_push() {
    parity::assert_case("arr_push");
}
#[test]
fn case_arr_pop() {
    parity::assert_case("arr_pop");
}
#[test]
fn case_arr_append() {
    parity::assert_case("arr_append");
}
#[test]
fn case_arr_first() {
    parity::assert_case("arr_first");
}
#[test]
fn case_arr_last() {
    parity::assert_case("arr_last");
}
#[test]
fn case_arr_reverse() {
    parity::assert_case("arr_reverse");
}
#[test]
fn case_arr_index_of() {
    parity::assert_case("arr_index_of");
}
#[test]
fn case_arr_contains() {
    parity::assert_case("arr_contains");
}
#[test]
fn case_arr_empty() {
    parity::assert_case("arr_empty");
}
#[test]
fn case_arr_single() {
    parity::assert_case("arr_single");
}
#[test]
fn case_arr_nested() {
    parity::assert_case("arr_nested");
}
#[test]
fn case_arr_as_argument() {
    parity::assert_case("arr_as_argument");
}
#[test]
fn case_err_undefined_variable() {
    parity::assert_case("err_undefined_variable");
}
#[test]
fn case_err_index_out_of_range() {
    parity::assert_case("err_index_out_of_range");
}
#[test]
fn case_err_type_mismatch() {
    parity::assert_case("err_type_mismatch");
}
