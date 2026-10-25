// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// Statement and block evaluation (split out of `core`).

use super::core::{Evaluator, LoopSignal};
use crate::interpreter::value::Value;
use crate::parser::ast::{Block, Position, Program, Statement, TopLevel};
use std::rc::Rc;

/// Append a source position to a runtime error exactly once.
///
/// Nested blocks annotate the innermost failing statement first; outer
/// statements then leave the message untouched.
pub(crate) fn annotate_error(message: String, pos: Position) -> String {
    const MARKER: &str = "\n  --> line ";
    if message.contains(MARKER) {
        message
    } else {
        format!("{}{}{}, column {}", message, MARKER, pos.line, pos.column)
    }
}

// ── Safety limits ─────────────────────────────────────────────────────────────

pub(crate) const MAX_LOOP_ITERATIONS: u64 = 1_000_000;

impl Evaluator {
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
                        .transpose()
                        .map_err(|err| annotate_error(err, item.pos()))?
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
    pub(crate) fn eval_block(&mut self, block: &Block) -> Result<Value, String> {
        let mut last = Value::Null;
        for stmt in &block.stmts {
            last = self.eval_stmt(stmt)?;
            if self.return_value.is_some() || self.loop_signal.is_some() {
                break;
            }
        }
        Ok(last)
    }

    pub(crate) fn eval_stmt(&mut self, stmt: &Statement) -> Result<Value, String> {
        self.eval_stmt_inner(stmt)
            .map_err(|err| annotate_error(err, stmt.pos()))
    }

    fn eval_stmt_inner(&mut self, stmt: &Statement) -> Result<Value, String> {
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
}
