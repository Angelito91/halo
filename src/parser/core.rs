// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::lexer::{Token, TokenKind};
use crate::parser::ast::Program;
use std::fmt;

/// A syntax error with the source position where it was detected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub line: u32,
    pub column: u32,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error at {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

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
    pub(crate) tokens: Vec<Token>,
    /// Index of the token that will be returned by the next call to [`peek`].
    pub(crate) cursor: usize,
    /// Syntax errors accumulated during parsing.
    pub(crate) errors: Vec<ParseError>,
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
    /// `Err(errors)` with the structured syntax errors otherwise.
    pub fn parse(&mut self) -> Result<Program, Vec<ParseError>> {
        let program = self.parse_program();
        if self.errors.is_empty() {
            Ok(program)
        } else {
            Err(self.errors.clone())
        }
    }

    // Top-level rules

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

    // Cursor helpers

    pub(crate) fn consume(&mut self, kind: TokenKind, message: &str) -> Option<&Token> {
        if self.check(kind) {
            Some(self.advance())
        } else {
            self.error(message);
            None
        }
    }

    pub(crate) fn advance_if(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(crate) fn check(&self, kind: TokenKind) -> bool {
        !self.is_at_end() && self.peek().kind == kind
    }

    pub(crate) fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.cursor += 1;
        }
        self.previous()
    }

    pub(crate) fn skip_newlines(&mut self) {
        while self.advance_if(TokenKind::Newline) {}
    }

    pub(crate) fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    pub(crate) fn peek(&self) -> &Token {
        &self.tokens[self.cursor]
    }

    pub(crate) fn previous(&self) -> &Token {
        &self.tokens[self.cursor - 1]
    }

    pub(crate) fn error(&mut self, message: &str) {
        let tok = self.peek();
        self.errors.push(ParseError {
            line: tok.position.line,
            column: tok.position.column,
            message: message.to_string(),
        });
    }
}
