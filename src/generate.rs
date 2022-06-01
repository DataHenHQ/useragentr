use clap::{Args, ArgEnum};
#[derive(Args)]
pub struct Generate {
    #[clap(arg_enum, help = "What kind of user-agent to generate.")]
    pub user_agent_type: UserAgentType,

    #[clap(long, short='n', help = "How many user agent strings to generate.", default_value="1")]
    pub count: u64,

    #[clap(long,  help = "Path to the user agent config path. If not specified, the default built-in user agent combinations will be used.")]
    pub ua_config: Option<String>,
}

#[derive(ArgEnum, Clone, Debug)]
#[clap(rename_all="snake_case")]
pub enum UserAgentType {
    Desktop,
    Mobile,
    Googlebot2,
}

impl Generate {
    pub fn run(args: &Generate) {
        println!("'useragentr' was used to generate, user agent type is: {:?} n is {:?}, ua_config {:?}", args.user_agent_type, args.count, args.ua_config);
    }
}
