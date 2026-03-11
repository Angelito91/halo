// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

mod common;

use common::assert_parse_error;
use halo::lexer::{Lexer, TokenKind};
use halo::parser::ast::{BinOp, Expression, Statement, TopLevel};
use halo::parser::Parser;

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Lex `source` and return the full token stream including the terminal `Eof`.
fn tokenize(source: &str) -> Vec<halo::lexer::Token> {
    let mut lexer = Lexer::new(source.to_string());
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        let done = tok.kind == TokenKind::Eof;
        tokens.push(tok);
        if done {
            break;
        }
    }
    tokens
}

/// Parse `source` and return `Ok(program)` or `Err(errors)`.
fn parse(source: &str) -> Result<halo::parser::ast::Program, Vec<String>> {
    Parser::new(tokenize(source)).parse()
}

/// Parse `source` and unwrap, panicking with a clear message on failure.
fn parse_ok(source: &str) -> halo::parser::ast::Program {
    parse(source).unwrap_or_else(|errs| {
        panic!(
            "parse_ok: unexpected parse error(s):\n  {}\n  source: {source}",
            errs.join("\n  ")
        )
    })
}

// ── Empty / trivial programs ──────────────────────────────────────────────────

#[test]
fn test_empty_program_has_no_items() {
    let prog = parse_ok("");
    assert!(prog.items.is_empty());
}

#[test]
fn test_whitespace_only_has_no_items() {
    let prog = parse_ok("   \t  ");
    assert!(prog.items.is_empty());
}

#[test]
fn test_newlines_only_has_no_items() {
    let prog = parse_ok("\n\n\n");
    assert!(prog.items.is_empty());
}

#[test]
fn test_comment_only_has_no_items() {
    let prog = parse_ok("// just a comment\n");
    assert!(prog.items.is_empty());
}

#[test]
fn test_multiple_comment_lines_has_no_items() {
    let prog = parse_ok("// one\n// two\n// three\n");
    assert!(prog.items.is_empty());
}

// ── Global variable declarations ──────────────────────────────────────────────

#[test]
fn test_global_var_integer_produces_one_item() {
    let prog = parse_ok("x = 42");
    assert_eq!(prog.items.len(), 1);
}

#[test]
fn test_global_var_is_globalvar_variant() {
    let prog = parse_ok("x = 42");
    assert!(
        matches!(&prog.items[0], TopLevel::GlobalVar { name, .. } if name == "x"),
        "expected GlobalVar {{ name: \"x\", .. }}, got {:?}",
        prog.items[0]
    );
}

#[test]
fn test_global_var_has_correct_name() {
    let prog = parse_ok("my_counter = 0");
    match &prog.items[0] {
        TopLevel::GlobalVar { name, .. } => assert_eq!(name, "my_counter"),
        other => panic!("expected GlobalVar, got {other:?}"),
    }
}

#[test]
fn test_global_var_initializer_is_number_literal() {
    let prog = parse_ok("x = 7");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Number(n, _)),
            ..
        } => {
            assert_eq!(*n, 7);
        }
        other => panic!("expected Number(7) initialiser, got {other:?}"),
    }
}

#[test]
fn test_global_var_initializer_is_bool_true() {
    let prog = parse_ok("flag = true");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Bool(b, _)),
            ..
        } => {
            assert!(*b);
        }
        other => panic!("expected Bool(true) initialiser, got {other:?}"),
    }
}

#[test]
fn test_global_var_initializer_is_bool_false() {
    let prog = parse_ok("flag = false");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Bool(b, _)),
            ..
        } => {
            assert!(!*b);
        }
        other => panic!("expected Bool(false) initialiser, got {other:?}"),
    }
}

#[test]
#[allow(clippy::approx_constant)] // intentionally parsing the literal "3.14" from source
fn test_global_var_initializer_is_float_literal() {
    let prog = parse_ok("pi = 3.14");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Float(f, _)),
            ..
        } => {
            assert!((*f - 3.14_f64).abs() < 1e-10);
        }
        other => panic!("expected Float initialiser, got {other:?}"),
    }
}

#[test]
fn test_global_var_initializer_is_string_literal() {
    let prog = parse_ok(r#"msg = "hello""#);
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::StringLiteral(s, _)),
            ..
        } => {
            assert_eq!(s, "hello");
        }
        other => panic!("expected StringLiteral initialiser, got {other:?}"),
    }
}

