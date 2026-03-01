// The Halo Programming Language
// Version: 0.1.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Variables' types
    Int,
    Float,
    Bool,
    String,

    // Speacial Words
    If,
    Else,
    While,
    Fn,
    True,
    False,

    // Identificadores y literales
    Identifier,
    Number,

    // Operadores
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

    // Comperative
    And, // &&
    Or,  // ||
    Not, // !

    // Delimitadores
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Colon,        // :
    Comma,        // ,
    Dot,          // .

    // Especiales
    Whitespace,
    Comment,
    EOF,
}

pub struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }

    pub fn to_string(&self) -> String {
        format!("Token({:?}) {}", self.token_type, self.value)
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}
