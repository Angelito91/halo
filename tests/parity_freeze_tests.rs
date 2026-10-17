//! Congelado de comportamiento: cada caso se ejecuta en el intérprete y en un
//! binario compilado, y debe comportarse igual — o estar declarado como
//! divergencia conocida en `KNOWN_DIVERGENCES` (ver el plan de R0).

mod common;

use common::parity::{self, KnownDivergence};

#[test]
fn harness_flags_an_undeclared_divergence() {
    if !parity::clang_available() { return; }
    let err = parity::compare("bool_print", "print(true)", &[]).unwrap_err();
    assert!(err.contains("undocumented divergence"), "{err}");
}

#[test]
fn harness_accepts_a_declared_divergence() {
    if !parity::clang_available() { return; }
    parity::compare("bool_print", "print(true)", parity::KNOWN_DIVERGENCES).unwrap();
}

#[test]
fn harness_flags_a_stale_entry() {
    if !parity::clang_available() { return; }
    let table = &[KnownDivergence { id: "int_truncation", reason: "test", phase: "P0" }];
    let err = parity::compare("int_truncation", "print(int(3.9))", table).unwrap_err();
    assert!(err.contains("stale"), "{err}");
}

#[test]
fn harness_reports_parity_for_an_agreeing_case() {
    if !parity::clang_available() { return; }
    parity::compare("int_truncation", "print(int(3.9))", &[]).unwrap();
}

// Corpus congelado: una prueba por caso (ver el plan, tabla de corpus).
#[test] fn case_bool_print() { parity::assert_parity("bool_print", "print(true)\nprint(false)"); }
#[test] fn case_float_print() { parity::assert_parity("float_print", "print(5.0)\nprint(3.14)\nprint(1.0 / 4.0)"); }
#[test] fn case_str_runtime_int() { parity::assert_parity("str_runtime_int", "n = 7\nprint(str(n))\nprint(\"n=\" + str(n))"); }
#[test] fn case_str_runtime_float() { parity::assert_parity("str_runtime_float", "x = 1.5\nprint(str(x))"); }
#[test] fn case_str_runtime_bool() { parity::assert_parity("str_runtime_bool", "b = true\nprint(str(b))"); }
#[test] fn case_int_from_string() { parity::assert_parity("int_from_string", "print(int(\"42\"))"); }
#[test] fn case_float_from_string() { parity::assert_parity("float_from_string", "print(float(\"1.5\"))"); }
#[test] fn case_int_truncation() { parity::assert_parity("int_truncation", "print(int(3.9))\nprint(int(-2.7))"); }
#[test] fn case_abs_float() { parity::assert_parity("abs_float", "print(abs(-3.14))\nprint(abs(-7))"); }
#[test] fn case_array_literal() { parity::assert_parity("array_literal", "a = [1, 2, 3]\nprint(len(a))\nprint(a[0])\nprint(a[2])"); }
#[test] fn case_array_negative_idx() { parity::assert_parity("array_negative_idx", "a = [1, 2, 3]\nprint(a[-1])"); }
#[test] fn case_array_methods() { parity::assert_parity("array_methods", "a = [1, 2, 3]\na.push(4)\nprint(len(a))\nprint(a.last())\na.reverse()\nprint(a[0])"); }
#[test] fn case_array_sort() { parity::assert_parity("array_sort", "a = [3, 1, 2]\na.sort()\nprint(a[0])"); }
#[test] fn case_array_join() { parity::assert_parity("array_join", "a = [\"x\", \"y\"]\nprint(a.join(\"-\"))"); }
#[test] fn case_for_over_array() { parity::assert_parity("for_over_array", "for v in [10, 20] { print(v) }"); }
#[test] fn case_range_as_value() { parity::assert_parity("range_as_value", "r = range(3)\nprint(len(r))\nprint(r[1])"); }
#[test] fn case_string_param() { parity::assert_parity("string_param", "greet(name) { print(name + \"!\") }\ngreet(\"hola\")"); }
#[test] fn case_string_concat_num() { parity::assert_parity("string_concat_num", "print(\"n=\" + 5)"); }
#[test] fn case_string_repeat() { parity::assert_parity("string_repeat", "print(\"ab\" * 3)"); }
#[test] fn case_int_overflow() { parity::assert_parity("int_overflow", "print(9223372036854775807 + 1)"); }
#[test] fn case_div_by_zero() { parity::assert_parity("div_by_zero", "print(1 / 0)"); }
#[test] fn case_mod_negative() { parity::assert_parity("mod_negative", "print(-7 % 3)\nprint(7 % -3)"); }
#[test] fn case_mixed_equality() { parity::assert_parity("mixed_equality", "print(1 == \"1\")\nprint(true == 1)"); }
#[test] fn case_print_multiarg() { parity::assert_parity("print_multiarg", "print(\"a\", \"b\", 1)"); }
#[test] fn case_print_noargs() { parity::assert_parity("print_noargs", "print()"); }
#[test] fn case_len_number() { parity::assert_parity("len_number", "print(len(12345))"); }
#[test] fn case_type_of_int() { parity::assert_parity("type_of_int", "print(type(42))"); }
#[test] fn case_string_methods() { parity::assert_parity("string_methods", "s = \" Hola \"\nprint(s.trim())\nprint(s.upper())\nprint(\"abc\".contains(\"b\"))"); }
#[test] fn case_nested_call_str() { parity::assert_parity("nested_call_str", "f(n) { return str(n) }\nprint(\"v=\" + f(3))"); }
#[test] fn case_global_mutation() { parity::assert_parity("global_mutation", "x = 1\nf() { x = 2 }\nf()\nprint(x)"); }
#[test] fn case_return_none() { parity::assert_parity("return_none", "f() { }\nprint(f())"); }
#[test] fn case_early_return_loop() { parity::assert_parity("early_return_loop", "f() {\nfor i in range(5) { if i == 2 { return i } }\nreturn -1\n}\nprint(f())"); }
#[test] fn case_bool_var_decl() { parity::assert_parity("bool_var_decl", "b = true\nprint(b)\nprint(\"ok\")"); }
#[test] fn case_println_builtin() { parity::assert_parity("println_builtin", "println(\"hola\")"); }
