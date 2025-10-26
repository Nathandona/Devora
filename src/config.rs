//! Configuration management for Devora

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::result::{Result, DevoraError};

/// Devora configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevoraConfig {
    pub project: ProjectConfig,
    pub build: BuildConfig,
    pub dev: DevConfig,
    pub test: TestConfig,
    pub lint: LintConfig,
}

impl Default for DevoraConfig {
    fn default() -> Self {
        Self {
            project: ProjectConfig::default(),
            build: BuildConfig::default(),
            dev: DevConfig::default(),
            test: TestConfig::default(),
            lint: LintConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub cpp_standard: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub license: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "myapp".to_string(),
            version: "0.1.0".to_string(),
            cpp_standard: "20".to_string(),
            description: None,
            authors: vec![],
            license: Some("MIT".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub build_system: String,
    pub build_dir: String,
    pub target_dir: String,
    pub package_manager: Option<String>,
    pub dependencies: Vec<Dependency>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            build_system: "meson".to_string(),
            build_dir: "build".to_string(),
            target_dir: "target".to_string(),
            package_manager: Some("vcpkg".to_string()),
            dependencies: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevConfig {
    pub port: u16,
    pub auto_reload: bool,
    pub open_browser: bool,
    pub exclude_patterns: Vec<String>,
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            auto_reload: true,
            open_browser: false,
            exclude_patterns: vec![
                "build/**".to_string(),
                "*.o".to_string(),
                "*.so".to_string(),
                "*.dll".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub framework: String,
    pub test_dir: String,
    pub test_pattern: String,
    pub coverage: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            framework: "catch2".to_string(),
            test_dir: "tests".to_string(),
            test_pattern: "test_*.cpp".to_string(),
            coverage: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintConfig {
    pub enabled: bool,
    pub tool: String,
    pub config_file: Option<String>,
    pub fix_on_save: bool,
}

impl Default for LintConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            tool: "clang-tidy".to_string(),
            config_file: None,
            fix_on_save: false,
        }
    }
}

impl DevoraConfig {
    /// Load configuration from a file
    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| DevoraError::filesystem(format!("Failed to read config file {}: {}", path.display(), e)))?;

        toml::from_str(&content)
            .map_err(|e| DevoraError::config(format!("Failed to parse config file {}: {}", path.display(), e)))
    }

    /// Save configuration to a file
    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| DevoraError::config(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(path, content)
            .map_err(|e| DevoraError::filesystem(format!("Failed to write config file {}: {}", path.display(), e)))
    }

    /// Find and load configuration from current directory or parents
    pub fn find_and_load() -> Result<Option<Self>> {
        let current_dir = std::env::current_dir()
            .map_err(|e| DevoraError::filesystem(format!("Failed to get current directory: {}", e)))?;

        for dir in current_dir.ancestors() {
            let config_path = dir.join("devora.toml");
            if config_path.exists() {
                return Ok(Some(Self::load_from_file(&config_path)?));
            }
        }

        Ok(None)
    }
}