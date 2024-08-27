use std::path::PathBuf;

use crate::{
    args::Args,
    message::{Resolved, SlackMessage},
};
use anyhow::Result;
use arboard::Clipboard;
use clap::Parser;
use jiff::Timestamp;
use tera::{Context, Tera};
use tokio::fs::read_to_string;
use url::Url;

mod args;
mod message;
mod slack_client;

const TEMPLATE_TEXT: &str = include_str!("../templates/text");
const TEMPLATE_TEXT_QUOTE: &str = include_str!("../templates/text_quote");
const TEMPLATE_RICH_TEXT: &str = include_str!("../templates/rich_text");
const TEMPLATE_RICH_TEXT_QUOTE: &str = include_str!("../templates/rich_text_quote");

#[tokio::main]
async fn main() -> Result<()> {
    let Args {
        token,
        quote,
        timezone,
        template_text,
        template_text_quote,
        template_rich_text,
        template_rich_text_quote,
    } = Args::parse();

    let tera = setup_tera(
        template_text,
        template_text_quote,
        template_rich_text,
        template_rich_text_quote,
    )
    .await?;

    let mut clipboard = Clipboard::new()?;
    let content = clipboard.get_text()?;
    let url = Url::parse(content.trim())?;

    let message = SlackMessage::try_from(&url)?;
    let message = message.resolve(&token).await?;
    let context = setup_context(&message, &timezone)?;

    let (rich_text, text) = if quote {
        (tera.render("rich_text_quote", &context)?, tera.render("text_quote", &context)?)
    } else {
        (tera.render("rich_text", &context)?, tera.render("text", &context)?)
    };

    match clipboard.set_html(rich_text.trim(), Some(text.trim())) {
        Ok(_) => println!("{text}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}

async fn setup_tera(
    text: Option<PathBuf>,
    text_quote: Option<PathBuf>,
    rich_text: Option<PathBuf>,
    rich_text_quote: Option<PathBuf>,
) -> Result<Tera> {
    let mut tera = Tera::default();

    let template_text = match text {
        Some(path) => &read_to_string(path).await?,
        None => TEMPLATE_TEXT,
    };
    let template_text_quote = match text_quote {
        Some(path) => &read_to_string(path).await?,
        None => TEMPLATE_TEXT_QUOTE,
    };
    let template_rich_text = match rich_text {
        Some(path) => &read_to_string(path).await?,
        None => TEMPLATE_RICH_TEXT,
    };
    let template_rich_text_quote = match rich_text_quote {
        Some(path) => &read_to_string(path).await?,
        None => TEMPLATE_RICH_TEXT_QUOTE,
    };

    tera.add_raw_template("text", template_text)?;
    tera.add_raw_template("text_quote", template_text_quote)?;
    tera.add_raw_template("rich_text", template_rich_text)?;
    tera.add_raw_template("rich_text_quote", template_rich_text_quote)?;

    Ok(tera)
}

fn setup_context(message: &SlackMessage<Resolved>, timezone: &str) -> Result<Context> {
    let mut context = Context::new();

    context.insert("channel_name", &message.channel_name);
    context.insert("url", &message.url);
    context.insert(
        "timestamp",
        &Timestamp::from_microsecond(message.ts)?
            .intz(timezone)?
            .strftime("%Y-%m-%d %H:%M:%S (%Z)")
            .to_string(),
    );
    context.insert(
        "text",
        &message
            .body
            .trim()
            .replace("```", "\n```\n")
            .lines()
            .collect::<Vec<_>>(),
    );

    Ok(context)
}
