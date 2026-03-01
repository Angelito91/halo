// The Halo Programming Language
// Version: 0.1.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    // Get the current character without advancing the position
    pub fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    // Get the current character and advance the position
    pub fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.input.chars().nth(self.position) {
            self.position += 1;
            Some(c)
        } else {
            None
        }
    }

    // Skip whitespace characters
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    // A way to create simple tokens
    pub fn simple_token(&mut self, token_type: TokenType, c: char) -> Token {
        self.next_char();
        Token::new(token_type, c.to_string())
    }

    // Create next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut token_type = TokenType::Identifier;

        match self.current_char() {
            None => Token::new(TokenType::EOF, String::from("NONE")),

            Some(c) if c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(c) = self.current_char() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        self.next_char();

                        token_type = match ident.as_str() {
                            "int" => TokenType::Int,
                            "bool" => TokenType::Bool,
                            "string" => TokenType::String,
                            "float" => TokenType::Float,
                            "if" => TokenType::If,
                            "else" => TokenType::Else,
                            "while" => TokenType::While,
                            "fn" => TokenType::Fn,
                            "and" => TokenType::And,
                            "or" => TokenType::Or,
                            "not" => TokenType::Not,
                            "true" => TokenType::True,
                            "false" => TokenType::False,
                            "==" => TokenType::Equal,
                            "!=" => TokenType::NotEqual,
                            ">=" => TokenType::GreaterEqual,
                            "<=" => TokenType::LessEqual,
                            _ => TokenType::Identifier,
                        };

                        if token_type != TokenType::Identifier {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                Token::new(token_type, ident)
            }

            Some(c) if c.is_numeric() => {
                let mut num_str = String::new();
                while let Some(c) = self.current_char() {
                    if c.is_numeric() {
                        num_str.push(c);
                        self.next_char();
                    } else {
                        break;
                    }
                }

                Token::new(TokenType::Number, num_str.parse().unwrap())
            }

            // Operators and delimiters
            Some('+') => self.simple_token(TokenType::Plus, '+'),
            Some('-') => self.simple_token(TokenType::Minus, '-'),
            Some('*') => self.simple_token(TokenType::Star, '*'),
            Some('/') => self.simple_token(TokenType::Slash, '/'),
            Some('=') => self.simple_token(TokenType::Assign, '='),
            Some('(') => self.simple_token(TokenType::LeftParen, '('),
            Some(')') => self.simple_token(TokenType::RightParen, ')'),
            Some('{') => self.simple_token(TokenType::LeftBrace, '{'),
            Some('}') => self.simple_token(TokenType::RightBrace, '}'),
            Some('[') => self.simple_token(TokenType::LeftBracket, '['),
            Some(']') => self.simple_token(TokenType::RightBracket, ']'),
            Some(':') => self.simple_token(TokenType::Colon, ':'),
            Some(';') => self.simple_token(TokenType::Semicolon, ';'),
            Some(',') => self.simple_token(TokenType::Comma, ','),
            Some('>') => self.simple_token(TokenType::Greater, '>'),
            Some('<') => self.simple_token(TokenType::Less, '<'),
            Some('.') => self.simple_token(TokenType::Dot, '.'),

            // For Unknown characters
            Some(c) => {
                self.next_char();
                Token::new(TokenType::Identifier, format!("{} UNKNOWN", c))
            }
        }
    }
}
