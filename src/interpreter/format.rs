//! Canonical value-formatting rules. The interpreter's output is the
//! reference; P1 makes compiled binaries match it.
//!
//! Every rule here is pinned against the interpreter by the differential
//! tests at the bottom of this file, so this module cannot drift away from
//! the behaviour it describes.

/// How the interpreter prints a boolean.
pub fn format_bool(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}

/// How the interpreter prints an integer.
pub fn format_number(n: i64) -> String {
    n.to_string()
}

/// How the interpreter prints a float: `{:.1}` when the value is integral
/// (`5.0` → `5.0`), plain `Display` otherwise (`0.25` → `0.25`).
///
/// This mirrors `Value::Float(f).to_string_value()` exactly, including the
/// large and non-finite cases: `1e300` renders in full decimal expansion and
/// `NaN`/`inf` keep their plain spelling. The differential test below pins
/// the two together.
pub fn format_float(f: f64) -> String {
    if f.fract() == 0.0 {
        format!("{:.1}", f)
    } else {
        f.to_string()
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Value;

    #[test]
    fn format_float_matches_the_interpreter() {
        for f in [
            5.0,
            0.25,
            -std::f64::consts::PI,
            1e300,
            f64::INFINITY,
            f64::NAN,
        ] {
            assert_eq!(
                format_float(f),
                Value::Float(f).to_string_value(),
                "float formatting diverged for {f}"
            );
        }
    }

    #[test]
    fn format_number_matches_the_interpreter() {
        for n in [0i64, 42, -7, i64::MAX, i64::MIN] {
            assert_eq!(
                format_number(n),
                Value::Number(n).to_string_value(),
                "number formatting diverged for {n}"
            );
        }
    }

    #[test]
    fn format_bool_matches_the_interpreter() {
        assert_eq!(format_bool(true), Value::Bool(true).to_string_value());
        assert_eq!(format_bool(false), Value::Bool(false).to_string_value());
    }

    #[test]
    fn format_null_matches_the_interpreter() {
        assert_eq!(format_null(), Value::Null.to_string_value());
    }

    #[test]
    fn type_names_match_the_interpreter() {
        let expected = [
            ("number", Value::Number(1).type_name()),
            ("float", Value::Float(1.0).type_name()),
            ("bool", Value::Bool(true).type_name()),
            ("string", Value::String("s".into()).type_name()),
            ("array", Value::Array(vec![]).type_name()),
            ("null", Value::Null.type_name()),
        ];
        for (name, observed) in expected {
            assert!(
                TYPE_NAMES.contains(&(name, observed)),
                "type name mismatch for {name}: interpreter says {observed}"
            );
        }
    }
}
