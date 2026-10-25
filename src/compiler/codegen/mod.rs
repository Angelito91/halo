// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

pub mod builtins;
pub mod emitter;
pub mod expressions;
mod functions;
pub mod generator;
mod module;
pub mod statements;

pub use generator::CodeGenerator;
