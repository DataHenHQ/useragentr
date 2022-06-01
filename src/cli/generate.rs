use clap::Args;
use std::path::PathBuf;
use lib::user_agent::UserAgent;
use super::UserAgentType;

#[derive(Args)]
pub struct Generate {
    #[clap(arg_enum, help = "What kind of user-agent to generate.")]
    pub user_agent_type: UserAgentType,

    #[clap(long, short='n', help = "How many user agent strings to generate.", default_value="1")]
    pub count: u64,

    #[clap(long,  help = "Path to the user agent config path. If not specified, the default built-in user agent combinations will be used.")]
    pub ua_config: Option<PathBuf>,
}

impl Generate {
    pub fn run(args: &Generate) {
        let user_agent = UserAgent::new(args.ua_config.clone()).unwrap();
        match &args.ua_config {
            Some(_) => println!("ua config is {:?}", user_agent.config()),
            None => println!("default ua config is {:?}", user_agent.config())
        }

        println!("'useragentr' was used to generate, user agent type is: {:?} n is {:?}, ua_config {:?}", args.user_agent_type, args.count, args.ua_config);
    }
}
