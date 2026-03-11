// The Halo Programming Language
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

#[allow(clippy::module_inception)]
pub mod lexer;
pub mod token;

pub use lexer::Lexer;
pub use token::{Token, TokenKind};
