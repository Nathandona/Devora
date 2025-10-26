//! Dependency management and validation for Devora

use crate::config::DevoraConfig;
use crate::result::{Result, DevoraError};
use crate::utils::{execute_command, command_exists};
use console::style;
use std::collections::HashMap;

/// Information about a dependency
#[derive(Debug, Clone)]
pub struct DependencyInfo {
    pub name: String,
    pub description: String,
    pub install_methods: Vec<InstallMethod>,
    pub package_names: HashMap<String, String>, // package manager -> package name
    pub optional: bool,
}

/// Installation method for a dependency
#[derive(Debug, Clone)]
pub struct InstallMethod {
    pub package_manager: String,
    pub command: String,
    pub description: String,
}

/// Common dependency definitions
pub fn get_known_dependencies() -> HashMap<String, DependencyInfo> {
    let mut deps = HashMap::new();

    // Catch2 testing framework
    let mut catch2_package_names = HashMap::new();
    catch2_package_names.insert("apt".to_string(), "catch2-dev".to_string());
    catch2_package_names.insert("brew".to_string(), "catch2".to_string());
    catch2_package_names.insert("vcpkg".to_string(), "catch2".to_string());

    deps.insert("catch2".to_string(), DependencyInfo {
        name: "Catch2".to_string(),
        description: "Modern C++ test framework".to_string(),
        install_methods: vec![
            InstallMethod {
                package_manager: "apt".to_string(),
                command: "sudo apt install catch2-dev".to_string(),
                description: "Debian/Ubuntu: apt install catch2-dev".to_string(),
            },
            InstallMethod {
                package_manager: "brew".to_string(),
                command: "brew install catch2".to_string(),
                description: "macOS: brew install catch2".to_string(),
            },
            InstallMethod {
                package_manager: "vcpkg".to_string(),
                command: "vcpkg install catch2".to_string(),
                description: "vcpkg: vcpkg install catch2".to_string(),
            },
            InstallMethod {
                package_manager: "conan".to_string(),
                command: "conan install catch2/2.13.7@".to_string(),
                description: "Conan: conan install catch2/2.13.7@".to_string(),
            },
        ],
        package_names: catch2_package_names,
        optional: false,
    });

  
    // fmt
    let mut fmt_package_names = HashMap::new();
    fmt_package_names.insert("apt".to_string(), "libfmt-dev".to_string());
    fmt_package_names.insert("brew".to_string(), "fmt".to_string());
    fmt_package_names.insert("vcpkg".to_string(), "fmt".to_string());

    deps.insert("fmt".to_string(), DependencyInfo {
        name: "fmt".to_string(),
        description: "Modern C++ formatting library".to_string(),
        install_methods: vec![
            InstallMethod {
                package_manager: "apt".to_string(),
                command: "sudo apt install libfmt-dev".to_string(),
                description: "Debian/Ubuntu: apt install libfmt-dev".to_string(),
            },
            InstallMethod {
                package_manager: "brew".to_string(),
                command: "brew install fmt".to_string(),
                description: "macOS: brew install fmt".to_string(),
            },
            InstallMethod {
                package_manager: "vcpkg".to_string(),
                command: "vcpkg install fmt".to_string(),
                description: "vcpkg: vcpkg install fmt".to_string(),
            },
        ],
        package_names: fmt_package_names,
        optional: false,
    });

    // spdlog
    let mut spdlog_package_names = HashMap::new();
    spdlog_package_names.insert("apt".to_string(), "libspdlog-dev".to_string());
    spdlog_package_names.insert("brew".to_string(), "spdlog".to_string());
    spdlog_package_names.insert("vcpkg".to_string(), "spdlog".to_string());

    deps.insert("spdlog".to_string(), DependencyInfo {
        name: "spdlog".to_string(),
        description: "Fast C++ logging library".to_string(),
        install_methods: vec![
            InstallMethod {
                package_manager: "apt".to_string(),
                command: "sudo apt install libspdlog-dev".to_string(),
                description: "Debian/Ubuntu: apt install libspdlog-dev".to_string(),
            },
            InstallMethod {
                package_manager: "brew".to_string(),
                command: "brew install spdlog".to_string(),
                description: "macOS: brew install spdlog".to_string(),
            },
            InstallMethod {
                package_manager: "vcpkg".to_string(),
                command: "vcpkg install spdlog".to_string(),
                description: "vcpkg: vcpkg install spdlog".to_string(),
            },
        ],
        package_names: spdlog_package_names,
        optional: false,
    });

    deps
}

