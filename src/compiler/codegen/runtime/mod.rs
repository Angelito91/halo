// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Runtime support functions.
//!
//! This module provides low-level runtime support for compiled code:
//! - Memory management: `malloc()`, `free()`, custom allocation strategies
//! - Array runtime support: array construction, bounds checking (future)
//! - String runtime support: string concatenation, formatting
//! - Helper functions: type conversions, error handling
//!
//! These functions are emitted into the generated LLVM module for use by compiled code.
