use crate::error::{DevoraError, Result};
use std::process::Command;
use std::collections::HashMap;

pub fn execute_command(
    command: &str,
    working_dir: &std::path::Path,
    env_vars: Option<&HashMap<String, String>>,
) -> Result<String> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
       .arg(command)
       .current_dir(working_dir);

    // Set environment variables
    if let Some(env_vars) = env_vars {
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
    }

    let output = cmd.output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(DevoraError::HookExecutionError {
            hook: command.to_string(),
            details: stderr.to_string(),
        })
    }
}