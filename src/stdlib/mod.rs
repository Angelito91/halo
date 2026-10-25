// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0
//
// Built-in functions. This module knows `Value` and nothing about the AST or
// the evaluator: the evaluator evaluates every argument first and passes the
// resulting values in. All built-ins are pure: they return a new value and
// never mutate their arguments.

pub mod arrays;
pub mod builtins;
pub mod strings;

pub use arrays::*;
pub use builtins::{call_builtin, is_builtin, BUILTIN_NAMES};
pub use strings::*;
