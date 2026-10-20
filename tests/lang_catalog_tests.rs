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

#[test]
fn catalog_matches_observed_behavior() {
    if !parity::clang_available() { return; }
    // Cada builtin declarado Both debe coincidir en los dos modos;
    // cada InterpreterOnly debe funcionar en el intérprete y no compilar.
    let cases: &[(&str, &str)] = &[
        ("print", "print(1)"),
        ("len", "print(len(\"ab\"))"),
        ("str", "print(str(1))"),
        ("int", "print(int(1.9))"),
        ("float", "print(float(1))"),
        ("abs", "print(abs(-1))"),
        ("type", "print(type(1))"),
        ("range", "print(len(range(2)))"),
        ("push", "a = [1]\na.push(2)\nprint(len(a))"),
    ];
    for (name, code) in cases {
        let support = builtin(name).unwrap().support;
        let interpreter_ok = !parity::run_interpreter(code).is_error();
        let compiled_ok = parity::run_compiled(code).map(|r| !r.is_error()).unwrap_or(false);
        match support {
            Support::Both => assert!(interpreter_ok && compiled_ok, "{name}: declared Both"),
            Support::InterpreterOnly => assert!(interpreter_ok && !compiled_ok, "{name}: declared InterpreterOnly"),
            Support::CompilerOnly => assert!(!interpreter_ok && compiled_ok, "{name}: declared CompilerOnly"),
        }
    }
}
