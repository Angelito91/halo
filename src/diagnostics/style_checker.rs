// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Style checker for Halo code - detects style violations and suggests improvements.

use super::error_codes::ErrorCode;
use crate::parser::ast::{Program, Statement, TopLevel};

/// Style violation detected in code
#[derive(Debug, Clone)]
pub struct StyleViolation {
    pub code: ErrorCode,
    pub line: u32,
    pub column: u32,
    pub message: String,
    pub suggestion: String,
}

/// Configuration for style checking
#[derive(Debug, Clone)]
pub struct StyleConfig {
    /// Maximum line length (default: 100)
    pub max_line_length: usize,
    /// Maximum function length in lines (default: 50)
    pub max_function_length: usize,
    /// Maximum nesting depth (default: 4)
    pub max_nesting_depth: usize,
    /// Enforce snake_case for identifiers
    pub enforce_snake_case: bool,
    /// Check for unused variables
    pub check_unused: bool,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            max_line_length: 100,
            max_function_length: 50,
            max_nesting_depth: 4,
            enforce_snake_case: true,
            check_unused: true,
        }
    }
}

/// Style checker for Halo programs
pub struct StyleChecker {
    config: StyleConfig,
    pub violations: Vec<StyleViolation>,
}

impl StyleChecker {
    /// Create a new style checker with default config
    pub fn new() -> Self {
        Self::with_config(StyleConfig::default())
    }

    /// Create a new style checker with custom config
    pub fn with_config(config: StyleConfig) -> Self {
        Self {
            config,
            violations: Vec::new(),
        }
    }

    /// Check a program for style violations
    pub fn check_program(&mut self, program: &Program, source: &str) -> Vec<StyleViolation> {
        self.violations.clear();

        // Check line lengths
        self.check_line_lengths(source);

        // Check naming conventions
        for item in &program.items {
            self.check_top_level_naming(item);
        }

        // Check function length
        for item in &program.items {
            if let TopLevel::Function { name, body, .. } = item {
                self.check_function_length(name, body);
            }
        }

        // Check nesting depth
        for item in &program.items {
            if let TopLevel::Function { body, .. } = item {
                self.check_nesting_depth(body, 0);
            } else if let TopLevel::Stmt { stmt, .. } = item {
                self.check_stmt_nesting(stmt, 0);
            }
        }

        self.violations.clone()
    }

    /// Check for overly long lines
    fn check_line_lengths(&mut self, source: &str) {
        for (line_num, line) in source.lines().enumerate() {
            if line.len() > self.config.max_line_length {
                self.violations.push(StyleViolation {
                    code: ErrorCode::StyleViolation,
                    line: (line_num + 1) as u32,
                    column: (self.config.max_line_length + 1) as u32,
                    message: format!(
                        "Line exceeds {} characters ({})",
                        self.config.max_line_length,
                        line.len()
                    ),
                    suggestion: "Break line into multiple statements".to_string(),
                });
            }
        }
    }

    /// Check naming conventions in top-level items
    fn check_top_level_naming(&mut self, item: &TopLevel) {
        match item {
            TopLevel::Function { name, .. } => {
                self.check_identifier_naming(name, "function", 1);
            }
            TopLevel::GlobalVar { name, .. } => {
                self.check_identifier_naming(name, "variable", 1);
            }
            _ => {}
        }
    }

    /// Check if identifier follows snake_case convention
    pub fn check_identifier_naming(&mut self, name: &str, kind: &str, line: u32) {
        if !self.config.enforce_snake_case {
            return;
        }

        // Skip special names
        if name.starts_with('_') || name == "__expr" {
            return;
        }

        // Check for camelCase (has uppercase followed by lowercase)
        if name.contains(|c: char| c.is_uppercase()) {
            let suggested = self.to_snake_case(name);
            self.violations.push(StyleViolation {
                code: ErrorCode::StyleViolation,
                line,
                column: 1,
                message: format!("{} '{}' should use snake_case naming", kind, name),
                suggestion: format!("Rename to '{}'", suggested),
            });
        }
    }

    /// Convert camelCase to snake_case
    pub fn to_snake_case(&self, name: &str) -> String {
        let mut result = String::new();
        for (i, c) in name.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
            } else {
                result.push(c);
            }
        }
        result
    }

    /// Check if function is too long
    fn check_function_length(&mut self, _name: &str, body: &crate::parser::ast::Block) {
        let length = body.stmts.len();
        if length > self.config.max_function_length {
            self.violations.push(StyleViolation {
                code: ErrorCode::PerformanceIssue,
                line: 1,
                column: 1,
                message: format!(
                    "Function body has {} statements (max: {})",
                    length, self.config.max_function_length
                ),
                suggestion: "Consider breaking function into smaller parts".to_string(),
            });
        }
    }

    /// Check nesting depth in blocks
    fn check_nesting_depth(&mut self, block: &crate::parser::ast::Block, depth: usize) {
        if depth > self.config.max_nesting_depth {
            self.violations.push(StyleViolation {
                code: ErrorCode::StyleViolation,
                line: 1,
                column: 1,
                message: format!(
                    "Code nesting depth {} exceeds maximum {}",
                    depth, self.config.max_nesting_depth
                ),
                suggestion: "Extract nested code into separate function".to_string(),
            });
        }

        for stmt in &block.stmts {
            self.check_stmt_nesting(stmt, depth + 1);
        }
    }

    /// Check nesting depth in statements
    fn check_stmt_nesting(&mut self, stmt: &Statement, depth: usize) {
        if depth > self.config.max_nesting_depth {
            return;
        }

        match stmt {
            Statement::If {
                then_branch,
                else_if_branches,
                else_branch,
                ..
            } => {
                self.check_nesting_depth(then_branch, depth + 1);
                for branch in else_if_branches {
                    self.check_nesting_depth(&branch.body, depth + 1);
                }
                if let Some(block) = else_branch {
                    self.check_nesting_depth(block, depth + 1);
                }
            }
            Statement::While { body, .. } => {
                self.check_nesting_depth(body, depth + 1);
            }
            _ => {}
        }
    }
}

impl Default for StyleChecker {
    fn default() -> Self {
        Self::new()
    }
}
