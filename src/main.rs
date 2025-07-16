mod dsl;
mod cli;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "./")]
    directory: String,

    #[command(subcommand)]
    command: Commands
}

#[derive(Parser)]
enum Commands {
    /// Initialize
    Init {
        #[arg(short, long)]
        name: String,
    },
    /// Running tests in project
    Test {
        #[arg(short, long, default_value_t = false)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    println!("Current default path is : {}", cli.directory)
}
