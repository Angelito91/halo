// The Halo Programming Language
// Version: 0.1.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use parser::ast::{Expression, Statement, TopLevel};

fn main() {
    let code = "a = 5\n";

    println!("=== Halo Compiler Test ===\n");
    println!("Input:\n{}\n", code);

    // Lexer
    println!("--- Tokens ---");
    let mut lexer = Lexer::new(code.to_string());
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        println!("  {}", token.to_string());
        let is_eof = token.token_type == lexer::TokenType::EOF;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    // Parser
    println!("\n--- AST ---");
    println!("Total tokens: {}", tokens.len());
    for (i, t) in tokens.iter().enumerate() {
        println!("  [{}] {:?}", i, t.token_type);
    }
    println!();

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("✓ Parse successful!\n");
            println!("Program ({} items)\n", program.items.len());
            for item in program.items {
                print_toplevel(&item, 0);
            }
        }
        Err(errors) => {
            println!("✗ Parse failed:");
            for err in errors {
                println!("  {}", err);
            }
        }
    }
}

fn indent(level: usize) -> String {
    "  ".repeat(level)
}

fn print_toplevel(item: &TopLevel, level: usize) {
    match item {
        TopLevel::Function {
            name, params, body, ..
        } => {
            println!("{}Function: {}", indent(level), name);
            println!("{}  Parameters: {:?}", indent(level), params);
            println!("{}  Body:", indent(level));
            for stmt in &body.stmts {
                print_statement(stmt, level + 2);
            }
        }
        TopLevel::GlobalVar { name, init, .. } => {
            println!("{}GlobalVar: {}", indent(level), name);
            if let Some(expr) = init {
                println!("{}  Init:", indent(level));
                print_expression(expr, level + 2);
            }
        }
    }
}

fn print_statement(stmt: &Statement, level: usize) {
    match stmt {
        Statement::Expr(expr) => {
            println!("{}Expr:", indent(level));
            print_expression(expr, level + 1);
        }
        Statement::VarDecl { name, init, .. } => {
            println!("{}VarDecl: {}", indent(level), name);
            if let Some(expr) = init {
                println!("{}  Init:", indent(level));
                print_expression(expr, level + 2);
            }
        }
        Statement::If {
            cond,
            then_branch,
            else_branch,
            ..
        } => {
            println!("{}If:", indent(level));
            println!("{}  Condition:", indent(level));
            print_expression(cond, level + 2);
            println!("{}  Then:", indent(level));
            for stmt in &then_branch.stmts {
                print_statement(stmt, level + 2);
            }
            if let Some(else_b) = else_branch {
                println!("{}  Else:", indent(level));
                for stmt in &else_b.stmts {
                    print_statement(stmt, level + 2);
                }
            }
        }
        Statement::While { cond, body, .. } => {
            println!("{}While:", indent(level));
            println!("{}  Condition:", indent(level));
            print_expression(cond, level + 2);
            println!("{}  Body:", indent(level));
            for stmt in &body.stmts {
                print_statement(stmt, level + 2);
            }
        }
        Statement::Return { value, .. } => {
            println!("{}Return", indent(level));
            if let Some(expr) = value {
                println!("{}  Value:", indent(level));
                print_expression(expr, level + 2);
            }
        }
    }
}

fn print_expression(expr: &Expression, level: usize) {
    match expr {
        Expression::Number(n, _) => {
            println!("{}Number({})", indent(level), n);
        }
        Expression::Float(f, _) => {
            println!("{}Float({})", indent(level), f);
        }
        Expression::Bool(b, _) => {
            println!("{}Bool({})", indent(level), b);
        }
        Expression::Var(name, _) => {
            println!("{}Var({})", indent(level), name);
        }
        Expression::Unary { operator, expr, .. } => {
            println!("{}Unary({})", indent(level), operator);
            print_expression(expr, level + 1);
        }
        Expression::Binary {
            left, op, right, ..
        } => {
            println!("{}Binary({})", indent(level), op);
            println!("{}  Left:", indent(level));
            print_expression(left, level + 2);
            println!("{}  Right:", indent(level));
            print_expression(right, level + 2);
        }
        Expression::Assign { name, value, .. } => {
            println!("{}Assign: {}", indent(level), name);
            println!("{}  Value:", indent(level));
            print_expression(value, level + 2);
        }
        Expression::Call { name, args, .. } => {
            println!("{}Call: {}()", indent(level), name);
            if !args.is_empty() {
                println!("{}  Args:", indent(level));
                for arg in args {
                    print_expression(arg, level + 2);
                }
            }
        }
    }
}
