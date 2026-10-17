// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

pub mod ast;
mod core;
mod declarations;
mod expressions;
mod statements;
pub mod visitor;

pub use core::Parser;
