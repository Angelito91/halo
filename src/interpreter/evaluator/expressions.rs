// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// Expression evaluation (split out of `core`).

use super::core::Evaluator;
use super::methods;
use crate::interpreter::value::Value;
use crate::parser::ast::{BinOp, Expression, IndexType};
use crate::utils::find_suggestions_smart;

impl Evaluator {
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
