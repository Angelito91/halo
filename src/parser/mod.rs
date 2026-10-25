// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

pub mod ast;
mod core;
mod declarations;
mod expressions;
mod statements;

pub use core::{ParseError, Parser};
