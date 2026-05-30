use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkManifest {
    pub framework: FrameworkInfo,
    pub variables: Option<HashMap<String, TemplateVariable>>,
    pub pre_hooks: Option<Vec<Hook>>,
    pub post_hooks: Option<Vec<Hook>>,
    pub requirements: Option<Vec<String>>,
    pub init_commands: Option<Vec<String>>,
    pub conditional_files: Option<Vec<ConditionalFile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hook {
    pub command: String,
    pub description: Option<String>,
    pub working_directory: Option<String>,
    pub environment: Option<HashMap<String, String>>,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub description: String,
    pub prompt: Option<String>,
    pub default: Option<serde_json::Value>,
    pub required: bool,
    pub validation: Option<ValidationRule>,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub regex: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub allowed_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalFile {
    pub path: String,
    pub condition: String,
    pub description: Option<String>,
}
