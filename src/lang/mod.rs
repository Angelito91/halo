//! Single source of truth for language-level metadata shared by the
//! interpreter and the compiler: builtin/method catalog, value formatting
//! rules and runtime error messages.
//!
//! In R0 this module is *descriptive*: it records how each backend behaves
//! today. P1/P2 make the backends consume it, which is when their observable
//! behavior is allowed to change.

pub mod catalog;
pub mod errors;
pub mod format;
