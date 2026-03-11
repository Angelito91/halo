// The Halo Programming Language
// Lexer — converts source text into a token stream
// Version: 0.2.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::Position;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: u32,
    column: u32,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    // ── Character navigation ──────────────────────────────────────────────────

    pub fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    pub fn peek_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    pub fn next_char(&mut self) -> Option<char> {
        let c = self.input.get(self.position).copied()?;
        self.position += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    fn get_position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    // ── Whitespace / comment skipping ─────────────────────────────────────────

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c != '\n' && c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        // Caller already verified current == '/' && peek == '/'
        self.next_char(); // first  /
        self.next_char(); // second /
        while let Some(c) = self.current_char() {
            if c == '\n' {
                break; // leave the newline for the main loop
            }
            self.next_char();
        }
    }

    // ── Keyword table ─────────────────────────────────────────────────────────

    fn keyword(word: &str) -> Option<TokenType> {
        match word {
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "while" => Some(TokenType::While),
            "return" => Some(TokenType::Return),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "break" => Some(TokenType::Break),
            "continue" => Some(TokenType::Continue),
            "and" => Some(TokenType::And),
            "or" => Some(TokenType::Or),
            "not" => Some(TokenType::Not),
            _ => None,
        }
    }

    // ── Simple single-character token helper ──────────────────────────────────

    pub fn simple_token(&mut self, tt: TokenType, c: char) -> Token {
        let pos = self.get_position();
        self.next_char();
        Token::new(tt, c.to_string(), pos)
    }

    // ── String literal tokenizer ──────────────────────────────────────────────

    /// Scan a double-quoted string literal starting after the opening `"`.
    /// Supports the following escape sequences:
    ///   \\  →  \
    ///   \"  →  "
    ///   \n  →  newline
    ///   \t  →  tab
    ///   \r  →  carriage return
    /// Any other `\x` is left as `\x` (passed through unchanged).
    fn scan_string(&mut self, pos: Position) -> Token {
        let mut value = String::new();

        loop {
            match self.current_char() {
                None => {
                    // Unterminated string — emit what we have with Unknown type
                    // so the parser can surface a meaningful error.
                    return Token::new(TokenType::Unknown, value, pos);
                }
                Some('"') => {
                    self.next_char(); // consume closing "
                    break;
                }
                Some('\\') => {
                    self.next_char(); // consume backslash
                    match self.current_char() {
                        Some('\\') => {
                            self.next_char();
                            value.push('\\');
                        }
                        Some('"') => {
                            self.next_char();
                            value.push('"');
                        }
                        Some('n') => {
                            self.next_char();
                            value.push('\n');
                        }
                        Some('t') => {
                            self.next_char();
                            value.push('\t');
                        }
                        Some('r') => {
                            self.next_char();
                            value.push('\r');
                        }
                        Some(c) => {
                            self.next_char();
                            value.push('\\');
                            value.push(c);
                        }
                        None => break,
                    }
                }
                Some('\n') => {
                    // Newline inside a string — include it and keep scanning.
                    self.next_char();
                    value.push('\n');
                }
                Some(c) => {
                    self.next_char();
                    value.push(c);
                }
            }
        }

        Token::new(TokenType::StringLit, value, pos)
    }

    // ── Main tokenizer ────────────────────────────────────────────────────────

    pub fn next_token(&mut self) -> Token {
        // Skip horizontal whitespace and line comments, then repeat.
        loop {
            self.skip_whitespace();
            if self.current_char() == Some('/') && self.peek_char() == Some('/') {
                self.skip_line_comment();
            } else {
                break;
            }
        }

        let pos = self.get_position();

        match self.current_char() {
            // ── EOF ───────────────────────────────────────────────────────────
            None => Token::new(TokenType::EOF, "EOF".to_string(), pos),

            // ── Identifiers / keywords ────────────────────────────────────────
            Some(c) if c.is_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while let Some(c) = self.current_char() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        self.next_char();
                    } else {
                        break;
                    }
                }
                let tt = Self::keyword(&ident).unwrap_or(TokenType::Identifier);
                Token::new(tt, ident, pos)
            }

            // ── Numeric literals ──────────────────────────────────────────────
            Some(c) if c.is_ascii_digit() => {
                let mut num = String::new();
                let mut has_dot = false;
                while let Some(c) = self.current_char() {
                    if c.is_ascii_digit() {
                        num.push(c);
                        self.next_char();
                    } else if c == '.'
                        && !has_dot
                        && self.peek_char().map_or(false, |p| p.is_ascii_digit())
                    {
                        has_dot = true;
                        num.push(c);
                        self.next_char();
                    } else {
                        break;
                    }
                }
                Token::new(TokenType::Number, num, pos)
            }

            // ── String literals ───────────────────────────────────────────────
            Some('"') => {
                self.next_char(); // consume opening "
                self.scan_string(pos)
            }

            // ── Two-character operators ───────────────────────────────────────
            Some('=') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Token::new(TokenType::Equal, "==".to_string(), pos)
                } else {
                    Token::new(TokenType::Assign, "=".to_string(), pos)
                }
            }
            Some('!') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Token::new(TokenType::NotEqual, "!=".to_string(), pos)
                } else {
                    Token::new(TokenType::Not, "!".to_string(), pos)
                }
            }
            Some('<') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Token::new(TokenType::LessEqual, "<=".to_string(), pos)
                } else {
                    Token::new(TokenType::Less, "<".to_string(), pos)
                }
            }
            Some('>') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Token::new(TokenType::GreaterEqual, ">=".to_string(), pos)
                } else {
                    Token::new(TokenType::Greater, ">".to_string(), pos)
                }
            }
            Some('&') => {
                self.next_char();
                if self.current_char() == Some('&') {
                    self.next_char();
                    Token::new(TokenType::And, "&&".to_string(), pos)
                } else {
                    Token::new(TokenType::Unknown, "&".to_string(), pos)
                }
            }
            Some('|') => {
                self.next_char();
                if self.current_char() == Some('|') {
                    self.next_char();
                    Token::new(TokenType::Or, "||".to_string(), pos)
                } else {
                    Token::new(TokenType::Unknown, "|".to_string(), pos)
                }
            }

            // ── Single-character tokens ───────────────────────────────────────
            Some('+') => self.simple_token(TokenType::Plus, '+'),
            Some('-') => self.simple_token(TokenType::Minus, '-'),
            Some('*') => self.simple_token(TokenType::Star, '*'),
            Some('/') => self.simple_token(TokenType::Slash, '/'),
            Some('%') => self.simple_token(TokenType::Modulo, '%'),
            Some('(') => self.simple_token(TokenType::LeftParen, '('),
            Some(')') => self.simple_token(TokenType::RightParen, ')'),
            Some('{') => self.simple_token(TokenType::LeftBrace, '{'),
            Some('}') => self.simple_token(TokenType::RightBrace, '}'),
            Some('[') => self.simple_token(TokenType::LeftBracket, '['),
            Some(']') => self.simple_token(TokenType::RightBracket, ']'),
            Some(':') => self.simple_token(TokenType::Colon, ':'),
            Some(',') => self.simple_token(TokenType::Comma, ','),
            Some('.') => self.simple_token(TokenType::Dot, '.'),

            // ── Newline (statement terminator) ────────────────────────────────
            Some('\n') => {
                self.next_char();
                Token::new(TokenType::Newline, "\n".to_string(), pos)
            }

            // ── Unknown / invalid character ───────────────────────────────────
            Some(c) => {
                self.next_char();
                Token::new(TokenType::Unknown, c.to_string(), pos)
            }
        }
    }
}
