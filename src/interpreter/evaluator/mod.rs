// Module organization for the evaluator

pub mod builtins;
pub mod core;
pub mod methods;

// Re-export the main Evaluator type for convenience
pub use core::Evaluator;
