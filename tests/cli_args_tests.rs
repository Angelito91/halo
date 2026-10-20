// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Behaviour of the CLI argument layer.
//!
//! These tests pin the toolchain parsing contract so the `Toolchain`
//! implementation can move from an inherent `from_str` to the standard
//! `std::str::FromStr` trait without changing what the CLI accepts or how
//! it reports a bad value.

use halo::cli::args::Toolchain;
use std::str::FromStr;

#[test]
fn toolchain_parses_clang() {
    assert_eq!(Toolchain::from_str("clang").unwrap(), Toolchain::Clang);
}

#[test]
fn toolchain_parses_llc() {
    assert_eq!(Toolchain::from_str("llc").unwrap(), Toolchain::Llc);
}

#[test]
fn toolchain_rejects_an_unknown_name() {
    let err = Toolchain::from_str("gcc").unwrap_err();
    assert_eq!(
        err,
        "Unknown toolchain 'gcc'. Valid options are: clang, llc."
    );
}

#[test]
fn toolchain_default_is_clang() {
    assert_eq!(Toolchain::default(), Toolchain::Clang);
}

#[test]
fn toolchain_name_is_the_display_label() {
    // `name()` is a human label, not the accepted CLI spelling: `llc` is
    // accepted by the parser but displayed as "llc + cc".
    assert_eq!(Toolchain::Clang.name(), "clang");
    assert_eq!(Toolchain::Llc.name(), "llc + cc");
}

#[test]
fn toolchain_parses_through_the_standard_trait() {
    // The same contract, reached through `str::parse`.
    assert_eq!("clang".parse::<Toolchain>().unwrap(), Toolchain::Clang);
    assert_eq!("llc".parse::<Toolchain>().unwrap(), Toolchain::Llc);
    assert!("gcc".parse::<Toolchain>().is_err());
}
