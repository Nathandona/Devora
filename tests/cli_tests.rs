// CLI-specific tests for Devora
// These tests focus on command-line interface behavior

mod common;
use common::*;

#[test]
fn test_cli_help_flag() {
    let output = run_cargo_command(&["run", "--", "devora", "--help"]).unwrap();
    assert!(output.success());
    assert!(output.contains("Usage"));
    assert!(output.contains("devora"));
    assert!(output.contains("new"));
    assert!(output.contains("list"));
    assert!(output.contains("info"));
}

#[test]
fn test_cli_version_flag() {
    let output = run_cargo_command(&["run", "--", "devora", "--version"]).unwrap();
    assert!(output.success());
    assert!(output.contains("devora"));
    assert!(output.contains("0.1.0"));
}

#[test]
fn test_cli_no_arguments() {
    let output = run_cargo_command(&["run", "--", "devora"]).unwrap();
    assert!(output.success());
    // Should show help or usage information
    assert!(output.contains("Usage") || output.contains("help"));
}

#[test]
fn test_cli_invalid_flag() {
    let output = run_cargo_command(&["run", "--", "devora", "--invalid-flag"]).unwrap();
    assert!(!output.success());
    assert!(output.contains("unrecognized") || output.contains("unexpected") || output.contains("error"));
}

#[test]
fn test_cli_list_command() {
    let output = run_cargo_command(&["run", "--", "devora", "list"]).unwrap();
    assert!(output.success());
    assert!(output.contains("Available Languages"));
    assert!(output.contains("rust"));
    assert!(output.contains("cpp"));
}

#[test]
fn test_cli_list_with_language() {
    let output = run_cargo_command(&["run", "--", "devora", "list", "cpp"]).unwrap();
    assert!(output.success());
    assert!(output.contains("cpp"));
    assert!(output.contains("cmake"));
}

#[test]
fn test_cli_info_command() {
    let output = run_cargo_command(&["run", "--", "devora", "info", "cpp"]).unwrap();
    assert!(output.success());
    assert!(output.contains("Language Information"));
    assert!(output.contains("C++"));
}

#[test]
fn test_cli_info_with_framework() {
    let output = run_cargo_command(&["run", "--", "devora", "info", "cpp", "cmake"]).unwrap();
    assert!(output.success());
    assert!(output.contains("Framework Information"));
    assert!(output.contains("CMake"));
}

#[test]
fn test_cli_new_command_help() {
    let output = run_cargo_command(&["run", "--", "devora", "new", "--help"]).unwrap();
    assert!(output.success());
    assert!(output.contains("Create a new project"));
    assert!(output.contains("project-name"));
    assert!(output.contains("language"));
    assert!(output.contains("--framework"));
    assert!(output.contains("--var"));
    assert!(output.contains("--non-interactive"));
    assert!(output.contains("--dry-run"));
}

#[test]
fn test_cli_new_missing_arguments() {
    let output = run_cargo_command(&["run", "--", "devora", "new"]).unwrap();
    assert!(!output.success());
    assert!(output.contains("required") || output.contains("missing") || output.contains("error"));
}

#[test]
fn test_cli_new_missing_language() {
    let output = run_cargo_command(&["run", "--", "devora", "new", "test-project"]).unwrap();
    assert!(!output.success());
    assert!(output.contains("required") || output.contains("missing") || output.contains("error"));
}

#[test]
fn test_cli_new_invalid_language() {
    let output = run_cargo_command(&["run", "--", "devora", "new", "test-project", "invalid-lang"]).unwrap();
    assert!(!output.success());
    assert!(output.contains("not found") || output.contains("invalid") || output.contains("error"));
}

#[test]
fn test_cli_new_dry_run() {
    let project = TestProject::new().unwrap();
    let output = project.run_devora(&[
        "new",
        "test-project",
        "cpp",
        "--framework", "cmake",
        "--dry-run"
    ]).unwrap();

    assert!(output.success());
    assert!(output.contains("Dry run mode"));
    assert!(output.contains("Would create"));

    // Ensure no files were actually created
    assert!(!project.join("test-project").exists());
}

#[test]
fn test_cli_new_non_interactive() {
    let project = TestProject::new().unwrap();
    let output = project.run_devora(&[
        "new",
        "test-project",
        "cpp",
        "--framework", "cmake",
        "--non-interactive"
    ]).unwrap();

    assert!(output.success());
    assert!(!output.contains("Dry run mode"));

    let project_path = project.join("test-project");
    assert_dir_exists(&project_path);
    assert_file_exists(&project_path.join("CMakeLists.txt"));
    assert_dir_exists(&project_path.join("src"));
    assert_dir_exists(&project_path.join("include"));
}

