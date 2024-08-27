use std::path::PathBuf;

use anyhow::Result;
use arboard::Clipboard;
use clap::Parser;
use strum::EnumProperty;
use tera::{Context, Tera};
use tokio::fs::read_to_string;

use crate::{
    args::{
        Args,
        Templates,
    },
    context_key::{
        ContextKey,
        ContextKey::{
            AmPm, AmPmLower, ChannelName, Clock, Day, DaySpace, Hour12, Hour24, IsoDate, Minute,
            Month, Month2Digit, MonthAbbrev, Offset, OffsetColon, Second, Timestamp, TzAbbrev,
            TzIana, Url, Weekday, WeekdayAbbrev, Year, Year2Digit,
        },
    },
    message::{Resolved, SlackMessage},
};
use crate::template_type::TemplateType::{RichText, RichTextQuote, Text, TextQuote};

mod args;
mod context_key;
mod message;
mod slack_client;
mod template_type;

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
    let url = url::Url::parse(content.trim())?;

    let message = SlackMessage::try_from(&url)?;
    let message = message.resolve(&token).await?;

    let context = setup_context(&message, &timezone)?;
    let (rich_text, text) = if quote {
        (tera.render(RichTextQuote.as_ref(), &context)?, tera.render(TextQuote.as_ref(), &context)?)
    } else {
        (tera.render(RichText.as_ref(), &context)?, tera.render(Text.as_ref(), &context)?)
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
    tera.add_raw_template(Text.as_ref(), get_template(text, TEMPLATE_TEXT).await)?;
    tera.add_raw_template(TextQuote.as_ref(), get_template(text_quote, TEMPLATE_TEXT_QUOTE).await)?;
    tera.add_raw_template(RichText.as_ref(), get_template(rich_text, TEMPLATE_RICH_TEXT).await)?;
    tera.add_raw_template(
        RichTextQuote.as_ref(),
        get_template(rich_text_quote, TEMPLATE_RICH_TEXT_QUOTE).await,
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
    let datetime = jiff::Timestamp::from_microsecond(message.ts)?.intz(timezone)?;

    context.insert(ChannelName.as_ref(), &message.channel_name);
    context.insert(Url.as_ref(), &message.url.as_str());
    context.insert(
        ContextKey::Text.as_ref(),
        &message
            .body
            .trim()
            .replace("```", "\n```\n")
            .lines()
            .collect::<Vec<_>>(),
    );

    let mut insert_datetime_var = |key: ContextKey| {
        context.insert(key.as_ref(), &datetime.strftime(key.get_str("format").unwrap()).to_string())
    };

    insert_datetime_var(Timestamp);
    insert_datetime_var(IsoDate);
    insert_datetime_var(Clock);

    insert_datetime_var(Year);
    insert_datetime_var(Year2Digit);
    insert_datetime_var(Month);
    insert_datetime_var(MonthAbbrev);
    insert_datetime_var(Month2Digit);
    insert_datetime_var(Day);
    insert_datetime_var(DaySpace);

    insert_datetime_var(Hour24);
    insert_datetime_var(Hour12);
    insert_datetime_var(Minute);
    insert_datetime_var(Second);
    insert_datetime_var(AmPm);
    insert_datetime_var(AmPmLower);
    insert_datetime_var(Weekday);
    insert_datetime_var(WeekdayAbbrev);

    insert_datetime_var(TzIana);
    insert_datetime_var(TzAbbrev);
    insert_datetime_var(Offset);
    insert_datetime_var(OffsetColon);

    Ok(context)
}
