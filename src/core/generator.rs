use crate::core::embedded;
use crate::error::{DevoraError, Result};
use crate::models::{ConditionalFile, Hook};
use serde_json::{Map, Value};
use std::path::Path;
use tera::{Context, Tera};

pub struct TemplateGenerator {
    tera: Tera,
    /// (path relative to templates dir, raw bytes), normalized to `/`.
    files: Vec<(String, Vec<u8>)>,
}

impl TemplateGenerator {
    pub fn new(templates_dir: &str) -> Result<Self> {
        let mut tera = Self::configure_tera();
        let files = embedded::template_files(templates_dir)?;

        // Collect every `.tera` file under its slash-normalized name (without the
        // `.tera` suffix), then register them in one batch. Batch registration
        // builds Tera's inheritance chains once at the end, so template order
        // (e.g. a child `extends` a parent defined later) doesn't matter.
        let mut raw_templates: Vec<(String, String)> = Vec::new();
        for (rel, bytes) in &files {
            let name = rel.replace('\\', "/");
            if name.ends_with(".tera") {
                let clean_name = name.strip_suffix(".tera").unwrap_or(&name).to_string();
                let content =
                    std::str::from_utf8(bytes).map_err(|_| DevoraError::FileSystemError {
                        path: rel.clone(),
                        message: "Template file is not valid UTF-8".to_string(),
                    })?;
                raw_templates.push((clean_name, content.to_string()));
            }
        }
        tera.add_raw_templates(raw_templates)?;

        Ok(Self { tera, files })
    }

    /// Render the templates into `output_dir`. Returns the list of created
    /// file paths (relative to `output_dir`, `/`-separated).
    pub fn generate_project(
        &mut self,
        output_dir: &Path,
        context: &Map<String, Value>,
        conditional_files: &[ConditionalFile],
        dry_run: bool,
    ) -> Result<Vec<String>> {
        let tera_context = self.build_tera_context(context);
        let files = self.files.clone();
        let mut created = Vec::new();

        for (rel, bytes) in &files {
            let rel = rel.replace('\\', "/");
            let file_name = rel.rsplit('/').next().unwrap_or(&rel);

            // Skip manifests, base templates (used only via inheritance), and partials.
            if file_name == "manifest.toml" {
                continue;
            }
            if file_name.starts_with("base.") {
                continue;
            }
            if rel.split('/').any(|c| c == "partials") {
                continue;
            }

            // Conditional file inclusion.
            if !self.should_include_file(&rel, conditional_files, context)? {
                continue;
            }

            let output_rel = self.render_output_path(&rel, &tera_context)?;
            let final_output_path = output_dir.join(&output_rel);

            if dry_run {
                println!("Would create: {}", final_output_path.display());
                created.push(output_rel);
                continue;
            }

            if let Some(parent) = final_output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            if rel.ends_with(".tera") {
                let template_name = rel.strip_suffix(".tera").unwrap_or(&rel);
                let rendered = self.tera.render(template_name, &tera_context)?;
                std::fs::write(&final_output_path, rendered)?;
            } else {
                std::fs::write(&final_output_path, bytes)?;
            }

            created.push(output_rel);
        }

        Ok(created)
    }

    fn build_tera_context(&self, context: &Map<String, Value>) -> Context {
        let mut tera_context = Context::new();
        for (key, value) in context {
            tera_context.insert(key, value);
        }
        tera_context
    }

    fn render_output_path(&mut self, rel: &str, context: &Context) -> Result<String> {
        // Substitute any `{{ var }}` in the path itself (e.g. directories named
        // after the project), then drop the `.tera` suffix.
        let rendered = self.tera.render_str(rel, context)?;
        let final_path = rendered
            .strip_suffix(".tera")
            .unwrap_or(&rendered)
            .to_string();
        Ok(final_path)
    }

    pub fn execute_hooks(
        &mut self,
        hooks: &[Hook],
        working_dir: &Path,
        context: &Map<String, Value>,
    ) -> Result<()> {
        for hook in hooks {
            // Evaluate condition if present.
            if let Some(condition) = &hook.condition {
                if !self.evaluate_condition(condition, context)? {
                    println!("Skipping hook '{}' - condition not met", hook.command);
                    continue;
                }
            }

            // Substitute `{{ var }}` placeholders in the hook command itself
            // (e.g. `cmake --build build --config {{build_type}}`).
            let tera_ctx = self.build_tera_context(context);
            let command = Tera::one_off(&hook.command, &tera_ctx, false)
                .unwrap_or_else(|_| hook.command.clone());

            // Handle special Devora commands (no shell required).
            if command == "devora_git_init" {
                crate::utils::git::initialize_git_repo(working_dir, context)?;
                continue;
            }

            let hook_dir = if let Some(custom_dir) = &hook.working_directory {
                working_dir.join(custom_dir)
            } else {
                working_dir.to_path_buf()
            };

            if !hook_dir.exists() {
                return Err(DevoraError::HookExecutionError {
                    hook: hook.command.clone(),
                    details: format!("Working directory does not exist: {}", hook_dir.display()),
                });
            }

            let mut cmd = shell_command(&command);
            cmd.current_dir(&hook_dir);

            if let Some(env_vars) = &hook.environment {
                for (key, value) in env_vars {
                    cmd.env(key, value);
                }
            }

            let output = cmd.output().map_err(|e| DevoraError::HookExecutionError {
                hook: command.clone(),
                details: format!("Failed to execute command '{}': {}", command, e),
            })?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if !output.status.success() {
                let error_msg = format!(
                    "Hook '{}' failed with exit code {}\nStdout: {}\nStderr: {}",
                    command,
                    output.status.code().unwrap_or(-1),
                    stdout.trim(),
                    stderr.trim()
                );

                return Err(DevoraError::HookExecutionError {
                    hook: command.clone(),
                    details: error_msg,
                });
            }
        }

        Ok(())
    }

