// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

pub mod builtins;
pub mod core;
pub mod methods;

// Re-export the main Evaluator type for convenience
pub use core::Evaluator;
