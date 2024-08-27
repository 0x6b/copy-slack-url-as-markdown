use std::path::PathBuf;

use anyhow::Result;
use arboard::Clipboard;
use clap::Parser;
use jiff::Timestamp;
use tera::{Context, Tera};
use tokio::fs::read_to_string;
use url::Url;

use crate::{
    args::{
        Args,
        TemplateType::{RichText, RichTextQuote, Text, TextQuote},
        Templates,
    },
    message::{Resolved, SlackMessage},
};

mod args;
mod message;
mod slack_client;

const TEMPLATE_TEXT: &str = include_str!("../templates/text");
const TEMPLATE_TEXT_QUOTE: &str = include_str!("../templates/text_quote");
const TEMPLATE_RICH_TEXT: &str = include_str!("../templates/rich_text");
const TEMPLATE_RICH_TEXT_QUOTE: &str = include_str!("../templates/rich_text_quote");

#[tokio::main]
async fn main() -> Result<()> {
    let Args { token, quote, timezone, templates } = Args::parse();
    let tera = setup_tera(&templates).await?;

    let mut clipboard = Clipboard::new()?;
    let content = clipboard.get_text()?;
    let url = Url::parse(content.trim())?;

    let message = SlackMessage::try_from(&url)?;
    let message = message.resolve(&token).await?;

    let context = setup_context(&message, &timezone)?;
    let (rich_text, text) = if quote {
        (tera.render(&RichTextQuote, &context)?, tera.render(&TextQuote, &context)?)
    } else {
        (tera.render(&RichText, &context)?, tera.render(&Text, &context)?)
    };

    match clipboard.set_html(rich_text.trim(), Some(text.trim())) {
        Ok(_) => println!("{text}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}

async fn setup_tera(templates: &Templates) -> Result<Tera> {
    let Templates { text, text_quote, rich_text, rich_text_quote } = templates;

    let mut tera = Tera::default();
    tera.add_raw_template(&Text, get_template(&text, TEMPLATE_TEXT).await)?;
    tera.add_raw_template(&TextQuote, get_template(&text_quote, TEMPLATE_TEXT_QUOTE).await)?;
    tera.add_raw_template(&RichText, get_template(&rich_text, TEMPLATE_RICH_TEXT).await)?;
    tera.add_raw_template(
        &RichTextQuote,
        get_template(&rich_text_quote, TEMPLATE_RICH_TEXT_QUOTE).await,
    )?;

    Ok(tera)
}

async fn get_template<'a>(input: &'a Option<String>, default: &'a str) -> &'a str {
    match input {
        Some(pathlike) => {
            if PathBuf::from(&pathlike).exists() {
                let content = read_to_string(&pathlike).await.unwrap_or_default();
                Box::leak(content.into_boxed_str())
            } else {
                Box::leak(pathlike.clone().into_boxed_str())
            }
        }
        None => default,
    }
}

fn setup_context(message: &SlackMessage<Resolved>, timezone: &str) -> Result<Context> {
    let mut context = Context::new();
    let datetime = Timestamp::from_microsecond(message.ts)?.intz(timezone)?;

    context.insert("channel_name", &message.channel_name);
    context.insert("url", &message.url.as_str());
    context.insert(
        "text",
        &message
            .body
            .trim()
            .replace("```", "\n```\n")
            .lines()
            .collect::<Vec<_>>(),
    );

    let mut insert = |key, format| context.insert(key, &datetime.strftime(format).to_string());
    insert("timestamp", "%Y-%m-%d %H:%M:%S (%Z)");
    insert("iso_date", "%F");
    insert("clock", "%T");

    insert("year", "%Y");
    insert("year_2digit", "%y");
    insert("month", "%B");
    insert("month_abbrev", "%b");
    insert("month_2digit", "%m");
    insert("day", "%d");
    insert("day_space", "%e");

    insert("hour24", "%H");
    insert("hour12", "%I");
    insert("minute", "%M");
    insert("second", "%S");
    insert("ampm", "%p");
    insert("ampm_lower", "%P");
    insert("weekday", "%A");
    insert("weekday_abbrev", "%a");

    insert("tz_iana", "%V");
    insert("tz_iana_colon", "%:V");
    insert("tz_abbrev", "%Z");
    insert("offset", "%z");
    insert("offset_colon", "%:z");

    Ok(context)
}
