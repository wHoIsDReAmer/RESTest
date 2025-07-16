pub mod commands;

use crate::cli::commands::Commands;
use clap::{Parser, command};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long, default_value = "./")]
    pub(crate) directory: String,

    #[command(subcommand)]
    pub(crate) command: Commands,
}
