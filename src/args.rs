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

    /// Include the message body as a quote
    #[arg(short, long)]
    pub quote: bool,
    
    /// Path to the template file for 
}
