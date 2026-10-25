// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

pub mod cli;
pub mod diagnostics;
pub mod interpreter;
pub mod lexer;
pub mod parser;

pub use diagnostics::colors;
pub use diagnostics::suggestions::{find_suggestions, find_suggestions_smart, Suggestion};
pub use interpreter::Evaluator;
pub use lexer::Lexer;
pub use parser::Parser;
