pub mod cli;
pub mod core;
pub mod models;
pub mod error;
pub mod utils;
pub mod commands;

pub use cli::Cli;

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        cli::Commands::New(args) => commands::new::execute(args).await.map_err(Into::into),
        cli::Commands::List(args) => commands::list::execute(args).await.map_err(Into::into),
        cli::Commands::Info(args) => commands::info::execute(args).await.map_err(Into::into),
    }
}