/// Validate that required dependencies are available
pub async fn validate_dependencies(config: &DevoraConfig) -> Result<()> {
    let known_deps = get_known_dependencies();
    let missing_deps = find_missing_dependencies(config, &known_deps).await?;

    if missing_deps.is_empty() {
        log::debug!("All dependencies validated successfully");
        return Ok(());
    }

    // Print helpful error message with installation suggestions
    print_dependency_suggestions(&missing_deps, &known_deps);

    Err(DevoraError::dependency(format!(
        "Missing {} required dependenc{}. Please install them and try again.",
        missing_deps.len(),
        if missing_deps.len() == 1 { "y" } else { "ies" }
    )))
}

/// Find missing dependencies by checking if they're available to Meson
async fn find_missing_dependencies(
    config: &DevoraConfig,
    known_deps: &HashMap<String, DependencyInfo>
) -> Result<Vec<String>> {
    let mut missing = Vec::new();

    // Check test framework dependency
    if config.test.framework != "none" {
        let test_dep_name = config.test.framework.as_str();
        if let Some(_dep_info) = known_deps.get(test_dep_name) {
            if !is_dependency_available(test_dep_name, config).await? {
                missing.push(test_dep_name.to_string());
            }
        }
    }

    // Check explicit dependencies from config
    for dep in &config.build.dependencies {
        if let Some(_dep_info) = known_deps.get(&dep.name) {
            if !is_dependency_available(&dep.name, config).await? {
                missing.push(dep.name.clone());
            }
        }
    }

    Ok(missing)
}

/// Check if a dependency is available to Meson
async fn is_dependency_available(dep_name: &str, _config: &DevoraConfig) -> Result<bool> {
    // Create a temporary meson.build to test dependency availability
    let temp_dir = std::env::temp_dir().join("devora_dep_check");
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| DevoraError::filesystem(format!("Failed to create temp dir: {}", e)))?;

    let meson_build_content = format!(
        "project('dep_check', 'cpp')\n{} = dependency('{}', required: false)\n",
        dep_name.replace("-", "_"), // Meson uses underscores
        dep_name
    );

    let meson_build_path = temp_dir.join("meson.build");
    std::fs::write(&meson_build_path, meson_build_content)
        .map_err(|e| DevoraError::filesystem(format!("Failed to write temp meson.build: {}", e)))?;

    // Try to configure the project to check dependency
    let build_dir = temp_dir.join("build");
    let args = vec!["setup", build_dir.to_str().unwrap(), "--wipe"];

    let output = execute_command("meson", &args, Some(&temp_dir)).await;

    // Clean up
    let _ = std::fs::remove_dir_all(&temp_dir);

    match output {
        Ok(result) => {
            // Check if meson succeeded
            if result.status.success() {
                Ok(true)
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                // Check if the error is about the missing dependency
                if stderr.contains(&format!("{} dependency", dep_name)) ||
                   stderr.contains(&format!("'{}' not found", dep_name)) {
                    Ok(false)
                } else {
                    // Some other error, assume dependency is available
                    log::debug!("Meson error while checking {}: {}", dep_name, stderr);
                    Ok(true)
                }
            }
        }
        Err(_) => {
            // Command failed, assume dependency is not available
            Ok(false)
        }
    }
}

/// Print helpful installation suggestions for missing dependencies
fn print_dependency_suggestions(missing_deps: &[String], known_deps: &HashMap<String, DependencyInfo>) {
    println!("\n{}", style("❌ Missing Dependencies").red().bold());

    for dep_name in missing_deps {
        if let Some(dep_info) = known_deps.get(dep_name) {
            println!("\n📦 {} - {}", style(dep_info.name.clone()).yellow().bold(), dep_info.description);

            // Detect available package managers
            let available_methods = detect_available_package_managers(&dep_info.install_methods);

            if available_methods.is_empty() {
                println!("  {} {}", style("⚠️").yellow(), style("No supported package manager detected").yellow());
                println!("  Please install {} manually or add it to your package manager.", dep_info.name);
            } else {
                println!("  {}", style("💡 Installation options:").green());
                for method in &available_methods {
                    println!("    • {}", method.description);
                    if !method.command.is_empty() {
                        println!("      {}", style(&method.command).dim());
                    }
                }
            }
        }
    }

    println!("\n{}", style("💡 Tip:").cyan());
    println!("  You can also configure a different package manager in devora.toml:");
    println!("  [build]");
    println!("  package_manager = \"vcpkg\"  # or \"conan\"");
}

