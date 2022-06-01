use clap::Args;
#[derive(Args)]
pub struct Generate {
    #[clap(help = "What kind of user-agent to generate. Possible values: desktop, mobile, googlebot2")]
    pub user_agent_type: String,

    #[clap(long, short='n', help = "How many user agent strings to generate", default_value="1")]
    pub count: u64,

    #[clap(long,  help = "Path to the user agent config path. If not specified, the default built-in user agent combinations will be used")]
    pub ua_config: Option<String>,
}

impl Generate {
    pub fn run(args: &Generate) {
        println!("'useragentr' was used to generate, user agent type is: {:?} n is {:?}, ua_config {:?}", args.user_agent_type, args.count, args.ua_config);
    }
}
