// The Halo Programming Language
// Token definitions
// Version: 0.2.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::parser::ast::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum TokenType {
    // Keywords
    If,
    Else,
    While,
    Return,
    True,
    False,
    Break,
    Continue,

    // Identifiers and literals
    Identifier,
    Number,
    StringLit, // "hello world"

    // Arithmetic operators
    Plus,   // +
    Minus,  // -
    Star,   // *
    Slash,  // /
    Modulo, // %

    // Assignment
    Assign, // =

    // Comparison operators
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // Logical operators
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
    Newline, // Statement terminator (no semicolons)
    Unknown, // Invalid / unrecognised character
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