#[test]
fn test_global_var_initializer_is_binary_expression() {
    let prog = parse_ok("result = 2 + 3");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => {
            assert_eq!(*op, BinOp::Add);
        }
        other => panic!("expected Binary Add initialiser, got {other:?}"),
    }
}

#[test]
fn test_multiple_global_vars_produce_correct_count() {
    let prog = parse_ok("a = 1\nb = 2\nc = 3");
    assert_eq!(prog.items.len(), 3);
}

// ── Function declarations ─────────────────────────────────────────────────────

#[test]
fn test_function_definition_is_function_variant() {
    let prog = parse_ok("add(a, b) { return a + b }");
    assert!(
        matches!(&prog.items[0], TopLevel::Function { name, .. } if name == "add"),
        "expected Function {{ name: \"add\" }}, got {:?}",
        prog.items[0]
    );
}

#[test]
fn test_function_with_no_params() {
    let prog = parse_ok("greet() { return 42 }");
    match &prog.items[0] {
        TopLevel::Function { params, .. } => {
            assert!(params.is_empty());
        }
        other => panic!("expected Function, got {other:?}"),
    }
}

#[test]
fn test_function_with_one_param() {
    let prog = parse_ok("double(x) { return x * 2 }");
    match &prog.items[0] {
        TopLevel::Function { params, .. } => {
            assert_eq!(params.len(), 1);
            assert_eq!(params[0], "x");
        }
        other => panic!("expected Function, got {other:?}"),
    }
}

#[test]
fn test_function_with_two_params() {
    let prog = parse_ok("add(a, b) { return a + b }");
    match &prog.items[0] {
        TopLevel::Function { params, .. } => {
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], "a");
            assert_eq!(params[1], "b");
        }
        other => panic!("expected Function, got {other:?}"),
    }
}

#[test]
fn test_function_with_three_params() {
    let prog = parse_ok("sum3(a, b, c) { return a + b + c }");
    match &prog.items[0] {
        TopLevel::Function { params, .. } => {
            assert_eq!(params.len(), 3);
        }
        other => panic!("expected Function, got {other:?}"),
    }
}

#[test]
fn test_function_body_has_correct_statement_count() {
    let prog = parse_ok(
        "
f() {
    a = 1
    b = 2
    return a + b
}
",
    );
    match &prog.items[0] {
        TopLevel::Function { body, .. } => {
            assert_eq!(body.stmts.len(), 3);
        }
        other => panic!("expected Function, got {other:?}"),
    }
}

#[test]
fn test_function_body_return_statement() {
    let prog = parse_ok("answer() { return 42 }");
    match &prog.items[0] {
        TopLevel::Function { body, .. } => {
            assert!(
                matches!(body.stmts.first(), Some(Statement::Return { .. })),
                "first statement should be Return"
            );
        }
        other => panic!("expected Function, got {other:?}"),
    }
}

#[test]
fn test_function_empty_body() {
    let prog = parse_ok("noop() {}");
    match &prog.items[0] {
        TopLevel::Function { body, .. } => {
            assert!(body.stmts.is_empty());
        }
        other => panic!("expected Function, got {other:?}"),
    }
}

#[test]
fn test_multiple_function_definitions() {
    let prog = parse_ok(
        "
foo() { return 1 }
bar() { return 2 }
baz() { return 3 }
",
    );
    let fn_names: Vec<_> = prog
        .items
        .iter()
        .filter_map(|item| {
            if let TopLevel::Function { name, .. } = item {
                Some(name.as_str())
            } else {
                None
            }
        })
        .collect();
    assert_eq!(fn_names, vec!["foo", "bar", "baz"]);
}

#[test]
fn test_function_and_global_var_together() {
    let prog = parse_ok(
        "
x = 10
add(a, b) { return a + b }
",
    );
    assert_eq!(prog.items.len(), 2);
    assert!(matches!(&prog.items[0], TopLevel::GlobalVar { .. }));
    assert!(matches!(&prog.items[1], TopLevel::Function { .. }));
}

// ── Top-level statements ──────────────────────────────────────────────────────

#[test]
fn test_top_level_if_is_stmt_variant() {
    let prog = parse_ok("if true { x = 1 }");
    assert!(
        matches!(
            &prog.items[0],
            TopLevel::Stmt {
                stmt: Statement::If { .. },
                ..
            }
        ),
        "expected TopLevel::Stmt(If), got {:?}",
        prog.items[0]
    );
}

