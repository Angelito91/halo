// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// Expression evaluation (split out of `core`).

use super::core::Evaluator;
use crate::diagnostics::suggestions::find_suggestions_smart;
use crate::interpreter::value::Value;
use crate::parser::ast::{BinOp, Expression, IndexType};

impl Evaluator {
    /// Generate an error message for an undefined variable with suggestions
    fn undefined_variable_error(&self, name: &str) -> String {
        let mut candidates: Vec<&str> = self.all_variables.iter().map(|s| s.as_str()).collect();
        candidates.sort();
        candidates.dedup();

        let suggestions = find_suggestions_smart(name, &candidates, 3);

        let mut message = format!(
            "{} '{name}'",
            crate::interpreter::errors::UNDEFINED_VARIABLE
        );
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
                                        "{} {}",
                                        crate::interpreter::errors::NEGATIVE_INDEX,
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

            // -- Index assignment: arr[i] = value
            Expression::IndexAssign {
                array,
                index,
                value,
                ..
            } => self.eval_index_assign(array, index, value),
        }
    }

    /// Evaluate `array[index] = value`, updating the variable that owns the
    /// indexed array. Nested indexing (`matrix[i][j] = value`) is supported.
    ///
    /// The left-hand side must be a variable or a chain of single-element
    /// indexes. Slices, literals and call results are not assignable.
    fn eval_index_assign(
        &mut self,
        array: &Expression,
        index: &Expression,
        value: &Expression,
    ) -> Result<Value, String> {
        let mut path = Vec::new();
        let root = self.collect_index_path(array, index, &mut path)?;

        let mut indices = Vec::with_capacity(path.len());
        for index_expr in path {
            match self.eval_expr(index_expr)? {
                Value::Number(n) => indices.push(n),
                other => {
                    return Err(format!(
                        "Array index must be a number, got '{}'",
                        other.type_name()
                    ))
                }
            }
        }

        let new_value = self.eval_expr(value)?;
        self.assign_index_path(&root, &indices, new_value.clone())?;
        Ok(new_value)
    }

    /// Walk an assignment target and collect the root variable plus the
    /// index expressions in left-to-right order.
    fn collect_index_path<'a>(
        &self,
        array: &'a Expression,
        index: &'a Expression,
        path: &mut Vec<&'a Expression>,
    ) -> Result<String, String> {
        match array {
            Expression::Var(name, _) => {
                path.push(index);
                Ok(name.clone())
            }
            Expression::Index {
                array: parent,
                index: parent_index,
                ..
            } => {
                let root = match parent_index {
                    IndexType::Single(parent_index) => {
                        self.collect_index_path(parent, parent_index, path)?
                    }
                    IndexType::Slice { .. } => {
                        return Err(
                            "Cannot assign through a slice; index a single element".to_string()
                        )
                    }
                };
                path.push(index);
                Ok(root)
            }
            _ => Err(
                "Cannot assign to this expression; expected a variable or an indexed array"
                    .to_string(),
            ),
        }
    }

    /// Apply a previously evaluated index path to the root binding.
    fn assign_index_path(
        &mut self,
        root: &str,
        indices: &[i64],
        value: Value,
    ) -> Result<(), String> {
        if self.env.get_ref(root).is_none() {
            return Err(self.undefined_variable_error(root));
        }

        let mut current = self
            .env
            .get_mut(root)
            .expect("variable existence checked above");

        for (depth, &index) in indices.iter().enumerate() {
            if index < 0 {
                return Err(format!(
                    "{} {}",
                    crate::interpreter::errors::NEGATIVE_INDEX,
                    index
                ));
            }

            let index_usize = index as usize;
            let is_last = depth + 1 == indices.len();

            let array = match current {
                Value::Array(array) => array,
                other => {
                    return Err(format!(
                        "Cannot index a value of type '{}'",
                        other.type_name()
                    ))
                }
            };

            if index_usize >= array.len() {
                return Err(format!(
                    "Array index {} out of bounds (length {})",
                    index,
                    array.len()
                ));
            }

            if is_last {
                array[index_usize] = value;
                return Ok(());
            }

            current = &mut array[index_usize];
        }

        unreachable!("index paths always contain at least one index")
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
}
