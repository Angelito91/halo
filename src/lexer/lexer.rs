// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::lexer::token::{Token, TokenKind};
use crate::parser::ast::Position;

/// Converts Halo source code into a sequence of [`Token`]s.
///
/// Single-pass, hand-written scanner that does not produce `Whitespace` or `Comment` tokens.
/// Newlines are emitted as tokens since Halo uses them as statement terminators (no semicolons).
pub struct Lexer {
    chars: Vec<char>,
    cursor: usize,
    line: u32,
    column: u32,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            chars: source.chars().collect(),
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn current_char(&self) -> Option<char> {
        self.chars.get(self.cursor).copied()
    }

    /// Lookahead for two-character tokens (e.g. `==`, `//`).
    pub fn peek_char(&self) -> Option<char> {
        self.chars.get(self.cursor + 1).copied()
    }

    pub fn next_char(&mut self) -> Option<char> {
        let ch = self.chars.get(self.cursor).copied()?;
        self.cursor += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    /// Snapshot of the current source position.
    fn current_position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    /// Skip horizontal whitespace but not newlines (which are statement terminators).
    pub fn skip_whitespace(&mut self) {
        while matches!(self.current_char(), Some(c) if c != '\n' && c.is_whitespace()) {
            self.next_char();
        }
    }

    /// Skip from `//` to end of line, leaving `\n` for the tokenizer to emit.
    fn skip_line_comment(&mut self) {
        self.next_char();
        self.next_char();
        while !matches!(self.current_char(), None | Some('\n')) {
            self.next_char();
        }
    }

    fn keyword_kind(word: &str) -> Option<TokenKind> {
        match word {
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "while" => Some(TokenKind::While),
            "return" => Some(TokenKind::Return),
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            "break" => Some(TokenKind::Break),
            "continue" => Some(TokenKind::Continue),
            "and" => Some(TokenKind::And),
            "or" => Some(TokenKind::Or),
            "not" => Some(TokenKind::Not),
            _ => None,
        }
    }

    fn single_char_token(&mut self, kind: TokenKind, ch: char) -> Token {
        let pos = self.current_position();
        self.next_char();
        Token::new(kind, ch.to_string(), pos)
    }

    fn two_char_token(kind: TokenKind, lexeme: &'static str, pos: Position) -> Token {
        Token::new(kind, lexeme.to_string(), pos)
    }

    /// Scan a string literal delimited by `quote` (either `"` or `'`).
    /// The opening quote must be consumed beforehand.
    /// Supports escape sequences: `\\`, `\"`, `\'`, `\n`, `\t`, `\r`.
    /// Unterminated strings produce an `Unknown` token for parser error reporting.
    fn scan_string_literal(&mut self, start_pos: Position, quote: char) -> Token {
        let mut value = String::new();

        loop {
            match self.current_char() {
                // EOF before closing quote — surface the error in the parser.
                None => return Token::new(TokenKind::Unknown, value, start_pos),

                Some(c) if c == quote => {
                    self.next_char(); // consume closing quote
                    break;
                }

                Some('\\') => {
                    self.next_char();
                    let escaped = match self.current_char() {
                        Some('\\') => '\\',
                        Some('"') => '"',
                        Some('\'') => '\'',
                        Some('n') => '\n',
                        Some('t') => '\t',
                        Some('r') => '\r',
                        // Unknown escape: emit the backslash and the raw char.
                        Some(c) => {
                            self.next_char();
                            value.push('\\');
                            value.push(c);
                            continue;
                        }
                        None => break,
                    };
                    self.next_char();
                    value.push(escaped);
                }

                Some('\n') => {
                    self.next_char();
                    value.push('\n');
                }

                Some(c) => {
                    self.next_char();
                    value.push(c);
                }
            }
        }

        Token::new(TokenKind::StringLit, value, start_pos)
    }

    /// Scan and return the next token. Call in a loop until [`TokenKind::Eof`].
    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();
            if self.current_char() == Some('/') && self.peek_char() == Some('/') {
                self.skip_line_comment();
            } else {
                break;
            }
        }

