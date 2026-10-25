// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use super::Parser;
use crate::lexer::TokenKind;
use crate::parser::ast::TopLevel;

impl Parser {
    pub(crate) fn parse_toplevel(&mut self) -> Option<TopLevel> {
        self.skip_newlines();

        if self.is_at_end() {
            return None;
        }

        // Bare control-flow at the top level is legal — it will be emitted
        // inside the generated `main` function by the compiler.
        if matches!(
            self.peek().kind,
            TokenKind::If
                | TokenKind::While
                | TokenKind::For
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Return
        ) {
            return self.parse_toplevel_statement();
        }

        if self.check(TokenKind::Identifier) {
            // We need one token of look-ahead past the identifier to decide
            // whether we are looking at a function declaration, a global
            // variable assignment, or a standalone expression.
            return self.parse_toplevel_identifier();
        }

        // Bare literal expressions and unary-prefixed expressions at the top
        // level are valid (e.g. `1 + 2`, `"hello"`, `-x`, `!flag`, `(a + b)`).
        if matches!(
            self.peek().kind,
            TokenKind::Number
                | TokenKind::StringLit
                | TokenKind::True
                | TokenKind::False
                | TokenKind::Minus
                | TokenKind::Not
                | TokenKind::LeftParen
        ) {
            return self.parse_toplevel_expression();
        }

        self.error("Expected a function definition, global variable, or statement");
        None
    }

    /// Dispatch on what follows a leading identifier at the top level.
    pub(crate) fn parse_toplevel_identifier(&mut self) -> Option<TopLevel> {
        // Save cursor so we can restore it after peeking ahead.
        let saved = self.cursor;
        self.advance(); // consume the identifier

        if self.check(TokenKind::LeftParen) {
            // Skip over the parameter list to check for a trailing `{`.
            while !self.check(TokenKind::RightParen) && !self.is_at_end() {
                self.advance();
            }
            if !self.is_at_end() {
                self.advance(); // consume `)`
            }

            let is_function_decl = self.check(TokenKind::LeftBrace);
            self.cursor = saved;

            if is_function_decl {
                self.parse_function()
            } else {
                self.parse_toplevel_expression()
            }
        } else if self.check(TokenKind::Assign) {
            self.cursor = saved;
            self.parse_global_var()
        } else {
            self.cursor = saved;
            self.parse_toplevel_expression()
        }
    }

    pub(crate) fn parse_function(&mut self) -> Option<TopLevel> {
        let name_tok = self.consume(TokenKind::Identifier, "Expected function name")?;
        let name = name_tok.lexeme.clone();
        let pos = name_tok.position;

        self.consume(TokenKind::LeftParen, "Expected '(' after function name")?;

        let mut params = Vec::new();
        if !self.check(TokenKind::RightParen) {
            loop {
                let param = self.consume(TokenKind::Identifier, "Expected parameter name")?;
                params.push(param.lexeme.clone());
                if !self.advance_if(TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenKind::RightParen, "Expected ')' after parameters")?;
        self.consume(TokenKind::LeftBrace, "Expected '{' before function body")?;
        let body = self.parse_block();
        self.consume(TokenKind::RightBrace, "Expected '}' after function body")?;

        Some(TopLevel::Function {
            name,
            params,
            body,
            pos,
        })
    }

    pub(crate) fn parse_global_var(&mut self) -> Option<TopLevel> {
        let name_tok = self.consume(TokenKind::Identifier, "Expected variable name")?;
        let name = name_tok.lexeme.clone();
        let pos = name_tok.position;

        self.consume(
            TokenKind::Assign,
            "Expected '=' in global variable declaration",
        )?;
        let init = self.parse_expression()?;

        Some(TopLevel::GlobalVar {
            name,
            init: Some(init),
            pos,
        })
    }

    /// Wrap a bare top-level expression (e.g. a `print(…)` call) as a
    /// synthetic `GlobalVar` so the compiler can collect it into `main`.
    pub(crate) fn parse_toplevel_expression(&mut self) -> Option<TopLevel> {
        let expr = self.parse_expression()?;
        let pos = expr.pos();
        Some(TopLevel::GlobalVar {
            name: "__expr".to_string(),
            init: Some(expr),
            pos,
        })
    }

    /// Wrap a bare top-level statement (`if`, `while`, `break`, `continue`,
    /// `return`) as a [`TopLevel::Stmt`] so the compiler emits it inside
    /// `main`.
    pub(crate) fn parse_toplevel_statement(&mut self) -> Option<TopLevel> {
        let stmt = self.parse_statement()?;
        let pos = stmt.pos();
        Some(TopLevel::Stmt { stmt, pos })
    }
}
