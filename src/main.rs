use anyhow::{bail, Result};
use arboard::Clipboard;
use clap::Parser;
use jiff::Timestamp;
use tera::{Context, Tera};

use crate::{args::Args, message::SlackMessage};

mod args;
mod message;
mod slack_client;

#[tokio::main]
async fn main() -> Result<()> {
    let Args { token, timezone, quote } = Args::parse();

    let mut clipboard = Clipboard::new().expect("failed to access system clipboard");
    let content = clipboard.get_text()?;

    let message = SlackMessage::try_from(content.as_str())?;
    let message = message.resolve(&token).await?;

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => bail!("Parsing error(s): {}", e),
    };

    let mut context = Context::new();
    context.insert("url", &message.url);
    context.insert("channel_name", &message.channel_name);
    context.insert(
        "text",
        &message
            .body
            .trim()
            .replace("```", "\n```\n")
            .lines()
            .collect::<Vec<_>>(),
    );
    let time = Timestamp::from_microsecond(message.ts)?.intz(&timezone)?;
    context.insert("timestamp", &time.strftime("%Y-%m-%d %H:%M:%S (%Z)").to_string());

    let (rich_text, text) = if quote {
        (tera.render("rich_text_quote.template", &context)?, tera.render("text_quote.template", &context)?)
    } else {
        (tera.render("rich_text.template", &context)?, tera.render("text.template", &context)?)
    };

    match clipboard.set_html(rich_text.trim(), Some(text.trim())) {
        Ok(_) => println!("{text}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}
