use clap::Parser;

use crate::{client::state::Uninitialized, template::Templates};

/// The CLI arguments.
#[derive(Parser)]
#[clap(version, author, about)]
pub struct Args {
    /// Slack API token.
    #[arg(long, env = "SLACK_TOKEN")]
    pub token: String,

    /// Include the message body as a quote.
    #[arg(short, long)]
    pub quote: bool,

    /// The IANA time zone database identifiers to use for the timestamp.
    #[arg(short, long, default_value = "Asia/Tokyo")]
    pub timezone: String,

    #[command(flatten)]
    pub templates: Templates,

    /// Slack message URL to process. Leave empty to use the clipboard.
    #[arg()]
    pub url: Option<String>,
}

impl<'a> From<&'a Args> for Uninitialized<'a> {
    fn from(args: &'a Args) -> Self {
        Self {
            token: args.token.as_str(),
            quote: args.quote,
            timezone: args.timezone.as_str(),
            templates: args.templates.clone(),
        }
    }
}
