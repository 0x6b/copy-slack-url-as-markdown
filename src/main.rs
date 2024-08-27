use std::path::PathBuf;

use anyhow::Result;
use arboard::Clipboard;
use clap::Parser;
use jiff::Timestamp;
use tera::{Context, Tera};
use tokio::fs::read_to_string;
use url::Url;

use crate::{
    args::{Args, Templates},
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

async fn setup_tera(templates: &Templates) -> Result<Tera> {
    let Templates { text, text_quote, rich_text, rich_text_quote } = templates;

    let mut tera = Tera::default();
    tera.add_raw_template("text", get_template(&text, TEMPLATE_TEXT).await)?;
    tera.add_raw_template("text_quote", get_template(&text_quote, TEMPLATE_TEXT_QUOTE).await)?;
    tera.add_raw_template("rich_text", get_template(&rich_text, TEMPLATE_RICH_TEXT).await)?;
    tera.add_raw_template(
        "rich_text_quote",
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

    context.insert("timestamp", &datetime.strftime("%Y-%m-%d %H:%M:%S (%Z)").to_string());
    context.insert("weekday_full", &datetime.strftime("%A").to_string());
    context.insert("weekday_abbrev", &datetime.strftime("%a").to_string());
    context.insert("month_full", &datetime.strftime("%B").to_string());
    context.insert("month_abbrev", &datetime.strftime("%b").to_string());
    context.insert("day_zero", &datetime.strftime("%d").to_string());
    context.insert("day_space", &datetime.strftime("%e").to_string());
    context.insert("iso_date", &datetime.strftime("%F").to_string());
    context.insert("hour24", &datetime.strftime("%H").to_string());
    context.insert("hour12", &datetime.strftime("%I").to_string());
    context.insert("minute", &datetime.strftime("%M").to_string());
    context.insert("month", &datetime.strftime("%m").to_string());
    context.insert("ampm_lower", &datetime.strftime("%P").to_string());
    context.insert("ampm_upper", &datetime.strftime("%p").to_string());
    context.insert("second", &datetime.strftime("%S").to_string());
    context.insert("clock", &datetime.strftime("%T").to_string());
    context.insert("iana_nocolon", &datetime.strftime("%V").to_string());
    context.insert("iana_colon", &datetime.strftime("%:V").to_string());
    context.insert("year", &datetime.strftime("%Y").to_string());
    context.insert("year_2digit", &datetime.strftime("%y").to_string());
    context.insert("tzabbrev", &datetime.strftime("%Z").to_string());
    context.insert("offset_nocolon", &datetime.strftime("%z").to_string());
    context.insert("offset_colon", &datetime.strftime("%:z").to_string());

    Ok(context)
}
