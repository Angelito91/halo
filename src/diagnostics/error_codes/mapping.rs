// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use super::ErrorCode;

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
}
