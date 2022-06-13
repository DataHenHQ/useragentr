use super::UserAgentType;
use clap::Args;
use lib::user_agent::UserAgent;
use std::path::PathBuf;

#[derive(Args)]
pub struct Generate {
    #[clap(value_parser, arg_enum, help = "What kind of user-agent to generate.")]
    pub user_agent_type: UserAgentType,

    #[clap(
        value_parser,
        long,
        short = 'n',
        help = "How many user agent strings to generate.",
        default_value = "1"
    )]
    pub count: u64,

    #[clap(
        value_parser,
        long,
        help = "Path to the user agent config path. If not specified, the default built-in user agent combinations will be used."
    )]
    pub ua_config: Option<PathBuf>,
}

impl Generate {
    pub fn run(args: &Generate) {
        let user_agent = UserAgent::new(args.ua_config.clone()).unwrap();
        // loop n times based on need
        for _ in 0..args.count {

            // generate the ua
            let ua_string = match user_agent.generate_ua(&args.user_agent_type.into()) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };
            // print it
            println!("{}", ua_string);
        }
    }
}
