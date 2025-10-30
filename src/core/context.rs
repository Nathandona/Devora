use crate::error::Result;
use crate::models::TemplateVariable;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub struct ContextBuilder {
    builtins: HashMap<String, Value>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        let mut builtins = HashMap::new();

        // Add built-in variables
        builtins.insert("date".to_string(),
            Value::String(chrono::Utc::now().format("%Y-%m-%d").to_string()));
        builtins.insert("year".to_string(),
            Value::String(chrono::Utc::now().format("%Y").to_string()));

        // Try to get git config for author info
        if let Ok(name) = get_git_config("user.name") {
            builtins.insert("author".to_string(), Value::String(name));
        }
        if let Ok(email) = get_git_config("user.email") {
            builtins.insert("email".to_string(), Value::String(email));
        }

        Self { builtins }
    }

    pub fn build_context(
        &self,
        project_name: &str,
        variables: &HashMap<String, TemplateVariable>,
        provided_vars: &HashMap<String, String>,
    ) -> Result<Map<String, Value>> {
        let mut context = Map::new();

        // Add project-specific built-ins
        context.insert("project_name".to_string(), Value::String(project_name.to_string()));
        context.insert("project_slug".to_string(),
            Value::String(slugify(project_name)));

        // Add system built-ins
        for (key, value) in &self.builtins {
            context.insert(key.clone(), value.clone());
        }

        // Add user-provided variables
        for (name, var_def) in variables {
            if let Some(value) = provided_vars.get(name) {
                context.insert(name.clone(), Value::String(value.clone()));
            } else if let Some(default) = &var_def.default {
                // Try to resolve default value
                if let Value::String(template) = default {
                    let resolved = self.resolve_template(template, &context)?;
                    context.insert(name.clone(), Value::String(resolved));
                } else {
                    context.insert(name.clone(), default.clone());
                }
            } else if var_def.required {
                return Err(crate::error::DevoraError::ValidationError {
                    field: name.clone(),
                    message: "Required variable not provided".to_string(),
                });
            }
        }

        Ok(context)
    }

    fn resolve_template(&self, template: &str, context: &Map<String, Value>) -> Result<String> {
        // Simple template resolution for environment variables and references
        let mut result = template.to_string();

        // Environment variable substitution
        if result.starts_with("env:") {
            let env_var = &result[4..];
            if let Ok(value) = std::env::var(env_var) {
                result = value;
            }
        }

        // Reference substitution (simple case)
        if result.starts_with("ref:") {
            let reference = &result[4..];
            if let Some(value) = context.get(reference) {
                if let Some(s) = value.as_str() {
                    result = s.to_string();
                }
            }
        }

        Ok(result)
    }
}

fn slugify(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

fn get_git_config(key: &str) -> Result<String> {
    let output = std::process::Command::new("git")
        .args(&["config", "--global", key])
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(crate::error::DevoraError::FileSystemError {
            path: format!("git config {}", key),
            message: "Failed to get git config".to_string(),
        })
    }
}