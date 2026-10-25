mod common;

use common::parity;
use halo::lang::catalog::{builtin, method, Support, BUILTINS, METHODS};

#[test]
fn catalog_has_no_duplicate_names() {
    let mut names: Vec<&str> = BUILTINS.iter().map(|b| b.name).collect();
    names.sort_unstable();
    let mut dedup = names.clone();
    dedup.dedup();
    assert_eq!(names, dedup, "duplicate builtin names in catalog");
}

#[test]
fn methods_catalog_has_no_duplicate_names() {
    let mut names: Vec<&str> = METHODS.iter().map(|m| m.name).collect();
    names.sort_unstable();
    let mut dedup = names.clone();
    dedup.dedup();
    assert_eq!(names, dedup, "duplicate method names in catalog");
}

#[test]
fn catalog_declares_the_measured_support() {
    // Verificado contra el probe del spec §1.1:
    assert_eq!(builtin("print").unwrap().support, Support::Both);
    assert_eq!(builtin("range").unwrap().support, Support::InterpreterOnly);
    assert_eq!(builtin("range").unwrap().arity, 1..=2);
    assert_eq!(builtin("println").unwrap().support, Support::CompilerOnly);
    assert_eq!(builtin("len").unwrap().support, Support::Both);
    assert_eq!(builtin("push").unwrap().support, Support::InterpreterOnly);
    assert_eq!(method("join").unwrap().support, Support::InterpreterOnly);
    assert_eq!(method("upper").unwrap().support, Support::Both);
    assert_eq!(method("substr").unwrap().arity, 2..=2);
    assert!(builtin("nope").is_none());
}

/// Snippet mínimo que ejercita un built-in.
///
/// Los built-ins son funciones globales: la forma es `nombre(arg, …)`. Las
/// de array reciben el array como primer argumento (`push([1], 2)`), y
/// `append` concatena **dos** arrays.
fn builtin_probe(name: &str) -> Option<&'static str> {
    Some(match name {
        "print" => "print(1)",
        "println" => "println(\"x\")",
        "len" => "print(len(\"ab\"))",
        "str" => "print(str(1))",
        "int" => "print(int(1.9))",
        "float" => "print(float(1))",
        "abs" => "print(abs(-1))",
        "type" => "print(type(1))",
        "range" => "print(len(range(2)))",
        "push" => "push([1], 2)",
        "pop" => "pop([1])",
        "append" => "append([1], [2])",
        "first" => "first([1])",
        "last" => "last([1])",
        "reverse" => "reverse(\"ab\")",
        "join" => "join([\"x\"], \"-\")",
        "sort" => "sort([1])",
        "contains" => "contains(\"ab\", \"a\")",
        "index_of" => "index_of(\"ab\", \"b\")",
        "upper" => "upper(\"a\")",
        "lower" => "lower(\"A\")",
        "trim" => "trim(\" a \")",
        "starts_with" => "starts_with(\"ab\", \"a\")",
        "ends_with" => "ends_with(\"ab\", \"b\")",
        "replace" => "replace(\"ab\", \"a\", \"c\")",
        "substr" => "substr(\"abc\", 0, 2)",
        _ => return None,
    })
}

/// Snippet mínimo que ejercita un método (con su receptor).
///
/// El catálogo de métodos no distingue el receptor: cuando el backend
/// compilado implementa la versión de string, la sonda usa un string.
fn method_probe(name: &str) -> Option<&'static str> {
    Some(match name {
        "push" => "a = [1]\na.push(2)",
        "pop" => "a = [1]\na.pop()",
        "first" => "a = [1]\nprint(a.first())",
        "last" => "a = [1]\nprint(a.last())",
        "join" => "a = [\"x\"]\nprint(a.join(\"-\"))",
        "sort" => "a = [1]\na.sort()",
        "reverse" => "print(\"ab\".reverse())",
        "contains" => "print(\"ab\".contains(\"a\"))",
        "index_of" => "print(\"ab\".index_of(\"b\"))",
        "upper" => "print(\"a\".upper())",
        "lower" => "print(\"A\".lower())",
        "trim" => "print(\" a \".trim())",
        "starts_with" => "print(\"ab\".starts_with(\"a\"))",
        "ends_with" => "print(\"ab\".ends_with(\"b\"))",
        "replace" => "print(\"ab\".replace(\"a\", \"c\"))",
        "substr" => "print(\"abc\".substr(0, 2))",
        _ => return None,
    })
}

#[test]
fn every_catalog_entry_has_a_probe() {
    for b in BUILTINS {
        assert!(
            builtin_probe(b.name).is_some(),
            "builtin '{}' has no probe in the catalog test",
            b.name
        );
    }
    for m in METHODS {
        assert!(
            method_probe(m.name).is_some(),
            "method '{}' has no probe in the catalog test",
            m.name
        );
    }
}

#[test]
fn catalog_matches_observed_behavior() {
    if !parity::clang_available() {
        return;
    }
    // Cada builtin declarado Both debe coincidir en los dos modos;
    // cada InterpreterOnly debe funcionar en el intérprete y no compilar.
    for b in BUILTINS {
        let code = builtin_probe(b.name).unwrap();
        let interpreter_ok = !parity::run_interpreter(code).is_error();
        let compiled_ok = parity::run_compiled(code)
            .map(|r| !r.is_error())
            .unwrap_or(false);
        match b.support {
            Support::Both => {
                assert!(interpreter_ok && compiled_ok, "{}: declared Both", b.name)
            }
            Support::InterpreterOnly => assert!(
                interpreter_ok && !compiled_ok,
                "{}: declared InterpreterOnly",
                b.name
            ),
            Support::CompilerOnly => assert!(
                !interpreter_ok && compiled_ok,
                "{}: declared CompilerOnly",
                b.name
            ),
        }
    }
    for m in METHODS {
        let code = method_probe(m.name).unwrap();
        let interpreter_ok = !parity::run_interpreter(code).is_error();
        let compiled_ok = parity::run_compiled(code)
            .map(|r| !r.is_error())
            .unwrap_or(false);
        match m.support {
            Support::Both => {
                assert!(
                    interpreter_ok && compiled_ok,
                    "method {}: declared Both",
                    m.name
                )
            }
            Support::InterpreterOnly => assert!(
                interpreter_ok && !compiled_ok,
                "method {}: declared InterpreterOnly",
                m.name
            ),
            Support::CompilerOnly => assert!(
                !interpreter_ok && compiled_ok,
                "method {}: declared CompilerOnly",
                m.name
            ),
        }
    }
}
