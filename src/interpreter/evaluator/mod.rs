// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

mod calls;
pub mod core;
mod expressions;
mod statements;

// Re-export the main Evaluator type for convenience
pub use core::Evaluator;
