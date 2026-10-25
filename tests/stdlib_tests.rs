//! Behaviour at the evaluator/stdlib boundary.
//!
//! The interpreter evaluates call arguments before dispatching to `stdlib`.
//! For valid calls this is invisible; for malformed calls it means an
//! argument error is reported before an arity or unknown-method error.

mod common;

use common::eval_code;

#[test]
fn arity_errors_are_still_reported_for_valid_arguments() {
    let err = eval_code("upper(1, 2)").unwrap_err();
    assert!(err.contains("expects exactly 1 argument"), "{err}");

    let err = eval_code("replace(1)").unwrap_err();
    assert!(err.contains("expects exactly 3 arguments"), "{err}");
}

#[test]
fn argument_errors_win_over_builtin_arity_errors() {
    let err = eval_code("upper(missing)").unwrap_err();
    assert!(err.contains("Undefined variable"), "{err}");
}

#[test]
fn argument_errors_win_over_unknown_method_errors() {
    let err = eval_code("x = [1]\nx.nope(missing)").unwrap_err();
    assert!(err.contains("Undefined variable"), "{err}");
}
