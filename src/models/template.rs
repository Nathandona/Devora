use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    pub path: String,
    pub output_path: String,
    pub is_binary: bool,
    pub condition: Option<String>,
    pub permissions: Option<u32>,
}
