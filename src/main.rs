use clap::Parser;
use devora::{run, Cli};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let json = cli.json;

    if let Err(e) = run(cli).await {
        if json {
            let envelope = serde_json::json!({ "error": e.to_string() });
            eprintln!("{}", envelope);
        } else {
            eprintln!("Error: {}", e);
        }
        std::process::exit(1);
    }
}
