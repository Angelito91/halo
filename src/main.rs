// The Halo Programming Language
// Version: 0.1.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
mod ast;
mod lexer;
mod token;

use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

fn main() {
    let input = "
        int a = 5
        int b = 10
        if a == b {
            print(\"a is equal to b\");
        } else {
            print(\"a is not equal to b\");
        }
        "
    .to_string();
    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token.to_string());

        if token.get_token_type() == TokenType::EOF {
            break;
        }
    }
}
