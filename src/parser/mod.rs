// The Halo Programming Language
// Parser module
// Version: 0.2.0
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

pub mod ast;
#[allow(clippy::module_inception)]
pub mod parser;
pub mod visitor;

pub use parser::*;
