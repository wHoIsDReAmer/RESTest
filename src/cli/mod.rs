pub mod commands;

use clap::{command, Parser};
use crate::cli::commands::Commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long, default_value = "./")]
    pub(crate) directory: String,

    #[command(subcommand)]
    pub(crate) command: Commands
}
