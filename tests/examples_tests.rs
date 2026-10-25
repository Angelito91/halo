//! Smoke tests for the programs shipped in `examples/`.
//!
//! Every example must be executable by the interpreter. `09_test_error.halo`
//! is the deliberate exception: it contains syntax errors and must fail.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn examples_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples")
}

fn halo() -> Command {
    Command::new(env!("CARGO_BIN_EXE_halo"))
}

fn run_example(path: &Path) -> Output {
    halo()
        .arg(path)
        .output()
        .unwrap_or_else(|e| panic!("cannot run {}: {e}", path.display()))
}

fn assert_success(path: &Path, out: &Output) {
    assert!(
        out.status.success(),
        "{} failed with {:?}\nstdout:\n{}\nstderr:\n{}",
        path.display(),
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn every_functional_example_runs() {
    let mut checked = 0;

    for entry in std::fs::read_dir(examples_dir()).expect("read examples/") {
        let path = entry.expect("directory entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("halo") {
            continue;
        }
        if path.file_name().and_then(|n| n.to_str()) == Some("09_test_error.halo") {
            continue;
        }

        let out = run_example(&path);
        assert_success(&path, &out);
        checked += 1;
    }

    assert_eq!(checked, 10, "expected 10 functional examples");
}

#[test]
fn the_error_example_fails_with_a_diagnostic() {
    let path = examples_dir().join("09_test_error.halo");
    let out = halo()
        .arg("check")
        .arg(&path)
        .output()
        .expect("run halo check");

    assert_eq!(out.status.code(), Some(1), "the error example must fail");
    assert!(
        !out.stderr.is_empty(),
        "the error example must explain what is wrong"
    );
}
