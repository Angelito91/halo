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

use super::super::environment::Environment;
use super::super::value::Value;
use super::builtins;
use super::methods;
use crate::parser::ast::{BinOp, Block, Expression, IndexType, Program, Statement, TopLevel};
use crate::utils::find_suggestions_smart;
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
const MAX_RECURSION_DEPTH: usize = 15;
const MAX_LOOP_ITERATIONS: u64 = 1_000_000;

// ── Control-flow signals ──────────────────────────────────────────────────────

/// Signals that short-circuit normal statement sequencing inside a loop body.
/// They are stored on the `Evaluator` rather than being carried in the `Result`
/// return type so that the existing `Result<Value, String>` signature is kept.
#[derive(Debug, Clone, PartialEq)]
pub enum LoopSignal {
    Break,
    Continue,
}

// ── Function definition cache ─────────────────────────────────────────────────

/// A reference-counted, immutable snapshot of a user-defined function.
///
/// Storing definitions behind an `Rc` lets `eval_call` clone the handle
/// cheaply (a single atomic increment) instead of deep-cloning the entire
/// `Block` AST on every call.
pub type FnDef = Rc<(Vec<String>, Block)>;

// ── Evaluator ─────────────────────────────────────────────────────────────────

pub struct Evaluator {
    /// Variable bindings, organised in a stack of scopes.
    pub env: Environment,
    /// All user-defined functions discovered during the first pass.
    pub functions: HashMap<String, FnDef>,
    /// Holds the value produced by the most recent `return` statement until it
    /// is collected by `eval_call`.  `None` means no return is pending.
    pub return_value: Option<Value>,
    /// Pending `break` or `continue` signal, cleared by the enclosing loop.
    pub loop_signal: Option<LoopSignal>,
    /// How many `for` bodies are currently being executed. Inside a `for`
    /// body, assignments update the nearest existing binding instead of
    /// declaring a loop-scoped shadow, so accumulators keep working while
    /// the loop variable itself stays scoped to the loop. Reset to 0 when
    /// entering a function frame.
    pub loop_scope_depth: usize,
    /// How many user-function frames are currently on the call stack.
    pub recursion_depth: usize,
    /// Total loop iterations executed in the current program run.
    pub loop_iterations: u64,
    /// All available variables across all scopes (for error suggestions)
    pub all_variables: Vec<String>,
    /// Call stack for tracking function execution (for stack traces)
    pub call_stack: Vec<String>,
}

