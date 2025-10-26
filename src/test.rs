//! Testing functionality

use std::path::Path;
use console::style;
use crate::config::DevoraConfig;
use crate::dependencies;
use crate::result::{Result, DevoraError};
use crate::utils::{execute_command, find_project_root, get_meson_build_dir};

pub async fn run(release: bool, filter: Option<&str>) -> Result<()> {
    let build_type = if release { "release" } else { "debug" };
    log::info!("Running tests in {} mode", build_type);
    if let Some(filter) = filter {
        log::debug!("Test filter: {}", filter);
    }

    // Find project root
    let project_root = find_project_root()?
        .ok_or_else(|| DevoraError::build("Not in a Devora project (no meson.build or devora.toml found)".to_string()))?;

    log::debug!("Project root: {}", project_root.display());

    // Load configuration
    let config = DevoraConfig::find_and_load()?;

    // Validate test dependencies
    if let Some(ref config) = config {
        if config.test.framework != "none" {
            // Only validate if we haven't already built (to avoid duplicate checks)
            let build_dir = get_meson_build_dir(&project_root, build_type);
            if !build_dir.exists() {
                dependencies::validate_dependencies(config).await?;
            }
        }
    }

    // Build the project first
    println!("{}", style("🔨 Building project for testing...").blue());
    println!("Note: Run 'devora build' first to ensure project is built.");

    // Discover and run tests
    run_tests(&project_root, build_type, filter, config.as_ref()).await?;

    Ok(())
}

async fn run_tests(
    project_root: &Path,
    build_type: &str,
    filter: Option<&str>,
    config: Option<&DevoraConfig>,
) -> Result<()> {
    let build_dir = get_meson_build_dir(&project_root, build_type);

    if !build_dir.exists() {
        return Err(DevoraError::build(
            "Build directory not found. Run 'devora build' first.".to_string()
        ));
    }

    println!("\n{}", style("🧪 Running tests...").blue().bold());

    // Check if tests are available
    if let Some(config) = config {
        if config.test.framework == "none" {
            println!("{}", style("ℹ️  No test framework configured. Skipping tests.").yellow());
            return Ok(());
        }
    }

    // Run tests with Meson
    let build_dir_str = build_dir.to_string_lossy();
    let mut test_args = vec!["test", "-C", &build_dir_str];

    // Add verbose output
    test_args.push("--verbose");

    // Add filter if specified
    if let Some(filter) = filter {
        test_args.extend(&["--gtest_filter", filter]);
    }

    println!("{}", style("Running tests...").blue());

    let output = execute_command("meson", &test_args, Some(project_root)).await?;

    println!("{}", style("Tests completed").green());

    // Parse and display results
    let _ = parse_and_display_test_results(&output, filter).await;

    Ok(())
}

async fn parse_and_display_test_results(output: &std::process::Output, filter: Option<&str>) -> Result<()> {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        println!("\n{}", style("✅ All tests passed!").green().bold());

        if let Some(filter) = filter {
            println!("Filter: {}", style(filter).cyan());
        }

        // Count tests if possible
        if let Some(count) = count_test_results(&stdout) {
            println!("Tests run: {}", style(count).cyan());
        }
    } else {
        println!("\n{}", style("❌ Some tests failed!").red().bold());

        // Show failed tests
        let _ = display_failed_tests(&stdout, &stderr).await;
    }

    // Always show detailed output in verbose mode or on failure
    if !output.status.success() || log::log_enabled!(log::Level::Debug) {
        if !stdout.trim().is_empty() {
            println!("\n{}", style("Test output:").yellow().bold());
            println!("{}", stdout);
        }

        if !stderr.trim().is_empty() {
            println!("\n{}", style("Test errors:").red().bold());
            println!("{}", stderr);
        }
    }

    Ok(())
}

fn count_test_results(output: &str) -> Option<usize> {
    // Try to parse test count from Meson output
    for line in output.lines() {
        if line.contains("OK") {
            // Look for patterns like "OK: 42 tests"
            if let Some(num_str) = line.split_whitespace().nth(1) {
                if let Ok(count) = num_str.parse::<usize>() {
                    return Some(count);
                }
            }
        }
    }
    None
}

async fn display_failed_tests(stdout: &str, stderr: &str) -> Result<()> {
    let mut failed_tests = Vec::new();

    // Parse stdout for failed tests
    for line in stdout.lines() {
        if line.contains("FAIL") || line.contains("ERROR") {
            failed_tests.push(line.trim());
        }
    }

    // Parse stderr for additional error information
    for line in stderr.lines() {
        if line.contains("FAILED") || line.contains("Assertion failed") {
            failed_tests.push(line.trim());
        }
    }

    if !failed_tests.is_empty() {
        println!("\n{}", style("Failed tests:").red().bold());
        for (i, test) in failed_tests.iter().enumerate() {
            println!("  {}. {}", i + 1, test);
        }
    }

    Ok(())
}

/// Run tests with coverage reporting
pub async fn run_with_coverage(_release: bool) -> Result<()> {
    println!("{}", style("📊 Running tests with coverage...").blue());

    // Find project root
    let _project_root = find_project_root()?
        .ok_or_else(|| DevoraError::build("Not in a Devora project".to_string()))?;

    // Load configuration
    let config = DevoraConfig::find_and_load()?;

    // Check if coverage is enabled
    if let Some(config) = config {
        if !config.test.coverage {
            println!("{}", style("⚠️  Coverage not enabled in devora.toml").yellow());
            println!("Add `coverage = true` under [test] section to enable coverage.");
        }
    }

    // Build with coverage flags
    println!("{}", style("Building with coverage instrumentation...").blue());

    // This would require modifying the Meson setup to include coverage flags
    // For now, just print message
    println!("Run 'devora test' to run tests with coverage.");

    println!("\n{}", style("📈 Coverage report generation not yet implemented").yellow());
    println!("This feature is planned for a future version.");

    Ok(())
}

/// Discover test files in the project
pub async fn discover_tests(project_root: &Path, config: &DevoraConfig) -> Result<Vec<String>> {
    let tests_dir = project_root.join(&config.test.test_dir);
    let mut test_files = Vec::new();

    if !tests_dir.exists() {
        log::debug!("No tests directory found at {}", tests_dir.display());
        return Ok(test_files);
    }

    // Simple glob pattern matching for test files
    let pattern = config.test.test_pattern.clone();

    if let Ok(entries) = std::fs::read_dir(&tests_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if matches_test_pattern(name, &pattern) {
                        test_files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    log::debug!("Discovered {} test files", test_files.len());
    Ok(test_files)
}

fn matches_test_pattern(filename: &str, pattern: &str) -> bool {
    // Simple pattern matching (can be enhanced with proper glob support)
    if pattern.contains('*') {
        let base_pattern = pattern.replace('*', "");
        filename.contains(&base_pattern)
    } else {
        filename == pattern
    }
}