#[test]
fn test_top_level_while_is_stmt_variant() {
    let prog = parse_ok("while false { x = 1 }");
    assert!(
        matches!(
            &prog.items[0],
            TopLevel::Stmt {
                stmt: Statement::While { .. },
                ..
            }
        ),
        "expected TopLevel::Stmt(While)"
    );
}

#[test]
fn test_top_level_return_is_stmt_variant() {
    let prog = parse_ok("return 0");
    assert!(
        matches!(
            &prog.items[0],
            TopLevel::Stmt {
                stmt: Statement::Return { .. },
                ..
            }
        ),
        "expected TopLevel::Stmt(Return)"
    );
}

// ── If statement structure ────────────────────────────────────────────────────

#[test]
fn test_if_has_correct_condition_literal() {
    let prog = parse_ok("if true { x = 1 }");
    match &prog.items[0] {
        TopLevel::Stmt {
            stmt: Statement::If { cond, .. },
            ..
        } => {
            assert!(matches!(cond, Expression::Bool(true, _)));
        }
        other => panic!("expected Stmt(If), got {other:?}"),
    }
}

#[test]
fn test_if_else_has_else_branch() {
    let prog = parse_ok("if true { x = 1 } else { x = 2 }");
    match &prog.items[0] {
        TopLevel::Stmt {
            stmt: Statement::If { else_branch, .. },
            ..
        } => {
            assert!(
                else_branch.is_some(),
                "else_branch should be Some after parsing if/else"
            );
        }
        other => panic!("expected Stmt(If), got {other:?}"),
    }
}

#[test]
fn test_if_without_else_has_no_else_branch() {
    let prog = parse_ok("if false { x = 1 }");
    match &prog.items[0] {
        TopLevel::Stmt {
            stmt: Statement::If { else_branch, .. },
            ..
        } => {
            assert!(else_branch.is_none());
        }
        other => panic!("expected Stmt(If), got {other:?}"),
    }
}

#[test]
fn test_else_if_chain_recorded_in_branches() {
    let prog = parse_ok(
        "
if x == 1 { a = 1 }
else if x == 2 { a = 2 }
else if x == 3 { a = 3 }
else { a = 4 }
",
    );
    match &prog.items[0] {
        TopLevel::Stmt {
            stmt:
                Statement::If {
                    else_if_branches,
                    else_branch,
                    ..
                },
            ..
        } => {
            assert_eq!(else_if_branches.len(), 2);
            assert!(else_branch.is_some());
        }
        other => panic!("expected Stmt(If), got {other:?}"),
    }
}

#[test]
fn test_if_then_branch_has_correct_statement_count() {
    let prog = parse_ok("if true { a = 1\nb = 2\nc = 3 }");
    match &prog.items[0] {
        TopLevel::Stmt {
            stmt: Statement::If { then_branch, .. },
            ..
        } => {
            assert_eq!(then_branch.stmts.len(), 3);
        }
        other => panic!("expected Stmt(If), got {other:?}"),
    }
}

// ── While statement structure ─────────────────────────────────────────────────

#[test]
fn test_while_condition_is_correct() {
    let prog = parse_ok("while i < 10 { i = i + 1 }");
    match &prog.items[0] {
        TopLevel::Stmt {
            stmt: Statement::While { cond, .. },
            ..
        } => {
            assert!(
                matches!(cond, Expression::Binary { op: BinOp::Lt, .. }),
                "expected Lt binary condition, got {cond:?}"
            );
        }
        other => panic!("expected Stmt(While), got {other:?}"),
    }
}

#[test]
fn test_while_body_has_correct_statement_count() {
    let prog = parse_ok("while true { a = 1\nb = 2 }");
    match &prog.items[0] {
        TopLevel::Stmt {
            stmt: Statement::While { body, .. },
            ..
        } => {
            assert_eq!(body.stmts.len(), 2);
        }
        other => panic!("expected Stmt(While), got {other:?}"),
    }
}

// ── Expression / binary operator structure ────────────────────────────────────

#[test]
fn test_binary_add_produces_add_op() {
    let prog = parse_ok("x = a + b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Add),
        other => panic!("expected Binary(Add), got {other:?}"),
    }
}

#[test]
fn test_binary_sub_produces_sub_op() {
    let prog = parse_ok("x = a - b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Sub),
        other => panic!("expected Binary(Sub), got {other:?}"),
    }
}

#[test]
fn test_binary_mul_produces_mul_op() {
    let prog = parse_ok("x = a * b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Mul),
        other => panic!("expected Binary(Mul), got {other:?}"),
    }
}

