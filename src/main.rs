use anyhow::Result;
use arboard::Clipboard;
use clap::Parser;
use jiff::Timestamp;

use crate::{args::Args, message::SlackMessage};

mod args;
mod message;
mod slack_client;

#[tokio::main]
async fn main() -> Result<()> {
    let Args { token, timezone, quote, prefix, style } = Args::parse();

    let mut clipboard = Clipboard::new().expect("failed to access system clipboard");
    let content = clipboard.get_text()?;

    let message = SlackMessage::try_from(content.as_str())?;
    let message = message.resolve(&token).await?;

    let title = format!("{}{}", prefix, message.channel_name);
    let url = &message.url;
    let (text, html) = if quote {
        let body = &message.body;
        let time = Timestamp::from_microsecond(message.ts)?.intz(&timezone)?;
        (
            format!(
                " at {}\n\n{}",
                time.strftime("%Y-%m-%d %H:%M:%S (%Z)"),
                body.lines().map(|l| format!("> {l}")).collect::<Vec<_>>().join("\n")
            ),
            format!(
                r#" at {}<blockquote style="{style}">{body}</blockquote>"#,
                time.strftime("%Y-%m-%d %H:%M:%S (%Z)")
            ),
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
