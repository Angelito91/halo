// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

pub mod similarity;
pub mod statistics;

pub use similarity::{
    find_suggestions, find_suggestions_adaptive, find_suggestions_smart, Suggestion,
};
pub use statistics::{CompilationStatistics, Timer};
