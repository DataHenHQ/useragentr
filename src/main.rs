use clap::{Parser, Subcommand};
mod generate;
use generate::Generate;

#[derive(Parser)]
#[clap(author, version, about="DataHen User-Agent generator tool", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates user-agent strings
    Generate(Generate),
}


fn main() {
    let cli = Cli::parse();

    
    match &cli.command {
        Commands::Generate(args) => {
            Generate::run(args);
        }
    }
}