    /// Evaluate a hook condition against the current context.
    /// Supports simple boolean expressions like:
    /// - "feature_x == true"
    /// - "author != \"\""
    /// - "framework == \"react\" && license == \"MIT\""
    fn evaluate_condition(&self, condition: &str, context: &Map<String, Value>) -> Result<bool> {
        let expr = condition.trim();

        if expr.is_empty() {
            return Ok(true);
        }

        if expr.contains("&&") {
            let parts: Vec<&str> = expr.split("&&").collect();
            for part in parts {
                if !self.evaluate_simple_condition(part.trim(), context)? {
                    return Ok(false);
                }
            }
            return Ok(true);
        }

        if expr.contains("||") {
            let parts: Vec<&str> = expr.split("||").collect();
            for part in parts {
                if self.evaluate_simple_condition(part.trim(), context)? {
                    return Ok(true);
                }
            }
            return Ok(false);
        }

        self.evaluate_simple_condition(expr, context)
    }

    fn evaluate_simple_condition(
        &self,
        condition: &str,
        context: &Map<String, Value>,
    ) -> Result<bool> {
        let condition = condition.trim();

        if let Some(eq_pos) = condition.find("==") {
            let left = condition[..eq_pos].trim();
            let right = condition[eq_pos + 2..].trim();
            let left_val = self.get_context_value(left, context)?;
            let right_val = self.parse_value(right)?;
            return Ok(left_val == right_val);
        }

        if let Some(ne_pos) = condition.find("!=") {
            let left = condition[..ne_pos].trim();
            let right = condition[ne_pos + 2..].trim();
            let left_val = self.get_context_value(left, context)?;
            let right_val = self.parse_value(right)?;
            return Ok(left_val != right_val);
        }

        let val = self.get_context_value(condition, context)?;
        match val {
            serde_json::Value::Bool(b) => Ok(b),
            serde_json::Value::String(s) => Ok(!s.is_empty()),
            serde_json::Value::Null => Ok(false),
            _ => Ok(true),
        }
    }

    fn get_context_value(&self, path: &str, context: &Map<String, Value>) -> Result<Value> {
        let path = path.trim().trim_matches('"');

        if let Some(value) = context.get(path) {
            return Ok(self.normalize_value(value.clone()));
        }

        if path.contains('.') {
            let parts: Vec<&str> = path.split('.').collect();
            let mut current = context;

            for (i, part) in parts.iter().enumerate() {
                if i == parts.len() - 1 {
                    if let Some(value) = current.get(*part) {
                        return Ok(self.normalize_value(value.clone()));
                    }
                } else {
                    match current.get(*part) {
                        Some(Value::Object(obj)) => {
                            current = obj;
                        }
                        _ => return Ok(Value::Null),
                    }
                }
            }
        }

        Ok(Value::Null)
    }

    fn normalize_value(&self, value: Value) -> Value {
        match value {
            Value::String(s) => {
                if s == "true" {
                    Value::Bool(true)
                } else if s == "false" {
                    Value::Bool(false)
                } else {
                    Value::String(s)
                }
            }
            _ => value,
        }
    }

    fn parse_value(&self, value: &str) -> Result<Value> {
        let value = value.trim().trim_matches('"');

        if value == "true" {
            return Ok(Value::Bool(true));
        }
        if value == "false" {
            return Ok(Value::Bool(false));
        }

        if let Ok(num) = value.parse::<i64>() {
            return Ok(Value::Number(num.into()));
        }
        if let Ok(num) = value.parse::<f64>() {
            return Ok(Value::Number(
                serde_json::Number::from_f64(num).unwrap_or(0.into()),
            ));
        }

        Ok(Value::String(value.to_string()))
    }

    fn should_include_file(
        &self,
        file_path: &str,
        conditional_files: &[ConditionalFile],
        context: &Map<String, Value>,
    ) -> Result<bool> {
        let clean_path = file_path.strip_suffix(".tera").unwrap_or(file_path);

        for conditional_file in conditional_files {
            if clean_path == conditional_file.path {
                let should_include =
                    self.evaluate_condition(&conditional_file.condition, context)?;
                if let Some(description) = &conditional_file.description {
                    println!(
                        "Conditional file '{}': {} -> {}",
                        clean_path,
                        description,
                        if should_include {
                            "included"
                        } else {
                            "excluded"
                        }
                    );
                }
                return Ok(should_include);
            }
        }

        Ok(true)
    }

    fn configure_tera() -> Tera {
        let mut tera = Tera::default();
        tera.autoescape_on(vec!["html"]);
        tera
    }
}

/// Build a platform-appropriate shell command. Windows has no `sh`, so route
/// through `cmd /C`; everywhere else use `sh -c`.
fn shell_command(command: &str) -> std::process::Command {
    if cfg!(windows) {
        let mut cmd = std::process::Command::new("cmd");
        cmd.arg("/C").arg(command);
        cmd
    } else {
        let mut cmd = std::process::Command::new("sh");
        cmd.arg("-c").arg(command);
        cmd
    }
}
