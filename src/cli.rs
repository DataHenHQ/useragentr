pub mod generate;

use clap::{ArgEnum, Parser, Subcommand};
use lib::user_agent::Type as UATC;

use self::generate::Generate;

#[derive(Parser)]
#[clap(author, version, about="DataHen User-Agent generator tool", long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();

        // let default_ua_config_bytes = include_bytes!("./../config/default-ua-config.json");
        match &cli.command {
            Commands::Generate(args) => {
                Generate::run(args);
            }
        }
    }
}

#[derive(ArgEnum, Clone, Debug, Copy)]
#[clap(rename_all="snake_case")]
pub enum UserAgentType {
    Desktop,
    Mobile,
    Tablet,
}

impl From<self::UserAgentType> for UATC {
    fn from(value: UserAgentType) -> Self {
        match value {
            UserAgentType::Desktop => Self::Desktop,
            UserAgentType::Mobile => Self::Mobile,
            UserAgentType::Tablet => Self::Tablet,
        }
    }
}

impl From<UATC> for UserAgentType {
    fn from(value: UATC) -> Self {
        match value {
            UATC::Desktop => Self::Desktop,
            UATC::Mobile => Self::Mobile,
            UATC::Tablet => Self::Tablet,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Generates user-agent strings
    Generate(Generate),
}
