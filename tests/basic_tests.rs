//! Smoke tests against the compiled binary.

mod common;
use common::*;

#[test]
fn version_works() {
    let out = run_devora(&["--version"]);
    assert!(out.success(), "stderr: {}", out.stderr);
    assert!(out.contains("devora"));
}

#[test]
fn help_works() {
    let out = run_devora(&["--help"]);
    assert!(out.success(), "stderr: {}", out.stderr);
    assert!(out.contains("Usage"));
}

#[test]
fn list_works() {
    let out = run_devora(&["list"]);
    assert!(out.success(), "stderr: {}", out.stderr);
    assert!(out.contains("rust"));
}
