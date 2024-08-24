use anyhow::Result;
use arboard::Clipboard;
use clap::Parser;

use crate::message::SlackMessage;

mod message;

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

    /// Style of the quoted message.
    #[clap(long, default_value = "color: rgb(113, 133, 153); font-style: italic;")]
    pub style: String,
}

fn main() -> Result<()> {
    let Args { token, quote, prefix, style } = Args::parse();
    let mut clipboard = Clipboard::new().expect("failed to access system clipboard");
    let content = clipboard.get_text()?;

    let message = SlackMessage::try_from(content.as_str())?;
    let message = message.resolve(&token)?;

    let body_text = if quote {
        format!(
            "\n\n{}",
            message
                .body
                .lines()
                .map(|l| format!("> {l}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    } else {
        "".to_string()
    };

    let body_html = if quote {
        format!(
            r#"<blockquote style="{}">{}</blockquote>"#,
            style,
            message.body
        )
    } else {
        "".to_string()
    };

    let text = format!("[{}{}]({}){}", prefix, message.channel_name, message.url, body_text);
    let html = format!(r#"<a href="{}">{}{}</a>{}"#, message.url, prefix, message.channel_name, body_html);

    match clipboard.set_html(html.trim(), Some(&text)) {
        Ok(_) => println!("{text}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}
