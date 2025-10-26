//! Command line interface for Devora

use clap::{Parser, Subcommand};
use crate::result::Result;

#[derive(Parser)]
#[command(name = "devora")]
#[command(about = "A modern developer tool for C++ projects")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "Nathan Dona <nathan@devora.dev>")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new C++ project
    Create {
        /// Project name
        name: String,
        /// Project directory (defaults to name)
        #[arg(short, long)]
        dir: Option<String>,
        /// C++ standard to use
        #[arg(long, default_value = "20")]
        cpp_std: String,
        /// Testing framework
        #[arg(long, default_value = "prompt")]
        test_framework: String,
        /// Package manager
        #[arg(long, default_value = "vcpkg")]
        package_manager: String,
    },
    /// Start development server with live rebuild and reload
    Dev {
        /// Port for development server
        #[arg(short, long, default_value = "3000")]
        port: u16,
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Run unit tests
    Test {
        /// Run tests in release mode
        #[arg(long)]
        release: bool,
        /// Filter tests by name
        #[arg(long)]
        filter: Option<String>,
    },
    /// Run linting checks
    Lint {
        /// Fix linting issues automatically
        #[arg(long)]
        fix: bool,
        /// Check specific file or directory
        path: Option<String>,
    },
    /// Build the project
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,
        /// Target directory
        #[arg(long, default_value = "build")]
        target_dir: String,
    },
    /// Show version information
    Version,
    /// Check and suggest installation of dependencies
    Check,
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Create { name, dir, cpp_std, test_framework, package_manager } => {
                crate::create::run(name, dir.as_deref(), cpp_std, test_framework, package_manager).await
            }
            Commands::Dev { port, verbose } => {
                crate::dev::run(*port, *verbose).await
            }
            Commands::Test { release, filter } => {
                crate::test::run(*release, filter.as_deref()).await
            }
            Commands::Lint { fix, path } => {
                crate::lint::run(*fix, path.as_deref()).await
            }
            Commands::Build { release, target_dir } => {
                crate::build::run(*release, target_dir).await
            }
            Commands::Version => {
                println!("devora {}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
            Commands::Check => {
                crate::dependencies::check_command().await
            }
        }
    }
}