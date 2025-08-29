mod cli;
mod dsl;
mod request;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    println!("Current default path is : {}", cli.directory)
}
