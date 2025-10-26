use clap::Parser;
use devora_cli::{cli::Cli, logger, result::Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging early
    logger::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Execute the command
    cli.run().await
}