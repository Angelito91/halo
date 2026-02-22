use crate::token::{Token, TokenType};

// Easy way to create tokens for operators
macro_rules! token_op {
    ($self:ident, $c:expr, $token:expr) => {{
        $self.next_char();
        Token::new($token, $c.to_string())
    }};
}

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

                Token::new(TokenType::Integer, num_str.parse().unwrap())
            }

            // Operators and delimiters
            Some('+') => token_op!(self, '+', TokenType::Plus),
            Some('-') => token_op!(self, '-', TokenType::Minus),
            Some('*') => token_op!(self, '*', TokenType::Star),
            Some('/') => token_op!(self, '/', TokenType::Slash),
            Some('=') => token_op!(self, '=', TokenType::Equal),
            Some('(') => token_op!(self, '(', TokenType::LeftParen),
            Some(')') => token_op!(self, ')', TokenType::RightParen),
            Some('{') => token_op!(self, '{', TokenType::LeftBrace),
            Some('}') => token_op!(self, '}', TokenType::RightBrace),
            Some('[') => token_op!(self, '[', TokenType::LeftBracket),
            Some(']') => token_op!(self, ']', TokenType::RightBracket),
            Some(';') => token_op!(self, ';', TokenType::Semicolon),
            Some(',') => token_op!(self, ',', TokenType::Comma),

            Some(c) => {
                self.next_char();
                Token::new(TokenType::Identifier, format!("{} UNKNOWN", c))
            }
        }
    }
}
