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

impl ErrorCode {
    pub fn code(&self) -> &'static str {
        match self {
            ErrorCode::ExpectedCloseParen => "E001",
            ErrorCode::ExpectedCloseBrace => "E002",
            ErrorCode::ExpectedCloseBracket => "E003",
            ErrorCode::UnexpectedToken => "E004",
            ErrorCode::InvalidFunctionDef => "E005",
            ErrorCode::InvalidVariableDecl => "E006",
            ErrorCode::ExpectedIdentifier => "E007",
            ErrorCode::ExpectedExpression => "E008",
            ErrorCode::InvalidAssignment => "E009",
            ErrorCode::UnexpectedEOF => "E010",

            ErrorCode::TypeMismatch => "E011",
            ErrorCode::UndefinedType => "E012",
            ErrorCode::InvalidTypeConversion => "E013",
            ErrorCode::IncompatibleFunctionArgs => "E014",
            ErrorCode::ReturnTypeMismatch => "E015",
            ErrorCode::InvalidOperationForType => "E016",
            ErrorCode::MissingTypeAnnotation => "E017",
            ErrorCode::RecursiveTypeDefinition => "E018",
            ErrorCode::GenericTypeMismatch => "E019",
            ErrorCode::TypeInferenceFailed => "E020",

            ErrorCode::UndefinedVariable => "E021",
            ErrorCode::UndefinedFunction => "E022",
            ErrorCode::VariableAlreadyDefined => "E023",
            ErrorCode::FunctionAlreadyDefined => "E024",
            ErrorCode::InfiniteRecursion => "E025",
            ErrorCode::StackOverflow => "E026",
            ErrorCode::NullPointerDeref => "E027",
            ErrorCode::OutOfBoundsAccess => "E028",
            ErrorCode::InvalidFunctionCall => "E029",
            ErrorCode::DivisionByZero => "E030",

            ErrorCode::UnusedVariable => "W031",
            ErrorCode::UnusedFunction => "W032",
            ErrorCode::UnusedImport => "W033",
            ErrorCode::UnreachableCode => "W034",
            ErrorCode::VariableShadowing => "W035",
            ErrorCode::FunctionShadowing => "W036",
            ErrorCode::DeprecatedFeature => "W037",
            ErrorCode::PerformanceIssue => "W038",
            ErrorCode::StyleViolation => "W039",
            ErrorCode::PossibleLogicError => "W040",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ErrorCode::ExpectedCloseParen => "Expected closing parenthesis ')'",
            ErrorCode::ExpectedCloseBrace => "Expected closing brace '}'",
            ErrorCode::ExpectedCloseBracket => "Expected closing bracket ']'",
            ErrorCode::UnexpectedToken => "Unexpected token in expression",
            ErrorCode::InvalidFunctionDef => "Invalid function definition syntax",
            ErrorCode::InvalidVariableDecl => "Invalid variable declaration",
            ErrorCode::ExpectedIdentifier => "Expected identifier",
            ErrorCode::ExpectedExpression => "Expected expression",
            ErrorCode::InvalidAssignment => "Invalid assignment",
            ErrorCode::UnexpectedEOF => "Unexpected end of file",

            ErrorCode::TypeMismatch => "Type mismatch in operation",
            ErrorCode::UndefinedType => "Undefined type",
            ErrorCode::InvalidTypeConversion => "Cannot convert between these types",
            ErrorCode::IncompatibleFunctionArgs => "Incompatible function arguments",
            ErrorCode::ReturnTypeMismatch => "Return type does not match function signature",
            ErrorCode::InvalidOperationForType => "Operation is invalid for this type",
            ErrorCode::MissingTypeAnnotation => "Type annotation required",
            ErrorCode::RecursiveTypeDefinition => "Type definition is recursive",
            ErrorCode::GenericTypeMismatch => "Generic type parameter mismatch",
            ErrorCode::TypeInferenceFailed => "Type inference failed",

            ErrorCode::UndefinedVariable => "Variable is not defined",
            ErrorCode::UndefinedFunction => "Function is not defined",
            ErrorCode::VariableAlreadyDefined => "Variable is already defined",
            ErrorCode::FunctionAlreadyDefined => "Function is already defined",
            ErrorCode::InfiniteRecursion => "Infinite recursion detected",
            ErrorCode::StackOverflow => "Stack overflow during execution",
            ErrorCode::NullPointerDeref => "Null pointer dereference",
            ErrorCode::OutOfBoundsAccess => "Array access out of bounds",
            ErrorCode::InvalidFunctionCall => "Invalid function call",
            ErrorCode::DivisionByZero => "Division by zero",

            ErrorCode::UnusedVariable => "Variable is declared but not used",
            ErrorCode::UnusedFunction => "Function is declared but not used",
            ErrorCode::UnusedImport => "Import is not used",
            ErrorCode::UnreachableCode => "Code is unreachable",
            ErrorCode::VariableShadowing => "Variable shadows outer scope",
            ErrorCode::FunctionShadowing => "Function shadows outer scope",
            ErrorCode::DeprecatedFeature => "Deprecated feature",
            ErrorCode::PerformanceIssue => "Potential performance issue",
            ErrorCode::StyleViolation => "Code style violation",
            ErrorCode::PossibleLogicError => "Possible logic error",
        }
    }

    pub fn suggestions(&self) -> Vec<&'static str> {
        match self {
            ErrorCode::ExpectedCloseParen => {
                vec!["Make sure all opening parentheses '(' are closed with ')'"]
            }
            ErrorCode::ExpectedCloseBrace => {
                vec!["Make sure all opening braces '{' are closed with '}'"]
            }
            ErrorCode::ExpectedCloseBracket => {
                vec!["Make sure all opening brackets '[' are closed with ']'"]
            }
            ErrorCode::UnexpectedToken => {
                vec![
                    "Check syntax around this token",
                    "Ensure proper operator usage",
                ]
            }
            ErrorCode::InvalidFunctionDef => {
                vec![
                    "Function definition syntax: fn name(params) -> ReturnType { body }",
                    "Check parameter types and return type annotation",
                ]
            }
            ErrorCode::InvalidVariableDecl => {
                vec![
                    "Variable declaration syntax: let name: Type = value;",
                    "Provide proper type annotation",
                ]
            }
            ErrorCode::ExpectedIdentifier => {
                vec!["An identifier is required at this location"]
            }
            ErrorCode::ExpectedExpression => {
                vec!["An expression is required at this location"]
            }
            ErrorCode::InvalidAssignment => {
                vec!["Assignment target must be a variable"]
            }
            ErrorCode::UnexpectedEOF => {
                vec![
                    "File ended unexpectedly",
                    "Check for unclosed blocks or expressions",
                ]
            }

            ErrorCode::TypeMismatch => {
                vec![
                    "Ensure types on both sides of the operation match",
                    "Consider type conversion if appropriate",
                ]
            }
            ErrorCode::UndefinedType => {
                vec![
                    "Type must be defined before use",
                    "Check spelling and imports",
                ]
            }
            ErrorCode::InvalidTypeConversion => {
                vec![
                    "These types cannot be converted",
                    "Use an explicit cast if supported",
                ]
            }
            ErrorCode::IncompatibleFunctionArgs => {
                vec!["Check argument types match function parameters"]
            }
            ErrorCode::ReturnTypeMismatch => {
                vec!["Returned value does not match declared return type"]
            }
            ErrorCode::InvalidOperationForType => {
                vec!["This operation is not supported for this type"]
            }
            ErrorCode::MissingTypeAnnotation => {
                vec!["Provide explicit type annotation"]
            }
            ErrorCode::RecursiveTypeDefinition => {
                vec![
                    "Type definition contains a cycle",
                    "Use generics or references to break cycle",
                ]
            }
            ErrorCode::GenericTypeMismatch => {
                vec!["Generic type parameters do not match"]
            }
            ErrorCode::TypeInferenceFailed => {
                vec!["Provide explicit type annotation to help type inference"]
            }

            ErrorCode::UndefinedVariable => {
                vec!["Variable must be defined before use", "Check spelling"]
            }
            ErrorCode::UndefinedFunction => {
                vec![
                    "Function must be defined before use",
                    "Check spelling and imports",
                ]
            }
            ErrorCode::VariableAlreadyDefined => {
                vec!["Use a different variable name or remove the duplicate declaration"]
            }
            ErrorCode::FunctionAlreadyDefined => {
                vec!["Use a different function name or remove the duplicate definition"]
            }
            ErrorCode::InfiniteRecursion => {
                vec!["Ensure recursive calls have a base case"]
            }
            ErrorCode::StackOverflow => {
                vec!["Reduce recursion depth or increase stack size"]
            }
            ErrorCode::NullPointerDeref => {
                vec!["Check that pointer is not null before dereferencing"]
            }
            ErrorCode::OutOfBoundsAccess => {
                vec!["Ensure array index is within valid bounds"]
            }
            ErrorCode::InvalidFunctionCall => {
                vec!["Check function name, arguments, and argument types"]
            }
            ErrorCode::DivisionByZero => {
                vec!["Ensure divisor is not zero"]
            }

            ErrorCode::UnusedVariable => {
                vec!["Remove unused variable or use it in code"]
            }
            ErrorCode::UnusedFunction => {
                vec!["Remove unused function or call it from somewhere"]
            }
            ErrorCode::UnusedImport => {
                vec!["Remove unused import statement"]
            }
            ErrorCode::UnreachableCode => {
                vec!["Remove unreachable code or adjust control flow"]
            }
            ErrorCode::VariableShadowing => {
                vec!["Consider using different variable names to avoid shadowing"]
            }
            ErrorCode::FunctionShadowing => {
                vec!["Consider using different function names to avoid shadowing"]
            }
            ErrorCode::DeprecatedFeature => {
                vec!["Use the recommended alternative"]
            }
            ErrorCode::PerformanceIssue => {
                vec!["Consider optimizing this code"]
            }
            ErrorCode::StyleViolation => {
                vec!["Follow the project's code style guidelines"]
            }
            ErrorCode::PossibleLogicError => {
                vec!["Review logic to ensure correctness"]
            }
        }
    }

    pub fn category(&self) -> ErrorCategory {
        match self {
            ErrorCode::ExpectedCloseParen
            | ErrorCode::ExpectedCloseBrace
            | ErrorCode::ExpectedCloseBracket
            | ErrorCode::UnexpectedToken
            | ErrorCode::InvalidFunctionDef
            | ErrorCode::InvalidVariableDecl
            | ErrorCode::ExpectedIdentifier
            | ErrorCode::ExpectedExpression
            | ErrorCode::InvalidAssignment
            | ErrorCode::UnexpectedEOF => ErrorCategory::Parse,

            ErrorCode::TypeMismatch
            | ErrorCode::UndefinedType
            | ErrorCode::InvalidTypeConversion
            | ErrorCode::IncompatibleFunctionArgs
            | ErrorCode::ReturnTypeMismatch
            | ErrorCode::InvalidOperationForType
            | ErrorCode::MissingTypeAnnotation
            | ErrorCode::RecursiveTypeDefinition
            | ErrorCode::GenericTypeMismatch
            | ErrorCode::TypeInferenceFailed => ErrorCategory::Type,

            ErrorCode::UndefinedVariable
            | ErrorCode::UndefinedFunction
            | ErrorCode::VariableAlreadyDefined
            | ErrorCode::FunctionAlreadyDefined
            | ErrorCode::InfiniteRecursion
            | ErrorCode::StackOverflow
            | ErrorCode::NullPointerDeref
            | ErrorCode::OutOfBoundsAccess
            | ErrorCode::InvalidFunctionCall
            | ErrorCode::DivisionByZero => ErrorCategory::Runtime,

            ErrorCode::UnusedVariable
            | ErrorCode::UnusedFunction
            | ErrorCode::UnusedImport
            | ErrorCode::UnreachableCode
            | ErrorCode::VariableShadowing
            | ErrorCode::FunctionShadowing
            | ErrorCode::DeprecatedFeature
            | ErrorCode::PerformanceIssue
            | ErrorCode::StyleViolation
            | ErrorCode::PossibleLogicError => ErrorCategory::Warning,
        }
    }

    pub fn severity(&self) -> ErrorSeverity {
        match self.category() {
            ErrorCategory::Warning => ErrorSeverity::Warning,
            _ => ErrorSeverity::Error,
        }
    }

    pub fn is_warning(&self) -> bool {
        self.severity() == ErrorSeverity::Warning
    }

    pub fn is_error(&self) -> bool {
        self.severity() == ErrorSeverity::Error
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code(), self.description())
    }
}
