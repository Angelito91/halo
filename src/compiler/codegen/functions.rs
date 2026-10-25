// The Halo Programming Language — AST inference helpers for function signatures.

use crate::parser::ast::{BinOp, Block, Expression, Statement};

/// Scan a function body and determine its return type.
/// Returns one of "i64", "f64", or "string".
pub(crate) fn determine_return_type(body: &Block) -> &'static str {
    // A function with no value-returning `return` statement cannot return a
    // string or a float. Without this guard, a function that merely prints a
    // string expression would be declared as returning `i8*` and its implicit
    // `return i64 0` would produce invalid IR.
    if body
        .stmts
        .iter()
        .all(|stmt| return_type_of_stmt(stmt).is_none())
    {
        return "i64";
    }

    // First pass: check explicit return expressions
    for stmt in &body.stmts {
        if let Some(ty) = return_type_of_stmt(stmt) {
            if ty == "string" || ty == "f64" {
                return ty;
            }
        }
    }

    // Second pass: check if the function body contains string operations
    // (string concat `+`, string repeat `*`, or string literals being assigned).
    // If so, it might return a string via a variable.
    if contains_string_op(body) {
        return "string";
    }

    // Fallback: check for any non-i64 return type
    for stmt in &body.stmts {
        if let Some(ty) = return_type_of_stmt(stmt) {
            return ty;
        }
    }

    "i64"
}

/// Check if a block contains any string-related operations.
pub(crate) fn contains_string_op(body: &Block) -> bool {
    for stmt in &body.stmts {
        if stmt_has_string(stmt) {
            return true;
        }
    }
    false
}

pub(crate) fn stmt_has_string(stmt: &Statement) -> bool {
    match stmt {
        Statement::Return {
            value: Some(expr), ..
        } => expr_has_string(expr),
        Statement::Expr(expr) => expr_has_string(expr),
        Statement::VarDecl {
            init: Some(expr), ..
        } => expr_has_string(expr),
        Statement::VarDecl { init: None, .. } => false,
        Statement::Return { value: None, .. } => false,
        Statement::If {
            then_branch,
            else_branch,
            else_if_branches,
            ..
        } => {
            for s in &then_branch.stmts {
                if stmt_has_string(s) {
                    return true;
                }
            }
            if let Some(else_b) = else_branch {
                for s in &else_b.stmts {
                    if stmt_has_string(s) {
                        return true;
                    }
                }
            }
            for branch in else_if_branches {
                if expr_has_string(&branch.cond) {
                    return true;
                }
                for s in &branch.body.stmts {
                    if stmt_has_string(s) {
                        return true;
                    }
                }
            }
            false
        }
        Statement::While { cond, body, .. } => {
            expr_has_string(cond) || body.stmts.iter().any(stmt_has_string)
        }
        Statement::For { iterable, body, .. } => {
            expr_has_string(iterable) || body.stmts.iter().any(stmt_has_string)
        }
        Statement::Break { .. } | Statement::Continue { .. } => false,
    }
}

pub(crate) fn expr_has_string(expr: &Expression) -> bool {
    match expr {
        Expression::StringLiteral(_, _) => true,
        Expression::Binary {
            left, op, right, ..
        } => {
            matches!(op, BinOp::Add | BinOp::Mul)
                && (expr_has_string(left) || expr_has_string(right))
        }
        Expression::Var(_, _) => false, // variable — could be string, but conservative
        _ => false,
    }
}

/// Check if a statement contains a return expression that reveals the return type.
pub(crate) fn return_type_of_stmt(stmt: &Statement) -> Option<&'static str> {
    match stmt {
        Statement::Return {
            value: Some(expr), ..
        } => return_type_of_expr(expr),
        Statement::If {
            then_branch,
            else_branch,
            else_if_branches,
            ..
        } => {
            for s in &then_branch.stmts {
                if let Some(ty) = return_type_of_stmt(s) {
                    return Some(ty);
                }
            }
            if let Some(else_b) = else_branch {
                for s in &else_b.stmts {
                    if let Some(ty) = return_type_of_stmt(s) {
                        return Some(ty);
                    }
                }
            }
            for branch in else_if_branches {
                for s in &branch.body.stmts {
                    if let Some(ty) = return_type_of_stmt(s) {
                        return Some(ty);
                    }
                }
            }
            None
        }
        Statement::While { body, .. } => {
            for s in &body.stmts {
                if let Some(ty) = return_type_of_stmt(s) {
                    return Some(ty);
                }
            }
            None
        }
        Statement::For { body, .. } => {
            for s in &body.stmts {
                if let Some(ty) = return_type_of_stmt(s) {
                    return Some(ty);
                }
            }
            None
        }
        _ => None,
    }
}

/// Return type of a built-in call when it is used as the value of a
/// `return` statement. Unknown calls default to the integer ABI.
fn builtin_return_type(name: &str, args: &[Expression]) -> &'static str {
    match name {
        "str" | "type" | "upper" | "lower" | "trim" | "replace" | "substr" => "string",
        "starts_with" | "ends_with" | "contains" => "bool",
        "float" => "f64",
        // `abs` preserves the numeric type of its argument.
        "abs" => args.first().and_then(return_type_of_expr).unwrap_or("i64"),
        _ => "i64",
    }
}

/// Return type of a method call used in a `return` statement.
fn method_return_type(method: &str) -> &'static str {
    match method {
        "upper" | "lower" | "trim" | "replace" | "substr" | "join" => "string",
        "starts_with" | "ends_with" | "contains" => "bool",
        _ => "i64",
    }
}

/// Check what type a return expression would yield.
pub(crate) fn return_type_of_expr(expr: &Expression) -> Option<&'static str> {
    match expr {
        Expression::StringLiteral(_, _) => Some("string"),
        Expression::Float(_, _) => Some("f64"),
        Expression::Number(_, _) => Some("i64"),
        Expression::Bool(_, _) => Some("bool"),
        Expression::Unary { operator, expr, .. } if operator == "!" => Some("bool"),
        Expression::Unary { operator, expr, .. } if operator == "-" => return_type_of_expr(expr),
        Expression::Call { name, args, .. } => Some(builtin_return_type(name, args)),
        Expression::MethodCall { method, .. } => Some(method_return_type(method)),
        Expression::Binary {
            left, op, right, ..
        } => {
            // String concat/repeat → string return
            if matches!(op, BinOp::Add | BinOp::Mul)
                && (return_type_of_expr(left) == Some("string")
                    || return_type_of_expr(right) == Some("string"))
            {
                return Some("string");
            }
            if matches!(
                op,
                BinOp::Eq
                    | BinOp::Neq
                    | BinOp::Lt
                    | BinOp::Gt
                    | BinOp::Le
                    | BinOp::Ge
                    | BinOp::And
                    | BinOp::Or
            ) {
                return Some("bool");
            }
            // Arithmetic → i64 or f64
            if return_type_of_expr(left) == Some("f64") || return_type_of_expr(right) == Some("f64")
            {
                return Some("f64");
            }
            Some("i64")
        }
        _ => Some("i64"),
    }
}

/// Returns true if the given expression is a valid constant initializer for
/// a global variable. LLVM requires global initializers to be compile-time
/// constants; we only support number, float, and bool literals. Strings and
/// any computed value must be promoted to a local assignment inside `main`.
pub(crate) fn is_constant_initializer(expr: &Expression) -> bool {
    matches!(
        expr,
        Expression::Number(_, _) | Expression::Float(_, _) | Expression::Bool(_, _)
    )
}
