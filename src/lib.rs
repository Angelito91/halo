// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
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
