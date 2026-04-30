// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Global constants used throughout the Halo programming language and CLI.
//!
//! This module centralizes version information, UI strings, and other important
//! constants to ensure consistency across the codebase.

/// The version of the Halo programming language.
pub const VERSION: &str = "0.2.0";

/// The banner text displayed in CLI output.
pub const BANNER: &str = "🌟 Halo Programming Language";

/// Default LLVM optimization level (2 = `-O2`).
pub const DEFAULT_OPT_LEVEL: u32 = 2;

/// Default toolchain for final linking step (clang).
pub const DEFAULT_TOOLCHAIN: &str = "clang";

/// Checkmark emoji used for success indicators.
pub const SUCCESS_INDICATOR: &str = "✅";

/// Cross mark emoji used for error indicators.
pub const ERROR_INDICATOR: &str = "❌";

/// Folder emoji used for file path indicators.
pub const FILE_INDICATOR: &str = "📂";

/// Hammer emoji used for compilation/linking indicators.
pub const BUILD_INDICATOR: &str = "🔨";

/// Rocket emoji used for execution indicators.
pub const RUN_INDICATOR: &str = "🚀";

/// Hourglass emoji used for waiting indicators.
pub const WAIT_INDICATOR: &str = "⏳";
