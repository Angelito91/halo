// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::parser::ast::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum TokenType {
    // Special Words
    If,
    Else,
    While,
    True,
    Return,
    False,

    // Identifiers and literals
    Identifier,
    Number,

    // Operators
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Modulo,       // %
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
    #[allow(dead_code)]
    Whitespace,
    #[allow(dead_code)]
    Comment,
    Newline, // For statement termination without ;
    Unknown, // For invalid/unknown characters
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
}
