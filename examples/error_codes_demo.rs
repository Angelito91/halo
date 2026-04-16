// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Example demonstrating the error code system in Halo
//!
//! This example shows how to use the error code system for reporting
//! consistent errors throughout the compiler.

use halo::diagnostics::error_codes::{ErrorCategory, ErrorCode, ErrorSeverity};

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        Halo Language - Error Codes System Demo            ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Demonstrate parser errors
    println!("📍 PARSER ERRORS (E001-E010)");
    println!("{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n", "─".repeat(20));
    demonstrate_error(ErrorCode::ExpectedCloseParen);
    demonstrate_error(ErrorCode::UnexpectedEOF);

    println!("\n");

    // Demonstrate type errors
    println!("🔤 TYPE ERRORS (E011-E020)");
    println!("{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n", "─".repeat(20));
    demonstrate_error(ErrorCode::TypeMismatch);
    demonstrate_error(ErrorCode::UndefinedType);

    println!("\n");

    // Demonstrate runtime errors
    println!("⚙️  RUNTIME ERRORS (E021-E030)");
    println!("{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n", "─".repeat(20));
    demonstrate_error(ErrorCode::UndefinedVariable);
    demonstrate_error(ErrorCode::DivisionByZero);

    println!("\n");

    // Demonstrate warnings
    println!("⚠️  WARNINGS (E031-E040)");
    println!("{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n", "─".repeat(20));
    demonstrate_error(ErrorCode::UnusedVariable);
    demonstrate_error(ErrorCode::UnreachableCode);

    println!("\n");

    // Statistics
    println!("📊 ERROR CODE STATISTICS");
    println!("{}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n", "─".repeat(20));
    print_statistics();
}

/// Demonstrate a single error code with all its information
fn demonstrate_error(code: ErrorCode) {
    println!("{}", code);
    println!("  Code:        {}", code.code());
    println!("  Category:    {}", code.category());
    println!("  Severity:    {}", code.severity());
    println!("  Description: {}", code.description());

    println!("  Suggestions:");
    for (i, suggestion) in code.suggestions().iter().enumerate() {
        println!("    {}. {}", i + 1, suggestion);
    }
    println!();
}

/// Print statistics about error codes
fn print_statistics() {
    let all_errors = vec![
        // Parser errors
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
        // Type errors
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
        // Runtime errors
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
        // Warnings
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

    let mut parse_count = 0;
    let mut type_count = 0;
    let mut runtime_count = 0;
    let mut warning_count = 0;
    let mut error_count = 0;

    for code in &all_errors {
        match code.category() {
            ErrorCategory::Parse => parse_count += 1,
            ErrorCategory::Type => type_count += 1,
            ErrorCategory::Runtime => runtime_count += 1,
            ErrorCategory::Warning => warning_count += 1,
        }

        match code.severity() {
            ErrorSeverity::Error => error_count += 1,
            ErrorSeverity::Warning => {}
        }
    }

    println!("Total Error Codes:     {}", all_errors.len());
    println!("  Parse Errors:        {}", parse_count);
    println!("  Type Errors:         {}", type_count);
    println!("  Runtime Errors:      {}", runtime_count);
    println!("  Warnings:            {}", warning_count);
    println!();
    println!("Total Errors:          {}", error_count);
    println!("Total Warnings:        {}", warning_count);

    println!("\n✨ Example formatted output:\n");
    println!("  {}", ErrorCode::ExpectedCloseParen);
    println!("  {}", ErrorCode::TypeMismatch);
    println!("  {}", ErrorCode::UnusedVariable);
}
