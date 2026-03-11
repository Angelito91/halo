// The Halo Programming Language
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::lexer::{Token, TokenKind};
use crate::parser::ast::{BinOp, Block, ElseIfBranch, Expression, Program, Statement, TopLevel};

/// Recursive-descent parser that converts a flat token stream into a typed
/// [`Program`] AST.
///
/// # Error recovery
///
/// The parser is designed to keep going after a syntax error: when a rule
/// fails it records the error in `errors` and returns `None`.  The caller
/// skips one token and retries so that a single typo does not abort the whole
/// file.  After [`Parser::parse`] returns you should check whether any errors
/// were recorded before using the resulting AST.
pub struct Parser {
    /// Complete token stream, including the terminal [`TokenKind::Eof`].
    tokens: Vec<Token>,
    /// Index of the token that will be returned by the next call to [`peek`].
    cursor: usize,
    /// Syntax errors accumulated during parsing.
    errors: Vec<String>,
}

impl Parser {
    /// Create a new `Parser` for the given token stream.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0,
            errors: Vec::new(),
        }
    }

    /// Parse the full token stream into a [`Program`].
    ///
    /// Returns `Ok(program)` when no syntax errors were encountered, or
    /// `Err(errors)` with a list of human-readable messages otherwise.
    pub fn parse(&mut self) -> Result<Program, Vec<String>> {
        let program = self.parse_program();
        if self.errors.is_empty() {
            Ok(program)
        } else {
            Err(self.errors.clone())
        }
    }

    // =========================================================================
    // Top-level rules
    // =========================================================================

    fn parse_program(&mut self) -> Program {
        let mut items = Vec::new();

        while !self.is_at_end() {
            self.skip_newlines();

            if self.is_at_end() {
                break;
            }

            match self.parse_toplevel() {
                Some(item) => items.push(item),
                // On failure: skip the offending token and keep going.
                None => {
                    self.advance();
                }
            }
        }

        Program::new(items)
    }

    fn parse_toplevel(&mut self) -> Option<TopLevel> {
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

        self.error("Expected a function definition, global variable, or statement");
        None
    }

    /// Dispatch on what follows a leading identifier at the top level.
    fn parse_toplevel_identifier(&mut self) -> Option<TopLevel> {
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

    fn parse_function(&mut self) -> Option<TopLevel> {
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

    fn parse_global_var(&mut self) -> Option<TopLevel> {
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
    fn parse_toplevel_expression(&mut self) -> Option<TopLevel> {
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
    fn parse_toplevel_statement(&mut self) -> Option<TopLevel> {
        let stmt = self.parse_statement()?;
        let pos = stmt.pos();
        Some(TopLevel::Stmt { stmt, pos })
    }

    // =========================================================================
    // Block and statement rules
    // =========================================================================

    fn parse_block(&mut self) -> Block {
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

    fn parse_statement(&mut self) -> Option<Statement> {
        if self.is_at_end() || self.check(TokenKind::RightBrace) {
            return None;
        }

        if self.advance_if(TokenKind::If) {
            self.parse_if_statement()
        } else if self.advance_if(TokenKind::Return) {
            self.parse_return_statement()
        } else if self.advance_if(TokenKind::While) {
            self.parse_while_statement()
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

    fn parse_if_statement(&mut self) -> Option<Statement> {
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

    fn parse_while_statement(&mut self) -> Option<Statement> {
        let pos = self.previous().position;
        let cond = self.parse_expression()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after while condition")?;
        let body = self.parse_block();
        self.consume(TokenKind::RightBrace, "Expected '}' after while body")?;
        Some(Statement::While { cond, body, pos })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
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

    fn parse_var_decl(&mut self) -> Option<Statement> {
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

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        self.parse_expression().map(Statement::Expr)
    }

    // =========================================================================
    // Expression rules  (Pratt / precedence-climbing style)
    //
    // Precedence, lowest → highest:
    //   logical-or  →  logical-and  →  equality  →  comparison
    //   →  term  →  factor  →  unary  →  primary
    // =========================================================================

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Option<Expression> {
        let mut lhs = self.parse_logical_and()?;

        while self.advance_if(TokenKind::Or) {
            let rhs = self.parse_logical_and()?;
            lhs = Expression::Binary {
                left: Box::new(lhs),
                op: BinOp::Or,
                right: Box::new(rhs),
                pos: self.previous().position,
            };
        }

        Some(lhs)
    }

    fn parse_logical_and(&mut self) -> Option<Expression> {
        let mut lhs = self.parse_equality()?;

        while self.advance_if(TokenKind::And) {
            let rhs = self.parse_equality()?;
            lhs = Expression::Binary {
                left: Box::new(lhs),
                op: BinOp::And,
                right: Box::new(rhs),
                pos: self.previous().position,
            };
        }

        Some(lhs)
    }

    fn parse_equality(&mut self) -> Option<Expression> {
        let mut lhs = self.parse_comparison()?;

        while self.advance_if(TokenKind::Equal) || self.advance_if(TokenKind::NotEqual) {
            let op = match self.previous().kind {
                TokenKind::Equal => BinOp::Eq,
                TokenKind::NotEqual => BinOp::Neq,
                _ => unreachable!(),
            };
            let rhs = self.parse_comparison()?;
            lhs = Expression::Binary {
                left: Box::new(lhs),
                op,
                right: Box::new(rhs),
                pos: self.previous().position,
            };
        }

        Some(lhs)
    }

    fn parse_comparison(&mut self) -> Option<Expression> {
        let mut lhs = self.parse_term()?;

        while self.advance_if(TokenKind::Greater)
            || self.advance_if(TokenKind::GreaterEqual)
            || self.advance_if(TokenKind::Less)
            || self.advance_if(TokenKind::LessEqual)
        {
            let op = match self.previous().kind {
                TokenKind::Greater => BinOp::Gt,
                TokenKind::GreaterEqual => BinOp::Ge,
                TokenKind::Less => BinOp::Lt,
                TokenKind::LessEqual => BinOp::Le,
                _ => unreachable!(),
            };
            let rhs = self.parse_term()?;
            lhs = Expression::Binary {
                left: Box::new(lhs),
                op,
                right: Box::new(rhs),
                pos: self.previous().position,
            };
        }

        Some(lhs)
    }

    fn parse_term(&mut self) -> Option<Expression> {
        let mut lhs = self.parse_factor()?;

        while self.advance_if(TokenKind::Plus) || self.advance_if(TokenKind::Minus) {
            let op = match self.previous().kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            let rhs = self.parse_factor()?;
            lhs = Expression::Binary {
                left: Box::new(lhs),
                op,
                right: Box::new(rhs),
                pos: self.previous().position,
            };
        }

        Some(lhs)
    }

    fn parse_factor(&mut self) -> Option<Expression> {
        let mut lhs = self.parse_unary()?;

        while self.advance_if(TokenKind::Star)
            || self.advance_if(TokenKind::Slash)
            || self.advance_if(TokenKind::Modulo)
        {
            let op = match self.previous().kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::Modulo => BinOp::Mod,
                _ => unreachable!(),
            };
            let rhs = self.parse_unary()?;
            lhs = Expression::Binary {
                left: Box::new(lhs),
                op,
                right: Box::new(rhs),
                pos: self.previous().position,
            };
        }

        Some(lhs)
    }

    fn parse_unary(&mut self) -> Option<Expression> {
        if self.advance_if(TokenKind::Minus) {
            let pos = self.previous().position;
            let operand = self.parse_unary()?;
            Some(Expression::Unary {
                operator: "-".to_string(),
                expr: Box::new(operand),
                pos,
            })
        } else if self.advance_if(TokenKind::Not) {
            let pos = self.previous().position;
            let operand = self.parse_unary()?;
            Some(Expression::Unary {
                operator: "!".to_string(),
                expr: Box::new(operand),
                pos,
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Option<Expression> {
        if self.advance_if(TokenKind::Number) {
            let tok = self.previous();
            // `parse::<i64>()` returns `None` on overflow — the `?` propagates
            // the failure up and adds an implicit error via the `ok()?` chain.
            let value = tok.lexeme.parse::<i64>().ok()?;
            return Some(Expression::Number(value, tok.position));
        }

        if self.advance_if(TokenKind::StringLit) {
            let tok = self.previous();
            return Some(Expression::StringLiteral(tok.lexeme.clone(), tok.position));
        }

        if self.advance_if(TokenKind::True) {
            return Some(Expression::Bool(true, self.previous().position));
        }

        if self.advance_if(TokenKind::False) {
            return Some(Expression::Bool(false, self.previous().position));
        }

        if self.advance_if(TokenKind::Identifier) {
            let name = self.previous().lexeme.clone();
            let id_pos = self.previous().position;

            if self.advance_if(TokenKind::LeftParen) {
                // Function call: `name(arg, …)`
                let mut args = Vec::new();
                if !self.check(TokenKind::RightParen) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.advance_if(TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.consume(TokenKind::RightParen, "Expected ')' after arguments")?;
                return Some(Expression::Call {
                    name,
                    args,
                    pos: id_pos,
                });
            }

            return Some(Expression::Var(name, id_pos));
        }

        if self.advance_if(TokenKind::LeftParen) {
            let inner = self.parse_expression()?;
            self.consume(TokenKind::RightParen, "Expected ')' after expression")?;
            return Some(inner);
        }

        self.error("Expected an expression");
        None
    }

    // =========================================================================
    // Cursor helpers
    // =========================================================================

    /// Consume the current token and return it if its kind matches `kind`;
    /// otherwise record an error and return `None`.
    fn consume(&mut self, kind: TokenKind, message: &str) -> Option<&Token> {
        if self.check(kind) {
            Some(self.advance())
        } else {
            self.error(message);
            None
        }
    }

    /// Advance past the current token if its kind matches `kind`.
    /// Returns `true` when a token was consumed (replaces the old
    /// `match_token` name, which was ambiguous with the `match` keyword).
    fn advance_if(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Return `true` when the current token has the given `kind`.
    /// Always returns `false` at EOF to avoid out-of-bounds access.
    fn check(&self, kind: TokenKind) -> bool {
        !self.is_at_end() && self.peek().kind == kind
    }

    /// Advance the cursor by one and return a reference to the consumed token.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.cursor += 1;
        }
        self.previous()
    }

    /// Skip zero or more consecutive `Newline` tokens.
    fn skip_newlines(&mut self) {
        while self.advance_if(TokenKind::Newline) {}
    }

    /// `true` when the current token is [`TokenKind::Eof`].
    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    /// The token at the current cursor position without consuming it.
    fn peek(&self) -> &Token {
        &self.tokens[self.cursor]
    }

    /// The most recently consumed token.
    ///
    /// # Panics
    /// Panics if called before any token has been consumed (cursor == 0).
    fn previous(&self) -> &Token {
        &self.tokens[self.cursor - 1]
    }

    /// Record a syntax error at the current token's position.
    fn error(&mut self, message: &str) {
        let tok = self.peek();
        self.errors.push(format!(
            "Error at {}:{}: {}",
            tok.position.line, tok.position.column, message
        ));
    }
}
