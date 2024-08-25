use clap::Parser;

#[derive(Parser)]
#[clap(version, author, about)]
pub struct Args {
    /// Slack API token.
    #[arg(long, env = "SLACK_TOKEN")]
    pub token: String,

    /// Timezone to use i.e. UTC. Defaults to Asia/Tokyo.
    #[arg(short, long, default_value = "Asia/Tokyo")]
    pub timezone: String,

    /// Include the message body as a quote, with timestamp
    #[clap(short, long)]
    pub quote: bool,

    /// Prefix to the title.
    #[clap(long, default_value = "Slack#")]
    pub prefix: String,

    /// Style of the quoted message in rich text.
    #[clap(long, default_value = "color: rgb(96, 96, 96);")]
    pub style: String,
}
