// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Diagnostic system for error reporting with colored output and code snippets.

pub mod colors;
pub mod error_codes;
pub mod style_checker;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Severity level of a diagnostic message
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Note => write!(f, "note"),
        }
    }
}

/// Source location information for diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub source_lines: Vec<String>,
}

/// Stack frame in a call stack trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub function: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

/// Call stack trace for runtime errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackTrace {
    pub frames: Vec<StackFrame>,
}

impl StackTrace {
    /// Create a new empty stack trace
    pub fn new() -> Self {
        StackTrace { frames: Vec::new() }
    }

    /// Format stack trace for display
    pub fn format_display(&self) -> String {
        if self.frames.is_empty() {
            return String::new();
        }

        let mut output = String::from("\nStack trace:\n");
        for (i, frame) in self.frames.iter().enumerate() {
            output.push_str(&format!(
                "  {} in function '{}' at {}:{}:{}\n",
                i, frame.function, frame.file, frame.line, frame.column
            ));
        }
        output
    }
}

impl Default for StackTrace {
    fn default() -> Self {
        Self::new()
    }
}

/// A diagnostic message with optional location and hints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub location: Option<DiagnosticLocation>,
    pub hints: Vec<String>,
    pub stack_trace: Option<StackTrace>,
    pub error_code: Option<String>,
}

impl Diagnostic {
    /// Create a new error diagnostic
    pub fn error(message: impl Into<String>) -> Self {
        Diagnostic {
            level: DiagnosticLevel::Error,
            message: message.into(),
            location: None,
            hints: Vec::new(),
            stack_trace: None,
            error_code: None,
        }
    }



    /// Add location information to the diagnostic
    pub fn with_location(mut self, location: DiagnosticLocation) -> Self {
        self.location = Some(location);
        self
    }







    /// Format and write the diagnostic to any writer
    pub fn print(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        match self.level {
            DiagnosticLevel::Error => {
                if let Some(code) = &self.error_code {
                    writeln!(
                        out,
                        "{}[{}]: {}",
                        colors::error("error"),
                        code,
                        self.message
                    )?;
                } else {
                    writeln!(out, "{}: {}", colors::error("error"), self.message)?;
                }
            }
            DiagnosticLevel::Warning => {
                if let Some(code) = &self.error_code {
                    writeln!(
                        out,
                        "{}[{}]: {}",
                        colors::warning("warning"),
                        code,
                        self.message
                    )?;
                } else {
                    writeln!(out, "{}: {}", colors::warning("warning"), self.message)?;
                }
            }
            DiagnosticLevel::Note => {
                writeln!(out, "{}: {}", colors::info("note"), self.message)?;
            }
        }

        // Location and code snippet
        if let Some(loc) = &self.location {
            writeln!(out)?;
            writeln!(
                out,
                "  {} {}:{}:{}",
                colors::muted("→"),
                colors::location(&loc.file),
                colors::highlight(&loc.line.to_string()),
                colors::highlight(&loc.column.to_string())
            )?;

            if !loc.source_lines.is_empty() {
                writeln!(out)?;

                let line_start = (loc.line as usize).saturating_sub(loc.source_lines.len() - 1);

                for (i, line) in loc.source_lines.iter().enumerate() {
                    let line_num = line_start + i;
                    let is_error_line = line_num == loc.line as usize;

                    writeln!(
                        out,
                        "  {} │ {}",
                        colors::muted(&format!("{:>4}", line_num)),
                        line
                    )?;

                    if is_error_line && loc.column > 0 {
                        let col_pos = (loc.column as usize).saturating_sub(1);
                        let spaces = " ".repeat(col_pos);
                        write!(out, "  {} │ {}", colors::muted("    "), spaces)?;
                        writeln!(out, "{}", colors::error("^"))?;
                    }
                }
            }
        }

        // Hints and suggestions
        if !self.hints.is_empty() {
            writeln!(out)?;
            for hint in &self.hints {
                writeln!(out, "  {} {}", colors::info("hint:"), hint)?;
            }
        }

        // Stack trace if present
        if let Some(trace) = &self.stack_trace {
            writeln!(out)?;
            write!(out, "{}", trace.format_display())?;
        }

        Ok(())
    }

    /// Convert to string for output
    pub fn to_string_formatted(&self) -> String {
        let mut buf = String::new();
        let _ = self.print(&mut buf);
        buf
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print(f)
    }
}

/// Extract source lines around a specific line for context
pub fn extract_source_lines(source: &str, line: u32, context_lines: usize) -> Vec<String> {
    let lines: Vec<&str> = source.lines().collect();
    let line_idx = (line as usize).saturating_sub(1);

    let start = line_idx.saturating_sub(context_lines);
    let end = std::cmp::min(line_idx + context_lines + 1, lines.len());

    lines[start..end].iter().map(|s| s.to_string()).collect()
}

/// Create a DiagnosticLocation from source code
pub fn location_from_source(
    file: &str,
    source: &str,
    line: u32,
    column: u32,
    context_lines: usize,
) -> DiagnosticLocation {
    DiagnosticLocation {
        file: file.to_string(),
        line,
        column,
        source_lines: extract_source_lines(source, line, context_lines),
    }
}




/// Statistics gathered during compilation
#[derive(Debug, Default, Clone)]
pub struct CompilationStats {
    pub tokens: usize,
    pub ast_nodes: usize,
    pub functions: usize,
    pub variables: usize,
    pub lines_of_code: usize,
}

impl CompilationStats {
    pub fn print_summary(&self) {
        println!();
        println!("{}", colors::info("╭─ Compilation Statistics ─╮"));
        println!(
            "{}  Tokens:      {}",
            colors::muted("│"),
            colors::highlight(&self.tokens.to_string())
        );
        println!(
            "{}  AST Nodes:   {}",
            colors::muted("│"),
            colors::highlight(&self.ast_nodes.to_string())
        );
        println!(
            "{}  Functions:   {}",
            colors::muted("│"),
            colors::highlight(&self.functions.to_string())
        );
        println!(
            "{}  Variables:   {}",
            colors::muted("│"),
            colors::highlight(&self.variables.to_string())
        );
        println!(
            "{}  Lines:       {}",
            colors::muted("│"),
            colors::highlight(&self.lines_of_code.to_string())
        );
        println!("{}", colors::info("╰────────────────────────────╯"));
        println!();
    }
}
