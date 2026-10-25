//! Cobertura de la red de paridad derivada del catálogo.

mod common;

use common::cases::CASES;
use halo::lang::catalog::{Support, BUILTINS, METHODS};

fn covered_features() -> Vec<&'static str> {
    CASES
        .iter()
        .flat_map(|case| case.covers.iter().copied())
        .collect()
}

#[test]
fn every_builtin_with_both_support_is_covered() {
    let covered = covered_features();
    for builtin in BUILTINS {
        if builtin.support != Support::Both {
            continue;
        }
        assert!(
            covered.contains(&builtin.name),
            "builtin '{}' has no parity case covering it",
            builtin.name
        );
    }
}

#[test]
fn every_method_with_both_support_is_covered() {
    let covered = covered_features();
    for method in METHODS {
        if method.support != Support::Both {
            continue;
        }
        assert!(
            covered.contains(&method.name),
            "method '{}' has no parity case covering it",
            method.name
        );
    }
}

#[test]
fn every_covered_feature_exists_in_the_catalog() {
    let covered = covered_features();
    for name in covered {
        let known = BUILTINS.iter().any(|builtin| builtin.name == name)
            || METHODS.iter().any(|method| method.name == name);
        assert!(
            known,
            "a parity case covers '{name}', which is not in the catalog"
        );
    }
}
