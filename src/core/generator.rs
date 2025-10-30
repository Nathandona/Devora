use crate::error::{DevoraError, Result};
use crate::models::{Hook, ConditionalFile};
use serde_json::{Map, Value};
use std::fs;
use std::path::Path;
use tera::{Tera, Context};

pub struct TemplateGenerator {
    tera: Tera,
}

impl TemplateGenerator {
    pub fn new<P: AsRef<Path>>(templates_dir: P) -> Result<Self> {
        let templates_dir = templates_dir.as_ref();
        let mut tera = Self::configure_tera();

        if templates_dir.exists() {
            // Load all template files
            for entry in walkdir::WalkDir::new(templates_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                let path = entry.path();
                if let Some(template_name) = path.strip_prefix(templates_dir).ok()
                    .and_then(|p| p.to_str()) {
                    let content = fs::read_to_string(path)?;

                    // Register template with Tera
                    let template_name = template_name.replace('\\', "/");
                    if template_name.ends_with(".tera") {
                        let clean_name = template_name.strip_suffix(".tera").unwrap_or(&template_name);
                        tera.add_raw_template(clean_name, &content)?;
                    }
                }
            }
        }

        Ok(Self { tera })
    }

    pub fn generate_project(
        &mut self,
        templates_dir: &Path,
        output_dir: &Path,
        context: &Map<String, Value>,
        conditional_files: &[ConditionalFile],
        dry_run: bool,
    ) -> Result<()> {
        let tera_context = self.build_tera_context(context);

        for entry in walkdir::WalkDir::new(templates_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let source_path = entry.path();
            let relative_path = source_path.strip_prefix(templates_dir)
                .map_err(|_| DevoraError::FileSystemError {
                    path: source_path.to_string_lossy().to_string(),
                    message: "Failed to get relative path".to_string(),
                })?;

            // Skip manifest files and base templates (templates that start with "base.")
            if relative_path.file_name() == Some(std::ffi::OsStr::new("manifest.toml")) {
                continue;
            }

            // Skip base templates and partials directories (they shouldn't be generated as separate files)
            if let Some(file_name) = relative_path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with("base.") {
                    continue;
                }
            }

            // Skip partials directory entirely
            if relative_path.components().any(|c| c.as_os_str() == "partials") {
                continue;
            }

            // Check if this file should be included based on conditional files
            let relative_path_str = relative_path.to_string_lossy();
            if !self.should_include_file(&relative_path_str, conditional_files, context)? {
                continue;
            }

            let output_path = self.render_output_path(relative_path, &tera_context)?;
            let final_output_path = output_dir.join(output_path);

            if dry_run {
                println!("Would create: {}", final_output_path.display());
                continue;
            }

            // Create parent directories if needed
            if let Some(parent) = final_output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Copy or render the file
            if let Some(extension) = relative_path.extension() {
                if extension == "tera" {
                    // Render template using the full relative path without .tera extension
                    let relative_path_str = relative_path.to_string_lossy();
                    let template_name = relative_path_str.strip_suffix(".tera").unwrap_or(&relative_path_str);
                    let rendered = self.tera.render(template_name, &tera_context)?;
                    fs::write(&final_output_path, rendered)?;
                } else {
                    // Copy binary file as-is
                    fs::copy(source_path, &final_output_path)?;
                }
            } else {
                // Copy file without extension
                fs::copy(source_path, &final_output_path)?;
            }
        }

