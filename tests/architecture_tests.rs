//! Guards for the interpreter-only architecture.
//!
//! These tests fail if the LLVM backend, its dependency, or its CLI surface
//! comes back by accident.

use std::path::Path;

fn manifest() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn the_compiler_module_is_gone() {
    assert!(
        !manifest().join("src/compiler").exists(),
        "src/compiler/ must not exist: Halo is interpreter-only"
    );
}

#[test]
fn the_manifest_has_no_llvm_dependency() {
    let cargo = std::fs::read_to_string(manifest().join("Cargo.toml")).expect("read Cargo.toml");
    assert!(
        !cargo.contains("inkwell"),
        "Cargo.toml must not depend on inkwell"
    );
    assert!(
        !cargo.contains("llvm-sys"),
        "Cargo.toml must not depend on llvm-sys"
    );
}

#[test]
fn the_source_tree_has_no_llvm_references() {
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
                !lower.contains("inkwell")
                    && !lower.contains("halo build")
                    && !lower.contains("halo llvm"),
                "unexpected LLVM reference in {}",
                path.display()
            );
        }
    }

    visit(&manifest().join("src"));
}
