use std::{ops::Deref, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use strum::EnumProperty;
use tera::{Context, Tera};
use tokio::fs::read_to_string;

use crate::{
    context_key::{ContextKey, ContextKey::*},
    message::SlackMessage,
    template_type::TemplateType::{RichText, RichTextQuote, Text, TextQuote},
};

const TEMPLATE_TEXT: &str = include_str!("../templates/text");
const TEMPLATE_TEXT_QUOTE: &str = include_str!("../templates/text_quote");
const TEMPLATE_RICH_TEXT: &str = include_str!("../templates/rich_text");
const TEMPLATE_RICH_TEXT_QUOTE: &str = include_str!("../templates/rich_text_quote");

pub trait State {}
impl State for Uninitialized {}
impl State for Initialized {}
impl State for Resolved {}

pub type CliArgs = Uninitialized;

#[derive(Parser)]
#[clap(version, author, about)]
pub struct Uninitialized {
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

pub struct Initialized {
    pub token: String,
    pub quote: bool,
    pub timezone: String,
    pub tera: Tera,
}

pub struct Resolved {
    pub quote: bool,
    pub tera: Tera,
    pub context: Context,
}

pub struct Cli<S>
where
    S: State,
{
    state: S,
}

impl<S> Deref for Cli<S>
where
    S: State,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl Cli<Uninitialized> {
    pub async fn new() -> Result<Cli<Initialized>> {
        let Uninitialized { token, quote, timezone, templates } = CliArgs::parse();

        Ok(Cli {
            state: Initialized {
                token,
                quote,
                timezone,
                tera: Self::setup_tera(&templates).await?,
            },
        })
    }

    async fn setup_tera(arg: &Templates) -> Result<Tera> {
        let mut tera = Tera::default();

        #[rustfmt::skip]
        for (name, pathlike, default) in [
            (Text,          &arg.text,            TEMPLATE_TEXT),
            (TextQuote,     &arg.text_quote,      TEMPLATE_TEXT_QUOTE),
            (RichText,      &arg.rich_text,       TEMPLATE_RICH_TEXT),
            (RichTextQuote, &arg.rich_text_quote, TEMPLATE_RICH_TEXT_QUOTE),
        ] {
            tera.add_raw_template(name.as_ref(), Self::get_template(pathlike, default).await)?;
        }

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
}

impl Cli<Initialized> {
    pub async fn resolve(&self, url: &url::Url) -> Result<Cli<Resolved>> {
        let message: SlackMessage<crate::message::Initialized> = SlackMessage::try_from(url)?;
        let message: SlackMessage<crate::message::Resolved> = message.resolve(&self.token).await?;

        Ok(Cli {
            state: Resolved {
                quote: self.quote,
                tera: self.tera.clone(),
                context: Self::setup_context(&message, &self.timezone).await?,
            },
        })
    }
    async fn setup_context(
        message: &SlackMessage<crate::message::Resolved<'_>>,
        timezone: &str,
    ) -> Result<Context> {
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

        [
            Timestamp,
            IsoDate,
            Clock,
            Year,
            Year2Digit,
            Month,
            MonthAbbrev,
            Month2Digit,
            Day,
            DaySpace,
            Hour24,
            Hour12,
            Minute,
            Second,
            AmPm,
            AmPmLower,
            Weekday,
            WeekdayAbbrev,
            TzIana,
            TzAbbrev,
            Offset,
            OffsetColon,
        ]
        .iter()
        .for_each(|key| {
            context.insert(
                key.as_ref(),
                &datetime.strftime(key.get_str("format").unwrap()).to_string(),
            )
        });

        Ok(context)
    }
}

impl Cli<Resolved> {
    pub fn render(&self) -> Result<(String, String)> {
        let (rich_text, text) = if self.quote {
            (
                self.tera.render(RichTextQuote.as_ref(), &self.context)?,
                self.tera.render(TextQuote.as_ref(), &self.context)?,
            )
        } else {
            (
                self.tera.render(RichText.as_ref(), &self.context)?,
                self.tera.render(Text.as_ref(), &self.context)?,
            )
        };

        Ok((rich_text, text))
    }
}
