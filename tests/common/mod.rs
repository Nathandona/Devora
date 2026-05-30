//! Shared test utilities.
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// A throwaway working directory for generation tests.
pub struct TestProject {
    pub dir: TempDir,
    pub path: PathBuf,
}

impl TestProject {
    pub fn new() -> Self {
        let dir = TempDir::new().expect("create temp dir");
        let path = dir.path().to_path_buf();
        TestProject { dir, path }
    }

    pub fn run(&self, args: &[&str]) -> CommandOutput {
        run_devora_in_dir(&self.path, args)
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

    /// True if either stream contains `text`.
    pub fn contains(&self, text: &str) -> bool {
        self.stdout.contains(text) || self.stderr.contains(text)
    }
}

/// Run the compiled `devora` binary with `args` in `dir`.
pub fn run_devora_in_dir(dir: &Path, args: &[&str]) -> CommandOutput {
    let output = Command::new(env!("CARGO_BIN_EXE_devora"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run devora binary");

    CommandOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        status: output.status,
    }
}

/// Run the compiled `devora` binary with `args` in a temp dir (for read-only
/// commands like list/info/help where the cwd is irrelevant).
pub fn run_devora(args: &[&str]) -> CommandOutput {
    let tmp = TempDir::new().expect("create temp dir");
    run_devora_in_dir(tmp.path(), args)
}

pub fn assert_file_exists(path: &Path) {
    assert!(path.exists(), "expected file to exist: {:?}", path);
}

pub fn read_to_string(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|_| panic!("could not read {:?}", path))
}