        let pos = self.current_position();

        match self.current_char() {
            None => Token::new(TokenKind::Eof, String::new(), pos),

            Some(c) if c.is_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while matches!(self.current_char(), Some(c) if c.is_alphanumeric() || c == '_') {
                    ident.push(self.next_char().unwrap());
                }
                let kind = Self::keyword_kind(&ident).unwrap_or(TokenKind::Identifier);
                Token::new(kind, ident, pos)
            }

            Some(c) if c.is_ascii_digit() => {
                let mut num = String::new();
                let mut has_dot = false;
                loop {
                    match self.current_char() {
                        Some(d) if d.is_ascii_digit() => {
                            num.push(d);
                            self.next_char();
                        }
                        Some('.')
                            if !has_dot && self.peek_char().is_some_and(|p| p.is_ascii_digit()) =>
                        {
                            has_dot = true;
                            num.push('.');
                            self.next_char();
                        }
                        _ => break,
                    }
                }
                Token::new(TokenKind::Number, num, pos)
            }

            Some('"') | Some('\'') => {
                let quote = self.current_char().unwrap();
                self.next_char(); // consume opening quote
                self.scan_string_literal(pos, quote)
            }

            Some('=') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Self::two_char_token(TokenKind::Equal, "==", pos)
                } else {
                    Token::new(TokenKind::Assign, "=".to_string(), pos)
                }
            }
            Some('!') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Self::two_char_token(TokenKind::NotEqual, "!=", pos)
                } else {
                    Token::new(TokenKind::Not, "!".to_string(), pos)
                }
            }
            Some('<') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Self::two_char_token(TokenKind::LessEqual, "<=", pos)
                } else {
                    Token::new(TokenKind::Less, "<".to_string(), pos)
                }
            }
            Some('>') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Self::two_char_token(TokenKind::GreaterEqual, ">=", pos)
                } else {
                    Token::new(TokenKind::Greater, ">".to_string(), pos)
                }
            }
            Some('&') => {
                self.next_char();
                if self.current_char() == Some('&') {
                    self.next_char();
                    Self::two_char_token(TokenKind::And, "&&", pos)
                } else {
                    Token::new(TokenKind::Unknown, "&".to_string(), pos)
                }
            }
            Some('|') => {
                self.next_char();
                if self.current_char() == Some('|') {
                    self.next_char();
                    Self::two_char_token(TokenKind::Or, "||", pos)
                } else {
                    Token::new(TokenKind::Unknown, "|".to_string(), pos)
                }
            }

            Some('+') => self.single_char_token(TokenKind::Plus, '+'),
            Some('-') => self.single_char_token(TokenKind::Minus, '-'),
            Some('*') => self.single_char_token(TokenKind::Star, '*'),
            Some('/') => self.single_char_token(TokenKind::Slash, '/'),
            Some('%') => self.single_char_token(TokenKind::Modulo, '%'),
            Some('(') => self.single_char_token(TokenKind::LeftParen, '('),
            Some(')') => self.single_char_token(TokenKind::RightParen, ')'),
            Some('{') => self.single_char_token(TokenKind::LeftBrace, '{'),
            Some('}') => self.single_char_token(TokenKind::RightBrace, '}'),
            Some('[') => self.single_char_token(TokenKind::LeftBracket, '['),
            Some(']') => self.single_char_token(TokenKind::RightBracket, ']'),
            Some(':') => self.single_char_token(TokenKind::Colon, ':'),
            Some(',') => self.single_char_token(TokenKind::Comma, ','),
            Some('.') => self.single_char_token(TokenKind::Dot, '.'),

            Some('\n') => {
                self.next_char();
                Token::new(TokenKind::Newline, "\n".to_string(), pos)
            }

            Some(c) => {
                self.next_char();
                Token::new(TokenKind::Unknown, c.to_string(), pos)
            }
        }
    }
}