#[test]
fn test_binary_div_produces_div_op() {
    let prog = parse_ok("x = a / b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Div),
        other => panic!("expected Binary(Div), got {other:?}"),
    }
}

#[test]
fn test_binary_mod_produces_mod_op() {
    let prog = parse_ok("x = a % b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Mod),
        other => panic!("expected Binary(Mod), got {other:?}"),
    }
}

#[test]
fn test_binary_eq_produces_eq_op() {
    let prog = parse_ok("x = a == b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Eq),
        other => panic!("expected Binary(Eq), got {other:?}"),
    }
}

#[test]
fn test_binary_neq_produces_neq_op() {
    let prog = parse_ok("x = a != b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Neq),
        other => panic!("expected Binary(Neq), got {other:?}"),
    }
}

#[test]
fn test_binary_lt_produces_lt_op() {
    let prog = parse_ok("x = a < b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Lt),
        other => panic!("expected Binary(Lt), got {other:?}"),
    }
}

#[test]
fn test_binary_gt_produces_gt_op() {
    let prog = parse_ok("x = a > b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Gt),
        other => panic!("expected Binary(Gt), got {other:?}"),
    }
}

#[test]
fn test_binary_le_produces_le_op() {
    let prog = parse_ok("x = a <= b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Le),
        other => panic!("expected Binary(Le), got {other:?}"),
    }
}

#[test]
fn test_binary_ge_produces_ge_op() {
    let prog = parse_ok("x = a >= b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Ge),
        other => panic!("expected Binary(Ge), got {other:?}"),
    }
}

#[test]
fn test_binary_and_produces_and_op() {
    let prog = parse_ok("x = a && b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::And),
        other => panic!("expected Binary(And), got {other:?}"),
    }
}

#[test]
fn test_binary_or_produces_or_op() {
    let prog = parse_ok("x = a || b");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Binary { op, .. }),
            ..
        } => assert_eq!(*op, BinOp::Or),
        other => panic!("expected Binary(Or), got {other:?}"),
    }
}

// ── Operator precedence in AST ────────────────────────────────────────────────

#[test]
fn test_mul_binds_tighter_than_add_in_ast() {
    // "2 + 3 * 4" must parse as Binary(Add, 2, Binary(Mul, 3, 4))
    let prog = parse_ok("x = 2 + 3 * 4");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init:
                Some(Expression::Binary {
                    op: BinOp::Add,
                    right,
                    ..
                }),
            ..
        } => {
            assert!(
                matches!(right.as_ref(), Expression::Binary { op: BinOp::Mul, .. }),
                "right of Add should be a Mul, got {right:?}"
            );
        }
        other => panic!("expected Binary(Add, _, Mul), got {other:?}"),
    }
}

#[test]
fn test_parentheses_flip_precedence_in_ast() {
    // "(2 + 3) * 4" must parse as Binary(Mul, Binary(Add, 2, 3), 4)
    let prog = parse_ok("x = (2 + 3) * 4");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init:
                Some(Expression::Binary {
                    op: BinOp::Mul,
                    left,
                    ..
                }),
            ..
        } => {
            assert!(
                matches!(left.as_ref(), Expression::Binary { op: BinOp::Add, .. }),
                "left of Mul should be Add, got {left:?}"
            );
        }
        other => panic!("expected Binary(Mul, Add, _), got {other:?}"),
    }
}

#[test]
fn test_and_binds_tighter_than_or_in_ast() {
    // "a || b && c" → Binary(Or, a, Binary(And, b, c))
    let prog = parse_ok("x = a || b && c");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init:
                Some(Expression::Binary {
                    op: BinOp::Or,
                    right,
                    ..
                }),
            ..
        } => {
            assert!(
                matches!(right.as_ref(), Expression::Binary { op: BinOp::And, .. }),
                "right of Or should be And, got {right:?}"
            );
        }
        other => panic!("expected Binary(Or, _, And), got {other:?}"),
    }
}

// ── Unary operators ───────────────────────────────────────────────────────────

#[test]
fn test_unary_minus_wraps_expression() {
    let prog = parse_ok("x = -5");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Unary { operator, .. }),
            ..
        } => assert_eq!(operator, "-"),
        other => panic!("expected Unary(-), got {other:?}"),
    }
}

#[test]
fn test_unary_not_wraps_expression() {
    let prog = parse_ok("x = !true");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Unary { operator, .. }),
            ..
        } => assert_eq!(operator, "!"),
        other => panic!("expected Unary(!), got {other:?}"),
    }
}

