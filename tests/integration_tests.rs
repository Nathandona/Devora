//! End-to-end generation tests against the compiled binary.

mod common;
use common::*;

#[test]
fn generates_rust_project_structure() {
    let project = TestProject::new();
    let out = project.run(&["new", "demo", "rust", "--non-interactive", "--no-hooks"]);
    assert!(out.success(), "stderr: {}", out.stderr);

    let app = project.join("demo");
    assert_file_exists(&app.join("Cargo.toml"));
    assert_file_exists(&app.join("README.md"));
    assert_file_exists(&app.join(".gitignore"));
    assert_file_exists(&app.join("src/main.rs"));

    let cargo = read_to_string(&app.join("Cargo.toml"));
    assert!(cargo.contains("name = \"demo\""));
}

#[test]
fn include_tests_true_adds_test_module() {
    let project = TestProject::new();
    let out = project.run(&[
        "new",
        "with-tests",
        "rust",
        "--var",
        "include_tests=true",
        "--non-interactive",
        "--no-hooks",
    ]);
    assert!(out.success(), "stderr: {}", out.stderr);

    let main = read_to_string(&project.join("with-tests/src/main.rs"));
    assert!(main.contains("#[cfg(test)]"));
}

#[test]
fn include_tests_false_omits_test_module() {
    let project = TestProject::new();
    let out = project.run(&[
        "new",
        "no-tests",
        "rust",
        "--var",
        "include_tests=false",
        "--non-interactive",
        "--no-hooks",
    ]);
    assert!(out.success(), "stderr: {}", out.stderr);

    let main = read_to_string(&project.join("no-tests/src/main.rs"));
    assert!(!main.contains("#[cfg(test)]"));
}

#[test]
fn refuses_to_overwrite_existing_directory() {
    let project = TestProject::new();
    let first = project.run(&["new", "dup", "rust", "--non-interactive", "--no-hooks"]);
    assert!(first.success());
    let second = project.run(&["new", "dup", "rust", "--non-interactive", "--no-hooks"]);
    assert!(!second.success());
    assert!(second.contains("already exists"));
}

#[test]
fn dry_run_previews_without_writing() {
    let project = TestProject::new();
    let out = project.run(&["new", "preview", "rust", "--non-interactive", "--dry-run"]);
    assert!(out.success());
    assert!(out.contains("Would create"));
    assert!(!project.join("preview").exists());
}

#[test]
fn default_framework_is_used_when_omitted() {
    let project = TestProject::new();
    let out = project.run(&[
        "new",
        "defaulted",
        "rust",
        "--non-interactive",
        "--no-hooks",
    ]);
    assert!(out.success(), "stderr: {}", out.stderr);
    assert_file_exists(&project.join("defaulted/Cargo.toml"));
}

#[test]
fn generates_cpp_cmake_project_structure() {
    let project = TestProject::new();
    let out = project.run(&[
        "new",
        "cpp-demo",
        "cpp",
        "--var",
        "cpp_standard=20",
        "--non-interactive",
        "--no-hooks",
    ]);
    assert!(out.success(), "stderr: {}", out.stderr);

    let app = project.join("cpp-demo");
    assert_file_exists(&app.join("CMakeLists.txt"));
    assert_file_exists(&app.join("README.md"));
    assert_file_exists(&app.join(".gitignore"));
    assert_file_exists(&app.join("src/main.cpp"));
    // doctest is vendored so the suite builds offline.
    assert_file_exists(&app.join("tests/test_main.cpp"));
    assert_file_exists(&app.join("tests/doctest.h"));

    let cmake = read_to_string(&app.join("CMakeLists.txt"));
    assert!(cmake.contains("project(cpp-demo"));
    assert!(cmake.contains("set(CMAKE_CXX_STANDARD 20)"));
    assert!(cmake.contains("add_executable(cpp-demo src/main.cpp)"));
}

