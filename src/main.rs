use clap::Parser;
use restest::cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(err) = cli.run().await {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
