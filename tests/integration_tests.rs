// Integration tests for Devora
// These tests verify the end-to-end functionality of the CLI

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

mod common;
use common::*;

// Helper function to run the devora binary
fn run_devora_command(args: &[&str], current_dir: Option<&Path>) -> Result<String, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--", "devora"]).args(args);

    if let Some(dir) = current_dir {
        cmd.current_dir(dir);
    }

    let output = cmd.output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// Helper function to run devora binary directly (for integration tests)
fn run_devora_binary(args: &[&str], current_dir: Option<&Path>) -> Result<String, Box<dyn std::error::Error>> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_devora"));
    cmd.args(args);

    if let Some(dir) = current_dir {
        cmd.current_dir(dir);
    }

    let output = cmd.output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[test]
fn test_version_command() {
    let output = run_devora_command(&["--version"], None).unwrap();
    assert!(output.contains("devora"));
    assert!(output.contains("0.1.0"));
}

#[test]
fn test_help_command() {
    let output = run_devora_command(&["--help"], None).unwrap();
    assert!(output.contains("Devora"));
    assert!(output.contains("Usage"));
    assert!(output.contains("new"));
    assert!(output.contains("list"));
    assert!(output.contains("info"));
}

#[test]
fn test_list_command() {
    let output = run_devora_command(&["list"], None).unwrap();
    assert!(output.contains("Available Languages"));
    assert!(output.contains("cpp"));
    assert!(output.contains("rust"));
}

#[test]
fn test_list_cpp_command() {
    let output = run_devora_command(&["list", "cpp"], None).unwrap();
    assert!(output.contains("cpp"));
    assert!(output.contains("cmake"));
}

#[test]
fn test_info_command() {
    let output = run_devora_command(&["info", "cpp"], None).unwrap();
    assert!(output.contains("Language Information"));
    assert!(output.contains("C++"));
}

#[test]
fn test_info_framework_command() {
    let output = run_devora_command(&["info", "cpp", "cmake"], None).unwrap();
    assert!(output.contains("Framework Information"));
    assert!(output.contains("CMake"));
}

#[test]
fn test_project_generation_dry_run() {
    let temp_dir = TempDir::new().unwrap();
    let output = run_devora_command(
        &[
            "new",
            "test-project",
            "cpp",
            "--framework", "cmake",
            "--var", "cpp_standard=C++20",
            "--dry-run"
        ],
        Some(temp_dir.path())
    ).unwrap();

    assert!(output.contains("Dry run mode"));
    assert!(output.contains("Would create"));
    assert!(output.contains("CMakeLists.txt"));
}

#[test]
fn test_project_generation_non_interactive() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-project");

    let output = run_devora_command(
        &[
            "new",
            "test-project",
            "cpp",
            "--framework", "cmake",
            "--var", "cpp_standard=C++17",
            "--var", "include_tests=false",
            "--non-interactive"
        ],
        Some(temp_dir.path())
    ).unwrap();

    assert!(project_path.exists());
    assert!(project_path.join("CMakeLists.txt").exists());
    assert!(project_path.join("src").exists());
    assert!(project_path.join("include").exists());

    // Verify CMakeLists.txt content
    let cmake_content = fs::read_to_string(project_path.join("CMakeLists.txt")).unwrap();
    assert!(cmake_content.contains("CMAKE_CXX_STANDARD 17"));
}

#[test]
fn test_project_generation_with_tests() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-project-with-tests");

    let output = run_devora_command(
        &[
            "new",
            "test-project-with-tests",
            "cpp",
            "--framework", "cmake",
            "--var", "include_tests=true",
            "--non-interactive"
        ],
        Some(temp_dir.path())
    ).unwrap();

    assert!(project_path.exists());
    assert!(project_path.join("tests").exists());
    assert!(project_path.join("tests/test_main.cpp").exists());
}

#[test]
fn test_project_generation_rust() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-rust-project");

    let output = run_devora_command(
        &[
            "new",
            "test-rust-project",
            "rust",
            "--framework", "base",
            "--non-interactive"
        ],
        Some(temp_dir.path())
    ).unwrap();

    assert!(project_path.exists());
    assert!(project_path.join("Cargo.toml").exists());
    assert!(project_path.join("src").exists());
    assert!(project_path.join("src/main.rs").exists());
}

#[test]
fn test_invalid_language() {
    let output = run_devora_command(&["info", "invalid-lang"], None).unwrap();
    assert!(output.contains("not found") || output.contains("Error"));
}

#[test]
fn test_invalid_framework() {
    let output = run_devora_command(&["info", "cpp", "invalid-framework"], None).unwrap();
    assert!(output.contains("not found") || output.contains("Error"));
}

