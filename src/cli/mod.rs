pub mod commands;

use crate::cli::commands::Commands;
use crate::request::errors::RequestError;
use clap::{Parser, command};
use thiserror::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "./")]
    pub(crate) directory: String,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("tokenization error: {0}")]
    Tokenization(#[from] crate::dsl::lexer::TokenError),
    #[error("parse error: {0}")]
    Parse(#[from] crate::dsl::parser::errors::ParseError),
    #[error("no test files found in {0}")]
    NoTestsFound(String),
    #[error("file already exists: {0}")]
    AlreadyExists(String),
    #[error("request error: {0}")]
    Request(#[from] RequestError),
}

pub type Result<T> = std::result::Result<T, CliError>;

impl Cli {
    pub fn run(&self) -> Result<()> {
        self.command.run(&self.directory)
    }
}
