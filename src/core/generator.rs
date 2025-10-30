use crate::error::{DevoraError, Result};
use crate::models::Hook;
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
        let mut tera = Tera::default();

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

            // Skip manifest files
            if relative_path.file_name() == Some(std::ffi::OsStr::new("manifest.toml")) {
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

    pub fn execute_hooks(&mut self, hooks: &[Hook], working_dir: &Path) -> Result<()> {
        for hook in hooks {
            if let Some(_condition) = &hook.condition {
                // TODO: Evaluate condition
                // For now, skip conditional hooks
                continue;
            }

            let hook_dir = if let Some(custom_dir) = &hook.working_directory {
                working_dir.join(custom_dir)
            } else {
                working_dir.to_path_buf()
            };

            println!("Executing hook: {}", hook.command);

            let mut cmd = std::process::Command::new("sh");
            cmd.arg("-c")
               .arg(&hook.command)
               .current_dir(&hook_dir);

            // Set environment variables
            if let Some(env_vars) = &hook.environment {
                for (key, value) in env_vars {
                    cmd.env(key, value);
                }
            }

            let output = cmd.output()
                .map_err(|e| DevoraError::HookExecutionError {
                    hook: hook.command.clone(),
                    details: format!("Failed to execute: {}", e),
                })?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(DevoraError::HookExecutionError {
                    hook: hook.command.clone(),
                    details: stderr.to_string(),
                });
            }

            // Print output if verbose
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }

        Ok(())
    }
}