impl Evaluator {
    /// Create a fresh evaluator with empty state.
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            functions: HashMap::new(),
            return_value: None,
            loop_signal: None,
            loop_scope_depth: 0,
            recursion_depth: 0,
            loop_iterations: 0,
            all_variables: Vec::new(),
            call_stack: Vec::new(),
        }
    }

    /// Get the current call stack as a vector of function names
    #[allow(dead_code)]
    pub fn get_call_stack(&self) -> Vec<String> {
        self.call_stack.clone()
    }

    /// Format call stack as a readable string for error reporting
    pub fn format_call_stack(&self) -> String {
        if self.call_stack.is_empty() {
            return String::new();
        }

        let mut trace = String::from("\nStack trace:\n");
        for (i, func) in self.call_stack.iter().enumerate() {
            trace.push_str(&format!("  {} in function '{}'\n", i, func));
        }
        trace
    }

    /// Generate an error message for an undefined variable with suggestions
    fn undefined_variable_error(&self, name: &str) -> String {
        let mut candidates: Vec<&str> = self.all_variables.iter().map(|s| s.as_str()).collect();
        candidates.sort();
        candidates.dedup();

        let suggestions = find_suggestions_smart(name, &candidates, 3);

        let mut message = format!("Undefined variable: '{name}'");
        if !suggestions.is_empty() {
            message.push_str("\n\n  Did you mean:");
            for (i, suggestion) in suggestions.iter().enumerate() {
                message.push_str(&format!("\n    {}", suggestion.text));
                if i == 0 {
                    message.push_str(" (most similar)");
                }
            }
        }
        message
    }

    /// Generate an error message for an undefined function with suggestions
    fn undefined_function_error(&self, name: &str) -> String {
        let candidates: Vec<&str> = self.functions.keys().map(|s| s.as_str()).collect();

        let suggestions = find_suggestions_smart(name, &candidates, 3);

        let mut message = format!("Undefined function: '{name}'");
        if !suggestions.is_empty() {
            message.push_str("\n\n  Did you mean:");
            for (i, suggestion) in suggestions.iter().enumerate() {
                message.push_str(&format!("\n    {}", suggestion.text));
                if i == 0 {
                    message.push_str(" (most similar)");
                }
            }
        }
        message
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
        self.all_variables.clear();

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
                if self.loop_scope_depth > 0 {
                    // Inside a `for` body, an assignment to an existing name
                    // targets that binding (even outside the loop scope);
                    // a brand-new name lands in the loop scope and disappears
                    // once the loop ends.
                    self.env.update(name, value)?;
                } else {
                    self.env.set(name.clone(), value);
                }
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

            Statement::For {
                var,
                iterable,
                body,
                ..
            } => {
                // The iterable is evaluated exactly once, before the first
                // iteration.
                let iterable_value = self.eval_expr(iterable)?;
                let items = match iterable_value {
                    Value::Array(items) => items,
                    other => {
                        return Err(format!(
                            "for expects an array to iterate, got '{}'",
                            other.type_name()
                        ))
                    }
                };

                // The loop variable lives in a scope of its own: it shadows
                // any outer binding of the same name and is gone afterwards.
                self.env.push_scope();
                self.loop_scope_depth += 1;

                let mut loop_result = Ok(());
                for item in items {
                    self.loop_iterations += 1;
                    if self.loop_iterations > MAX_LOOP_ITERATIONS {
                        loop_result = Err(format!(
                            "Loop iteration limit exceeded ({})",
                            MAX_LOOP_ITERATIONS
                        ));
                        break;
                    }

                    self.env.set(var.clone(), item);
                    if let Err(e) = self.eval_block(body) {
                        loop_result = Err(e);
                        break;
                    }

                    if self.return_value.is_some() {
                        break;
                    }

                    match self.loop_signal.take() {
                        Some(LoopSignal::Break) => break,
                        Some(LoopSignal::Continue) => continue,
                        None => {}
                    }
                }

                self.loop_scope_depth -= 1;
                self.env.pop_scope();
                loop_result?;
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

    pub fn eval_expr(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            // ── Literals ─────────────────────────────────────────────────────
            Expression::Number(n, _) => Ok(Value::Number(*n)),
            Expression::Float(f, _) => Ok(Value::Float(*f)),
            Expression::Bool(b, _) => Ok(Value::Bool(*b)),
            Expression::StringLiteral(s, _) => Ok(Value::String(s.clone())),

            // ── Variable read ─────────────────────────────────────────────────
            Expression::Var(name, _) => {
                // Track all accessed variables for suggestions
                if !self.all_variables.contains(name) {
                    self.all_variables.push(name.clone());
                }
                self.env
                    .get(name)
                    .ok_or_else(|| self.undefined_variable_error(name))
            }

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

            // ── Array literal ─────────────────────────────────────────────────
            Expression::ArrayLiteral { elements, .. } => {
                let mut arr = Vec::new();
                for elem in elements {
                    arr.push(self.eval_expr(elem)?);
                }
                Ok(Value::Array(arr))
            }

            // ── Array indexing ────────────────────────────────────────────────
            Expression::Index { array, index, .. } => {
                let arr_val = self.eval_expr(array)?;

                match index {
                    IndexType::Single(idx_expr) => {
                        // Single index: arr[5]
                        let idx_val = self.eval_expr(idx_expr)?;

                        match (arr_val, idx_val) {
                            (Value::Array(arr), Value::Number(idx)) => {
                                if idx < 0 {
                                    return Err(format!(
                                        "Array index cannot be negative, got {}",
                                        idx
                                    ));
                                }
                                let idx_usize = idx as usize;
                                arr.get(idx_usize).cloned().ok_or_else(|| {
                                    format!(
                                        "Array index {} out of bounds (length {})",
                                        idx,
                                        arr.len()
                                    )
                                })
                            }
                            (Value::Array(_), other) => Err(format!(
                                "Array index must be a number, got '{}'",
                                other.type_name()
                            )),
                            (other, _) => Err(format!(
                                "Cannot index a value of type '{}'",
                                other.type_name()
                            )),
                        }
                    }
                    IndexType::Slice { start, end } => {
                        // Slice: arr[start:end]
                        match arr_val {
                            Value::Array(arr) => {
                                let len = arr.len() as i64;

                                // Evaluate start index (default: 0)
                                let start_idx = if let Some(start_expr) = start {
                                    let val = self.eval_expr(start_expr)?;
                                    match val {
                                        Value::Number(n) => n,
                                        _ => {
                                            return Err(format!(
                                                "Slice start must be a number, got '{}'",
                                                val.type_name()
                                            ))
                                        }
                                    }
                                } else {
                                    0
                                };

                                // Evaluate end index (default: len)
                                let end_idx = if let Some(end_expr) = end {
                                    let val = self.eval_expr(end_expr)?;
                                    match val {
                                        Value::Number(n) => n,
                                        _ => {
                                            return Err(format!(
                                                "Slice end must be a number, got '{}'",
                                                val.type_name()
                                            ))
                                        }
                                    }
                                } else {
                                    len
                                };

                                // Validate bounds
                                if start_idx < 0 || start_idx > len {
                                    return Err(format!(
                                        "Slice start index {} out of bounds for array of length {}",
                                        start_idx, len
                                    ));
                                }
                                if end_idx < 0 || end_idx > len {
                                    return Err(format!(
                                        "Slice end index {} out of bounds for array of length {}",
                                        end_idx, len
                                    ));
                                }
                                if start_idx > end_idx {
                                    return Err(format!(
                                        "Slice start index {} cannot be greater than end index {}",
                                        start_idx, end_idx
                                    ));
                                }

                                // Extract slice
                                let start_usize = start_idx as usize;
                                let end_usize = end_idx as usize;
                                let slice = arr[start_usize..end_usize].to_vec();
                                Ok(Value::Array(slice))
                            }
                            other => Err(format!(
                                "Cannot slice a value of type '{}'",
                                other.type_name()
                            )),
                        }
                    }
                }
            }

            // -- Method call
            Expression::MethodCall {
                object,
                method,
                args,
                ..
            } => {
                let obj_value = self.eval_expr(object)?;
                methods::try_eval_method_call(self, &obj_value, method, args)
            }

            // -- Index assignment: arr[i] = value
            Expression::IndexAssign {
                array,
                index,
                value,
                ..
            } => {
                let arr_val = self.eval_expr(array)?;
                let idx_val = self.eval_expr(index)?;
                let new_val = self.eval_expr(value)?;

                match (arr_val, idx_val) {
                    (Value::Array(mut arr), Value::Number(idx)) => {
                        if idx < 0 {
                            return Err(format!("Array index cannot be negative, got {}", idx));
                        }
                        let idx_usize = idx as usize;
                        if idx_usize >= arr.len() {
                            return Err(format!(
                                "Array index {} out of bounds (length {})",
                                idx,
                                arr.len()
                            ));
                        }
                        arr[idx_usize] = new_val.clone();
                        Ok(new_val)
                    }
                    (Value::Array(_), other) => Err(format!(
                        "Array index must be a number, got '{}'",
                        other.type_name()
                    )),
                    (other, _) => Err(format!(
                        "Cannot index a value of type '{}'",
                        other.type_name()
                    )),
                }
            }
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

    pub fn eval_call(&mut self, name: &str, args: &[Expression]) -> Result<Value, String> {
        // Built-ins never count towards the recursion limit.
        if let Some(result) = builtins::try_eval_builtin(self, name, args)? {
            return Ok(result);
        }

        // ── User-defined function ─────────────────────────────────────────────

        // Guard: check and increment recursion depth before doing anything else
        // so that the decrement in the cleanup path is always balanced.
        self.recursion_depth += 1;
        if self.recursion_depth > MAX_RECURSION_DEPTH {
            self.recursion_depth -= 1;
            return Err(format!(
                "Infinite recursion detected in function '{}' — maximum depth {} exceeded\n{}",
                name,
                MAX_RECURSION_DEPTH,
                self.format_call_stack()
            ));
        }

        // Retrieve the definition — clone the Rc handle (cheap), not the AST.
        let def = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| self.undefined_function_error(name))?;

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

        // Push function name to call stack for tracking
        self.call_stack.push(name.to_string());

        // Open a new scope and bind parameters to the evaluated arguments.
        self.env.push_scope();
        for (param, value) in params.iter().zip(evaluated_args) {
            self.env.set(param.clone(), value);
        }

        // Execute the body, then tear down the scope regardless of outcome.
        self.return_value = None;
        // A function frame starts a fresh declaration regime: assignments
        // inside it declare function-locals instead of updating outer scopes.
        let outer_loop_depth = self.loop_scope_depth;
        self.loop_scope_depth = 0;
        let body_result = self.eval_block(body);
        self.loop_scope_depth = outer_loop_depth;
        self.env.pop_scope();

        // Pop function name from call stack
        self.call_stack.pop();

        self.recursion_depth -= 1;

        // Surface any error from the body.
        body_result?;

        // Collect the pending return value (or `null` for void functions).
        Ok(self.return_value.take().unwrap_or(Value::Null))
    }

    pub fn eval_args(&mut self, args: &[Expression]) -> Result<Vec<Value>, String> {
        let mut values = Vec::new();
        for arg in args {
            values.push(self.eval_expr(arg)?);
        }
        Ok(values)
    }

    /// Convert a Value back to an Expression for use in method calls.
    /// This is a helper function to adapt between the evaluator's need to pass
    /// values to built-in functions that expect Expression arguments.
    pub fn value_to_expression(&self, value: &Value) -> Expression {
        let pos = crate::parser::ast::Position { line: 0, column: 0 };

        match value {
            Value::Number(n) => Expression::Number(*n, pos),
            Value::Float(f) => Expression::Float(*f, pos),
            Value::Bool(b) => Expression::Bool(*b, pos),
            Value::String(s) => Expression::StringLiteral(s.clone(), pos),
            Value::Array(arr) => {
                let elements = arr.iter().map(|v| self.value_to_expression(v)).collect();
                Expression::ArrayLiteral { elements, pos }
            }
            Value::Null => Expression::Bool(false, pos),
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
