// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
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
        let chars: Vec<char> = input.chars().collect();
        Lexer {
            input: chars,
            position: 0,
            line: 1u32,
            column: 1u32,
        }
    }

    // Get the current character without advancing the position
    pub fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    // Peek at the next character without advancing
    pub fn peek_char(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    // Get the current character and advance the position
    pub fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    // Skip whitespace characters
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() && c != '\n' {
                self.next_char();
            } else {
                break;
            }
        }
    }

    // Get current position
    fn get_position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    // A way to create simple tokens
    pub fn simple_token(&mut self, token_type: TokenType, c: char) -> Token {
        let pos = self.get_position();
        self.next_char();
        Token::new(token_type, c.to_string(), pos)
    }

    // Check if a word is a keyword and return the appropriate TokenType
    fn get_keyword_token_type(word: &str) -> Option<TokenType> {
        match word {
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "while" => Some(TokenType::While),
            "return" => Some(TokenType::Return),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "and" => Some(TokenType::And),
            "or" => Some(TokenType::Or),
            "not" => Some(TokenType::Not),
            _ => None,
        }
    }

    // Skip single-line comments (// ... end of line)
    fn skip_comment(&mut self) {
        if self.current_char() == Some('/') && self.peek_char() == Some('/') {
            self.next_char(); // consume first /
            self.next_char(); // consume second /

            // Read until we find newline or EOF
            while let Some(c) = self.current_char() {
                if c == '\n' {
                    // Don't consume the newline, let next_token handle it
                    break;
                }
                self.next_char();
            }
        }
    }

    // Create next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        // Skip comments
        while self.current_char() == Some('/') && self.peek_char() == Some('/') {
            self.skip_comment();
            self.skip_whitespace();
        }

        let pos = self.get_position();

        match self.current_char() {
            None => Token::new(TokenType::EOF, String::from("EOF"), pos),

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

                let token_type =
                    Self::get_keyword_token_type(&ident).unwrap_or(TokenType::Identifier);

                Token::new(token_type, ident, pos)
            }

            Some(c) if c.is_numeric() => {
                let mut num_str = String::new();
                while let Some(c) = self.current_char() {
                    if c.is_numeric() || c == '.' {
                        num_str.push(c);
                        self.next_char();
                    } else {
                        break;
                    }
                }

                Token::new(TokenType::Number, num_str, pos)
            }

            // Two-character operators
            Some('=') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Token::new(TokenType::Equal, String::from("=="), pos)
                } else {
                    Token::new(TokenType::Assign, String::from("="), pos)
                }
            }

            Some('!') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Token::new(TokenType::NotEqual, String::from("!="), pos)
                } else {
                    Token::new(TokenType::Not, String::from("!"), pos)
                }
            }

            Some('<') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Token::new(TokenType::LessEqual, String::from("<="), pos)
                } else {
                    Token::new(TokenType::Less, String::from("<"), pos)
                }
            }

            Some('>') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Token::new(TokenType::GreaterEqual, String::from(">="), pos)
                } else {
                    Token::new(TokenType::Greater, String::from(">"), pos)
                }
            }

            Some('&') => {
                self.next_char();
                if self.current_char() == Some('&') {
                    self.next_char();
                    Token::new(TokenType::And, String::from("&&"), pos)
                } else {
                    Token::new(TokenType::Unknown, String::from("&"), pos)
                }
            }

            Some('|') => {
                self.next_char();
                if self.current_char() == Some('|') {
                    self.next_char();
                    Token::new(TokenType::Or, String::from("||"), pos)
                } else {
                    Token::new(TokenType::Unknown, String::from("|"), pos)
                }
            }

            // Single character operators and delimiters
            Some('+') => self.simple_token(TokenType::Plus, '+'),
            Some('-') => self.simple_token(TokenType::Minus, '-'),
            Some('*') => self.simple_token(TokenType::Star, '*'),
            Some('/') => {
                // Check if this is a comment (// requires peeking ahead)
                if self.peek_char() == Some('/') {
                    // This is a comment, skip it
                    self.skip_comment();
                    // Return next token after comment
                    return self.next_token();
                } else {
                    // This is division operator
                    self.simple_token(TokenType::Slash, '/')
                }
            }
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
            Some('\n') => {
                self.next_char();
                Token::new(TokenType::Newline, String::from("\n"), pos)
            }

            // For unknown characters
            Some(c) => {
                self.next_char();
                Token::new(TokenType::Unknown, c.to_string(), pos)
            }
        }
    }
}
