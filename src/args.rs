use clap::Parser;

#[derive(Parser)]
#[clap(version, author, about)]
pub struct Args {
    /// Slack API token.
    #[arg(long, env = "SLACK_TOKEN")]
    pub token: String,

    /// The IANA time zone database identifiers to use for the timestamp. Timestamp will be shown
    /// only if the `quote` option is enabled.
    #[arg(short, long, default_value = "Asia/Tokyo")]
    pub timezone: String,

    /// Include the message body as a quote, with timestamp
    #[clap(short, long)]
    pub quote: bool,

    /// Prefix to the link title.
    #[clap(long, default_value = "Slack#")]
    pub prefix: String,

    /// CSS style to apply to the quote. This is a string that will be applied to the blockquote
    /// element. Obviously, this is only effective for rich text.
    #[clap(long, default_value = "color: rgb(96, 96, 96);")]
    pub style: String,
}
