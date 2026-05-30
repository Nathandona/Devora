use crate::core::embedded::{self, PLUGINS};
use crate::error::{DevoraError, Result};
use crate::models::{FrameworkManifest, LanguageManifest};
use std::collections::HashMap;

pub struct PluginRegistry {
    languages: HashMap<String, LanguageManifest>,
    frameworks: HashMap<String, HashMap<String, FrameworkManifest>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            languages: HashMap::new(),
            frameworks: HashMap::new(),
        }
    }

    /// Discover every language and framework plugin baked into the binary.
    pub fn discover(&mut self) -> Result<()> {
        for lang_id in embedded::language_ids() {
            let manifest_path = format!("{}/manifest.toml", lang_id);
            let content = match embedded::read_text(&manifest_path) {
                Some(c) => c,
                None => continue, // directory without a manifest is not a language plugin
            };

            let manifest: LanguageManifest =
                toml::from_str(content).map_err(|e| DevoraError::InvalidManifest {
                    file: manifest_path.clone(),
                    details: e.to_string(),
                })?;
            self.languages.insert(lang_id.clone(), manifest);

            // Load frameworks for this language.
            let mut frameworks_map = HashMap::new();
            let frameworks_root = format!("{}/frameworks", lang_id);
            if let Some(frameworks_dir) = PLUGINS.get_dir(&frameworks_root) {
                for framework_entry in frameworks_dir.dirs() {
                    let framework_id =
                        match framework_entry.path().file_name().and_then(|n| n.to_str()) {
                            Some(id) => id.to_string(),
                            None => continue,
                        };

                    let fw_manifest_path =
                        format!("{}/frameworks/{}/manifest.toml", lang_id, framework_id);
                    if let Some(c) = embedded::read_text(&fw_manifest_path) {
                        let fw: FrameworkManifest =
                            toml::from_str(c).map_err(|e| DevoraError::InvalidManifest {
                                file: fw_manifest_path.clone(),
                                details: e.to_string(),
                            })?;
                        frameworks_map.insert(framework_id, fw);
                    }
                }
            }
            self.frameworks.insert(lang_id, frameworks_map);
        }

        Ok(())
    }

    pub fn get_language(&self, language: &str) -> Result<&LanguageManifest> {
        self.languages
            .get(language)
            .ok_or_else(|| DevoraError::LanguageNotFound {
                language: language.to_string(),
            })
    }

    pub fn get_framework(&self, language: &str, framework: &str) -> Result<&FrameworkManifest> {
        let frameworks =
            self.frameworks
                .get(language)
                .ok_or_else(|| DevoraError::LanguageNotFound {
                    language: language.to_string(),
                })?;

        frameworks
            .get(framework)
            .ok_or_else(|| DevoraError::FrameworkNotFound {
                language: language.to_string(),
                framework: framework.to_string(),
            })
    }

    pub fn list_languages(&self) -> Vec<&str> {
        self.languages.keys().map(|s| s.as_str()).collect()
    }

    pub fn list_frameworks(&self, language: &str) -> Vec<&str> {
        self.frameworks
            .get(language)
            .map(|frameworks| frameworks.keys().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