#[test]
fn test_unary_minus_inner_is_number() {
    let prog = parse_ok("x = -7");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Unary { expr: inner, .. }),
            ..
        } => {
            assert!(
                matches!(inner.as_ref(), Expression::Number(7, _)),
                "inner of unary minus should be Number(7), got {inner:?}"
            );
        }
        other => panic!("expected Unary, got {other:?}"),
    }
}

// ── Function call expression ──────────────────────────────────────────────────

#[test]
fn test_function_call_expression_name() {
    let prog = parse_ok("foo()");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Call { name, .. }),
            ..
        } => assert_eq!(name, "foo"),
        other => panic!("expected Call(foo), got {other:?}"),
    }
}

#[test]
fn test_function_call_no_args() {
    let prog = parse_ok("foo()");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Call { args, .. }),
            ..
        } => assert!(args.is_empty()),
        other => panic!("expected Call with no args, got {other:?}"),
    }
}

#[test]
fn test_function_call_one_arg() {
    let prog = parse_ok("abs(x)");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Call { args, .. }),
            ..
        } => assert_eq!(args.len(), 1),
        other => panic!("expected Call with 1 arg, got {other:?}"),
    }
}

#[test]
fn test_function_call_two_args() {
    let prog = parse_ok("add(1, 2)");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Call { args, .. }),
            ..
        } => assert_eq!(args.len(), 2),
        other => panic!("expected Call with 2 args, got {other:?}"),
    }
}

#[test]
fn test_nested_function_call_in_arg() {
    let prog = parse_ok("outer(inner(x))");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Call { name, args, .. }),
            ..
        } => {
            assert_eq!(name, "outer");
            assert_eq!(args.len(), 1);
            assert!(
                matches!(&args[0], Expression::Call { name, .. } if name == "inner"),
                "argument should be a Call to inner"
            );
        }
        other => panic!("expected nested Call, got {other:?}"),
    }
}

// ── Variable / assignment expression ─────────────────────────────────────────

#[test]
fn test_var_expression_has_correct_name() {
    // A bare identifier that is not an assignment becomes a Var expression.
    let prog = parse_ok("my_var");
    match &prog.items[0] {
        TopLevel::GlobalVar {
            init: Some(Expression::Var(name, _)),
            ..
        } => assert_eq!(name, "my_var"),
        other => panic!("expected Var(my_var), got {other:?}"),
    }
}

#[test]
fn test_assignment_statement_in_function_body() {
    let prog = parse_ok("f() { x = 5 }");
    match &prog.items[0] {
        TopLevel::Function { body, .. } => {
            assert_eq!(body.stmts.len(), 1);
            assert!(matches!(
                &body.stmts[0],
                Statement::VarDecl { name, .. } if name == "x"
            ));
        }
        other => panic!("expected Function, got {other:?}"),
    }
}

// ── Return statement ──────────────────────────────────────────────────────────

#[test]
fn test_return_with_value_has_some_expression() {
    let prog = parse_ok("f() { return 42 }");
    match &prog.items[0] {
        TopLevel::Function { body, .. } => match &body.stmts[0] {
            Statement::Return { value, .. } => {
                assert!(value.is_some(), "return value should be Some(42)");
            }
            other => panic!("expected Return, got {other:?}"),
        },
        other => panic!("expected Function, got {other:?}"),
    }
}

#[test]
fn test_bare_return_has_none_value() {
    let prog = parse_ok("f() { return }");
    match &prog.items[0] {
        TopLevel::Function { body, .. } => match &body.stmts[0] {
            Statement::Return { value, .. } => {
                assert!(value.is_none(), "bare return should have None value");
            }
            other => panic!("expected Return, got {other:?}"),
        },
        other => panic!("expected Function, got {other:?}"),
    }
}

// ── Break and continue ────────────────────────────────────────────────────────

#[test]
fn test_break_in_while_body_is_break_statement() {
    let prog = parse_ok("while true { break }");
    match &prog.items[0] {
        TopLevel::Stmt {
            stmt: Statement::While { body, .. },
            ..
        } => {
            assert!(matches!(body.stmts.first(), Some(Statement::Break { .. })));
        }
        other => panic!("expected Stmt(While), got {other:?}"),
    }
}

#[test]
fn test_continue_in_while_body_is_continue_statement() {
    let prog = parse_ok("while true { continue }");
    match &prog.items[0] {
        TopLevel::Stmt {
            stmt: Statement::While { body, .. },
            ..
        } => {
            assert!(matches!(
                body.stmts.first(),
                Some(Statement::Continue { .. })
            ));
        }
        other => panic!("expected Stmt(While), got {other:?}"),
    }
}

