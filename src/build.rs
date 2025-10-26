//! Build functionality

use std::path::Path;
use console::style;
use crate::config::DevoraConfig;
use crate::dependencies;
use crate::result::{Result, DevoraError};
use crate::utils::{command_exists, execute_command, find_project_root, is_meson_configured, get_meson_build_dir, get_output_path, detect_cpp_compiler};

pub async fn run(release: bool, target_dir: &str) -> Result<()> {
    let build_type = if release { "release" } else { "debug" };
    log::info!("Building project in {} mode", build_type);

    // Find project root
    let project_root = find_project_root()?
        .ok_or_else(|| DevoraError::build("Not in a Devora project (no meson.build or devora.toml found)".to_string()))?;

    log::debug!("Project root: {}", project_root.display());

    // Load configuration if available
    let config = DevoraConfig::find_and_load()?;

    // Validate build environment
    validate_build_environment().await?;

    // Validate dependencies if configuration is available
    if let Some(ref config) = config {
        dependencies::validate_dependencies(config).await?;
    }

    // Setup Meson if needed
    if !is_meson_configured(&project_root) {
        setup_meson(&project_root, build_type, config.as_ref()).await?;
    }

    // Perform the build
    perform_build(&project_root, build_type, target_dir).await?;

    let success_msg = if release {
        "✅ Release build completed successfully!"
    } else {
        "✅ Debug build completed successfully!"
    };

    println!("\n{}", style(success_msg).green().bold());

    if let Some(output_path) = get_output_path(&project_root, build_type) {
        println!("Output: {}", output_path.display());
    }

    Ok(())
}

async fn validate_build_environment() -> Result<()> {
    // Check for required tools
    let required_tools = ["meson", "ninja"];

    for tool in required_tools.iter() {
        if !command_exists(tool) {
            return Err(DevoraError::build(format!(
                "Required tool '{}' not found. Please install {} to continue.",
                tool, tool
            )));
        }
    }

    // Check for C++ compiler
    detect_cpp_compiler()?;

    log::debug!("Build environment validation passed");
    Ok(())
}

async fn setup_meson(project_dir: &Path, build_type: &str, config: Option<&DevoraConfig>) -> Result<()> {
    log::info!("Setting up Meson build system...");

    let build_dir = get_meson_build_dir(project_dir, build_type);
    let build_dir_str = build_dir.to_string_lossy();
    let mut args = vec!["setup", &build_dir_str];

    // Add build type configuration
    if build_type == "release" {
        args.push("--buildtype=release");
    } else {
        args.push("--buildtype=debug");
    }

    // Configure vcpkg if specified
    if let Some(config) = config {
        if let Some(ref pkg_manager) = config.build.package_manager {
            if pkg_manager == "vcpkg" {
                if let Ok(_vcpkg_root) = std::env::var("VCPKG_ROOT") {
                    args.extend(&["--wrap-mode=nofallback"]);
                    std::env::set_var("VCPKG_TARGET_TRIPLET", "x64-linux"); // TODO: Detect platform
                } else {
                    log::warn!("VCPKG_ROOT not set. vcpkg dependencies may not be found.");
                }
            }
        }
    }

    // Configure compiler if auto-detected
    if let Ok(compiler) = detect_cpp_compiler() {
        log::debug!("Using compiler: {}", compiler);
    }

    println!("{}", style("Setting up build directory...").blue());

    let output = execute_command("meson", &args, Some(project_dir)).await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DevoraError::build(format!("Meson setup failed: {}", stderr)));
    }

    log::info!("Meson setup completed successfully");
    Ok(())
}

async fn perform_build(project_dir: &Path, build_type: &str, _target_dir: &str) -> Result<()> {
    let build_dir = get_meson_build_dir(project_dir, build_type);

    log::info!("Starting build...");

    println!("{}", style("Compiling...").blue());

    let build_dir_str = build_dir.to_string_lossy();
    let args = vec!["compile", "-C", &build_dir_str];

    let output = execute_command("meson", &args, Some(project_dir)).await?;

    println!("{}", style("Build completed").green());

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        return Err(DevoraError::build(format!(
            "Build failed:\nstdout: {}\nstderr: {}",
            stdout, stderr
        )));
    }

    log::info!("Build completed successfully");
    Ok(())
}


/// Clean build artifacts
pub async fn clean(release: bool) -> Result<()> {
    let build_type = if release { "release" } else { "debug" };
    log::info!("Cleaning {} build artifacts", build_type);

    let project_root = find_project_root()?
        .ok_or_else(|| DevoraError::build("Not in a Devora project".to_string()))?;

    let build_dir = get_meson_build_dir(&project_root, build_type);

    if build_dir.exists() {
        std::fs::remove_dir_all(&build_dir)
            .map_err(|e| DevoraError::filesystem(format!("Failed to remove build directory {}: {}", build_dir.display(), e)))?;

        println!("{} {}", style("✓").green(), style("Cleaned build directory").dim());
    }

    Ok(())
}

/// Install the project
pub async fn install(release: bool) -> Result<()> {
    let build_type = if release { "release" } else { "debug" };
    log::info!("Installing project ({} build)", build_type);

    let project_root = find_project_root()?
        .ok_or_else(|| DevoraError::build("Not in a Devora project".to_string()))?;

    let build_dir = get_meson_build_dir(&project_root, build_type);

    if !build_dir.exists() {
        return Err(DevoraError::build("Build directory not found. Run 'devora build' first.".to_string()));
    }

    let build_dir_str = build_dir.to_string_lossy();
    let args = vec!["install", "-C", &build_dir_str];

    println!("{}", style("Installing project...").blue());

    let output = execute_command("meson", &args, Some(&project_root)).await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DevoraError::build(format!("Installation failed: {}", stderr)));
    }

    println!("{} {}", style("✓").green(), style("Project installed successfully").green());
    Ok(())
}