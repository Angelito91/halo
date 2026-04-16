// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Tests for error code system and diagnostics

use halo::diagnostics::error_codes::{ErrorCategory, ErrorCode, ErrorSeverity};

#[test]
fn test_error_code_display() {
    let code = ErrorCode::ExpectedCloseParen;
    let output = format!("{}", code);
    assert_eq!(output, "error[E001]: Expected ')' after parameters");
}

#[test]
fn test_type_mismatch_display() {
    let code = ErrorCode::TypeMismatch;
    let output = format!("{}", code);
    assert_eq!(output, "error[E011]: Type mismatch in operation");
}

#[test]
fn test_warning_display() {
    let code = ErrorCode::UnusedVariable;
    let output = format!("{}", code);
    assert_eq!(output, "warning[E031]: Variable is not used");
}

#[test]
fn test_all_codes_unique() {
    let codes = vec![
        ErrorCode::ExpectedCloseParen,
        ErrorCode::ExpectedCloseBrace,
        ErrorCode::ExpectedCloseBracket,
        ErrorCode::UnexpectedToken,
        ErrorCode::InvalidFunctionDef,
        ErrorCode::InvalidVariableDecl,
        ErrorCode::ExpectedIdentifier,
        ErrorCode::ExpectedExpression,
        ErrorCode::InvalidAssignment,
        ErrorCode::UnexpectedEOF,
        ErrorCode::TypeMismatch,
        ErrorCode::UndefinedType,
        ErrorCode::InvalidTypeConversion,
        ErrorCode::IncompatibleFunctionArgs,
        ErrorCode::ReturnTypeMismatch,
        ErrorCode::InvalidOperationForType,
        ErrorCode::MissingTypeAnnotation,
        ErrorCode::RecursiveTypeDefinition,
        ErrorCode::GenericTypeMismatch,
        ErrorCode::TypeInferenceFailed,
        ErrorCode::UndefinedVariable,
        ErrorCode::UndefinedFunction,
        ErrorCode::VariableAlreadyDefined,
        ErrorCode::FunctionAlreadyDefined,
        ErrorCode::InfiniteRecursion,
        ErrorCode::StackOverflow,
        ErrorCode::NullPointerDeref,
        ErrorCode::OutOfBoundsAccess,
        ErrorCode::InvalidFunctionCall,
        ErrorCode::DivisionByZero,
        ErrorCode::UnusedVariable,
        ErrorCode::UnusedFunction,
        ErrorCode::UnusedImport,
        ErrorCode::UnreachableCode,
        ErrorCode::VariableShadowing,
        ErrorCode::FunctionShadowing,
        ErrorCode::DeprecatedFeature,
        ErrorCode::PerformanceIssue,
        ErrorCode::StyleViolation,
        ErrorCode::PossibleLogicError,
    ];

    let mut code_strings: Vec<_> = codes.iter().map(|c| c.code()).collect();
    let original_len = code_strings.len();
    code_strings.sort();
    code_strings.dedup();

    assert_eq!(
        code_strings.len(),
        original_len,
        "All error codes must be unique"
    );
}

#[test]
fn test_parser_errors_category() {
    assert_eq!(
        ErrorCode::ExpectedCloseParen.category(),
        ErrorCategory::Parse
    );
    assert_eq!(ErrorCode::UnexpectedEOF.category(), ErrorCategory::Parse);
}

#[test]
fn test_type_errors_category() {
    assert_eq!(ErrorCode::TypeMismatch.category(), ErrorCategory::Type);
    assert_eq!(ErrorCode::UndefinedType.category(), ErrorCategory::Type);
}

#[test]
fn test_runtime_errors_category() {
    assert_eq!(
        ErrorCode::UndefinedVariable.category(),
        ErrorCategory::Runtime
    );
    assert_eq!(ErrorCode::DivisionByZero.category(), ErrorCategory::Runtime);
}

#[test]
fn test_warning_errors_category() {
    assert_eq!(ErrorCode::UnusedVariable.category(), ErrorCategory::Warning);
    assert_eq!(
        ErrorCode::PossibleLogicError.category(),
        ErrorCategory::Warning
    );
}

#[test]
fn test_error_severity() {
    assert!(ErrorCode::ExpectedCloseParen.is_error());
    assert!(ErrorCode::TypeMismatch.is_error());
    assert!(ErrorCode::DivisionByZero.is_error());
    assert!(!ErrorCode::UnusedVariable.is_error());

    assert!(ErrorCode::UnusedVariable.is_warning());
    assert!(!ErrorCode::ExpectedCloseParen.is_warning());
}

#[test]
fn test_suggestions_not_empty() {
    let code = ErrorCode::ExpectedCloseParen;
    let suggestions = code.suggestions();
    assert!(!suggestions.is_empty());
}

#[test]
fn test_all_errors_have_suggestions() {
    let codes = vec![
        ErrorCode::ExpectedCloseParen,
        ErrorCode::ExpectedCloseBrace,
        ErrorCode::TypeMismatch,
        ErrorCode::UnusedVariable,
    ];

    for code in codes {
        let suggestions = code.suggestions();
        assert!(
            !suggestions.is_empty(),
            "Error code {} should have suggestions",
            code.code()
        );
    }
}

#[test]
fn test_code_ranges() {
    // Parser errors: E001-E010
    assert!(ErrorCode::ExpectedCloseParen.code().starts_with("E00"));
    assert_eq!(ErrorCode::UnexpectedEOF.code(), "E010");

    // Type errors: E011-E020
    assert_eq!(ErrorCode::TypeMismatch.code(), "E011");
    assert_eq!(ErrorCode::TypeInferenceFailed.code(), "E020");

    // Runtime errors: E021-E030
    assert_eq!(ErrorCode::UndefinedVariable.code(), "E021");
    assert_eq!(ErrorCode::DivisionByZero.code(), "E030");

    // Warnings: E031-E040
    assert_eq!(ErrorCode::UnusedVariable.code(), "E031");
    assert_eq!(ErrorCode::PossibleLogicError.code(), "E040");
}
