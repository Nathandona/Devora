use crate::error::{DevoraError, Result};
use crate::models::{LanguageManifest, FrameworkManifest};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct PluginRegistry {
    languages: HashMap<String, LanguageManifest>,
    frameworks: HashMap<String, HashMap<String, FrameworkManifest>>,
    plugins_dir: std::path::PathBuf,
}

impl PluginRegistry {
    pub fn new<P: AsRef<Path>>(plugins_dir: P) -> Self {
        Self {
            languages: HashMap::new(),
            frameworks: HashMap::new(),
            plugins_dir: plugins_dir.as_ref().to_path_buf(),
        }
    }

    pub fn discover(&mut self) -> Result<()> {
        if !self.plugins_dir.exists() {
            return Err(DevoraError::FileSystemError {
                path: self.plugins_dir.to_string_lossy().to_string(),
                message: "Plugins directory does not exist".to_string(),
            });
        }

        // Discover language plugins
        for entry in fs::read_dir(&self.plugins_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(lang_id) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(manifest) = self.load_language_manifest(&path) {
                        self.languages.insert(lang_id.to_string(), manifest.clone());

                        // Load frameworks for this language
                        let mut frameworks_map = HashMap::new();
                        let frameworks_dir = path.join("frameworks");

                        if frameworks_dir.exists() {
                            for framework_entry in fs::read_dir(&frameworks_dir)? {
                                let framework_entry = framework_entry?;
                                let framework_path = framework_entry.path();

                                if framework_path.is_dir() {
                                    if let Some(framework_id) = framework_path.file_name().and_then(|n| n.to_str()) {
                                        if let Ok(framework_manifest) = self.load_framework_manifest(&framework_path) {
                                            frameworks_map.insert(framework_id.to_string(), framework_manifest);
                                        }
                                    }
                                }
                            }
                        }

                        self.frameworks.insert(lang_id.to_string(), frameworks_map);
                    }
                }
            }
        }

        Ok(())
    }

    fn load_language_manifest(&self, lang_dir: &Path) -> Result<LanguageManifest> {
        let manifest_path = lang_dir.join("manifest.toml");
        let content = fs::read_to_string(&manifest_path)?;

        let manifest: LanguageManifest = toml::from_str(&content).map_err(|e| DevoraError::InvalidManifest {
            file: manifest_path.to_string_lossy().to_string(),
            details: e.to_string(),
        })?;

        Ok(manifest)
    }

    fn load_framework_manifest(&self, framework_dir: &Path) -> Result<FrameworkManifest> {
        let manifest_path = framework_dir.join("manifest.toml");
        let content = fs::read_to_string(&manifest_path)?;

        toml::from_str(&content).map_err(|e| DevoraError::InvalidManifest {
            file: manifest_path.to_string_lossy().to_string(),
            details: e.to_string(),
        })
    }

    pub fn get_language(&self, language: &str) -> Result<&LanguageManifest> {
        self.languages.get(language)
            .ok_or_else(|| DevoraError::LanguageNotFound {
                language: language.to_string()
            })
    }

    pub fn get_framework(&self, language: &str, framework: &str) -> Result<&FrameworkManifest> {
        let frameworks = self.frameworks.get(language)
            .ok_or_else(|| DevoraError::LanguageNotFound {
                language: language.to_string()
            })?;

        frameworks.get(framework)
            .ok_or_else(|| DevoraError::FrameworkNotFound {
                language: language.to_string(),
                framework: framework.to_string()
            })
    }

    pub fn list_languages(&self) -> Vec<&str> {
        self.languages.keys().map(|s| s.as_str()).collect()
    }

    pub fn list_frameworks(&self, language: &str) -> Vec<&str> {
        self.frameworks.get(language)
            .map(|frameworks| frameworks.keys().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
}