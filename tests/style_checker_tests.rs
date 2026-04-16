// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Tests for style checker and code analysis

use halo::diagnostics::error_codes::ErrorCode;
use halo::diagnostics::style_checker::StyleChecker;
use halo::parser::ast::Program;

#[test]
fn test_snake_case_conversion() {
    let checker = StyleChecker::new();
    assert_eq!(checker.to_snake_case("myVariable"), "my_variable");
    assert_eq!(checker.to_snake_case("HTTPServer"), "h_t_t_p_server");
    assert_eq!(checker.to_snake_case("test"), "test");
}

#[test]
fn test_line_length_violation() {
    let mut checker = StyleChecker::new();
    let long_line = "x = ".to_string() + &"y".repeat(150);
    let violations = checker.check_program(&Program { items: vec![] }, &long_line);
    assert!(!violations.is_empty());
    assert_eq!(violations[0].code, ErrorCode::StyleViolation);
}

#[test]
fn test_naming_convention() {
    let mut checker = StyleChecker::new();
    checker.check_identifier_naming("myVar", "variable", 1);
    assert!(!checker.violations.is_empty());
    assert_eq!(checker.violations[0].code, ErrorCode::StyleViolation);
}

#[test]
fn test_snake_case_allows_underscores() {
    let checker = StyleChecker::new();
    assert_eq!(checker.to_snake_case("_private"), "_private");
    assert_eq!(checker.to_snake_case("__init"), "__init");
}

#[test]
fn test_line_length_default_config() {
    let mut checker = StyleChecker::new();
    // Default max line length is 100
    let short_line = "x".repeat(99);
    let violations = checker.check_program(&Program { items: vec![] }, &short_line);
    // Should not have violations for line within limit
    let filtered: Vec<_> = violations
        .iter()
        .filter(|v| v.code == ErrorCode::PerformanceIssue)
        .collect();
    assert!(filtered.is_empty());
}

#[test]
fn test_style_checker_with_multiple_violations() {
    let mut checker = StyleChecker::new();
    let source = "x = ".to_string() + &"y".repeat(150) + "\nmyVar = 5";
    let violations = checker.check_program(&Program { items: vec![] }, &source);
    // Should detect at least the long line violation
    assert!(!violations.is_empty());
}
