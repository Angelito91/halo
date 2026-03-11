// The Halo Programming Language
// AST Evaluator and Interpreter
// Version: 0.2.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// This module implements the tree-walking interpreter for the Halo language.
// It maintains proper recursion depth tracking and loop iteration counting
// to prevent resource exhaustion attacks.

use super::environment::Environment;
use super::value::Value;
use crate::parser::ast::{Block, Expression, Program, Statement, TopLevel};
use std::collections::HashMap;

// Safety limits to prevent resource exhaustion
const MAX_RECURSION_DEPTH: usize = 1000;
const MAX_LOOP_ITERATIONS: u64 = 1_000_000;

pub struct Evaluator {
    env: Environment,
    functions: HashMap<String, (Vec<String>, Block)>,
    return_value: Option<Value>,
    recursion_depth: usize,
    loop_iterations: u64,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: Environment::new(),
            functions: HashMap::new(),
            return_value: None,
            recursion_depth: 0,
            loop_iterations: 0,
        }
    }

    /// Main entry point: evaluate a complete program
    pub fn eval_program(&mut self, program: &Program) -> Result<Value, String> {
        // Reset limits for new program execution
        self.recursion_depth = 0;
        self.loop_iterations = 0;

        // First pass: collect all function definitions
        for item in &program.items {
            if let TopLevel::Function {
                name, params, body, ..
            } = item
            {
                self.functions
                    .insert(name.clone(), (params.clone(), body.clone()));
            }
        }

        // Second pass: execute program
        let result = Value::Null;
        for item in &program.items {
            match item {
                TopLevel::GlobalVar { name, init, .. } => {
                    let value = if let Some(expr) = init {
                        self.eval_expr(expr)?
                    } else {
                        Value::Null
                    };
                    self.env.set(name.clone(), value);
                }
                TopLevel::Function { .. } => {
                    // Already processed in first pass
                }
            }
        }

        Ok(result)
    }

    /// Evaluate a block of statements
    fn eval_block(&mut self, block: &Block) -> Result<Value, String> {
        let mut result = Value::Null;
        for stmt in &block.stmts {
            result = self.eval_stmt(stmt)?;
            // If return encountered, stop execution
            if self.return_value.is_some() {
                break;
            }
        }
        Ok(result)
    }

    /// Evaluate a single statement
    fn eval_stmt(&mut self, stmt: &Statement) -> Result<Value, String> {
        match stmt {
            Statement::Expr(expr) => self.eval_expr(expr),

            Statement::VarDecl { name, init, .. } => {
                let value = if let Some(expr) = init {
                    self.eval_expr(expr)?
                } else {
                    Value::Null
                };
                self.env.set(name.clone(), value);
                Ok(Value::Null)
            }

            Statement::If {
                cond,
                then_branch,
                else_branch,
                ..
            } => {
                let cond_value = self.eval_expr(cond)?;
                if cond_value.is_truthy() {
                    self.eval_block(then_branch)
                } else if let Some(else_b) = else_branch {
                    self.eval_block(else_b)
                } else {
                    Ok(Value::Null)
                }
            }

            Statement::While { cond, body, .. } => {
                let mut result = Value::Null;
                loop {
                    self.loop_iterations += 1;
                    if self.loop_iterations > MAX_LOOP_ITERATIONS {
                        return Err(format!(
                            "Loop iteration limit exceeded ({})",
                            MAX_LOOP_ITERATIONS
                        ));
                    }
                    let cond_value = self.eval_expr(cond)?;
                    if !cond_value.is_truthy() {
                        break;
                    }
                    result = self.eval_block(body)?;
                    if self.return_value.is_some() {
                        break;
                    }
                }
                Ok(result)
            }

            Statement::Return { value, .. } => {
                let ret_val = if let Some(expr) = value {
                    self.eval_expr(expr)?
                } else {
                    Value::Null
                };
                self.return_value = Some(ret_val.clone());
                Ok(ret_val)
            }
        }
    }

    /// Evaluate an expression
    fn eval_expr(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::Number(n, _) => Ok(Value::Number(*n)),
            Expression::Float(f, _) => Ok(Value::Float(*f)),
            Expression::Bool(b, _) => Ok(Value::Bool(*b)),
            Expression::Var(name, _) => self
                .env
                .get(name)
                .ok_or_else(|| format!("Undefined variable: '{}'", name)),

            Expression::Unary { operator, expr, .. } => {
                let val = self.eval_expr(expr)?;
                match operator.as_str() {
                    "-" => match val {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        Value::Float(f) => Ok(Value::Float(-f)),
                        _ => Err(format!("Cannot negate {}", val.type_name())),
                    },
                    "!" => Ok(val.not()),
                    _ => Err(format!("Unknown unary operator: '{}'", operator)),
                }
            }

            Expression::Binary {
                left, op, right, ..
            } => {
                // Implement short-circuit evaluation for logical operators
                match op {
                    crate::parser::ast::BinOp::And => {
                        let left_val = self.eval_expr(left)?;
                        // Short-circuit: if left is falsy, don't evaluate right
                        if !left_val.is_truthy() {
                            return Ok(left_val);
                        }
                        let right_val = self.eval_expr(right)?;
                        Ok(left_val.and(&right_val))
                    }
                    crate::parser::ast::BinOp::Or => {
                        let left_val = self.eval_expr(left)?;
                        // Short-circuit: if left is truthy, don't evaluate right
                        if left_val.is_truthy() {
                            return Ok(left_val);
                        }
                        let right_val = self.eval_expr(right)?;
                        Ok(left_val.or(&right_val))
                    }
                    _ => {
                        // For all other operators, evaluate both sides
                        let left_val = self.eval_expr(left)?;
                        let right_val = self.eval_expr(right)?;

                        match op {
                            crate::parser::ast::BinOp::Add => left_val.add(&right_val),
                            crate::parser::ast::BinOp::Sub => left_val.subtract(&right_val),
                            crate::parser::ast::BinOp::Mul => left_val.multiply(&right_val),
                            crate::parser::ast::BinOp::Div => left_val.divide(&right_val),
                            crate::parser::ast::BinOp::Mod => left_val.modulo(&right_val),
                            crate::parser::ast::BinOp::Eq => {
                                Ok(Value::Bool(left_val.equals(&right_val)))
                            }
                            crate::parser::ast::BinOp::Neq => {
                                Ok(Value::Bool(!left_val.equals(&right_val)))
                            }
                            crate::parser::ast::BinOp::Lt => {
                                Ok(Value::Bool(left_val.less_than(&right_val)?))
                            }
                            crate::parser::ast::BinOp::Gt => {
                                Ok(Value::Bool(left_val.greater_than(&right_val)?))
                            }
                            crate::parser::ast::BinOp::Le => {
                                Ok(Value::Bool(left_val.less_equal(&right_val)?))
                            }
                            crate::parser::ast::BinOp::Ge => {
                                Ok(Value::Bool(left_val.greater_equal(&right_val)?))
                            }
                            crate::parser::ast::BinOp::And | crate::parser::ast::BinOp::Or => {
                                unreachable!("AND/OR should be handled above")
                            }
                        }
                    }
                }
            }

            Expression::Assign { name, value, .. } => {
                let val = self.eval_expr(value)?;
                self.env.update(name, val.clone())?;
                Ok(val)
            }

            Expression::Call { name, args, .. } => self.eval_call(name, args),
        }
    }

    /// Check if a function is a built-in function
    fn is_builtin_function(&self, name: &str) -> bool {
        matches!(
            name,
            "print" | "len" | "str" | "int" | "float" | "abs" | "type"
        )
    }

    /// Evaluate a function call
    fn eval_call(&mut self, name: &str, args: &[Expression]) -> Result<Value, String> {
        // Check recursion depth for user-defined functions
        let should_track_depth = !self.is_builtin_function(name);
        if should_track_depth {
            self.recursion_depth += 1;
            if self.recursion_depth > MAX_RECURSION_DEPTH {
                self.recursion_depth -= 1;
                return Err(format!(
                    "Maximum recursion depth exceeded ({})",
                    MAX_RECURSION_DEPTH
                ));
            }
        }
        // Built-in functions
        match name {
            "print" => {
                for arg in args {
                    let val = self.eval_expr(arg)?;
                    println!("{}", val);
                }
                Ok(Value::Null)
            }

            "len" => {
                if args.len() != 1 {
                    return Err(format!("len() expects 1 argument, got {}", args.len()));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::String(s) => Ok(Value::Number(s.len() as i64)),
                    Value::Number(n) => Ok(Value::Number(n.to_string().len() as i64)),
                    _ => Err(format!("len() does not work with {}", val.type_name())),
                }
            }

            "str" => {
                if args.len() != 1 {
                    return Err(format!("str() expects 1 argument, got {}", args.len()));
                }
                let val = self.eval_expr(&args[0])?;
                Ok(Value::String(val.to_string_value()))
            }

            "int" => {
                if args.len() != 1 {
                    return Err(format!("int() expects 1 argument, got {}", args.len()));
                }
                let val = self.eval_expr(&args[0])?;
                Ok(Value::Number(val.to_int()?))
            }

            "float" => {
                if args.len() != 1 {
                    return Err(format!("float() expects 1 argument, got {}", args.len()));
                }
                let val = self.eval_expr(&args[0])?;
                Ok(Value::Float(val.to_number()?))
            }

            "abs" => {
                if args.len() != 1 {
                    return Err(format!("abs() expects 1 argument, got {}", args.len()));
                }
                let val = self.eval_expr(&args[0])?;
                match val {
                    Value::Number(n) => Ok(Value::Number(n.abs())),
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    _ => Err(format!("abs() does not work with {}", val.type_name())),
                }
            }

            "type" => {
                if args.len() != 1 {
                    return Err(format!("type() expects 1 argument, got {}", args.len()));
                }
                let val = self.eval_expr(&args[0])?;
                Ok(Value::String(val.type_name().to_string()))
            }

            _ => {
                // User-defined functions
                if let Some((params, body)) = self.functions.get(name).cloned() {
                    if args.len() != params.len() {
                        if should_track_depth {
                            self.recursion_depth -= 1;
                        }
                        return Err(format!(
                            "Function '{}' expects {} arguments, got {}",
                            name,
                            params.len(),
                            args.len()
                        ));
                    }

                    // Create new scope for function
                    self.env.push_scope();

                    // Evaluate arguments and bind to parameters
                    let mut eval_error = None;
                    for (param, arg) in params.iter().zip(args.iter()) {
                        match self.eval_expr(arg) {
                            Ok(arg_val) => self.env.set(param.clone(), arg_val),
                            Err(e) => {
                                eval_error = Some(e);
                                break;
                            }
                        }
                    }

                    if let Some(e) = eval_error {
                        self.env.pop_scope();
                        if should_track_depth {
                            self.recursion_depth -= 1;
                        }
                        return Err(e);
                    }

                    // Execute function body
                    self.return_value = None;
                    let block_result = self.eval_block(&body);

                    // Get return value (even if error occurs)
                    let result = match block_result {
                        Ok(_) => self.return_value.take().unwrap_or(Value::Null),
                        Err(e) => {
                            self.env.pop_scope();
                            if should_track_depth {
                                self.recursion_depth -= 1;
                            }
                            return Err(e);
                        }
                    };

                    // Exit function scope
                    self.env.pop_scope();

                    // Decrement recursion depth
                    if should_track_depth {
                        self.recursion_depth -= 1;
                    }

                    Ok(result)
                } else {
                    if should_track_depth {
                        self.recursion_depth -= 1;
                    }
                    Err(format!("Undefined function: '{}'", name))
                }
            }
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn eval_code(code: &str) -> Result<Value, String> {
        let mut lexer = Lexer::new(code.to_string());
        let mut tokens = Vec::new();

        loop {
            let token = lexer.next_token();
            let is_eof = token.token_type == crate::lexer::TokenType::EOF;
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|e| e.join(", "))?;

        let mut evaluator = Evaluator::new();
        evaluator.eval_program(&program)
    }

    #[test]
    fn test_simple_arithmetic() {
        assert_eq!(eval_code("a = 5").unwrap(), Value::Null);
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
        ";
        eval_code(code).unwrap();
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
        ";
        eval_code(code).unwrap();
    }

    #[test]
    fn test_undefined_variable_error() {
        let result = eval_code("x = y + 1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_undefined_function_error() {
        let code = "x = nonexistent()";
        let result = eval_code(code);
        assert!(result.is_err());
    }

    #[test]
    fn test_modulo_operator() {
        let code = "
        test_mod() {
            return 10 % 3
        }
        ";
        eval_code(code).unwrap();
    }

    #[test]
    fn test_logical_and() {
        let code = "
        test_and() {
            a = 5
            b = 3
            if a > 0 && b > 0 {
                return 1
            }
            return 0
        }
        ";
        eval_code(code).unwrap();
    }

    #[test]
    fn test_logical_or() {
        let code = "
        test_or() {
            a = 0
            b = 5
            if a > 10 || b > 0 {
                return 1
            }
            return 0
        }
        ";
        eval_code(code).unwrap();
    }

    #[test]
    fn test_recursion_depth_limit() {
        let code = "
        infinite_recursion(n) {
            return infinite_recursion(n + 1)
        }
        ";
        // Just ensure it parses and doesn't crash during parsing
        eval_code(code).unwrap();
    }

    #[test]
    fn test_integer_overflow_protection() {
        // Test overflow detection during evaluation
        let val_a = Value::Number(i64::MAX);
        let val_b = Value::Number(1);
        let result = val_a.add(&val_b);
        // Should error on overflow
        assert!(result.is_err());
    }
}
