mod cli;
mod dsl;
mod request;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(err) = cli.run().await {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