/// Detect which package managers are available on the system
fn detect_available_package_managers(methods: &[InstallMethod]) -> Vec<InstallMethod> {
    let mut available = Vec::new();

    for method in methods {
        if command_exists(&method.package_manager) {
            available.push(method.clone());
        }
    }

    available
}

/// Suggest dependency installation based on the current system
pub fn suggest_dependency_installation(dep_name: &str) -> Option<String> {
    let known_deps = get_known_dependencies();

    if let Some(dep_info) = known_deps.get(dep_name) {
        let available_methods = detect_available_package_managers(&dep_info.install_methods);

        if let Some(method) = available_methods.first() {
            Some(format!("{}: {}", dep_info.name, method.description))
        } else {
            Some(format!("{}: No supported package manager detected. Install manually.", dep_info.name))
        }
    } else {
        None
    }
}

/// Command to check dependencies and provide suggestions
pub async fn check_command() -> Result<()> {
    use crate::config::DevoraConfig;
    use console::style;

    println!("\n{}", style("🔍 Checking Dependencies").blue().bold());

    // Load configuration if available
    let config = DevoraConfig::find_and_load()?;

    if let Some(config) = config {
        let project_name = config.project.name.clone();
        println!("\n📁 Project: {}", style(project_name).green());

        // Check all configured dependencies
        let known_deps = get_known_dependencies();
        let mut missing_deps = Vec::new();
        let mut found_deps = Vec::new();

        // Check test framework
        if config.test.framework != "none" {
            if known_deps.contains_key(&config.test.framework) {
                match is_dependency_available(&config.test.framework, &config).await {
                    Ok(true) => found_deps.push(config.test.framework.clone()),
                    Ok(false) => missing_deps.push(config.test.framework.clone()),
                    Err(_) => missing_deps.push(config.test.framework.clone()),
                }
            }
        }

        // Check explicit dependencies
        for dep in &config.build.dependencies {
            if known_deps.contains_key(&dep.name) {
                match is_dependency_available(&dep.name, &config).await {
                    Ok(true) => found_deps.push(dep.name.clone()),
                    Ok(false) => missing_deps.push(dep.name.clone()),
                    Err(_) => missing_deps.push(dep.name.clone()),
                }
            }
        }

        // Display results
        if found_deps.is_empty() && missing_deps.is_empty() {
            println!("\n{}", style("ℹ️  No dependencies configured in devora.toml").yellow());
        } else {
            if !found_deps.is_empty() {
                println!("\n{}", style("✅ Found Dependencies:").green().bold());
                for dep in &found_deps {
                    println!("  ✓ {}", style(dep).green());
                }
            }

            if !missing_deps.is_empty() {
                print_dependency_suggestions(&missing_deps, &known_deps);
            }
        }
    } else {
        println!("\n{}", style("ℹ️  No devora.toml found. Not in a Devora project.").yellow());
        println!("Run this command from a project directory to check project-specific dependencies.");
    }

    // Check system dependencies
    println!("\n{}", style("🛠️  System Tools:").blue().bold());
    let system_tools = ["meson", "ninja", "gcc", "g++", "clang", "clang++"];

    for tool in system_tools.iter() {
        if command_exists(tool) {
            println!("  ✓ {}", style(tool).green());
        } else {
            println!("  ✗ {}", style(tool).red());
        }
    }

    // Check package managers
    println!("\n{}", style("📦 Package Managers:").blue().bold());
    let package_managers = ["apt", "brew", "vcpkg", "conan"];

    for pm in package_managers.iter() {
        if command_exists(pm) {
            println!("  ✓ {}", style(pm).green());
        } else {
            println!("  ✗ {}", style(pm).red());
        }
    }

    println!("\n{}", style("💡 Tip:").cyan());
    println!("  Use 'devora check' to verify dependencies before building.");
    println!("  Add dependencies to your devora.toml under [build] dependencies section.");

    Ok(())
}