//! Custom result types for Devora

use thiserror::Error;

pub type Result<T> = std::result::Result<T, DevoraError>;

#[derive(Error, Debug)]
pub enum DevoraError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Template error: {0}")]
    Template(String),

    #[error("Build system error: {0}")]
    Build(String),

    #[error("File system error: {0}")]
    FileSystem(String),

    #[error("Process execution error: {0}")]
    Process(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Template engine error: {0}")]
    TemplateEngine(#[from] tera::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Dependency error: {0}")]
    Dependency(String),

    #[error("Dialog error: {0}")]
    Dialog(#[from] dialoguer::Error),
}

impl DevoraError {
    pub fn config<S: Into<String>>(msg: S) -> Self {
        Self::Config(msg.into())
    }

    pub fn template<S: Into<String>>(msg: S) -> Self {
        Self::Template(msg.into())
    }

    pub fn build<S: Into<String>>(msg: S) -> Self {
        Self::Build(msg.into())
    }

    pub fn filesystem<S: Into<String>>(msg: S) -> Self {
        Self::FileSystem(msg.into())
    }

    pub fn process<S: Into<String>>(msg: S) -> Self {
        Self::Process(msg.into())
    }

    pub fn dependency<S: Into<String>>(msg: S) -> Self {
        Self::Dependency(msg.into())
    }
}