// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use super::{ErrorCategory, ErrorCode, ErrorSeverity};

impl ErrorCode {
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
