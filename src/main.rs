mod lexer;
mod token;

use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

fn main() {
    let input = "int name".to_string();
    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token.to_string());

        if token.get_token_type() == TokenType::EOF {
            break;
        }
    }
}
