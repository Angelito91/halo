// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use super::ErrorCode;

impl ErrorCode {
    #[allow(dead_code)] // PLANNED FOR v0.3.0: Error descriptions
    pub fn description(&self) -> &'static str {
        match self {
            ErrorCode::ExpectedCloseParen => "Expected ')' after parameters",
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

            ErrorCode::UnusedVariable => "Variable is not used",
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

    #[allow(dead_code)] // PLANNED FOR v0.3.0: Error suggestions
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
}
