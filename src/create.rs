//! Project creation functionality

use std::path::{Path, PathBuf};
use dialoguer::{Confirm, Input, Select};
use console::style;
use crate::config::{DevoraConfig, ProjectConfig, BuildConfig, DevConfig, TestConfig, LintConfig};
use crate::template::TemplateEngine;
use crate::utils::ensure_dir_exists;
use crate::result::{Result, DevoraError};

pub async fn run(
    name: &str,
    dir: Option<&str>,
    cpp_std: &str,
    test_framework: &str,
    package_manager: &str,
) -> Result<()> {
    log::info!("Creating new C++ project: {}", name);

    // Validate project name
    if !is_valid_project_name(name) {
        return Err(DevoraError::config(format!("Invalid project name: '{}'. Project names should contain only letters, numbers, and hyphens.", name)));
    }

    // Determine project directory
    let project_dir = PathBuf::from(dir.unwrap_or(name));

    // Check if directory already exists
    if project_dir.exists() {
        if !Confirm::new()
            .with_prompt(format!("Directory '{}' already exists. Continue and overwrite?", project_dir.display()))
            .default(false)
            .interact()?
        {
            return Ok(());
        }
    }

    // Interactive configuration
    let cpp_std = if cpp_std == "prompt" {
        select_cpp_standard()?
    } else {
        cpp_std.to_string()
    };

    let test_framework = if test_framework == "prompt" {
        select_test_framework()?
    } else {
        test_framework.to_string()
    };

    let package_manager = if package_manager == "prompt" {
        select_package_manager()?
    } else {
        package_manager.to_string()
    };

    let description = Input::<String>::new()
        .with_prompt("Project description")
        .allow_empty(true)
        .interact()?;

    let author = Input::<String>::new()
        .with_prompt("Author name")
        .default(get_git_user_name().unwrap_or_default())
        .interact()?;

    let license = select_license()?;

    // Create configuration
    let config = create_config(name, &cpp_std, &test_framework, &package_manager, &description, &author, &license)?;

    // Create project
    create_project(&project_dir, &config).await?;

    println!("\n{}", style("✨ Project created successfully!").green().bold());
    println!("  Directory: {}", project_dir.display());
    println!("\nNext steps:");
    println!("  cd {}", project_dir.display());
    println!("  devora dev    # Start development server");
    println!("  devora test   # Run tests");
    println!("  devora build  # Build project");

    Ok(())
}

fn is_valid_project_name(name: &str) -> bool {
    !name.is_empty() &&
    name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') &&
    !name.starts_with('-') &&
    !name.ends_with('-')
}

fn select_cpp_standard() -> Result<String> {
    let items = vec!["C++17", "C++20", "C++23"];
    let selection = Select::new()
        .with_prompt("Select C++ standard")
        .items(&items)
        .default(1) // C++20
        .interact()?;

    Ok(items[selection].replace("C++", ""))
}

fn select_test_framework() -> Result<String> {
    let items = vec!["Catch2", "Google Test", "doctest", "None"];
    let selection = Select::new()
        .with_prompt("Select testing framework")
        .items(&items)
        .default(0) // Catch2
        .interact()?;

    match selection {
        0 => Ok("catch2".to_string()),
        1 => Ok("gtest".to_string()),
        2 => Ok("doctest".to_string()),
        3 => Ok("none".to_string()),
        _ => Ok("catch2".to_string()),
    }
}

fn select_package_manager() -> Result<String> {
    let items = vec!["vcpkg", "conan", "None"];
    let selection = Select::new()
        .with_prompt("Select package manager")
        .items(&items)
        .default(0) // vcpkg
        .interact()?;

    match selection {
        0 => Ok("vcpkg".to_string()),
        1 => Ok("conan".to_string()),
        2 => Ok("none".to_string()),
        _ => Ok("vcpkg".to_string()),
    }
}

fn select_license() -> Result<String> {
    let items = vec!["MIT", "Apache-2.0", "GPL-3.0", "BSD-3-Clause", "None"];
    let selection = Select::new()
        .with_prompt("Select license")
        .items(&items)
        .default(0) // MIT
        .interact()?;

    match selection {
        4 => Ok(String::new()),
        _ => Ok(items[selection].to_string()),
    }
}

fn get_git_user_name() -> Option<String> {
    std::process::Command::new("git")
        .args(&["config", "user.name"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
}

fn create_config(
    name: &str,
    cpp_std: &str,
    test_framework: &str,
    package_manager: &str,
    description: &str,
    author: &str,
    license: &str,
) -> Result<DevoraConfig> {
    let package_manager = if package_manager == "none" { None } else { Some(package_manager.to_string()) };

    Ok(DevoraConfig {
        project: ProjectConfig {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            cpp_standard: cpp_std.to_string(),
            description: if description.is_empty() { None } else { Some(description.to_string()) },
            authors: if author.is_empty() { vec![] } else { vec![author.to_string()] },
            license: if license.is_empty() { None } else { Some(license.to_string()) },
        },
        build: BuildConfig {
            build_system: "meson".to_string(),
            build_dir: "build".to_string(),
            target_dir: "target".to_string(),
            package_manager,
            dependencies: vec![],
        },
        dev: DevConfig::default(),
        test: TestConfig {
            framework: test_framework.to_string(),
            ..Default::default()
        },
        lint: LintConfig::default(),
    })
}

async fn create_project(project_dir: &Path, config: &DevoraConfig) -> Result<()> {
    // Create project directory structure
    ensure_dir_exists(project_dir)?;
    ensure_dir_exists(&project_dir.join("src"))?;
    ensure_dir_exists(&project_dir.join("tests"))?;

    // Initialize template engine
    let template_engine = TemplateEngine::new()?;
    let context = TemplateEngine::create_context(config);

    // Generate files
    generate_file(&template_engine, "meson.build", &project_dir.join("meson.build"), &context)?;
    generate_file(&template_engine, "main.cpp", &project_dir.join("src/main.cpp"), &context)?;

    if config.test.framework != "none" {
        generate_file(&template_engine, "test_main.cpp", &project_dir.join("tests/test_main.cpp"), &context)?;
    }

    generate_file(&template_engine, "devora.toml", &project_dir.join("devora.toml"), &context)?;
    generate_file(&template_engine, "README.md", &project_dir.join("README.md"), &context)?;

    // Create .gitignore
    create_gitignore(project_dir)?;

    Ok(())
}

fn generate_file(
    template_engine: &TemplateEngine,
    template_name: &str,
    output_path: &Path,
    context: &tera::Context,
) -> Result<()> {
    let content = template_engine.render(template_name, context)?;
    std::fs::write(output_path, content)
        .map_err(|e| DevoraError::filesystem(format!("Failed to write {}: {}", output_path.display(), e)))?;

    log::debug!("Generated file: {}", output_path.display());
    Ok(())
}

fn create_gitignore(project_dir: &Path) -> Result<()> {
    let gitignore_content = r#"# Build directories
build/
builddir/
target/
out/

# Compiled output
*.o
*.obj
*.exe
*.dll
*.so
*.dylib
*.a
*.lib

# IDE files
.vscode/
.idea/
*.swp
*.swo
*~

# OS files
.DS_Store
Thumbs.db

# Logs
*.log

# Package manager files
vcpkg_installed/
conanbuildinfo.*
CMakeLists.txt.user"#;

    std::fs::write(project_dir.join(".gitignore"), gitignore_content)
        .map_err(|e| DevoraError::filesystem(format!("Failed to create .gitignore: {}", e)))?;

    log::debug!("Created .gitignore");
    Ok(())
}