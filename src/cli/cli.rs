use clap::Parser;

use crate::cli::commands::Commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "./")]
    directory: String,

    #[command(subcommand)]
    command: Commands
}
