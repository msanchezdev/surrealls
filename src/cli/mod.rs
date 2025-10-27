use clap::Parser;
use clap::Subcommand;

mod lsp;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Lsp,
}

pub async fn run() {
    let args = Cli::parse();
    match args.command {
        Commands::Lsp => lsp::run().await,
        // _ => unreachable!("Unimplemented command: {:?}", args.command),
    }
}
