mod dsl;
mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    println!("Current default path is : {}", cli.directory)
}
