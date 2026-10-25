// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::cli::args::Args;
use crate::cli::helpers::{file_stem, parse_tokens, read_source, tokenize};
use crate::diagnostics::colors;

pub fn cmd_ast(args: &Args) -> i32 {
    let source = match read_source(&args.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {e}");
            return 1;
        }
    };

    let tokens = tokenize(&source);

    let program = match parse_tokens(tokens, &source, &args.file) {
        Ok(p) => p,
        Err(diagnostics) => {
            eprintln!();
            for diag in &diagnostics {
                eprintln!("{}", diag.to_string_formatted());
            }
            return 1;
        }
    };

    println!("╔════════════════════════════════════════╗");
    println!(
        "║   🌳 Abstract Syntax Tree — {:<11} ║",
        file_stem(&args.file)
    );
    println!("╚════════════════════════════════════════╝");
    println!();

    for (i, item) in program.items.iter().enumerate() {
        println!(
            "{}  {}",
            colors::highlight(&format!("[{i}]")),
            colors::info(toplevel_kind(item))
        );
        match item {
            crate::parser::ast::TopLevel::Function {
                name, params, body, ..
            } => {
                println!(
                    "    {}  : {}",
                    colors::success("name"),
                    colors::highlight(name)
                );
                if params.is_empty() {
                    println!(
                        "    {}  : {}",
                        colors::success("params"),
                        colors::muted("(none)")
                    );
                } else {
                    println!(
                        "    {}  : {}",
                        colors::success("params"),
                        colors::highlight(&params.join(", "))
                    );
                }
                println!(
                    "    {}  : {} {}",
                    colors::success("body"),
                    colors::highlight(&body.stmts.len().to_string()),
                    colors::muted("statement(s)")
                );
                for (j, stmt) in body.stmts.iter().enumerate() {
                    println!(
                        "      {}  {}",
                        colors::muted(&format!("[{j}]")),
                        colors::success(&stmt.to_string())
                    );
                }
            }
            crate::parser::ast::TopLevel::GlobalVar { name, init, .. } => {
                println!(
                    "    {}  : {}",
                    colors::success("name"),
                    colors::highlight(name)
                );
                match init {
                    Some(expr) => println!(
                        "    {}  : {}",
                        colors::success("init"),
                        colors::highlight(&expr.to_string())
                    ),
                    None => println!(
                        "    {}  : {}",
                        colors::success("init"),
                        colors::muted("(none)")
                    ),
                }
            }
            crate::parser::ast::TopLevel::Stmt { stmt, .. } => {
                println!(
                    "    {}  : {}",
                    colors::success("stmt"),
                    colors::highlight(&stmt.to_string())
                );
            }
        }
        println!();
    }

    println!(
        "{} top-level item(s)",
        colors::highlight(&program.items.len().to_string())
    );
    0
}

pub fn toplevel_kind(item: &crate::parser::ast::TopLevel) -> &'static str {
    match item {
        crate::parser::ast::TopLevel::Function { .. } => "Function",
        crate::parser::ast::TopLevel::GlobalVar { .. } => "GlobalVar",
        crate::parser::ast::TopLevel::Stmt { .. } => "Stmt",
    }
}
