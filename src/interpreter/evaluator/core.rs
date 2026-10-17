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
//   - `recursion_depth`  – capped at `MAX_RECURSION_DEPTH` (this file).
//   - `loop_iterations`  – capped at `MAX_LOOP_ITERATIONS` (`statements`).
//
// The statement, expression and call evaluators are `impl Evaluator` blocks
// split into the sibling modules `statements`, `expressions` and `calls`.

use super::super::environment::Environment;
use super::super::value::Value;
use crate::parser::ast::Block;
use std::collections::HashMap;
use std::rc::Rc;

// ── Safety limits ─────────────────────────────────────────────────────────────

// In debug/test builds each Rust call frame is much larger (no inlining,
// debug info, etc.), so we use a lower limit to prevent a native stack
// overflow before the interpreter's own guard fires.  Release builds have
// far deeper native stacks and the higher limit is safe there.
#[cfg(not(test))]
pub(crate) const MAX_RECURSION_DEPTH: usize = 500;
#[cfg(test)]
pub(crate) const MAX_RECURSION_DEPTH: usize = 15;

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
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
