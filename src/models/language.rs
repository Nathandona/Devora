use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::framework::TemplateVariable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageManifest {
    pub language: LanguageInfo,
    pub default_framework: Option<String>,
    pub frameworks: Option<Vec<String>>,
    pub setup_requirements: Option<Vec<String>>,
    pub variables: Option<HashMap<String, TemplateVariable>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
}