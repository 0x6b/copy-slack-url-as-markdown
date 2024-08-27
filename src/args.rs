use std::ops::Deref;

use clap::Parser;

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
}

#[derive(Parser)]
pub struct Templates {
    /// Path to the template file or a string for plain text (without quote). Leave empty to use
    /// the default.
    #[arg(long, env = "TEMPLATE_TEXT")]
    pub text: Option<String>,

    /// Path to the template file or a string for plain text (with quote). Leave empty to use the
    /// default.
    #[arg(long, env = "TEMPLATE_TEXT_QUOTE")]
    pub text_quote: Option<String>,

    /// Path to the template file or a string for rich text (without quote). Leave empty to use the
    /// default.
    #[arg(long, env = "TEMPLATE_RICH_TEXT")]
    pub rich_text: Option<String>,

    /// Path to the template file or a string for rich text (with quote). Leave empty to use the
    /// template.
    #[arg(long, env = "TEMPLATE_RICH_TEXT_QUOTE")]
    pub rich_text_quote: Option<String>,
}

pub enum TemplateType {
    Text,
    TextQuote,
    RichText,
    RichTextQuote,
}

impl Deref for TemplateType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            TemplateType::Text => "text",
            TemplateType::TextQuote => "text_quote",
            TemplateType::RichText => "rich_text",
            TemplateType::RichTextQuote => "rich_text_quote",
        }
    }
}
