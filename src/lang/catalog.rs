//! Builtin and method catalog. `support` mirrors the measured behavior of
//! 2026-10-17 (spec §1.1); flipping a flag is a behavior change and belongs
//! to the parity phases.

use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Support {
    Both,
    InterpreterOnly,
    CompilerOnly,
}

#[derive(Debug, Clone)]
pub struct Builtin {
    pub name: &'static str,
    pub arity: RangeInclusive<usize>,
    pub support: Support,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: &'static str,
    pub arity: RangeInclusive<usize>,
    pub support: Support,
}

use Support::{Both, CompilerOnly, InterpreterOnly};

pub const BUILTINS: &[Builtin] = &[
    Builtin { name: "print",       arity: 0..=usize::MAX, support: Both },
    Builtin { name: "println",     arity: 0..=usize::MAX, support: CompilerOnly },
    Builtin { name: "len",         arity: 1..=1, support: Both },
    Builtin { name: "str",         arity: 1..=1, support: Both },
    Builtin { name: "int",         arity: 1..=1, support: Both },
    Builtin { name: "float",       arity: 1..=1, support: Both },
    Builtin { name: "abs",         arity: 1..=1, support: Both },
    Builtin { name: "type",        arity: 1..=1, support: Both },
    Builtin { name: "range",       arity: 1..=2, support: InterpreterOnly },
    Builtin { name: "push",        arity: 2..=2, support: InterpreterOnly },
    Builtin { name: "pop",         arity: 1..=1, support: InterpreterOnly },
    Builtin { name: "append",      arity: 2..=2, support: InterpreterOnly },
    Builtin { name: "first",       arity: 1..=1, support: InterpreterOnly },
    Builtin { name: "last",        arity: 1..=1, support: InterpreterOnly },
    Builtin { name: "reverse",     arity: 1..=1, support: Both },
    Builtin { name: "join",        arity: 2..=2, support: InterpreterOnly },
    Builtin { name: "sort",        arity: 1..=1, support: InterpreterOnly },
    Builtin { name: "contains",    arity: 2..=2, support: Both },
    Builtin { name: "index_of",    arity: 2..=2, support: Both },
    Builtin { name: "upper",       arity: 1..=1, support: Both },
    Builtin { name: "lower",       arity: 1..=1, support: Both },
    Builtin { name: "trim",        arity: 1..=1, support: Both },
    Builtin { name: "starts_with", arity: 2..=2, support: Both },
    Builtin { name: "ends_with",   arity: 2..=2, support: Both },
    Builtin { name: "replace",     arity: 3..=3, support: Both },
    Builtin { name: "substr",      arity: 3..=3, support: Both },
];

pub const METHODS: &[Method] = &[
    Method { name: "push",        arity: 1..=1, support: InterpreterOnly },
    Method { name: "pop",         arity: 0..=0, support: InterpreterOnly },
    Method { name: "first",       arity: 0..=0, support: InterpreterOnly },
    Method { name: "last",        arity: 0..=0, support: InterpreterOnly },
    Method { name: "join",        arity: 1..=1, support: InterpreterOnly },
    Method { name: "sort",        arity: 0..=0, support: InterpreterOnly },
    Method { name: "reverse",     arity: 0..=0, support: Both },
    Method { name: "contains",    arity: 1..=1, support: Both },
    Method { name: "index_of",    arity: 1..=1, support: Both },
    Method { name: "upper",       arity: 0..=0, support: Both },
    Method { name: "lower",       arity: 0..=0, support: Both },
    Method { name: "trim",        arity: 0..=0, support: Both },
    Method { name: "starts_with", arity: 1..=1, support: Both },
    Method { name: "ends_with",   arity: 1..=1, support: Both },
    Method { name: "replace",     arity: 2..=2, support: Both },
    Method { name: "substr",      arity: 2..=2, support: Both },
];

pub fn builtin(name: &str) -> Option<&'static Builtin> {
    BUILTINS.iter().find(|b| b.name == name)
}

pub fn method(name: &str) -> Option<&'static Method> {
    METHODS.iter().find(|m| m.name == name)
}
