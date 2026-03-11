// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// Tree-walking interpreter for Halo.
//
// # Execution model
//
// Programs are executed in two passes:
//   1. All `Function` items are registered in `self.functions`.
//   2. All other top-level items are executed in order.
//
// Function calls create a new environment scope and set `self.return_value`
// when a `return` statement is reached.  `break` and `continue` inside loops
// propagate via `self.loop_signal`.
//
// # Safety limits
//
// Two counters prevent runaway programs from exhausting resources:
//   - `recursion_depth`  – capped at `MAX_RECURSION_DEPTH`.
//   - `loop_iterations`  – capped at `MAX_LOOP_ITERATIONS`.

use super::environment::Environment;
use super::value::Value;
use crate::parser::ast::{BinOp, Block, Expression, Program, Statement, TopLevel};
use std::collections::HashMap;
use std::rc::Rc;

// ── Safety limits ─────────────────────────────────────────────────────────────

// In debug/test builds each Rust call frame is much larger (no inlining,
// debug info, etc.), so we use a lower limit to prevent a native stack
// overflow before the interpreter's own guard fires.  Release builds have
// far deeper native stacks and the higher limit is safe there.
#[cfg(not(test))]
const MAX_RECURSION_DEPTH: usize = 500;
#[cfg(test)]
const MAX_RECURSION_DEPTH: usize = 50;
const MAX_LOOP_ITERATIONS: u64 = 1_000_000;

// ── Control-flow signals ──────────────────────────────────────────────────────

/// Signals that short-circuit normal statement sequencing inside a loop body.
/// They are stored on the `Evaluator` rather than being carried in the `Result`
/// return type so that the existing `Result<Value, String>` signature is kept.
#[derive(Debug, Clone, PartialEq)]
enum LoopSignal {
    Break,
    Continue,
}

// ── Function definition cache ─────────────────────────────────────────────────

/// A reference-counted, immutable snapshot of a user-defined function.
///
/// Storing definitions behind an `Rc` lets `eval_call` clone the handle
/// cheaply (a single atomic increment) instead of deep-cloning the entire
/// `Block` AST on every call.
type FnDef = Rc<(Vec<String>, Block)>;

// ── Evaluator ─────────────────────────────────────────────────────────────────

pub struct Evaluator {
    /// Variable bindings, organised in a stack of scopes.
    env: Environment,
    /// All user-defined functions discovered during the first pass.
    functions: HashMap<String, FnDef>,
    /// Holds the value produced by the most recent `return` statement until it
    /// is collected by `eval_call`.  `None` means no return is pending.
    return_value: Option<Value>,
    /// Pending `break` or `continue` signal, cleared by the enclosing loop.
    loop_signal: Option<LoopSignal>,
    /// How many user-function frames are currently on the call stack.
    recursion_depth: usize,
    /// Total loop iterations executed in the current program run.
    loop_iterations: u64,
}