        Ok(())
    }

    fn build_tera_context(&self, context: &Map<String, Value>) -> Context {
        let mut tera_context = Context::new();
        for (key, value) in context {
            tera_context.insert(key, value);
        }
        tera_context
    }

    fn render_output_path(&mut self, path: &Path, context: &Context) -> Result<String> {
        let path_str = path.to_string_lossy();

        // Replace template variables in path
        let rendered = self.tera.render_str(&path_str, context)?;

        // Remove .tera extension if present
        let final_path = if rendered.ends_with(".tera") {
            &rendered[..rendered.len() - 5]
        } else {
            &rendered
        };

        Ok(final_path.to_string())
    }

    pub fn execute_hooks(&mut self, hooks: &[Hook], working_dir: &Path, context: &Map<String, Value>) -> Result<()> {
        for hook in hooks {
            // Evaluate condition if present
            if let Some(condition) = &hook.condition {
                if !self.evaluate_condition(condition, context)? {
                    println!("Skipping hook '{}' - condition not met", hook.command);
                    continue;
                }
            }

            let hook_dir = if let Some(custom_dir) = &hook.working_directory {
                working_dir.join(custom_dir)
            } else {
                working_dir.to_path_buf()
            };

            // Ensure working directory exists
            if !hook_dir.exists() {
                return Err(DevoraError::HookExecutionError {
                    hook: hook.command.clone(),
                    details: format!("Working directory does not exist: {}", hook_dir.display()),
                });
            }

            println!("Executing hook: {}", hook.command);
            if let Some(description) = &hook.description {
                println!("  Description: {}", description);
            }

            // Handle special Devora commands
            if hook.command == "devora_git_init" {
                return crate::utils::git::initialize_git_repo(working_dir, context);
            }

            let mut cmd = std::process::Command::new("sh");
            cmd.arg("-c")
               .arg(&hook.command)
               .current_dir(&hook_dir);

            // Set environment variables
            if let Some(env_vars) = &hook.environment {
                for (key, value) in env_vars {
                    cmd.env(key, value);
                    println!("  Environment: {}={}", key, value);
                }
            }

            // Execute with timeout to prevent hanging
            let output = cmd.output()
                .map_err(|e| DevoraError::HookExecutionError {
                    hook: hook.command.clone(),
                    details: format!("Failed to execute command '{}': {}", hook.command, e),
                })?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            // Always show hook output for better debugging
            if !stdout.is_empty() {
                println!("  Output: {}", stdout.trim());
            }
            if !stderr.is_empty() {
                eprintln!("  Error output: {}", stderr.trim());
            }

            if !output.status.success() {
                let error_msg = format!(
                    "Hook '{}' failed with exit code {}\nStdout: {}\nStderr: {}",
                    hook.command,
                    output.status.code().unwrap_or(-1),
                    stdout.trim(),
                    stderr.trim()
                );

                return Err(DevoraError::HookExecutionError {
                    hook: hook.command.clone(),
                    details: error_msg,
                });
            }

            println!("✓ Hook completed successfully");
        }

        Ok(())
    }

    /// Evaluate a hook condition against the current context
    /// Supports simple boolean expressions like:
    /// - "feature_x == true"
    /// - "author != \"\""
    /// - "framework == \"react\" && license == \"MIT\""
    fn evaluate_condition(&self, condition: &str, context: &Map<String, Value>) -> Result<bool> {
        // Parse simple boolean expressions
        // For now, support basic equality checks with && and || operators
        let expr = condition.trim();

        if expr.is_empty() {
            return Ok(true);
        }

        // Handle AND operators
        if expr.contains("&&") {
            let parts: Vec<&str> = expr.split("&&").collect();
            for part in parts {
                if !self.evaluate_simple_condition(part.trim(), context)? {
                    return Ok(false);
                }
            }
            return Ok(true);
        }

        // Handle OR operators
        if expr.contains("||") {
            let parts: Vec<&str> = expr.split("||").collect();
            for part in parts {
                if self.evaluate_simple_condition(part.trim(), context)? {
                    return Ok(true);
                }
            }
            return Ok(false);
        }

        // Handle simple condition
        self.evaluate_simple_condition(expr, context)
    }

    /// Evaluate a simple condition without logical operators
    fn evaluate_simple_condition(&self, condition: &str, context: &Map<String, Value>) -> Result<bool> {
        let condition = condition.trim();

        // Handle basic equality/inequality
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

        // Handle simple boolean variable check
        let val = self.get_context_value(condition, context)?;
        match val {
            serde_json::Value::Bool(b) => Ok(b),
            serde_json::Value::String(s) => Ok(!s.is_empty()),
            serde_json::Value::Null => Ok(false),
            _ => Ok(true), // Non-null values are considered "true"
        }
    }

    /// Get a value from the context, supporting dot notation
    fn get_context_value(&self, path: &str, context: &Map<String, Value>) -> Result<Value> {
        let path = path.trim().trim_matches('"');

        // Handle simple key lookup
        if let Some(value) = context.get(path) {
            return Ok(self.normalize_value(value.clone()));
        }

        // Handle nested access with dot notation
        if path.contains('.') {
            let parts: Vec<&str> = path.split('.').collect();
            let mut current = context;

            for (i, part) in parts.iter().enumerate() {
                if i == parts.len() - 1 {
                    // Last part - return the value
                    if let Some(value) = current.get(*part) {
                        return Ok(self.normalize_value(value.clone()));
                    }
                } else {
                    // Navigate to nested object
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

    /// Normalize values to handle type conversions (e.g., string "true" -> boolean true)
    fn normalize_value(&self, value: Value) -> Value {
        match value {
            Value::String(s) => {
                // Convert string booleans to actual booleans
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

    /// Parse a literal value from a condition
    fn parse_value(&self, value: &str) -> Result<Value> {
        let value = value.trim().trim_matches('"');

        // Parse boolean values
        if value == "true" {
            return Ok(Value::Bool(true));
        }
        if value == "false" {
            return Ok(Value::Bool(false));
        }

        // Parse numbers
        if let Ok(num) = value.parse::<i64>() {
            return Ok(Value::Number(num.into()));
        }
        if let Ok(num) = value.parse::<f64>() {
            return Ok(Value::Number(serde_json::Number::from_f64(num).unwrap_or(0.into())));
        }

        // Default to string
        Ok(Value::String(value.to_string()))
    }

    /// Determine if a file should be included based on conditional files configuration
    fn should_include_file(
        &self,
        file_path: &str,
        conditional_files: &[ConditionalFile],
        context: &Map<String, Value>,
    ) -> Result<bool> {
        // Remove .tera extension for comparison
        let clean_path = if file_path.ends_with(".tera") {
            &file_path[..file_path.len() - 5]
        } else {
            file_path
        };

        // Find matching conditional file rule
        for conditional_file in conditional_files {
            if clean_path == conditional_file.path {
                // Evaluate the condition
                let should_include = self.evaluate_condition(&conditional_file.condition, context)?;
                if let Some(description) = &conditional_file.description {
                    println!("Conditional file '{}': {} -> {}",
                            clean_path, description,
                            if should_include { "included" } else { "excluded" });
                }
                return Ok(should_include);
            }
        }

        // No conditional rule found, include by default
        Ok(true)
    }

    /// Configure Tera with inheritance and advanced features enabled
    fn configure_tera() -> Tera {
        let mut tera = Tera::default();

        // Enable autoescaping for HTML templates
        tera.autoescape_on(vec!["html"]);

        // Template inheritance is built into Tera by default
        // No additional configuration needed for {% extends %} and {% block %} support

        tera
    }
}