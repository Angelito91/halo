//! Canonical value-formatting rules. The interpreter's output is the
//! reference; P1 makes compiled binaries match it.

/// How the interpreter prints a boolean.
pub fn format_bool(b: bool) -> &'static str {
    if b { "true" } else { "false" }
}

/// How the interpreter prints an integer.
pub fn format_number(n: i64) -> String {
    n.to_string()
}

/// How the interpreter prints a float: minimal representation that always
/// keeps a decimal part (`5.0`, `0.25`).
pub fn format_float(f: f64) -> String {
    let s = format!("{f}");
    if s.contains('.') || s.contains('e') { s } else { format!("{s}.0") }
}

/// How the interpreter prints the null value.
pub fn format_null() -> &'static str {
    "null"
}

/// `type()` names by Halo type, as the interpreter reports them.
pub const TYPE_NAMES: &[(&str, &str)] = &[
    ("number", "number"),
    ("float", "float"),
    ("bool", "bool"),
    ("string", "string"),
    ("array", "array"),
    ("null", "null"),
];
