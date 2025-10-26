//! Linting functionality

use std::path::{Path, PathBuf};
use console::style;
use crate::config::DevoraConfig;
use crate::result::{Result, DevoraError};
use crate::utils::{command_exists, execute_command, find_project_root};

pub async fn run(fix: bool, path: Option<&str>) -> Result<()> {
    log::info!("Running linting checks");
    if fix {
        log::debug!("Auto-fix enabled");
    }
    if let Some(path) = path {
        log::debug!("Linting path: {}", path);
    }

    // Find project root
    let project_root = find_project_root()?
        .ok_or_else(|| DevoraError::build("Not in a Devora project (no meson.build or devora.toml found)".to_string()))?;

    log::debug!("Project root: {}", project_root.display());

    // Load configuration
    let config = DevoraConfig::find_and_load()?;

    // Check if linting is enabled
    if let Some(ref config) = config {
        if !config.lint.enabled {
            println!("{}", style("ℹ️  Linting is disabled in devora.toml").yellow());
            return Ok(());
        }
    }

    println!("{}", style("🔍 Running linting checks...").blue().bold());

    // Determine what to lint
    let lint_path = if let Some(path) = path {
        project_root.join(path)
    } else {
        project_root.clone()
    };

    // Validate linting environment
    validate_lint_environment().await?;

    // Run clang-tidy for static analysis
    run_clang_tidy(&lint_path, &project_root, fix, config.as_ref()).await?;

    // Run clang-format for code formatting
    run_clang_format(&lint_path, fix, config.as_ref()).await?;

    println!("\n{}", style("✅ Linting completed successfully!").green().bold());

    Ok(())
}

async fn validate_lint_environment() -> Result<()> {
    // Check for clang-tidy
    if !command_exists("clang-tidy") {
        return Err(DevoraError::build(
            "clang-tidy not found. Please install clang-tidy to enable linting functionality.".to_string()
        ));
    }

    // Check for clang-format
    if !command_exists("clang-format") {
        return Err(DevoraError::build(
            "clang-format not found. Please install clang-format to enable code formatting.".to_string()
        ));
    }

    log::debug!("Linting environment validation passed");
    Ok(())
}

async fn run_clang_tidy(
    lint_path: &Path,
    project_root: &Path,
    fix: bool,
    config: Option<&DevoraConfig>,
) -> Result<()> {
    println!("{}", style("Running clang-tidy...").blue());

    // Find C++ source files
    let cpp_files = find_cpp_files(lint_path)?;

    if cpp_files.is_empty() {
        println!("{}", style("No C++ files found for clang-tidy analysis.").yellow());
        return Ok(());
    }

    println!("Analyzing {} C++ files...", cpp_files.len());

    let mut issues_found = 0;

    for file in &cpp_files {

        let mut args: Vec<String> = vec!["--quiet".to_string()];

        if fix {
            args.push("--fix".to_string());
            args.push("--fix-errors".to_string());
        }

        // Add config file if specified
        if let Some(config) = config {
            if let Some(ref config_file) = config.lint.config_file {
                let config_path = project_root.join(config_file);
                if config_path.exists() {
                    args.push("--config-file".to_string());
                    args.push(config_path.to_string_lossy().into_owned());
                }
            }
        }

        // Add the file
        args.push(file.to_string_lossy().into_owned());

        // Run clang-tidy
        match execute_command("clang-tidy", &args.iter().map(|s| s.as_str()).collect::<Vec<_>>(), Some(project_root)).await {
            Ok(_) => {
                log::debug!("clang-tidy passed for {}", file.display());
            }
            Err(e) => {
                issues_found += 1;
                log::warn!("clang-tidy issues in {}: {}", file.display(), e);

                // Show issues if not fixing
                if !fix {
                    println!("  {} {}", style("⚠️").yellow(), file.display());
                }
            }
        }
    }

    if fix {
        println!("{} {}", style("clang-tidy auto-fixes applied").green(),
                if issues_found > 0 { format!("({} files fixed)", issues_found) } else { "(no fixes needed)".to_string() });
    } else if issues_found > 0 {
        println!("{} clang-tidy issues found in {} files", style(issues_found).yellow(), style(cpp_files.len()).cyan());
    } else {
        println!("{}", style("No clang-tidy issues found").green());
    }

    Ok(())
}

async fn run_clang_format(
    lint_path: &Path,
    fix: bool,
    config: Option<&DevoraConfig>,
) -> Result<()> {
    println!("{}", style("Running clang-format...").blue());

    // Find C++ source files
    let cpp_files = find_cpp_files(lint_path)?;

    if cpp_files.is_empty() {
        println!("{}", style("No C++ files found for formatting.").yellow());
        return Ok(());
    }

    let mut formatting_issues = 0;

    for file in &cpp_files {

        let mut args: Vec<String> = vec![];

        // Add config file if specified
        if let Some(config) = config {
            if let Some(ref config_file) = config.lint.config_file {
                let config_path = lint_path.join(config_file);
                if config_path.exists() {
                    args.push("--style=file".to_string());
                    args.push(format!("--fallback-style=file:{}", config_path.display()));
                }
            }
        } else {
            args.push("--style=file".to_string());
            args.push("--fallback-style=LLVM".to_string());
        }

        if fix {
            args.push("-i".to_string());
            args.push("--Wno-error=unknown".to_string());
        } else {
            args.push("--dry-run".to_string());
            args.push("--Werror".to_string());
        }

        args.push(file.to_string_lossy().into_owned());

        // Run clang-format
        match execute_command("clang-format", &args.iter().map(|s| s.as_str()).collect::<Vec<_>>(), Some(lint_path)).await {
            Ok(_) => {
                log::debug!("clang-format passed for {}", file.display());
            }
            Err(e) => {
                formatting_issues += 1;
                log::warn!("clang-format issues in {}: {}", file.display(), e);

                if !fix {
                    println!("  {} {}", style("📝").blue(), file.display());
                }
            }
        }
    }

    if fix {
        println!("{} {}", style("Code formatting applied").green(),
                if formatting_issues > 0 { format!("({} files formatted)", formatting_issues) } else { "(all files properly formatted)".to_string() });
    } else if formatting_issues > 0 {
        println!("{} files need formatting", style(formatting_issues).yellow());
        println!("Run with --fix to automatically format these files.");
    } else {
        println!("{}", style("All files properly formatted").green());
    }

    Ok(())
}

