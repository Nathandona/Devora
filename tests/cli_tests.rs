//! CLI behavior tests: invoke the real binary and assert on its output.

mod common;
use common::*;

#[test]
fn help_lists_subcommands() {
    let out = run_devora(&["--help"]);
    assert!(out.success());
    assert!(out.contains("Usage"));
    assert!(out.contains("new"));
    assert!(out.contains("list"));
    assert!(out.contains("info"));
}

#[test]
fn version_flag() {
    let out = run_devora(&["--version"]);
    assert!(out.success());
    assert!(out.contains("devora"));
    assert!(out.contains("0.1.0"));
}

#[test]
fn no_subcommand_is_error_with_usage() {
    let out = run_devora(&[]);
    assert!(!out.success());
    assert!(out.contains("Usage") || out.contains("usage"));
}

#[test]
fn invalid_flag_is_error() {
    let out = run_devora(&["--definitely-not-a-flag"]);
    assert!(!out.success());
}

#[test]
fn list_shows_status_board() {
    let out = run_devora(&["list"]);
    assert!(out.success());
    assert!(out.contains("rust"));
    assert!(out.contains("stable"));
    assert!(out.contains("c++"));
    assert!(out.contains("go"));
    assert!(out.contains("python"));
    assert!(out.contains("c#"));
}

#[test]
fn list_language_shows_frameworks() {
    let out = run_devora(&["list", "rust"]);
    assert!(out.success());
    assert!(out.contains("base"));
}

#[test]
fn list_json_is_parseable() {
    let out = run_devora(&["list", "--json"]);
    assert!(out.success());
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid json");
    assert!(v["languages"].is_array());
    assert_eq!(v["languages"][0]["name"], "rust");
}

#[test]
fn info_language() {
    let out = run_devora(&["info", "rust"]);
    assert!(out.success());
    assert!(out.contains("Rust"));
    assert!(out.contains("base"));
}

#[test]
fn info_framework() {
    let out = run_devora(&["info", "rust", "base"]);
    assert!(out.success());
    assert!(out.contains("Base"));
}

#[test]
fn info_json_is_parseable() {
    let out = run_devora(&["info", "rust", "--json"]);
    assert!(out.success());
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid json");
    assert_eq!(v["language"]["id"], "rust");
}

#[test]
fn new_help_lists_flags() {
    let out = run_devora(&["new", "--help"]);
    assert!(out.success());
    assert!(out.contains("--framework"));
    assert!(out.contains("--var"));
    assert!(out.contains("--non-interactive"));
    assert!(out.contains("--no-hooks"));
    assert!(out.contains("--dry-run"));
}

#[test]
fn new_missing_args_is_error() {
    let out = run_devora(&["new"]);
    assert!(!out.success());
}

#[test]
fn new_missing_language_is_error() {
    let out = run_devora(&["new", "myproj", "--non-interactive"]);
    assert!(!out.success());
}

#[test]
fn new_invalid_language_is_error() {
    let out = run_devora(&["new", "myproj", "nonsense", "--non-interactive"]);
    assert!(!out.success());
    assert!(out.contains("not found"));
}

#[test]
fn new_invalid_framework_is_error() {
    let out = run_devora(&[
        "new",
        "myproj",
        "rust",
        "--framework",
        "ghost",
        "--non-interactive",
    ]);
    assert!(!out.success());
    assert!(out.contains("not found"));
}

#[test]
fn new_dry_run_creates_nothing() {
    let project = TestProject::new();
    let out = project.run(&["new", "myproj", "rust", "--non-interactive", "--dry-run"]);
    assert!(out.success());
    assert!(out.contains("Dry run mode"));
    assert!(out.contains("Would create"));
    assert!(!project.join("myproj").exists());
}

#[test]
fn new_rust_no_hooks_generates_compilable_project() {
    let project = TestProject::new();
    let out = project.run(&["new", "myapp", "rust", "--non-interactive", "--no-hooks"]);
    assert!(out.success(), "stderr: {}", out.stderr);

    let app = project.join("myapp");
    assert_file_exists(&app.join("Cargo.toml"));
    assert_file_exists(&app.join("src/main.rs"));
    let main = read_to_string(&app.join("src/main.rs"));
    assert!(main.contains("fn main"));
}

#[test]
fn new_json_output_is_parseable() {
    let project = TestProject::new();
    let out = project.run(&["new", "svc", "rust", "--no-hooks", "--json"]);
    assert!(out.success(), "stderr: {}", out.stderr);
    let v: serde_json::Value = serde_json::from_str(&out.stdout).expect("valid json");
    assert_eq!(v["language"], "rust");
    assert_eq!(v["framework"], "base");
    assert!(v["files"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f == "src/main.rs"));
}