#[test]
fn test_cli_new_with_variables() {
    let project = TestProject::new().unwrap();
    let output = project.run_devora(&[
        "new",
        "test-project",
        "cpp",
        "--framework", "cmake",
        "--var", "cpp_standard=C++20",
        "--var", "include_tests=false",
        "--var", "description=Test project description",
        "--non-interactive"
    ]).unwrap();

    assert!(output.success());

    let project_path = project.join("test-project");
    let cmake_content = read_file_to_string(&project_path.join("CMakeLists.txt"));
    assert!(cmake_content.contains("CMAKE_CXX_STANDARD 20"));

    // Tests should not be included
    assert!(!project_path.join("tests").exists());
}

#[test]
fn test_cli_variable_formatting() {
    let project = TestProject::new().unwrap();

    // Test different variable formats
    let output = project.run_devora(&[
        "new",
        "test-var-project",
        "cpp",
        "--framework", "cmake",
        "--var", "cpp_standard=C++17",
        "--var", "include_examples=true",
        "--var", "build_type=Debug",
        "--non-interactive"
    ]).unwrap();

    assert!(output.success());

    let project_path = project.join("test-var-project");
    assert_file_exists(&project_path.join("examples"));
}

#[test]
fn test_cli_colored_output() {
    // Test that colored output doesn't break anything
    let output = run_cargo_command(&["run", "--", "devora", "list"]).unwrap();
    assert!(output.success());

    // The output should contain color codes or at least not fail
    assert!(output.stdout.len() > 0 || output.stderr.len() > 0);
}

#[test]
fn test_cli_verbose_mode() {
    // Test verbose mode if supported
    let output = run_cargo_command(&["run", "--", "devora", "list", "--verbose"]).unwrap();
    // May or may not succeed depending on implementation
    // Just ensure it doesn't crash
}

#[test]
fn test_cli_quiet_mode() {
    // Test quiet mode if supported
    let output = run_cargo_command(&["run", "--", "devora", "list", "--quiet"]).unwrap();
    // May or may not succeed depending on implementation
    // Just ensure it doesn't crash
}

#[test]
fn test_cli_project_name_validation() {
    let project = TestProject::new().unwrap();

    // Test invalid project names
    let invalid_names = ["", "project/invalid", "project\\invalid", "project with spaces", "123project"];

    for name in invalid_names {
        let output = project.run_devora(&[
            "new",
            name,
            "cpp",
            "--framework", "cmake",
            "--dry-run"
        ]);

        // Should either fail or sanitize the name
        match output {
            Ok(out) => {
                // If it succeeds, check that the name was sanitized
                assert!(out.contains("Dry run") || out.contains("sanitized"));
            }
            Err(_) => {
                // If it fails, that's also acceptable
            }
        }
    }
}

#[test]
fn test_cli_framework_validation() {
    let project = TestProject::new().unwrap();

    // Test invalid framework
    let output = project.run_devora(&[
        "new",
        "test-project",
        "cpp",
        "--framework", "invalid-framework",
        "--dry-run"
    ]).unwrap();

    assert!(!output.success());
    assert!(output.contains("not found") || output.contains("invalid") || output.contains("error"));
}

#[test]
fn test_cli_default_framework() {
    let project = TestProject::new().unwrap();

    // Test without specifying framework (should use default)
    let output = project.run_devora(&[
        "new",
        "test-project",
        "cpp",
        "--non-interactive"
    ]).unwrap();

    assert!(output.success());

    let project_path = project.join("test-project");
    // Should use the default framework for cpp (which is cmake)
    assert_file_exists(&project_path.join("CMakeLists.txt"));
}

#[test]
fn test_cli_conditional_variables() {
    let project = TestProject::new().unwrap();

    // Test conditional file inclusion
    let output = project.run_devora(&[
        "new",
        "test-conditional",
        "cpp",
        "--framework", "cmake",
        "--var", "include_examples=true",
        "--var", "include_tests=false",
        "--non-interactive"
    ]).unwrap();

    assert!(output.success());

    let project_path = project.join("test-conditional");
    assert!(project_path.join("examples").exists());
    assert!(!project_path.join("tests").exists());
}

#[test]
fn test_cli_error_messages() {
    let project = TestProject::new().unwrap();

    // Test various error conditions and ensure helpful error messages
    let test_cases = vec![
        (vec!["info", "invalid-lang"], "not found"),
        (vec!["info", "cpp", "invalid-framework"], "not found"),
        (vec!["list", "invalid-lang"], "not found"),
    ];

    for (args, expected_msg) in test_cases {
        let output = project.run_devora(&args).unwrap();
        assert!(!output.success());
        assert!(output.contains(expected_msg) || output.contains("Error"));
    }
}