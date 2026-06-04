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
#[allow(dead_code)] // PLANNED FOR v0.3.0: Optimization level configuration
pub const DEFAULT_OPT_LEVEL: u32 = 2;

/// Default toolchain for final linking step (clang).
#[allow(dead_code)] // PLANNED FOR v0.3.0: Toolchain configuration
pub const DEFAULT_TOOLCHAIN: &str = "clang";

/// Checkmark emoji used for success indicators.
#[allow(dead_code)] // PLANNED FOR v0.3.0: CLI output indicators
pub const SUCCESS_INDICATOR: &str = "✅";

/// Cross mark emoji used for error indicators.
#[allow(dead_code)] // PLANNED FOR v0.3.0: CLI output indicators
pub const ERROR_INDICATOR: &str = "❌";

/// Folder emoji used for file path indicators.
#[allow(dead_code)] // PLANNED FOR v0.3.0: CLI output indicators
pub const FILE_INDICATOR: &str = "📂";

/// Hammer emoji used for compilation/linking indicators.
#[allow(dead_code)] // PLANNED FOR v0.3.0: CLI build indicator
pub const BUILD_INDICATOR: &str = "🔨";

/// Rocket emoji used for execution indicators.
#[allow(dead_code)] // PLANNED FOR v0.3.0: CLI run indicator
pub const RUN_INDICATOR: &str = "🚀";

/// Hourglass emoji used for waiting indicators.
#[allow(dead_code)] // PLANNED FOR v0.3.0: CLI wait indicator
pub const WAIT_INDICATOR: &str = "⏳";
