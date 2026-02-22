use crate::token::{self, Token, TokenType};

// Easy way to create tokens for operators

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

        match self.current_char() {
            None => Token::new(TokenType::EOF, String::from("NONE")),

            Some(c) if c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(c) = self.current_char() {
                    if c.is_alphanumeric() {
                        ident.push(c);
                        self.next_char();
                    } else {
                        break;
                    }
                }

                Token::new(TokenType::Identifier, ident)
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
            Some('=') => self.simple_token(TokenType::Equal, '='),
            Some('(') => self.simple_token(TokenType::LeftParen, '('),
            Some(')') => self.simple_token(TokenType::RightParen, ')'),
            Some('{') => self.simple_token(TokenType::LeftBrace, '{'),
            Some('}') => self.simple_token(TokenType::RightBrace, '}'),
            Some('[') => self.simple_token(TokenType::LeftBracket, '['),
            Some(']') => self.simple_token(TokenType::RightBracket, ']'),
            Some(';') => self.simple_token(TokenType::Semicolon, ';'),
            Some(',') => self.simple_token(TokenType::Comma, ','),

            Some(c) => {
                self.next_char();
                Token::new(TokenType::Identifier, format!("{} UNKNOWN", c))
            }
        }
    }
}
