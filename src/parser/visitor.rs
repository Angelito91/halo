// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// NOTE: This visitor pattern is currently not used in the interpreter.
// It's kept for potential future use in optimization passes, linting, or analysis tools.
// To use this, enable the visitor module exports in parser/mod.rs

use crate::parser::ast::{Block, Expression, Program, Statement, TopLevel};

#[allow(dead_code)]
pub trait ASTVisitor {
    fn visit_program(&mut self, program: &Program) {
        for item in &program.items {
            self.visit_toplevel(item);
        }
    }

    fn visit_toplevel(&mut self, item: &TopLevel) {
        match item {
            TopLevel::Function { body, .. } => {
                self.visit_block(body);
            }
            TopLevel::GlobalVar { .. } => {}
            TopLevel::Stmt { stmt, .. } => {
                self.visit_statement(stmt);
            }
        }
    }

    fn visit_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.visit_statement(stmt);
        }
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Expr(e) => self.visit_expression(e),
            Statement::VarDecl { init, .. } => {
                if let Some(e) = init {
                    self.visit_expression(e);
                }
            }
            Statement::If {
                cond,
                then_branch,
                else_if_branches,
                else_branch,
                ..
            } => {
                self.visit_expression(cond);
                self.visit_block(then_branch);
                for branch in else_if_branches {
                    self.visit_expression(&branch.cond);
                    self.visit_block(&branch.body);
                }
                if let Some(else_b) = else_branch {
                    self.visit_block(else_b);
                }
            }
            Statement::While { cond, body, .. } => {
                self.visit_expression(cond);
                self.visit_block(body);
            }
            Statement::Return { value, .. } => {
                if let Some(e) = value {
                    self.visit_expression(e);
                }
            }
            Statement::Break { .. } | Statement::Continue { .. } => {}
        }
    }

    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Number(_, _)
            | Expression::Float(_, _)
            | Expression::Bool(_, _)
            | Expression::StringLiteral(_, _)
            | Expression::Var(_, _) => {}
            Expression::Unary { expr, .. } => self.visit_expression(expr),
            Expression::Binary { left, right, .. } => {
                self.visit_expression(left);
                self.visit_expression(right);
            }
            Expression::Assign { value, .. } => self.visit_expression(value),
            Expression::Call { args, .. } => {
                for a in args {
                    self.visit_expression(a);
                }
            }
        }
    }
}
