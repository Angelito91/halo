// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use super::Parser;
use crate::lexer::TokenKind;
use crate::parser::ast::{BinOp, Expression, IndexType};

impl Parser {
    // Expression rules (Pratt / precedence-climbing style)
    // Precedence: logical-or → logical-and → equality → comparison → term → factor → unary → primary

    pub(crate) fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_logical_or()
    }

    pub(crate) fn parse_logical_or(&mut self) -> Option<Expression> {
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

    pub(crate) fn parse_logical_and(&mut self) -> Option<Expression> {
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

    pub(crate) fn parse_equality(&mut self) -> Option<Expression> {
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

    pub(crate) fn parse_comparison(&mut self) -> Option<Expression> {
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

    pub(crate) fn parse_term(&mut self) -> Option<Expression> {
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

    pub(crate) fn parse_factor(&mut self) -> Option<Expression> {
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

    pub(crate) fn parse_unary(&mut self) -> Option<Expression> {
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
            self.parse_postfix()
        }
    }

    pub(crate) fn parse_postfix(&mut self) -> Option<Expression> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.advance_if(TokenKind::LeftBracket) {
                let pos = expr.pos();
                let index = self.parse_index_or_slice()?;
                self.consume(TokenKind::RightBracket, "Expected ']' after array index")?;
                expr = Expression::Index {
                    array: Box::new(expr),
                    index,
                    pos,
                };
            } else if self.advance_if(TokenKind::Dot) {
                // Method call: object.method(...) or object.method
                let method_pos = expr.pos();

                if !self.check(TokenKind::Identifier) {
                    self.error("Expected method name after dot");
                    return None;
                }

                self.advance();
                let method = self.previous().lexeme.clone();

                let mut args = Vec::new();
                if self.advance_if(TokenKind::LeftParen) {
                    // Method with arguments: object.method(arg1, arg2)
                    if !self.check(TokenKind::RightParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if !self.advance_if(TokenKind::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenKind::RightParen, "Expected ')' after method arguments")?;
                } else {
                    // Method without arguments: object.method
                    // This is valid - will be converted to method call with no additional args
                }

                expr = Expression::MethodCall {
                    object: Box::new(expr),
                    method,
                    args,
                    pos: method_pos,
                };
            } else {
                break;
            }
        }

        Some(expr)
    }

    /// Parse either a single index or a slice expression.
    /// Handles:
    /// - arr[5]       -> Single(5)
    /// - arr[1:4]     -> Slice { start: Some(1), end: Some(4) }
    /// - arr[..3]     -> Slice { start: None, end: Some(3) }
    /// - arr[2..]     -> Slice { start: Some(2), end: None }
    /// - arr[..]      -> Slice { start: None, end: None }
    pub(crate) fn parse_index_or_slice(&mut self) -> Option<IndexType> {
        // Check if we start with a colon (means no start, e.g., arr[..3])
        if self.check(TokenKind::Colon) {
            self.advance(); // consume the colon

            // Parse end if it exists (e.g., arr[..3])
            let end = if self.check(TokenKind::RightBracket) {
                None
            } else {
                Some(Box::new(self.parse_expression()?))
            };

            return Some(IndexType::Slice { start: None, end });
        }

        // Parse first expression (could be index or slice start)
        let first = self.parse_expression()?;

        // Check if colon follows (indicating a slice)
        if self.advance_if(TokenKind::Colon) {
            // It's a slice: first is the start
            let end = if self.check(TokenKind::RightBracket) {
                None
            } else {
                Some(Box::new(self.parse_expression()?))
            };

            Some(IndexType::Slice {
                start: Some(Box::new(first)),
                end,
            })
        } else {
            // It's a single index
            Some(IndexType::Single(Box::new(first)))
        }
    }

    pub(crate) fn parse_primary(&mut self) -> Option<Expression> {
        if self.advance_if(TokenKind::Number) {
            let tok = self.previous();
            let lexeme = tok.lexeme.clone();
            let pos = tok.position;
            // Try integer first; if the lexeme contains a dot (float literal)
            // or overflows i64, fall back to f64.
            if let Ok(value) = lexeme.parse::<i64>() {
                return Some(Expression::Number(value, pos));
            }
            if let Ok(value) = lexeme.parse::<f64>() {
                return Some(Expression::Float(value, pos));
            }
            // Neither parse succeeded — surface an error.
            self.error(&format!("Invalid numeric literal '{lexeme}'"));
            return None;
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

        if self.advance_if(TokenKind::LeftBracket) {
            let pos = self.previous().position;
            let mut elements = Vec::new();
            if !self.check(TokenKind::RightBracket) {
                loop {
                    elements.push(self.parse_expression()?);
                    if !self.advance_if(TokenKind::Comma) {
                        break;
                    }
                }
            }
            self.consume(TokenKind::RightBracket, "Expected ']' after array elements")?;
            return Some(Expression::ArrayLiteral { elements, pos });
        }

        if self.advance_if(TokenKind::LeftParen) {
            let inner = self.parse_expression()?;
            self.consume(TokenKind::RightParen, "Expected ')' after expression")?;
            return Some(inner);
        }

        self.error("Expected an expression");
        None
    }
}
