// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::cli::args::Args;
use crate::cli::helpers::{parse_tokens, read_source, tokenize};
use crate::diagnostics::colors;

pub fn cmd_check(args: &Args) -> i32 {
    let source = match read_source(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {e}");
            return 1;
        }
    };

    let tokens = tokenize(&source);

    match parse_tokens(tokens, &source, &args.file) {
        Ok(program) => {
            println!(
                "{}  {} — {} {}",
                colors::success("✅"),
                colors::highlight(&args.file),
                colors::success("OK"),
                colors::muted(&format!("({} top-level item(s))", program.items.len()))
            );
            0
        }
        Err(diagnostics) => {
            eprintln!(
                "{}  {} — {} {}",
                colors::error("❌"),
                colors::highlight(&args.file),
                colors::error(&format!("{} error(s)", diagnostics.len())),
                colors::muted(":")
            );
            eprintln!();
            for diag in &diagnostics {
                eprintln!("{}", diag.to_string_formatted());
            }
            1
        }
    }
}
