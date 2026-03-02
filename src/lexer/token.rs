// The Halo Programming Language
// Version: 0.1.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::parser::ast::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Special Words
    If,
    Else,
    While,
    True,
    False,

    // Identifiers and literals
    Identifier,
    Number,

    // Operators
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Assign,       // =
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // Comparative
    And, // &&
    Or,  // ||
    Not, // !

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Colon,        // :
    Comma,        // ,
    Dot,          // .

    // Special
    Whitespace,
    Comment,
    Newline, // For statement termination without ;
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub position: Position,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, position: Position) -> Self {
        Token {
            token_type,
            lexeme,
            position,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Token({:?}, '{}', {:?})",
            self.token_type, self.lexeme, self.position
        )
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_value(&self) -> String {
        self.lexeme.clone()
    }
}
