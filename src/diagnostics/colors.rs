// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! ANSI color codes for terminal output.

/// ANSI color codes for terminal output
pub struct Colors;

impl Colors {
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";

    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const MAGENTA: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const WHITE: &'static str = "\x1b[37m";
    pub const GRAY: &'static str = "\x1b[90m";

    pub const BRIGHT_RED: &'static str = "\x1b[91m";
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &'static str = "\x1b[95m";
    pub const BRIGHT_CYAN: &'static str = "\x1b[96m";
}

fn bold_colored(text: &str, color: &str) -> String {
    format!("{}{}{}{}", Colors::BOLD, color, text, Colors::RESET)
}

fn colored(text: &str, color: &str) -> String {
    format!("{}{}{}", color, text, Colors::RESET)
}

pub fn error(text: &str) -> String {
    bold_colored(text, Colors::BRIGHT_RED)
}

pub fn warning(text: &str) -> String {
    bold_colored(text, Colors::BRIGHT_YELLOW)
}

pub fn success(text: &str) -> String {
    bold_colored(text, Colors::BRIGHT_GREEN)
}

pub fn info(text: &str) -> String {
    colored(text, Colors::BRIGHT_BLUE)
}

pub fn muted(text: &str) -> String {
    colored(text, Colors::GRAY)
}

pub fn highlight(text: &str) -> String {
    bold_colored(text, Colors::BRIGHT_CYAN)
}

pub fn location(text: &str) -> String {
    colored(text, Colors::CYAN)
}