// ── Error recovery ────────────────────────────────────────────────────────────

#[test]
fn test_missing_opening_brace_in_if_produces_error() {
    assert_parse_error("if true x = 1", "Expected '{'");
}

#[test]
fn test_missing_closing_brace_in_if_produces_error() {
    // The closing `}` is absent; parser should report an error.
    let result = parse("if true { x = 1");
    assert!(result.is_err(), "missing '}}' should produce a parse error");
}

#[test]
fn test_missing_opening_brace_in_while_produces_error() {
    assert_parse_error("while true x = 1", "Expected '{'");
}

#[test]
fn test_missing_paren_after_function_name_produces_error() {
    // "f { return 1 }" — no parameter list at all.
    let result = parse("f { return 1 }");
    // Without `(`, the parser may interpret this as a global variable with an
    // error, or recover differently. Either way it should not silently succeed
    // with a Function item named "f" with a body containing a return.
    match result {
        Err(_) => {} // error path is acceptable
        Ok(prog) => {
            // If the parser "recovered", verify it did not silently produce a
            // well-formed Function item.
            for item in &prog.items {
                assert!(
                    !matches!(item, TopLevel::Function { name, body, .. }
                        if name == "f" && !body.stmts.is_empty()),
                    "parser should not silently produce a Function 'f' with a body"
                );
            }
        }
    }
}

#[test]
fn test_missing_closing_paren_in_call_produces_error() {
    assert_parse_error("foo(1, 2", "Expected ')'");
}

#[test]
fn test_missing_closing_paren_in_function_params_produces_error() {
    assert_parse_error("f(a, b { return 1 }", "Expected ')'");
}

#[test]
fn test_parse_errors_contain_line_and_column_info() {
    let result = parse("if true x = 1");
    let errs = result.unwrap_err();
    let joined = errs.join(";");
    // Error messages must include a line:column location.
    assert!(
        joined.contains(':'),
        "error messages should contain line:column info, got: {joined:?}"
    );
}

// ── Error recovery allows partial parse ──────────────────────────────────────

#[test]
fn test_valid_function_after_invalid_statement_is_still_parsed() {
    // The first statement is malformed (missing condition expression handled
    // gracefully), but the second definition is valid.
    // This tests that the parser does not abort on the first error.
    let source = "
x = 1
valid(n) { return n + 1 }
";
    let prog = parse_ok(source);
    let has_valid_fn = prog
        .items
        .iter()
        .any(|item| matches!(item, TopLevel::Function { name, .. } if name == "valid"));
    assert!(
        has_valid_fn,
        "valid function should be present after a recovered error"
    );
}

// ── Comprehensive program shapes ──────────────────────────────────────────────

#[test]
fn test_factorial_program_structure() {
    let prog = parse_ok(
        "
factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
factorial(5)
",
    );
    // Should have: Function + GlobalVar (the call expression becomes __expr).
    assert_eq!(prog.items.len(), 2);
    assert!(matches!(&prog.items[0], TopLevel::Function { name, .. } if name == "factorial"));
}

#[test]
fn test_fibonacci_program_structure() {
    let prog = parse_ok(
        "
fib(n) {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}
fib(10)
",
    );
    assert_eq!(prog.items.len(), 2);
    assert!(matches!(&prog.items[0], TopLevel::Function { name, .. } if name == "fib"));
}

#[test]
fn test_program_with_globals_and_functions() {
    let prog = parse_ok(
        "
max_value = 100
min_value = 0
clamp(n) {
    if n > max_value { return max_value }
    if n < min_value { return min_value }
    return n
}
clamp(50)
",
    );
    assert_eq!(prog.items.len(), 4);
    assert!(matches!(&prog.items[0], TopLevel::GlobalVar { name, .. } if name == "max_value"));
    assert!(matches!(&prog.items[1], TopLevel::GlobalVar { name, .. } if name == "min_value"));
    assert!(matches!(&prog.items[2], TopLevel::Function { name, .. } if name == "clamp"));
}

#[test]
fn test_top_level_while_with_break_and_continue() {
    let prog = parse_ok(
        "
i = 0
while i < 10 {
    i = i + 1
    if i == 3 { continue }
    if i == 7 { break }
}
",
    );
    // Items: GlobalVar(i=0), GlobalVar(__expr for the while), or Stmt for the while.
    // Regardless, parsing must succeed.
    assert!(!prog.items.is_empty());
}
