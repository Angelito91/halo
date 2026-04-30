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
#[allow(dead_code)]
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
    /// Undefined variable
    UndefinedVariable,
    /// Undefined function
    UndefinedFunction,
    /// Variable already defined
    VariableAlreadyDefined,
    /// Function already defined
    FunctionAlreadyDefined,
    /// Infinite recursion detected
    InfiniteRecursion,
    /// Stack overflow
    StackOverflow,
    /// Null pointer dereference
    NullPointerDeref,
    /// Out of bounds access
    OutOfBoundsAccess,
    /// Invalid function call
    InvalidFunctionCall,
    /// Division by zero
    DivisionByZero,

    // Warnings (E031-E040)
    /// Unused variable
    UnusedVariable,
    /// Unused function
    UnusedFunction,
    /// Unused import
    UnusedImport,
    /// Unreachable code
    UnreachableCode,
    /// Variable shadowing
    VariableShadowing,
    /// Function shadowing
    FunctionShadowing,
    /// Deprecated feature
    DeprecatedFeature,
    /// Performance issue
    PerformanceIssue,
    /// Style guide violation
    StyleViolation,
    /// Possible logic error
    PossibleLogicError,
}

impl ErrorCode {
    /// Get the numeric code as a string (e.g., "E001")
    #[allow(dead_code)]
    pub fn code(&self) -> &'static str {
        match self {
            // Parser errors
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

            // Type errors
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

            // Runtime errors
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

            // Warnings
            ErrorCode::UnusedVariable => "E031",
            ErrorCode::UnusedFunction => "E032",
            ErrorCode::UnusedImport => "E033",
            ErrorCode::UnreachableCode => "E034",
            ErrorCode::VariableShadowing => "E035",
            ErrorCode::FunctionShadowing => "E036",
            ErrorCode::DeprecatedFeature => "E037",
            ErrorCode::PerformanceIssue => "E038",
            ErrorCode::StyleViolation => "E039",
            ErrorCode::PossibleLogicError => "E040",
        }
    }

    /// Get the short description of the error
    #[allow(dead_code)]
    pub fn description(&self) -> &'static str {
        match self {
            // Parser errors
            ErrorCode::ExpectedCloseParen => "Expected ')' after parameters",
            ErrorCode::ExpectedCloseBrace => "Expected '}' after block",
            ErrorCode::ExpectedCloseBracket => "Expected ']' after array elements",
            ErrorCode::UnexpectedToken => "Unexpected token in expression",
            ErrorCode::InvalidFunctionDef => "Invalid function definition syntax",
            ErrorCode::InvalidVariableDecl => "Invalid variable declaration syntax",
            ErrorCode::ExpectedIdentifier => "Expected identifier",
            ErrorCode::ExpectedExpression => "Expected expression",
            ErrorCode::InvalidAssignment => "Invalid assignment",
            ErrorCode::UnexpectedEOF => "Unexpected end of input",

            // Type errors
            ErrorCode::TypeMismatch => "Type mismatch in operation",
            ErrorCode::UndefinedType => "Undefined type reference",
            ErrorCode::InvalidTypeConversion => "Cannot convert between types",
            ErrorCode::IncompatibleFunctionArgs => "Incompatible function arguments",
            ErrorCode::ReturnTypeMismatch => "Return type mismatch",
            ErrorCode::InvalidOperationForType => "Invalid operation for this type",
            ErrorCode::MissingTypeAnnotation => "Type annotation is required",
            ErrorCode::RecursiveTypeDefinition => "Recursive type definition",
            ErrorCode::GenericTypeMismatch => "Generic type parameter mismatch",
            ErrorCode::TypeInferenceFailed => "Type inference failed",

            // Runtime errors
            ErrorCode::UndefinedVariable => "Variable is not defined",
            ErrorCode::UndefinedFunction => "Function is not defined",
            ErrorCode::VariableAlreadyDefined => "Variable is already defined",
            ErrorCode::FunctionAlreadyDefined => "Function is already defined",
            ErrorCode::InfiniteRecursion => "Infinite recursion detected",
            ErrorCode::StackOverflow => "Stack overflow",
            ErrorCode::NullPointerDeref => "Attempted null pointer dereference",
            ErrorCode::OutOfBoundsAccess => "Array access out of bounds",
            ErrorCode::InvalidFunctionCall => "Invalid function call",
            ErrorCode::DivisionByZero => "Division by zero",

            // Warnings
            ErrorCode::UnusedVariable => "Variable is not used",
            ErrorCode::UnusedFunction => "Function is not used",
            ErrorCode::UnusedImport => "Import is not used",
            ErrorCode::UnreachableCode => "Unreachable code detected",
            ErrorCode::VariableShadowing => "Variable shadows previous definition",
            ErrorCode::FunctionShadowing => "Function shadows previous definition",
            ErrorCode::DeprecatedFeature => "Deprecated feature used",
            ErrorCode::PerformanceIssue => "Potential performance issue",
            ErrorCode::StyleViolation => "Style guide violation",
            ErrorCode::PossibleLogicError => "Possible logic error",
        }
    }

    /// Get helpful suggestions for fixing the error
    #[allow(dead_code)]
    pub fn suggestions(&self) -> Vec<&'static str> {
        match self {
            // Parser errors
            ErrorCode::ExpectedCloseParen => vec![
                "Check that all opening parentheses have matching closing parentheses",
                "Verify function parameter lists are properly closed",
            ],
            ErrorCode::ExpectedCloseBrace => vec![
                "Check that all opening braces have matching closing braces",
                "Verify block structure is correct",
            ],
            ErrorCode::ExpectedCloseBracket => vec![
                "Check that all opening brackets have matching closing brackets",
                "Verify array syntax is correct",
            ],
            ErrorCode::UnexpectedToken => vec![
                "Review the context where this token appears",
                "Check for syntax errors near this position",
            ],
            ErrorCode::InvalidFunctionDef => vec![
                "Use the correct function definition syntax: fn name(params) -> Type { body }",
                "Ensure function name is a valid identifier",
            ],
            ErrorCode::InvalidVariableDecl => vec![
                "Use the correct variable declaration syntax: let name: Type = value;",
                "Ensure variable name is a valid identifier",
            ],
            ErrorCode::ExpectedIdentifier => vec![
                "An identifier must start with a letter or underscore",
                "Identifiers can contain letters, numbers, and underscores",
            ],
            ErrorCode::ExpectedExpression => vec![
                "An expression was expected at this location",
                "Check for missing operators or operands",
            ],
            ErrorCode::InvalidAssignment => vec![
                "Assignment target must be a valid lvalue (variable)",
                "Check the left side of the assignment operator",
            ],
            ErrorCode::UnexpectedEOF => vec![
                "The input ended unexpectedly",
                "Check for unclosed brackets, braces, or parentheses",
            ],

            // Type errors
            ErrorCode::TypeMismatch => vec![
                "Ensure both operands have compatible types",
                "Use explicit type conversion if needed",
            ],
            ErrorCode::UndefinedType => vec![
                "Check that the type name is spelled correctly",
                "Ensure the type is imported or defined",
            ],
            ErrorCode::InvalidTypeConversion => vec![
                "These types cannot be converted automatically",
                "Use explicit conversion or a different approach",
            ],
            ErrorCode::IncompatibleFunctionArgs => vec![
                "Check that function arguments match the parameter types",
                "Verify the number of arguments is correct",
            ],
            ErrorCode::ReturnTypeMismatch => vec![
                "The return type does not match the function signature",
                "Check all return statements in the function",
            ],
            ErrorCode::InvalidOperationForType => vec![
                "This operation is not supported for this type",
                "Choose a different operation or convert the type",
            ],
            ErrorCode::MissingTypeAnnotation => vec![
                "Explicitly specify the type for this value",
                "The type cannot be inferred from context",
            ],
            ErrorCode::RecursiveTypeDefinition => vec![
                "A type cannot contain itself directly",
                "Use indirection (references) if you need recursive structures",
            ],
            ErrorCode::GenericTypeMismatch => vec![
                "Generic type parameters must be compatible",
                "Check the type arguments passed to the generic",
            ],
            ErrorCode::TypeInferenceFailed => vec![
                "Provide an explicit type annotation",
                "Ensure there is enough context for type inference",
            ],

            // Runtime errors
            ErrorCode::UndefinedVariable => vec![
                "Check that the variable is defined before use",
                "Verify the variable name is spelled correctly",
            ],
            ErrorCode::UndefinedFunction => vec![
                "Check that the function is defined before use",
                "Verify the function name is spelled correctly",
            ],
            ErrorCode::VariableAlreadyDefined => vec![
                "Use a different variable name",
                "Or remove the duplicate definition",
            ],
            ErrorCode::FunctionAlreadyDefined => vec![
                "Use a different function name",
                "Or remove the duplicate definition",
            ],
            ErrorCode::InfiniteRecursion => vec![
                "Ensure the recursive function has a base case",
                "Verify the recursion terminates",
            ],
            ErrorCode::StackOverflow => {
                vec!["Reduce recursion depth", "Increase stack size if possible"]
            }
            ErrorCode::NullPointerDeref => vec![
                "Check for null values before dereferencing",
                "Use null-safe operations when available",
            ],
            ErrorCode::OutOfBoundsAccess => vec![
                "Verify array indices are within bounds",
                "Use array length checks before access",
            ],
            ErrorCode::InvalidFunctionCall => vec![
                "Check function signature and arguments",
                "Ensure the function is properly defined",
            ],
            ErrorCode::DivisionByZero => vec![
                "Check divisor is not zero before dividing",
                "Add validation or error handling",
            ],

            // Warnings
            ErrorCode::UnusedVariable => vec![
                "Remove the variable if not needed",
                "Or use it in the code",
                "Prefix with _ if intentionally unused",
            ],
            ErrorCode::UnusedFunction => vec![
                "Remove the function if not needed",
                "Or ensure it is called somewhere",
            ],
            ErrorCode::UnusedImport => vec!["Remove the import if not needed"],
            ErrorCode::UnreachableCode => vec![
                "This code is never executed",
                "Check for early returns or unconditional branches",
            ],
            ErrorCode::VariableShadowing => vec![
                "Rename the variable to avoid shadowing",
                "Or remove the inner definition",
            ],
            ErrorCode::FunctionShadowing => vec![
                "Rename the function to avoid shadowing",
                "Or remove the inner definition",
            ],
            ErrorCode::DeprecatedFeature => vec![
                "Use the recommended alternative",
                "See documentation for migration guide",
            ],
            ErrorCode::PerformanceIssue => vec![
                "Consider optimizing this code",
                "Profile to identify performance bottlenecks",
            ],
            ErrorCode::StyleViolation => vec![
                "Follow the project's style guidelines",
                "See STYLE.md for conventions",
            ],
            ErrorCode::PossibleLogicError => vec![
                "Review this code for correctness",
                "Add tests to verify expected behavior",
            ],
        }
    }

    /// Get the category of this error code
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

    /// Get the severity level of this error code
    pub fn severity(&self) -> ErrorSeverity {
        match self.category() {
            ErrorCategory::Parse | ErrorCategory::Type | ErrorCategory::Runtime => {
                ErrorSeverity::Error
            }
            ErrorCategory::Warning => ErrorSeverity::Warning,
        }
    }

    /// Check if this error code is a warning
    #[allow(dead_code)]
    pub fn is_warning(&self) -> bool {
        self.severity() == ErrorSeverity::Warning
    }

    /// Check if this error code is an error
    #[allow(dead_code)]
    pub fn is_error(&self) -> bool {
        self.severity() == ErrorSeverity::Error
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}[{}]: {}",
            self.severity(),
            self.code(),
            self.description()
        )
    }
}
