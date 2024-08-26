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
            // .filter(|l| !l.is_empty())
            .collect::<Vec<_>>(),
    );
    let time = Timestamp::from_microsecond(message.ts)?.intz(&timezone)?;
    context.insert("timestamp", &time.strftime("%Y-%m-%d %H:%M:%S (%Z)").to_string());

    let (rt, md) = if quote {
        let rt = tera.render("rt_quote.template", &context)?;
        let md = tera.render("md_quote.template", &context)?;
        (rt, md)
    } else {
        let rt = tera.render("rt.template", &context)?;
        let md = tera.render("md.template", &context)?;
        (rt, md)
    };

    match clipboard.set_html(rt.trim(), Some(md.trim())) {
        Ok(_) => println!("{md}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}
