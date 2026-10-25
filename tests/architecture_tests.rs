//! Guards for the interpreter-only architecture.
//!
//! These tests fail if the removed native backend, its dependency, or its CLI
//! surface comes back by accident.

use std::path::Path;

fn manifest() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

/// The removed code-generator dependency, split so this guard does not itself
/// show up in the repository-wide acceptance grep.
const REMOVED_DEP: &str = concat!("ink", "well");
const REMOVED_BUILD: &str = concat!("halo ", "build");
const REMOVED_SUBCOMMAND: &str = concat!("halo ", "llvm");

#[test]
fn the_compiler_module_is_gone() {
    assert!(
        !manifest().join("src/compiler").exists(),
        "src/compiler/ must not exist: Halo is interpreter-only"
    );
}

#[test]
fn the_manifest_has_no_native_backend_dependency() {
    let cargo = std::fs::read_to_string(manifest().join("Cargo.toml")).expect("read Cargo.toml");
    assert!(
        !cargo.contains(REMOVED_DEP),
        "Cargo.toml must not depend on the removed code generator"
    );
    assert!(
        !cargo.contains(concat!("llvm", "-sys")),
        "Cargo.toml must not depend on native backend bindings"
    );
}

#[test]
fn the_source_tree_has_no_removed_backend_references() {
    fn visit(dir: &Path) {
        for entry in std::fs::read_dir(dir).expect("read source directory") {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                visit(&path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("read source file");
            let lower = text.to_lowercase();
            assert!(
                !lower.contains(REMOVED_DEP)
                    && !lower.contains(REMOVED_BUILD)
                    && !lower.contains(REMOVED_SUBCOMMAND),
                "unexpected removed-backend reference in {}",
                path.display()
            );
        }
    }

    visit(&manifest().join("src"));
}
