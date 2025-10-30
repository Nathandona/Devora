use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "devora")]
#[command(about = "A universal, modular project scaffolding framework")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project from a template
    New(NewArgs),
    /// List available languages and frameworks
    List(ListArgs),
    /// Show detailed information about a language or framework
    Info(InfoArgs),
}

#[derive(Parser)]
pub struct NewArgs {
    /// Project name
    pub name: String,

    /// Programming language
    pub language: Option<String>,

    /// Framework to use
    #[arg(long)]
    pub framework: Option<String>,

    /// Output directory (defaults to project name)
    #[arg(long)]
    pub output_dir: Option<String>,

    /// Run in dry-run mode (don't create files)
    #[arg(long)]
    pub dry_run: bool,

    /// Skip interactive prompts
    #[arg(long)]
    pub non_interactive: bool,

    /// Variables in KEY=VALUE format
    #[arg(long, value_parser = parse_key_value)]
    pub var: Vec<(String, String)>,
}

#[derive(Parser)]
pub struct ListArgs {
    /// Filter by language (optional)
    pub language: Option<String>,

    /// Show detailed information
    #[arg(long)]
    pub detailed: bool,
}

#[derive(Parser)]
pub struct InfoArgs {
    /// Language name
    pub language: String,

    /// Framework name (optional)
    pub framework: Option<String>,
}

/// Parse key-value pairs in the format KEY=VALUE
fn parse_key_value(s: &str) -> Result<(String, String), String> {
    let (key, value) = s.split_once('=').ok_or_else(|| {
        format!("Expected KEY=VALUE format, got: {}", s)
    })?;
    Ok((key.to_string(), value.to_string()))
}