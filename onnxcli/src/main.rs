/*Command-line interface for ONNX */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yuxin Song",
    about = "Command-line for ONNX"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yuxin Song")]
    Onnxrun {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Onnxrun {}) => {
            let result = onnxcli::run();
            println!("The result is {:#?}", result);
        }
        None => println!("No subcommand was used"),
    }
}