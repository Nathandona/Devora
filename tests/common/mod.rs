// Common test utilities and helpers

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use serde_json::Value;
use toml;

pub struct TestProject {
    pub dir: TempDir,
    pub path: PathBuf,
}

impl TestProject {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let path = dir.path().to_path_buf();
        Ok(TestProject { dir, path })
    }

    pub fn run_devora(&self, args: &[&str]) -> Result<CommandOutput, Box<dyn std::error::Error>> {
        run_devora_in_dir(&self.path, args)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn join(&self, path: &str) -> PathBuf {
        self.path.join(path)
    }
}

#[derive(Debug)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: std::process::ExitStatus,
}

impl CommandOutput {
    pub fn success(&self) -> bool {
        self.status.success()
    }

    pub fn contains(&self, text: &str) -> bool {
        self.stdout.contains(text) || self.stderr.contains(text)
    }
}

pub fn run_devora_in_dir(dir: &Path, args: &[&str]) -> Result<CommandOutput, Box<dyn std::error::Error>> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_devora"));
    cmd.args(args).current_dir(dir);

    let output = cmd.output()?;
    Ok(CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        status: output.status,
    })
}

pub fn run_cargo_command(args: &[&str]) -> Result<CommandOutput, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("cargo");
    cmd.args(args);

    let output = cmd.output()?;
    Ok(CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        status: output.status,
    })
}

pub fn assert_file_exists(path: &Path) {
    assert!(path.exists(), "File does not exist: {:?}", path);
}

pub fn assert_file_contains(path: &Path, content: &str) {
    assert_file_exists(path);
    let file_content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Could not read file: {:?}", path));
    assert!(
        file_content.contains(content),
        "File {:?} does not contain expected content: {:?}\nActual content:\n{}",
        path,
        content,
        file_content
    );
}

pub fn assert_dir_exists(path: &Path) {
    assert!(path.exists(), "Directory does not exist: {:?}", path);
    assert!(path.is_dir(), "Path is not a directory: {:?}", path);
}

pub fn read_file_to_string(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Could not read file: {:?}", path))
}

pub fn create_test_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|_| panic!("Could not create directory: {:?}", parent));
    }
    fs::write(path, content)
        .unwrap_or_else(|_| panic!("Could not write file: {:?}", path));
}

pub fn find_files_with_extension(dir: &Path, extension: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                files.extend(find_files_with_extension(&path, extension));
            } else if let Some(ext) = path.extension() {
                if ext == extension {
                    files.push(path);
                }
            }
        }
    }

    files
}

pub fn count_files_in_dir(dir: &Path) -> usize {
    if !dir.exists() {
        return 0;
    }

    fs::read_dir(dir)
        .unwrap()
        .filter_map(Result::ok)
        .count()
}

pub fn get_project_structure(dir: &Path) -> Vec<String> {
    let mut structure = Vec::new();
    collect_structure(dir, &mut structure, 0);
    structure
}

fn collect_structure(dir: &Path, structure: &mut Vec<String>, depth: usize) {
    if let Ok(entries) = fs::read_dir(dir) {
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let path = entry.path();
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Invalid name");

            let indent = "  ".repeat(depth);
            let display = if path.is_dir() {
                format!("{}{}/", indent, name)
            } else {
                format!("{}{}", indent, name)
            };

            structure.push(display);

            if path.is_dir() {
                collect_structure(&path, structure, depth + 1);
            }
        }
    }
}

pub fn parse_json_file(path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&content)?;
    Ok(json)
}

pub fn parse_toml_file(path: &Path) -> Result<toml::Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let toml: toml::Value = toml::from_str(&content)?;
    Ok(toml)
}

pub fn wait_for_file(path: &Path, timeout_ms: u64) -> bool {
    use std::time::{Duration, Instant};

    let start = Instant::now();
    let timeout = Duration::from_millis(timeout_ms);

    while start.elapsed() < timeout {
        if path.exists() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    false
}

pub fn cleanup_test_projects() {
    // Clean up any test projects that might have been left behind
    // This is useful for local development
    if let Ok(current_dir) = std::env::current_dir() {
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("test-") && path.is_dir() {
                        let _ = fs::remove_dir_all(&path);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_utilities() {
        let project = TestProject::new().unwrap();
        let test_file = project.join("test.txt");

        create_test_file(&test_file, "Hello, World!");

        assert_file_exists(&test_file);
        assert_file_contains(&test_file, "Hello");
        assert_eq!(read_file_to_string(&test_file), "Hello, World!");
    }

    #[test]
    fn test_project_structure() {
        let project = TestProject::new().unwrap();

        create_test_file(&project.join("src/main.rs"), "fn main() {}");
        create_test_file(&project.join("Cargo.toml"), "[package]");

        let structure = get_project_structure(&project.path);

        assert!(structure.iter().any(|s| s.contains("src/")));
        assert!(structure.iter().any(|s| s.contains("main.rs")));
        assert!(structure.iter().any(|s| s.contains("Cargo.toml")));
    }
}