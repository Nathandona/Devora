pub mod cli;
pub mod commands;
pub mod core;
pub mod error;
pub mod models;
pub mod utils;

pub use cli::Cli;

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    let json = cli.json;
    match cli.command {
        cli::Commands::New(args) => commands::new::execute(args, json).await.map_err(Into::into),
        cli::Commands::List(args) => commands::list::execute(args, json)
            .await
            .map_err(Into::into),
        cli::Commands::Info(args) => commands::info::execute(args, json)
            .await
            .map_err(Into::into),
    }
}
