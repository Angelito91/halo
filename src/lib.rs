// The Halo Programming Language
// Library root
// Version: 0.2.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

pub mod compiler;
pub mod interpreter;
pub mod lexer;
pub mod parser;

pub use compiler::CodeGenerator;
pub use interpreter::Evaluator;
pub use lexer::Lexer;
pub use parser::Parser;
