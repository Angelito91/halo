// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Consistent error codes for the entire application.
//!
//! Error codes are organized by category:
//! - E001-E010: Parser errors (syntax, structure)
//! - E011-E020: Type errors
//! - E021-E030: Runtime errors (variables, functions, recursion)
//! - E031-E040: Warnings (style, unused variables)

use std::fmt;

/// Represents a category of error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // PLANNED FOR v0.3.0: Error categorization system
pub enum ErrorCategory {
    Parse,
    Type,
    Runtime,
    Warning,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCategory::Parse => write!(f, "parser"),
            ErrorCategory::Type => write!(f, "type"),
            ErrorCategory::Runtime => write!(f, "runtime"),
            ErrorCategory::Warning => write!(f, "warning"),
        }
    }
}

/// Represents the severity level of an error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // PLANNED FOR v0.3.0: Severity levels
pub enum ErrorSeverity {
    Error,
    Warning,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Error => write!(f, "error"),
            ErrorSeverity::Warning => write!(f, "warning"),
        }
    }
}

/// Enumeration of all error codes in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)] // PLANNED FOR v0.3.0: Complete error code catalog
pub enum ErrorCode {
    // Parser errors (E001-E010)
    /// Expected ')' after parameters
    ExpectedCloseParen,
    /// Expected '}' after block
    ExpectedCloseBrace,
    /// Expected ']' after array elements
    ExpectedCloseBracket,
    /// Unexpected token in expression
    UnexpectedToken,
    /// Invalid function definition syntax
    InvalidFunctionDef,
    /// Invalid variable declaration syntax
    InvalidVariableDecl,
    /// Expected identifier
    ExpectedIdentifier,
    /// Expected expression
    ExpectedExpression,
    /// Invalid assignment
    InvalidAssignment,
    /// Unexpected end of input
    UnexpectedEOF,

    // Type errors (E011-E020)
    /// Type mismatch in operation
    TypeMismatch,
    /// Undefined type reference
    UndefinedType,
    /// Cannot convert between types
    InvalidTypeConversion,
    /// Type incompatibility in function call
    IncompatibleFunctionArgs,
    /// Return type mismatch
    ReturnTypeMismatch,
    /// Invalid operation for type
    InvalidOperationForType,
    /// Type annotation missing
    MissingTypeAnnotation,
    /// Recursive type definition
    RecursiveTypeDefinition,
    /// Generic type parameter mismatch
    GenericTypeMismatch,
    /// Type inference failure
    TypeInferenceFailed,

    // Runtime errors (E021-E030)
    /// Variable used before definition
    UndefinedVariable,
    /// Function called before definition
    UndefinedFunction,
    /// Variable declared twice
    VariableAlreadyDefined,
    /// Function declared twice
    FunctionAlreadyDefined,
    /// Infinite recursion detected
    InfiniteRecursion,
    /// Stack overflow during execution
    StackOverflow,
    /// Null pointer dereference
    NullPointerDeref,
    /// Array access out of bounds
    OutOfBoundsAccess,
    /// Invalid function call
    InvalidFunctionCall,
    /// Division by zero
    DivisionByZero,

    // Warnings (E031-E040)
    /// Variable declared but never used
    UnusedVariable,
    /// Function defined but never called
    UnusedFunction,
    /// Import statement unused
    UnusedImport,
    /// Code unreachable after return
    UnreachableCode,
    /// Variable shadows outer scope
    VariableShadowing,
    /// Function shadows outer scope
    FunctionShadowing,
    /// Use of deprecated feature
    DeprecatedFeature,
    /// Potential performance issue
    PerformanceIssue,
    /// Code style violation
    StyleViolation,
    /// Possible logic error
    PossibleLogicError,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity = if self.is_warning() {
            "warning"
        } else {
            "error"
        };
        write!(f, "{}[{}]: {}", severity, self.code(), self.description())
    }
}

mod category;
mod descriptions;
mod mapping;

