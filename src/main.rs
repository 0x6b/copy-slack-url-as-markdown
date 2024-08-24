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

    /// Style of the quoted message in rich text.
    #[clap(long, default_value = "color: rgb(96, 96, 96);")]
    pub style: String,
}

fn main() -> Result<()> {
    let Args { token, quote, prefix, style } = Args::parse();
    let mut clipboard = Clipboard::new().expect("failed to access system clipboard");
    let content = clipboard.get_text()?;

    let message = SlackMessage::try_from(content.as_str())?;
    let message = message.resolve(&token)?;

    let title = format!("{}{}", prefix, message.channel_name);
    let url = &message.url;
    let (text, html) = if quote {
        let body = &message.body;
        (
            format!(
                "\n\n{}",
                body.lines().map(|l| format!("> {l}")).collect::<Vec<_>>().join("\n")
            ),
            format!(r#"<blockquote style="{style}">{body}</blockquote>"#),
        )
    } else {
        ("".to_string(), "".to_string())
    };

    let text = format!("[{title}]({url}){text}");
    let html = format!(r#"<a href="{url}">{title}</a>{html}"#);

    match clipboard.set_html(html.trim(), Some(text.trim())) {
        Ok(_) => println!("{text}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}
