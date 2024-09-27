mod serializer;

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
    /// 프로젝트 Init하기
    Init {
        #[arg(short, long)]
        name: String,
    },
    /// 테스트 실행하기
    Test {
        #[arg(short, long, default_value_t = false)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    println!("Current default path is : {}", cli.directory)
}
