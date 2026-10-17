// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::cli::args::Args;
use crate::cli::helpers::{file_stem, read_source, tokenize};
use crate::diagnostics::colors;

pub fn cmd_tokens(args: &Args) -> i32 {
    let source = match read_source(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {e}");
            return 1;
        }
    };

    let tokens = tokenize(&source);

    println!("╔════════════════════════════════════════╗");
    println!("║       📋 Token Stream — {:<14} ║", file_stem(&args.file));
    println!("╚════════════════════════════════════════╝");
    println!();

    for (i, tok) in tokens.iter().enumerate() {
        use crate::lexer::TokenKind;
        match tok.kind {
            TokenKind::Eof => {
                println!("{i:4}  {}", colors::muted("EOF"));
            }
            TokenKind::Newline => {
                println!(
                    "{:4}  {:<20}          {}",
                    i,
                    colors::highlight("NEWLINE"),
                    colors::muted(&format!("{}:{}", tok.position.line, tok.position.column))
                );
            }
            _ => {
                println!(
                    "{i:4}  {:<20} {:>10}    {}",
                    colors::info(&format!("{:?}", tok.kind)),
                    colors::success(&format!("'{}'", tok.lexeme)),
                    colors::muted(&format!("{}:{}", tok.position.line, tok.position.column))
                );
            }
        }
    }

    println!();
    println!("{} token(s)", colors::highlight(&tokens.len().to_string()));
    0
}