fn find_cpp_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut cpp_files = Vec::new();

    // Common C++ file extensions
    let extensions = ["cpp", "cc", "cxx", "c++", "c", "hpp", "hh", "hxx", "h++", "h"];

    if dir.is_file() {
        // Check if single file is a C++ file
        if let Some(extension) = dir.extension().and_then(|ext| ext.to_str()) {
            if extensions.contains(&extension) {
                cpp_files.push(dir.to_path_buf());
            }
        }
    } else if dir.is_dir() {
        // Recursively search for C++ files
        visit_dirs(dir, &mut cpp_files, &extensions)?;
    }

    Ok(cpp_files)
}

fn visit_dirs(dir: &Path, cpp_files: &mut Vec<PathBuf>, extensions: &[&str]) -> Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)
            .map_err(|e| DevoraError::filesystem(format!("Failed to read directory {}: {}", dir.display(), e)))?
        {
            let entry = entry.map_err(|e| DevoraError::filesystem(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();

            if path.is_dir() {
                // Skip common build and ignore directories
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if ["build", "builddir", "target", ".git", "node_modules"].contains(&name) {
                        continue;
                    }
                }

                visit_dirs(&path, cpp_files, extensions)?;
            } else if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                if extensions.contains(&extension) {
                    cpp_files.push(path);
                }
            }
        }
    }
    Ok(())
}

/// Create default clang-tidy configuration
pub fn create_default_clang_tidy_config(project_root: &Path) -> Result<()> {
    let config_path = project_root.join(".clang-tidy");

    if config_path.exists() {
        println!("{}", style("clang-tidy config already exists").yellow());
        return Ok(());
    }

    let default_config = r#"---
Checks: >
    bugprone-*,
    modernize-*,
    performance-*,
    readability-*,
    -modernize-use-trailing-return-type,
    -readability-identifier-length

WarningsAsErrors: ''

HeaderFilterRegex: ''

FormatStyle: file
"#;

    std::fs::write(&config_path, default_config)
        .map_err(|e| DevoraError::filesystem(format!("Failed to create .clang-tidy: {}", e)))?;

    println!("{} {}", style("✓").green(), style("Created .clang-tidy config").green());
    Ok(())
}

/// Create default clang-format configuration
pub fn create_default_clang_format_config(project_root: &Path) -> Result<()> {
    let config_path = project_root.join(".clang-format");

    if config_path.exists() {
        println!("{}", style("clang-format config already exists").yellow());
        return Ok(());
    }

    let default_config = r#"---
Language: Cpp
BasedOnStyle: LLVM
IndentWidth: 4
TabWidth: 4
UseTab: Never
ColumnLimit: 100
BreakBeforeBraces: Linux
AllowShortFunctionsOnASingleLine: Empty
AllowShortIfStatementsOnASingleLine: false
AllowShortLoopsOnASingleLine: false
AlwaysBreakAfterDefinitionReturnType: None
AlwaysBreakAfterReturnType: None
AlwaysBreakBeforeMultilineStrings: false
AlwaysBreakTemplateDeclarations: Yes
BinPackArguments: true
BinPackParameters: true
BreakBeforeBinaryOperators: None
BreakBeforeTernaryOperators: true
BreakConstructorInitializersBeforeComma: false
BreakInheritanceList: BeforeComma
ConstructorInitializerAllOnOneLineOrOnePerLine: false
ConstructorInitializerIndentWidth: 4
ContinuationIndentWidth: 4
Cpp11BracedListStyle: true
DerivePointerAlignment: false
DisableFormat: false
FixNamespaceComments: true
IncludeBlocks: Preserve
IndentCaseLabels: true
IndentPPDirectives: BeforeHash
IndentWrappedFunctionNames: false
KeepEmptyLinesAtTheStartOfBlocks: true
MaxEmptyLinesToKeep: 1
NamespaceIndentation: None
PointerAlignment: Left
ReflowComments: true
SortIncludes: true
SortUsingDeclarations: true
SpaceAfterCStyleCast: false
SpaceAfterTemplateKeyword: true
SpaceBeforeAssignmentOperators: true
SpaceBeforeCpp11BracedList: false
SpaceBeforeCtorInitializerColon: true
SpaceBeforeInheritanceColon: true
SpaceBeforeParens: ControlStatements
SpaceBeforeRangeBasedForLoopColon: true
SpaceInEmptyParentheses: false
SpacesBeforeTrailingComments: 1
SpacesInAngles: false
SpacesInCStyleCastParentheses: false
SpacesInContainerLiterals: true
SpacesInParentheses: false
SpacesInSquareBrackets: false
Standard: c++17
"#;

    std::fs::write(&config_path, default_config)
        .map_err(|e| DevoraError::filesystem(format!("Failed to create .clang-format: {}", e)))?;

    println!("{} {}", style("✓").green(), style("Created .clang-format config").green());
    Ok(())
}