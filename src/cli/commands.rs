use clap::Parser;

#[derive(Parser)]
pub(crate) enum Commands {
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
