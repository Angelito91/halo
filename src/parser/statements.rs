// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::lexer::TokenKind;
use crate::parser::ast::{Block, ElseIfBranch, Statement};
use super::Parser;

impl Parser {
    // Block and statement rules

    pub(crate) fn parse_block(&mut self) -> Block {
        let pos = self.previous().position;
        let mut stmts = Vec::new();

        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_newlines();

            if self.check(TokenKind::RightBrace) || self.is_at_end() {
                break;
            }

            match self.parse_statement() {
                Some(stmt) => stmts.push(stmt),
                None => {
                    self.advance();
                }
            }
        }

        Block::new(stmts, pos)
    }

    pub(crate) fn parse_statement(&mut self) -> Option<Statement> {
        if self.is_at_end() || self.check(TokenKind::RightBrace) {
            return None;
        }

        if self.advance_if(TokenKind::If) {
            self.parse_if_statement()
        } else if self.advance_if(TokenKind::Return) {
            self.parse_return_statement()
        } else if self.advance_if(TokenKind::While) {
            self.parse_while_statement()
        } else if self.advance_if(TokenKind::For) {
            self.parse_for_statement()
        } else if self.advance_if(TokenKind::Break) {
            Some(Statement::Break {
                pos: self.previous().position,
            })
        } else if self.advance_if(TokenKind::Continue) {
            Some(Statement::Continue {
                pos: self.previous().position,
            })
        } else if self.check(TokenKind::Identifier) {
            // Peek past the identifier to decide: assignment vs. expression.
            let saved = self.cursor;
            self.advance();
            let is_assignment = self.check(TokenKind::Assign);
            self.cursor = saved;

            if is_assignment {
                self.parse_var_decl()
            } else {
                self.parse_expression_statement()
            }
        } else {
            self.parse_expression_statement()
        }
    }

    pub(crate) fn parse_if_statement(&mut self) -> Option<Statement> {
        let pos = self.previous().position;

        let cond = self.parse_expression()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after if condition")?;
        let then_branch = self.parse_block();
        self.consume(TokenKind::RightBrace, "Expected '}' after if body")?;

        let mut else_if_branches: Vec<ElseIfBranch> = Vec::new();
        let mut else_branch: Option<Block> = None;

        // Newlines are allowed between the closing `}` and an `else`.
        self.skip_newlines();

        while self.advance_if(TokenKind::Else) {
            self.skip_newlines();

            if self.advance_if(TokenKind::If) {
                // `else if <cond> { … }`
                let branch_pos = self.previous().position;
                let branch_cond = self.parse_expression()?;
                self.consume(
                    TokenKind::LeftBrace,
                    "Expected '{' after 'else if' condition",
                )?;
                let branch_body = self.parse_block();
                self.consume(TokenKind::RightBrace, "Expected '}' after 'else if' body")?;

                else_if_branches.push(ElseIfBranch {
                    cond: branch_cond,
                    body: branch_body,
                    pos: branch_pos,
                });

                self.skip_newlines();
            } else {
                // Plain `else { … }` — nothing can follow after this.
                self.consume(TokenKind::LeftBrace, "Expected '{' after 'else'")?;
                let block = self.parse_block();
                self.consume(TokenKind::RightBrace, "Expected '}' after 'else' body")?;
                else_branch = Some(block);
                break;
            }
        }

        Some(Statement::If {
            cond,
            then_branch,
            else_if_branches,
            else_branch,
            pos,
        })
    }

    pub(crate) fn parse_while_statement(&mut self) -> Option<Statement> {
        let pos = self.previous().position;
        let cond = self.parse_expression()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after while condition")?;
        let body = self.parse_block();
        self.consume(TokenKind::RightBrace, "Expected '}' after while body")?;
        Some(Statement::While { cond, body, pos })
    }

    /// `for <var> in <iterable> { … }`
    pub(crate) fn parse_for_statement(&mut self) -> Option<Statement> {
        let pos = self.previous().position;

        let var = if self.check(TokenKind::Identifier) {
            self.advance().lexeme.clone()
        } else {
            self.error("Expected loop variable after 'for'");
            return None;
        };

        self.consume(TokenKind::In, "Expected 'in' after for variable")?;
        let iterable = self.parse_expression()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after for iterable")?;
        let body = self.parse_block();
        self.consume(TokenKind::RightBrace, "Expected '}' after for body")?;

        Some(Statement::For {
            var,
            iterable,
            body,
            pos,
        })
    }

    pub(crate) fn parse_return_statement(&mut self) -> Option<Statement> {
        let pos = self.previous().position;

        // A `return` with no expression is allowed when it is immediately
        // followed by a newline, `}`, or EOF.
        let value = if self.check(TokenKind::Newline)
            || self.check(TokenKind::RightBrace)
            || self.is_at_end()
        {
            None
        } else {
            self.parse_expression()
        };

        Some(Statement::Return { value, pos })
    }

    pub(crate) fn parse_var_decl(&mut self) -> Option<Statement> {
        let name_tok = self.consume(TokenKind::Identifier, "Expected variable name")?;
        let name = name_tok.lexeme.clone();
        let pos = name_tok.position;

        self.consume(TokenKind::Assign, "Expected '=' in variable declaration")?;
        let init = self.parse_expression()?;

        Some(Statement::VarDecl {
            name,
            init: Some(init),
            pos,
        })
    }

    pub(crate) fn parse_expression_statement(&mut self) -> Option<Statement> {
        self.parse_expression().map(Statement::Expr)
    }
}
