//! Utility functions

use std::path::{Path, PathBuf};
use std::process::Output;
use crate::result::{Result, DevoraError};
use tokio::process::Command;

/// Check if a command exists in the system PATH
pub fn command_exists(command: &str) -> bool {
    which::which(command).is_ok()
}

/// Ensure a directory exists, creating it if necessary
pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)
            .map_err(|e| DevoraError::filesystem(format!("Failed to create directory {}: {}", path.display(), e)))?;
    }
    Ok(())
}

/// Get the current working directory as a String
pub fn current_dir() -> Result<String> {
    std::env::current_dir()
        .map_err(|e| DevoraError::filesystem(format!("Failed to get current directory: {}", e)))?
        .to_str()
        .ok_or_else(|| DevoraError::filesystem("Current directory is not valid UTF-8".to_string()))
        .map(|s| s.to_string())
}

/// Execute a command and return the output
pub async fn execute_command(cmd: &str, args: &[&str], cwd: Option<&Path>) -> Result<Output> {
    let mut command = Command::new(cmd);
    command.args(args);

    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }

    let output = command
        .output()
        .await
        .map_err(|e| DevoraError::process(format!("Failed to execute '{} {}': {}", cmd, args.join(" "), e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(DevoraError::process(format!(
            "Command '{}' failed with exit code: {}\nstdout: {}\nstderr: {}",
            cmd,
            output.status.code().unwrap_or(-1),
            stdout,
            stderr
        )));
    }

    Ok(output)
}

/// Find project root by looking for meson.build or devora.toml
pub fn find_project_root() -> Result<Option<PathBuf>> {
    let current_dir = std::env::current_dir()
        .map_err(|e| DevoraError::filesystem(format!("Failed to get current directory: {}", e)))?;

    for dir in current_dir.ancestors() {
        if dir.join("meson.build").exists() || dir.join("devora.toml").exists() {
            return Ok(Some(dir.to_path_buf()));
        }
    }

    Ok(None)
}

/// Check if Meson build directory is configured
pub fn is_meson_configured(project_dir: &Path) -> bool {
    let build_dir = project_dir.join("builddir");
    build_dir.exists() && build_dir.join("build.ninja").exists()
}

/// Get Meson build directory path
pub fn get_meson_build_dir(project_dir: &Path, build_type: &str) -> PathBuf {
    match build_type {
        "release" => project_dir.join("builddir-release"),
        _ => project_dir.join("builddir"),
    }
}

/// Get the output path for the built executable
pub fn get_output_path(project_dir: &Path, build_type: &str) -> Option<PathBuf> {
    let build_dir = get_meson_build_dir(project_dir, build_type);

    // Try to find the executable in common locations
    let common_paths = [
        build_dir.join("main"),  // Linux/macOS
        build_dir.join("main.exe"),  // Windows
        build_dir.join("src").join("main"),  // Nested structure
        build_dir.join("src").join("main.exe"),
    ];

    for path in common_paths.iter() {
        if path.exists() {
            return Some(path.clone());
        }
    }

    None
}

/// Detect available C++ compilers
pub fn detect_cpp_compiler() -> Result<String> {
    // Check for common compilers in order of preference
    let compilers = ["clang++", "g++", "cl"];

    for compiler in compilers {
        if command_exists(compiler) {
            log::debug!("Found C++ compiler: {}", compiler);
            return Ok(compiler.to_string());
        }
    }

    Err(DevoraError::build(
        "No C++ compiler found. Please install clang++, g++, or MSVC".to_string()
    ))
}