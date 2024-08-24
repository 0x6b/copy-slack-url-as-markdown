use clap::Parser;

#[derive(Parser)]
#[clap(version, author, about)]
pub struct Args {
    /// Slack API token.
    #[arg(short, long, env = "SLACK_TOKEN")]
    pub token: String,

    /// Include the message body as a quote.
    #[clap(short, long)]
    pub quote: bool,

    /// Prefix to the title.
    #[clap(long, default_value = "Slack#")]
    pub prefix: String,

    /// Style of the quoted message in rich text.
    #[clap(long, default_value = "color: rgb(96, 96, 96);")]
    pub style: String,
}
