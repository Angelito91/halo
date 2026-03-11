// The Halo Programming Language
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use std::fmt;

use crate::parser::ast::Position;

/// Every distinct kind of token the lexer can produce.
///
/// Variants are grouped by category to make the match arms in the parser
/// easier to scan visually.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // ── Keywords ─────────────────────────────────────────────────────────────
    If,
    Else,
    While,
    Return,
    True,
    False,
    Break,
    Continue,

    // ── Literals & identifiers ────────────────────────────────────────────────
    /// An unquoted name: `foo`, `_bar`, `my_var2`.
    Identifier,
    /// An integer or floating-point numeric literal: `42`, `3.14`.
    Number,
    /// A double-quoted string literal: `"hello world"`.
    StringLit,

    // ── Arithmetic operators ──────────────────────────────────────────────────
    Plus,   // +
    Minus,  // -
    Star,   // *
    Slash,  // /
    Modulo, // %

    // ── Assignment ────────────────────────────────────────────────────────────
    Assign, // =

    // ── Comparison operators ──────────────────────────────────────────────────
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // ── Logical operators ─────────────────────────────────────────────────────
    And, // &&
    Or,  // ||
    Not, // !

    // ── Delimiters ────────────────────────────────────────────────────────────
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Colon,        // :
    Comma,        // ,
    Dot,          // .

    // ── Special ───────────────────────────────────────────────────────────────
    /// Horizontal whitespace (currently not emitted into the token stream).
    #[allow(dead_code)]
    Whitespace,
    /// A `//` line comment (currently not emitted into the token stream).
    #[allow(dead_code)]
    Comment,
    /// A `\n` newline, which acts as the statement terminator (no semicolons).
    Newline,
    /// An unrecognised character; the parser will surface a meaningful error.
    Unknown,
    /// Signals the end of the token stream.
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::While => "while",
            TokenKind::Return => "return",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Break => "break",
            TokenKind::Continue => "continue",
            TokenKind::Identifier => "<identifier>",
            TokenKind::Number => "<number>",
            TokenKind::StringLit => "<string>",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Modulo => "%",
            TokenKind::Assign => "=",
            TokenKind::Equal => "==",
            TokenKind::NotEqual => "!=",
            TokenKind::Less => "<",
            TokenKind::Greater => ">",
            TokenKind::LessEqual => "<=",
            TokenKind::GreaterEqual => ">=",
            TokenKind::And => "&&",
            TokenKind::Or => "||",
            TokenKind::Not => "!",
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::LeftBracket => "[",
            TokenKind::RightBracket => "]",
            TokenKind::Colon => ":",
            TokenKind::Comma => ",",
            TokenKind::Dot => ".",
            TokenKind::Whitespace => "<whitespace>",
            TokenKind::Comment => "<comment>",
            TokenKind::Newline => "<newline>",
            TokenKind::Unknown => "<unknown>",
            TokenKind::Eof => "<eof>",
        };
        f.write_str(text)
    }
}

/// A single token produced by the lexer, carrying its kind, raw text, and
/// source location.
#[derive(Debug, Clone)]
pub struct Token {
    /// The syntactic category of this token.
    pub kind: TokenKind,
    /// The exact slice of source text that was matched.
    pub lexeme: String,
    /// Where in the source file this token begins.
    pub position: Position,
}

impl Token {
    /// Construct a new `Token`.
    pub fn new(kind: TokenKind, lexeme: String, position: Position) -> Self {
        Self {
            kind,
            lexeme,
            position,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "'{}' ({}:{})",
            self.lexeme, self.position.line, self.position.column
        )
    }
}
