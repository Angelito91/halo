// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Compilation statistics and performance metrics

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Records timing information for different compilation stages
#[derive(Debug, Clone)]
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    pub fn new(name: impl Into<String>) -> Self {
        Timer {
            start: Instant::now(),
            name: name.into(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn elapsed_ms(&self) -> f64 {
        self.elapsed().as_secs_f64() * 1000.0
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Tracks statistics about the compilation process
#[derive(Debug, Default, Clone)]
pub struct CompilationStatistics {
    pub source_lines: usize,
    pub tokens: usize,
    pub ast_nodes: usize,
    pub functions: usize,
    pub variables: usize,
    pub strings: usize,
    pub errors: usize,
    pub warnings: usize,

    stage_times: HashMap<String, Duration>,
}

impl CompilationStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record the duration of a compilation stage
    pub fn record_stage(&mut self, name: &str, duration: Duration) {
        self.stage_times.insert(name.to_string(), duration);
    }

    /// Get the duration of a specific stage
    pub fn stage_duration(&self, name: &str) -> Option<Duration> {
        self.stage_times.get(name).copied()
    }

    /// Get total compilation time
    pub fn total_time(&self) -> Duration {
        self.stage_times.values().sum()
    }

    /// Get total time in milliseconds
    pub fn total_time_ms(&self) -> f64 {
        self.total_time().as_secs_f64() * 1000.0
    }

    /// Print a formatted summary
    pub fn print_summary(&self) {
        use crate::diagnostics::colors;

        println!();
        println!("{}", colors::info("╭─ Compilation Summary ─╮"));

        println!(
            "{}  {} {}",
            colors::muted("│"),
            "Source Lines:",
            colors::highlight(&self.source_lines.to_string())
        );
        println!(
            "{}  {} {}",
            colors::muted("│"),
            "Tokens:",
            colors::highlight(&self.tokens.to_string())
        );
        println!(
            "{}  {} {}",
            colors::muted("│"),
            "AST Nodes:",
            colors::highlight(&self.ast_nodes.to_string())
        );
        println!(
            "{}  {} {}",
            colors::muted("│"),
            "Functions:",
            colors::highlight(&self.functions.to_string())
        );
        println!(
            "{}  {} {}",
            colors::muted("│"),
            "Variables:",
            colors::highlight(&self.variables.to_string())
        );

        if !self.stage_times.is_empty() {
            println!("{}", colors::muted("├────────────────────"));

            let mut stages: Vec<_> = self.stage_times.iter().collect();
            stages.sort_by_key(|(name, _)| name.as_str());

            for (i, (name, duration)) in stages.iter().enumerate() {
                let ms = duration.as_secs_f64() * 1000.0;
                let is_last = i == stages.len() - 1;
                let prefix = if is_last { "└" } else { "├" };

                println!(
                    "{}  {} {} {}",
                    colors::muted(&format!("{} ", prefix)),
                    name,
                    colors::highlight(&format!("{:.2}ms", ms)),
                    if ms > 100.0 {
                        colors::warning("(slow)")
                    } else {
                        colors::success("✓")
                    }
                );
            }

            let total_ms = self.total_time_ms();
            println!(
                "{}  {} {} {}",
                colors::muted("   "),
                "Total:",
                colors::highlight(&format!("{:.2}ms", total_ms)),
                if total_ms > 500.0 {
                    colors::warning("(consider optimization)")
                } else {
                    String::new()
                }
            );
        }

        if self.errors > 0 || self.warnings > 0 {
            println!("{}", colors::muted("├────────────────────"));
            if self.errors > 0 {
                println!(
                    "{}  {} {}",
                    colors::muted("│"),
                    "Errors:",
                    colors::error(&self.errors.to_string())
                );
            }
            if self.warnings > 0 {
                println!(
                    "{}  {} {}",
                    colors::muted("│"),
                    "Warnings:",
                    colors::warning(&self.warnings.to_string())
                );
            }
        }

        println!("{}", colors::info("╰────────────────────╯"));
        println!();
    }
}

/// Measures execution time of a code block
#[macro_export]
macro_rules! measure {
    ($name:expr, $block:expr) => {{
        let _timer = $crate::utils::statistics::Timer::new($name);
        let result = $block;
        let elapsed_ms = _timer.elapsed_ms();
        if elapsed_ms > 100.0 {
            eprintln!("⚠️  {} took {:.2}ms", _timer.name(), elapsed_ms);
        }
        result
    }};
}
