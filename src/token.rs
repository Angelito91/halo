#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Speacial Words
    If,
    Else,
    While,
    Fn,
    True,
    False,

    // Identificadores y literales
    Identifier,
    Float,
    Integer,

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
    And,          // &&
    Or,           // ||
    Not,          // !

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
