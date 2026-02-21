#[derive(Debug)]
pub enum TokenType {
    // Speacial Words
    If,
    Else,
    While,
    Fn,
    True,
    False,

    // Identificadores y literales
    Identifier(String),
    Number(f64),
    String(String),

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
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    Semicolon, // ;
    Colon,     // :
    Comma,     // ,
    Dot,       // .

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
        format!("Token({:?}) '{}'", self.token_type, self.value)
    }
}