impl Evaluator {
    /// Create a fresh evaluator with empty state.
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            functions: HashMap::new(),
            return_value: None,
            loop_signal: None,
            recursion_depth: 0,
            loop_iterations: 0,
        }
    }

    // =========================================================================
    // Public API
    // =========================================================================

    /// Execute a complete [`Program`], returning the last evaluated value.
    ///
    /// State from a previous run (variables, functions) is intentionally
    /// preserved so that callers can run multiple snippets in the same REPL
    /// session.  Safety counters are reset on each call.
    pub fn eval_program(&mut self, program: &Program) -> Result<Value, String> {
        self.recursion_depth = 0;
        self.loop_iterations = 0;
        self.loop_signal = None;

        // Pass 1 – register all function definitions before executing anything
        // so that call-before-definition works at the top level.
        for item in &program.items {
            if let TopLevel::Function {
                name, params, body, ..
            } = item
            {
                self.functions
                    .insert(name.clone(), Rc::new((params.clone(), body.clone())));
            }
        }

        // Pass 2 – execute everything that is not a function definition.
        let mut last = Value::Null;
        for item in &program.items {
            last = match item {
                TopLevel::Function { .. } => Value::Null, // already registered

                TopLevel::GlobalVar { name, init, .. } => {
                    let value = init
                        .as_ref()
                        .map(|expr| self.eval_expr(expr))
                        .transpose()?
                        .unwrap_or(Value::Null);
                    // "__expr" is a synthetic name the parser gives to bare
                    // top-level expressions (e.g. `x`, `f()`, `1 + 2`).
                    // We still store it so downstream code can inspect it, but
                    // we also propagate its value as the "last" result so that
                    // eval_program returns it correctly.
                    if name == "__expr" {
                        value
                    } else {
                        self.env.set(name.clone(), value);
                        Value::Null
                    }
                }

                TopLevel::Stmt { stmt, .. } => self.eval_stmt(stmt)?,
            };
        }

        Ok(last)
    }

    // =========================================================================
    // Block and statement evaluation
    // =========================================================================

    /// Evaluate every statement in `block` in order, stopping early on
    /// `return`, `break`, or `continue`.
    fn eval_block(&mut self, block: &Block) -> Result<Value, String> {
        let mut last = Value::Null;
        for stmt in &block.stmts {
            last = self.eval_stmt(stmt)?;
            if self.return_value.is_some() || self.loop_signal.is_some() {
                break;
            }
        }
        Ok(last)
    }

    fn eval_stmt(&mut self, stmt: &Statement) -> Result<Value, String> {
        match stmt {
            Statement::Expr(expr) => self.eval_expr(expr),

            Statement::VarDecl { name, init, .. } => {
                let value = init
                    .as_ref()
                    .map(|expr| self.eval_expr(expr))
                    .transpose()?
                    .unwrap_or(Value::Null);
                self.env.set(name.clone(), value);
                Ok(Value::Null)
            }

            Statement::If {
                cond,
                then_branch,
                else_if_branches,
                else_branch,
                ..
            } => {
                if self.eval_expr(cond)?.is_truthy() {
                    return self.eval_block(then_branch);
                }

                // Walk the `else if` chain until one condition is truthy.
                for branch in else_if_branches {
                    if self.eval_expr(&branch.cond)?.is_truthy() {
                        return self.eval_block(&branch.body);
                    }
                }

                // Fall through to the optional `else` block.
                match else_branch {
                    Some(block) => self.eval_block(block),
                    None => Ok(Value::Null),
                }
            }

            Statement::While { cond, body, .. } => {
                loop {
                    self.loop_iterations += 1;
                    if self.loop_iterations > MAX_LOOP_ITERATIONS {
                        return Err(format!(
                            "Loop iteration limit exceeded ({})",
                            MAX_LOOP_ITERATIONS
                        ));
                    }

                    if !self.eval_expr(cond)?.is_truthy() {
                        break;
                    }

                    self.eval_block(body)?;

                    if self.return_value.is_some() {
                        break;
                    }

                    match self.loop_signal.take() {
                        Some(LoopSignal::Break) => break,
                        Some(LoopSignal::Continue) => continue,
                        None => {}
                    }
                }
                Ok(Value::Null)
            }

            Statement::Break { .. } => {
                self.loop_signal = Some(LoopSignal::Break);
                Ok(Value::Null)
            }

            Statement::Continue { .. } => {
                self.loop_signal = Some(LoopSignal::Continue);
                Ok(Value::Null)
            }

            Statement::Return { value, .. } => {
                let ret = value
                    .as_ref()
                    .map(|expr| self.eval_expr(expr))
                    .transpose()?
                    .unwrap_or(Value::Null);
                self.return_value = Some(ret.clone());
                Ok(ret)
            }
        }
    }

    // =========================================================================
    // Expression evaluation
    // =========================================================================

    fn eval_expr(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            // ── Literals ─────────────────────────────────────────────────────
            Expression::Number(n, _) => Ok(Value::Number(*n)),
            Expression::Float(f, _) => Ok(Value::Float(*f)),
            Expression::Bool(b, _) => Ok(Value::Bool(*b)),
            Expression::StringLiteral(s, _) => Ok(Value::String(s.clone())),

            // ── Variable read ─────────────────────────────────────────────────
            Expression::Var(name, _) => self
                .env
                .get(name)
                .ok_or_else(|| format!("Undefined variable: '{name}'")),

            // ── Unary operators ───────────────────────────────────────────────
            Expression::Unary { operator, expr, .. } => {
                let val = self.eval_expr(expr)?;
                match operator.as_str() {
                    "-" => match val {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        _ => Err(format!(
                            "Cannot negate a value of type '{}'",
                            val.type_name()
                        )),
                    },
                    "!" => Ok(val.not()),
                    op => Err(format!("Unknown unary operator: '{op}'")),
                }
            }

            // ── Binary operators ──────────────────────────────────────────────
            Expression::Binary {
                left, op, right, ..
            } => self.eval_binary(left, op, right),

            // ── Assignment ────────────────────────────────────────────────────
            Expression::Assign { name, value, .. } => {
                let val = self.eval_expr(value)?;
                self.env.update(name, val.clone())?;
                Ok(val)
            }

            // ── Function call ─────────────────────────────────────────────────
            Expression::Call { name, args, .. } => self.eval_call(name, args),
        }
    }

    /// Evaluate a binary expression, handling short-circuit operators first.
    fn eval_binary(
        &mut self,
        left: &Expression,
        op: &BinOp,
        right: &Expression,
    ) -> Result<Value, String> {
        // Short-circuit `&&` and `||` are handled before evaluating the RHS.
        match op {
            BinOp::And => {
                let lhs = self.eval_expr(left)?;
                if !lhs.is_truthy() {
                    return Ok(lhs); // short-circuit: false && _ = false
                }
                let rhs = self.eval_expr(right)?;
                return Ok(lhs.and(&rhs));
            }
            BinOp::Or => {
                let lhs = self.eval_expr(left)?;
                if lhs.is_truthy() {
                    return Ok(lhs); // short-circuit: true || _ = true
                }
                let rhs = self.eval_expr(right)?;
                return Ok(lhs.or(&rhs));
            }
            _ => {}
        }

        // All other operators evaluate both sides eagerly.
        let lhs = self.eval_expr(left)?;
        let rhs = self.eval_expr(right)?;

        match op {
            BinOp::Add => lhs.add(&rhs),
            BinOp::Sub => lhs.subtract(&rhs),
            BinOp::Mul => lhs.multiply(&rhs),
            BinOp::Div => lhs.divide(&rhs),
            BinOp::Mod => lhs.modulo(&rhs),
            BinOp::Eq => Ok(Value::Bool(lhs.equals(&rhs))),
            BinOp::Neq => Ok(Value::Bool(!lhs.equals(&rhs))),
            BinOp::Lt => Ok(Value::Bool(lhs.less_than(&rhs)?)),
            BinOp::Gt => Ok(Value::Bool(lhs.greater_than(&rhs)?)),
            BinOp::Le => Ok(Value::Bool(lhs.less_equal(&rhs)?)),
            BinOp::Ge => Ok(Value::Bool(lhs.greater_equal(&rhs)?)),
            // Already handled in the short-circuit block above.
            BinOp::And | BinOp::Or => unreachable!("&&/|| handled above"),
        }
    }

    // =========================================================================
    // Function call dispatch
    // =========================================================================

    fn eval_call(&mut self, name: &str, args: &[Expression]) -> Result<Value, String> {
        // Built-ins never count towards the recursion limit.
        if let Some(result) = self.try_eval_builtin(name, args)? {
            return Ok(result);
        }

        // ── User-defined function ─────────────────────────────────────────────

        // Guard: check and increment recursion depth before doing anything else
        // so that the decrement in the cleanup path is always balanced.
        self.recursion_depth += 1;
        if self.recursion_depth > MAX_RECURSION_DEPTH {
            self.recursion_depth -= 1;
            return Err(format!(
                "Maximum recursion depth exceeded ({})",
                MAX_RECURSION_DEPTH
            ));
        }

        // Retrieve the definition — clone the Rc handle (cheap), not the AST.
        let def = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined function: '{name}'"))?;

        let (params, body) = (def.0.as_slice(), &def.1);

        if args.len() != params.len() {
            self.recursion_depth -= 1;
            return Err(format!(
                "Function '{name}' expects {} argument(s), got {}",
                params.len(),
                args.len()
            ));
        }

        // Evaluate arguments in the *caller's* scope before pushing a new frame.
        let evaluated_args = self.eval_args(args)?;

        // Open a new scope and bind parameters to the evaluated arguments.
        self.env.push_scope();
        for (param, value) in params.iter().zip(evaluated_args) {
            self.env.set(param.clone(), value);
        }

        // Execute the body, then tear down the scope regardless of outcome.
        self.return_value = None;
        let body_result = self.eval_block(body);
        self.env.pop_scope();
        self.recursion_depth -= 1;

        // Surface any error from the body.
        body_result?;

        // Collect the pending return value (or `null` for void functions).
        Ok(self.return_value.take().unwrap_or(Value::Null))
    }

    /// Evaluate a slice of argument expressions in the current scope, returning
    /// the results as an owned `Vec`.  Stops and propagates the first error.
    fn eval_args(&mut self, args: &[Expression]) -> Result<Vec<Value>, String> {
        args.iter().map(|arg| self.eval_expr(arg)).collect()
    }

    /// Try to evaluate `name` as a built-in function.
    ///
    /// Returns:
    /// - `Ok(Some(value))` — the call was handled by a built-in.
    /// - `Ok(None)` — `name` is not a built-in; the caller should try
    ///   user-defined functions next.
    /// - `Err(msg)` — a built-in was matched but the call was invalid.
    fn try_eval_builtin(
        &mut self,
        name: &str,
        args: &[Expression],
    ) -> Result<Option<Value>, String> {
        let result = match name {
            "print" => self.builtin_print(args)?,
            "len" => self.builtin_single_arg(name, args, |v| match v {
                Value::String(s) => Ok(Value::Number(s.len() as i64)),
                Value::Number(n) => Ok(Value::Number(n.to_string().len() as i64)),
                other => Err(format!(
                    "len() does not support type '{}'",
                    other.type_name()
                )),
            })?,
            "str" => {
                self.builtin_single_arg(name, args, |v| Ok(Value::String(v.to_string_value())))?
            }
            "int" => self.builtin_single_arg(name, args, |v| Ok(Value::Number(v.to_int()?)))?,
            "float" => self.builtin_single_arg(name, args, |v| Ok(Value::Float(v.to_number()?)))?,
            "abs" => self.builtin_single_arg(name, args, |v| match v {
                Value::Number(n) => Ok(Value::Number(n.abs())),
                Value::Float(f) => Ok(Value::Float(f.abs())),
                other => Err(format!(
                    "abs() does not support type '{}'",
                    other.type_name()
                )),
            })?,
            "type" => self
                .builtin_single_arg(name, args, |v| Ok(Value::String(v.type_name().to_string())))?,
            // Not a built-in.
            _ => return Ok(None),
        };
        Ok(Some(result))
    }

    // ── Built-in helpers ──────────────────────────────────────────────────────

    /// `print(v1, v2, …)` — prints each argument on its own line.
    fn builtin_print(&mut self, args: &[Expression]) -> Result<Value, String> {
        for arg in args {
            let val = self.eval_expr(arg)?;
            println!("{val}");
        }
        Ok(Value::Null)
    }

    /// Helper for built-ins that accept exactly one argument.
    ///
    /// Evaluates the single argument, then calls `f` with the resulting
    /// [`Value`].  Returns an arity error if the argument count is wrong.
    fn builtin_single_arg(
        &mut self,
        name: &str,
        args: &[Expression],
        f: impl FnOnce(Value) -> Result<Value, String>,
    ) -> Result<Value, String> {
        if args.len() != 1 {
            return Err(format!(
                "{name}() expects exactly 1 argument, got {}",
                args.len()
            ));
        }
        let val = self.eval_expr(&args[0])?;
        f(val)
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::lexer::TokenKind;
    use crate::parser::Parser;

    /// Tokenise, parse, and evaluate a snippet of Halo source code.
    fn eval_code(code: &str) -> Result<Value, String> {
        let mut lexer = Lexer::new(code.to_string());
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            let is_eof = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|errs| errs.join("; "))?;
        let mut evaluator = Evaluator::new();
        evaluator.eval_program(&program)
    }

    #[test]
    fn test_simple_arithmetic() {
        assert_eq!(eval_code("x = 2 + 3").unwrap(), Value::Null);
    }

    #[test]
    fn test_variable_assignment() {
        assert_eq!(eval_code("x = 42").unwrap(), Value::Null);
    }

    #[test]
    fn test_addition() {
        let code = "
add(a, b) {
    return a + b
}
result = add(3, 4)
";
        assert_eq!(eval_code(code).unwrap(), Value::Null);
    }

    #[test]
    fn test_factorial() {
        let code = "
factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
result = factorial(5)
";
        eval_code(code).unwrap();
    }

    #[test]
    fn test_undefined_variable_error() {
        let result = eval_code("x = undefined_var");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_undefined_function_error() {
        let result = eval_code("result = undefined_func(1, 2)");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined function"));
    }

    #[test]
    fn test_modulo_operator() {
        let code = "
mod_test(a, b) {
    return a % b
}
result = mod_test(10, 3)
";
        eval_code(code).unwrap();
    }

    #[test]
    fn test_logical_and() {
        let code = "
check(a, b) {
    if a && b {
        return 1
    }
    return 0
}
r1 = check(1, 1)
r2 = check(1, 0)
r3 = check(0, 1)
";
        eval_code(code).unwrap();
    }

    #[test]
    fn test_logical_or() {
        let code = "
check(a, b) {
    if a || b {
        return 1
    }
    return 0
}
r1 = check(1, 0)
r2 = check(0, 1)
r3 = check(0, 0)
";
        eval_code(code).unwrap();
    }

    #[test]
    fn test_recursion_depth_limit() {
        let code = "
infinite(n) {
    return infinite(n + 1)
}
result = infinite(0)
";
        let result = eval_code(code);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Maximum recursion depth exceeded"));
    }

    #[test]
    fn test_integer_overflow_protection() {
        let code = "
big(n) {
    return n + 9223372036854775807
}
result = big(1)
";
        let result = eval_code(code);
        assert!(result.is_err());
    }
}