#[test]
fn cpp_include_tests_false_omits_test_files() {
    let project = TestProject::new();
    let out = project.run(&[
        "new",
        "cpp-no-tests",
        "cpp",
        "--var",
        "include_tests=false",
        "--non-interactive",
        "--no-hooks",
    ]);
    assert!(out.success(), "stderr: {}", out.stderr);

    let app = project.join("cpp-no-tests");
    assert_file_exists(&app.join("src/main.cpp"));
    assert!(!app.join("tests/test_main.cpp").exists());
    assert!(!app.join("tests/doctest.h").exists());

    let cmake = read_to_string(&app.join("CMakeLists.txt"));
    assert!(!cmake.contains("enable_testing()"));
}

#[test]
fn cpp_uses_cmake_as_default_framework() {
    let project = TestProject::new();
    let out = project.run(&["new", "cpp-default", "cpp", "--non-interactive", "--no-hooks"]);
    assert!(out.success(), "stderr: {}", out.stderr);
    assert_file_exists(&project.join("cpp-default/CMakeLists.txt"));
}

#[test]
fn invalid_language_reports_not_found() {
    let out = run_devora(&["info", "klingon"]);
    assert!(!out.success());
    assert!(out.contains("not found"));
}

#[test]
fn generates_multiple_projects() {
    let project = TestProject::new();
    for i in 0..3 {
        let name = format!("proj-{}", i);
        let out = project.run(&["new", &name, "rust", "--non-interactive", "--no-hooks"]);
        assert!(out.success(), "stderr: {}", out.stderr);
        assert_file_exists(&project.join(&format!("{}/src/main.rs", name)));
    }
}

#[test]
fn generates_go_module_structure() {
    let project = TestProject::new();
    let out = project.run(&["new", "go-demo", "go", "--non-interactive", "--no-hooks"]);
    assert!(out.success(), "stderr: {}", out.stderr);

    let app = project.join("go-demo");
    assert_file_exists(&app.join("go.mod"));
    assert_file_exists(&app.join("main.go"));
    assert_file_exists(&app.join("main_test.go"));
    assert_file_exists(&app.join("README.md"));

    let gomod = read_to_string(&app.join("go.mod"));
    assert!(gomod.contains("module go-demo"));
}

#[test]
fn generates_python_src_layout() {
    let project = TestProject::new();
    let out = project.run(&["new", "py-demo", "python", "--non-interactive", "--no-hooks"]);
    assert!(out.success(), "stderr: {}", out.stderr);

    let app = project.join("py-demo");
    assert_file_exists(&app.join("pyproject.toml"));
    // project_slug "py-demo" becomes the underscore module "py_demo".
    assert_file_exists(&app.join("src/py_demo/__init__.py"));
    assert_file_exists(&app.join("src/py_demo/__main__.py"));
    assert_file_exists(&app.join("tests/test_main.py"));

    let pyproject = read_to_string(&app.join("pyproject.toml"));
    assert!(pyproject.contains("name = \"py-demo\""));
}

#[test]
fn generates_csharp_console_and_test_project() {
    let project = TestProject::new();
    let out = project.run(&["new", "cs-demo", "csharp", "--non-interactive", "--no-hooks"]);
    assert!(out.success(), "stderr: {}", out.stderr);

    let app = project.join("cs-demo");
    assert_file_exists(&app.join("src/cs-demo/cs-demo.csproj"));
    assert_file_exists(&app.join("src/cs-demo/Program.cs"));
    assert_file_exists(&app.join("tests/cs-demo.Tests/cs-demo.Tests.csproj"));
    assert_file_exists(&app.join("tests/cs-demo.Tests/GreeterTests.cs"));

    let csproj = read_to_string(&app.join("src/cs-demo/cs-demo.csproj"));
    assert!(csproj.contains("<TargetFramework>net8.0</TargetFramework>"));
}

#[test]
fn csharp_include_tests_false_omits_test_project() {
    let project = TestProject::new();
    let out = project.run(&[
        "new",
        "cs-no-tests",
        "csharp",
        "--var",
        "include_tests=false",
        "--non-interactive",
        "--no-hooks",
    ]);
    assert!(out.success(), "stderr: {}", out.stderr);

    let app = project.join("cs-no-tests");
    assert_file_exists(&app.join("src/cs-no-tests/Program.cs"));
    assert!(!app.join("tests/cs-no-tests.Tests").exists());
}
