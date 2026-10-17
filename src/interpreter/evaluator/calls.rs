// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// Function-call dispatch and call-stack bookkeeping (split out of `core`).

use super::builtins;
use super::core::{Evaluator, MAX_RECURSION_DEPTH};
use crate::interpreter::value::Value;
use crate::parser::ast::Expression;
use crate::utils::find_suggestions_smart;

impl Evaluator {
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
}
