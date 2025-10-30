use thiserror::Error;

#[derive(Error, Debug)]
pub enum DevoraError {
    #[error("Language not found: {language}")]
    LanguageNotFound { language: String },

    #[error("Framework not found: {framework} for language: {language}")]
    FrameworkNotFound { language: String, framework: String },

    #[error("Invalid manifest: {file}\n{details}")]
    InvalidManifest { file: String, details: String },

    #[error("Template error: {0}")]
    TemplateError(#[from] tera::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Hook execution error: {hook}\n{details}")]
    HookExecutionError { hook: String, details: String },

    #[error("Validation error: {field}: {message}")]
    ValidationError { field: String, message: String },

    #[error("Serialization error: {0}")]
    SerializationError(#[from] toml::de::Error),

    #[error("Dialoguer error: {0}")]
    DialoguerError(#[from] dialoguer::Error),

    #[error("Walkdir error: {0}")]
    WalkdirError(#[from] walkdir::Error),

    #[error("File system error: {path} - {message}")]
    FileSystemError { path: String, message: String },
}

pub type Result<T> = std::result::Result<T, DevoraError>;