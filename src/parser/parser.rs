// The Halo Programming Language
// Parser module
// Version: 0.1.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::lexer::{Token, TokenType};
use crate::parser::ast::{BinOp, Block, Expression, Program, Statement, TopLevel};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, Vec<String>> {
        let program = self.parse_program();
        if self.errors.is_empty() {
            Ok(program)
        } else {
            Err(self.errors.clone())
        }
    }

    fn parse_program(&mut self) -> Program {
        let mut items = Vec::new();
        while !self.is_at_end() {
            // Skip newlines before top-level item
            while self.match_token(TokenType::Newline) {}

            if self.is_at_end() {
                break;
            }

            match self.parse_toplevel() {
                Some(item) => items.push(item),
                None => {
                    // Skip the problematic token and continue
                    self.advance();
                }
            }
        }
        Program::new(items)
    }

    fn parse_toplevel(&mut self) -> Option<TopLevel> {
        // Skip any remaining newlines
        while self.match_token(TokenType::Newline) {}

        if self.is_at_end() {
            return None;
        }

        if self.check(TokenType::Identifier) {
            // Peek to see if it's a function or variable
            let saved_current = self.current;
            self.advance();
            let is_function = self.check(TokenType::LeftParen);
            self.current = saved_current;

            if is_function {
                self.parse_function()
            } else {
                self.parse_global_var()
            }
        } else {
            self.error("Expected function or global variable declaration");
            None
        }
    }

    fn parse_function(&mut self) -> Option<TopLevel> {
        let name_token = self.consume(TokenType::Identifier, "Expected function name")?;
        let name = name_token.lexeme.clone();
        let pos = name_token.position;

        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                let param_token = self.consume(TokenType::Identifier, "Expected parameter name")?;
                params.push(param_token.lexeme.clone());
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;
        self.consume(TokenType::LeftBrace, "Expected '{' before function body")?;
        let body = self.parse_block();
        self.consume(TokenType::RightBrace, "Expected '}' after function body")?;
        Some(TopLevel::Function {
            name,
            params,
            body,
            pos,
        })
    }

    fn parse_global_var(&mut self) -> Option<TopLevel> {
        let name_token = self.consume(TokenType::Identifier, "Expected variable name")?;
        let name = name_token.lexeme.clone();
        let pos = name_token.position;

        if !self.check(TokenType::Assign) {
            self.error("Expected '=' in global variable");
            return None;
        }
        self.advance(); // consume '='

        let init = self.parse_expression()?;
        Some(TopLevel::GlobalVar {
            name,
            init: Some(init),
            pos,
        })
    }

    fn parse_block(&mut self) -> Block {
        let pos = self.previous().position;
        let mut stmts = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            // Skip newlines
            while self.match_token(TokenType::Newline) {}

            if self.check(TokenType::RightBrace) || self.is_at_end() {
                break;
            }

            match self.parse_statement() {
                Some(stmt) => stmts.push(stmt),
                None => {
                    // Skip the problematic token
                    self.advance();
                }
            }
        }
        Block::new(stmts, pos)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if self.is_at_end() || self.check(TokenType::RightBrace) {
            return None;
        }

        if self.match_token(TokenType::If) {
            self.parse_if_statement()
        } else if self.match_token(TokenType::While) {
            self.parse_while_statement()
        } else if self.check(TokenType::Identifier) {
            let saved_current = self.current;
            self.advance();
            let is_var_decl = self.check(TokenType::Assign);
            self.current = saved_current;

            if is_var_decl {
                self.parse_var_decl()
            } else {
                self.parse_expression_statement()
            }
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        let pos = self.previous().position;
        let cond = self.parse_expression()?;
        self.consume(TokenType::LeftBrace, "Expected '{' after if condition")?;
        let then_branch = self.parse_block();
        self.consume(TokenType::RightBrace, "Expected '}' after if body")?;
        let else_branch = if self.match_token(TokenType::Else) {
            self.consume(TokenType::LeftBrace, "Expected '{' after else")?;
            let block = self.parse_block();
            self.consume(TokenType::RightBrace, "Expected '}' after else body")?;
            Some(block)
        } else {
            None
        };
        Some(Statement::If {
            cond,
            then_branch,
            else_branch,
            pos,
        })
    }

    fn parse_while_statement(&mut self) -> Option<Statement> {
        let pos = self.previous().position;
        let cond = self.parse_expression()?;
        self.consume(TokenType::LeftBrace, "Expected '{' after while condition")?;
        let body = self.parse_block();
        self.consume(TokenType::RightBrace, "Expected '}' after while body")?;
        Some(Statement::While { cond, body, pos })
    }

    fn parse_var_decl(&mut self) -> Option<Statement> {
        let name_token = self.consume(TokenType::Identifier, "Expected variable name")?;
        let name = name_token.lexeme.clone();
        let pos = name_token.position;

        self.consume(TokenType::Assign, "Expected '=' in variable declaration")?;
        let init = self.parse_expression()?;
        Some(Statement::VarDecl {
            name,
            init: Some(init),
            pos,
        })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression()?;
        Some(Statement::Expr(expr))
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Option<Expression> {
        let mut expr = self.parse_comparison()?;
        while self.match_token(TokenType::Equal) || self.match_token(TokenType::NotEqual) {
            let op = match self.previous().token_type {
                TokenType::Equal => BinOp::Eq,
                TokenType::NotEqual => BinOp::Neq,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                pos: self.previous().position,
            };
        }
        Some(expr)
    }

    fn parse_comparison(&mut self) -> Option<Expression> {
        let mut expr = self.parse_term()?;
        while self.match_token(TokenType::Greater)
            || self.match_token(TokenType::GreaterEqual)
            || self.match_token(TokenType::Less)
            || self.match_token(TokenType::LessEqual)
        {
            let op = match self.previous().token_type {
                TokenType::Greater => BinOp::Gt,
                TokenType::GreaterEqual => BinOp::Ge,
                TokenType::Less => BinOp::Lt,
                TokenType::LessEqual => BinOp::Le,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                pos: self.previous().position,
            };
        }
        Some(expr)
    }

    fn parse_term(&mut self) -> Option<Expression> {
        let mut expr = self.parse_factor()?;
        while self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let op = match self.previous().token_type {
                TokenType::Plus => BinOp::Add,
                TokenType::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                pos: self.previous().position,
            };
        }
        Some(expr)
    }

    fn parse_factor(&mut self) -> Option<Expression> {
        let mut expr = self.parse_unary()?;
        while self.match_token(TokenType::Star) || self.match_token(TokenType::Slash) {
            let op = match self.previous().token_type {
                TokenType::Star => BinOp::Mul,
                TokenType::Slash => BinOp::Div,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                pos: self.previous().position,
            };
        }
        Some(expr)
    }

    fn parse_unary(&mut self) -> Option<Expression> {
        if self.match_token(TokenType::Minus) {
            let op = "-".to_string();
            let expr = self.parse_unary()?;
            Some(Expression::Unary {
                operator: op,
                expr: Box::new(expr),
                pos: self.previous().position,
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Option<Expression> {
        if self.match_token(TokenType::Number) {
            let prev_token = self.previous();
            let value: i64 = prev_token.lexeme.parse().ok()?;
            Some(Expression::Number(value, prev_token.position))
        } else if self.match_token(TokenType::True) {
            Some(Expression::Bool(true, self.previous().position))
        } else if self.match_token(TokenType::False) {
            Some(Expression::Bool(false, self.previous().position))
        } else if self.match_token(TokenType::Identifier) {
            let name = self.previous().lexeme.clone();
            let id_pos = self.previous().position;
            if self.match_token(TokenType::LeftParen) {
                let mut args = Vec::new();
                if !self.check(TokenType::RightParen) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.match_token(TokenType::Comma) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Expected ')' after arguments")?;
                Some(Expression::Call {
                    name,
                    args,
                    pos: id_pos,
                })
            } else {
                Some(Expression::Var(name, id_pos))
            }
        } else if self.match_token(TokenType::LeftParen) {
            let expr = self.parse_expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression")?;
            Some(expr)
        } else {
            self.error("Expected expression");
            None
        }
    }

    // Helper methods
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_next(&self) -> &Token {
        if self.current + 1 >= self.tokens.len() {
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.current + 1]
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Option<&Token> {
        if self.check(token_type) {
            Some(self.advance())
        } else {
            self.error(message);
            None
        }
    }

    fn error(&mut self, message: &str) {
        let token = self.peek();
        self.errors.push(format!(
            "Error at {}:{}: {}",
            token.position.line, token.position.column, message
        ));
    }
}