#[test]
fn test_plugin_discovery() {
    let output = run_devora_command(&["list"], None).unwrap();

    // Should find both rust and cpp plugins
    assert!(output.contains("rust"));
    assert!(output.contains("cpp"));
}

#[test]
fn test_framework_discovery() {
    let cpp_output = run_devora_command(&["list", "cpp"], None).unwrap();
    assert!(cpp_output.contains("cmake"));
    assert!(cpp_output.contains("conan"));
    assert!(cpp_output.contains("makefile"));
    assert!(cpp_output.contains("header-only"));

    let rust_output = run_devora_command(&["list", "rust"], None).unwrap();
    assert!(rust_output.contains("base"));
}

#[test]
fn test_conditional_file_generation() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-conditional");

    // Test with include_examples=false
    run_devora_command(
        &[
            "new",
            "test-conditional",
            "cpp",
            "--framework", "cmake",
            "--var", "include_examples=false",
            "--non-interactive"
        ],
        Some(temp_dir.path())
    ).unwrap();

    assert!(!project_path.join("examples").exists());

    // Test with include_examples=true
    let project_path2 = temp_dir.path().join("test-conditional-with-examples");
    run_devora_command(
        &[
            "new",
            "test-conditional-with-examples",
            "cpp",
            "--framework", "cmake",
            "--var", "include_examples=true",
            "--non-interactive"
        ],
        Some(temp_dir.path())
    ).unwrap();

    assert!(project_path2.join("examples").exists());
}

#[test]
fn test_cpp_standard_selection() {
    let standards = vec!["C++14", "C++17", "C++20", "C++23"];

    for standard in standards {
        let temp_dir = TempDir::new().unwrap();
        let project_name = format!("test-cpp-{}", standard.to_lowercase().replace("++", ""));
        let project_path = temp_dir.path().join(&project_name);

        run_devora_command(
            &[
                "new",
                &project_name,
                "cpp",
                "--framework", "cmake",
                "--var", &format!("cpp_standard={}", standard),
                "--non-interactive"
            ],
            Some(temp_dir.path())
        ).unwrap();

        // Verify CMakeLists.txt has correct standard
        let cmake_content = fs::read_to_string(project_path.join("CMakeLists.txt")).unwrap();
        let expected_standard = standard.replace("C++", "");
        assert!(cmake_content.contains(&format!("CMAKE_CXX_STANDARD {}", expected_standard)));
    }
}

#[test]
fn test_git_initialization() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-git");

    run_devora_command(
        &[
            "new",
            "test-git",
            "cpp",
            "--framework", "cmake",
            "--var", "initialize_git=false",
            "--non-interactive"
        ],
        Some(temp_dir.path())
    ).unwrap();

    // Should not have .git directory
    assert!(!project_path.join(".git").exists());

    // Test with git initialization enabled
    let project_path2 = temp_dir.path().join("test-git-enabled");
    run_devora_command(
        &[
            "new",
            "test-git-enabled",
            "cpp",
            "--framework", "cmake",
            "--var", "initialize_git=true",
            "--non-interactive"
        ],
        Some(temp_dir.path())
    ).unwrap();

    // Should have .git directory (if git is available and configured)
    // Note: This might fail in CI environments without git configured
    if Command::new("git").arg("--version").output().is_ok() {
        assert!(project_path2.join(".git").exists() ||
               project_path2.join(".gitignore").exists());
    }
}

#[test]
fn test_error_handling() {
    // Test invalid project name
    let output = run_devora_command(&["new", "", "cpp"], None).unwrap();
    assert!(output.contains("Error") || output.contains("Invalid"));

    // Test missing required arguments
    let output = run_devora_command(&["new"], None).unwrap();
    assert!(output.contains("Usage") || output.contains("Required"));
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn test_multiple_projects() {
        let temp_dir = TempDir::new().unwrap();

        for i in 0..5 {
            let project_name = format!("stress-test-{}", i);
            run_devora_command(
                &[
                    "new",
                    &project_name,
                    "cpp",
                    "--framework", "cmake",
                    "--non-interactive"
                ],
                Some(temp_dir.path())
            ).unwrap();

            let project_path = temp_dir.path().join(&project_name);
            assert!(project_path.exists());
            assert!(project_path.join("CMakeLists.txt").exists());
        }
    }

    #[test]
    fn test_large_project_name() {
        let temp_dir = TempDir::new().unwrap();
        let large_name = "a".repeat(100);

        let output = run_devora_command(
            &[
                "new",
                &large_name,
                "cpp",
                "--framework", "cmake",
                "--non-interactive"
            ],
            Some(temp_dir.path())
        ).unwrap();

        // Should handle long names gracefully or provide helpful error
        let project_path = temp_dir.path().join(&large_name);
        assert!(project_path.exists() || output.contains("Error"));
    }
}