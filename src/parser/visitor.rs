// The Halo Programming Language
// Visitor module
// Version: 0.1.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::parser::ast::{Block, Expression, Program, Statement, TopLevel};

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
                else_branch,
                ..
            } => {
                self.visit_expression(cond);
                self.visit_block(then_branch);
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
        }
    }

    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Number(_, _)
            | Expression::Float(_, _)
            | Expression::Bool(_, _)
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
