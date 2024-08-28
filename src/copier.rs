use std::{ops::Deref, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use slack_client::SlackMessage;
use strum::EnumProperty;
use tera::{Context, Tera};
use tokio::fs::read_to_string;

use crate::{
    state::{CliArgs, Initialized, Resolved, State, Templates, Uninitialized},
    template::{
        ContextKey,
        ContextKey::{
            AmPm, AmPmLower, ChannelName, Clock, Day, DaySpace, Hour12, Hour24, IsoDate, Minute,
            Month, Month2Digit, MonthAbbrev, Offset, OffsetColon, Second, Timestamp, TzAbbrev,
            TzIana, Url, UserName, Weekday, WeekdayAbbrev, Year, Year2Digit,
        },
        TemplateType::{RichText, RichTextQuote, Text, TextQuote},
    },
};

const TEMPLATE_TEXT: &str = include_str!("../templates/text");
const TEMPLATE_TEXT_QUOTE: &str = include_str!("../templates/text_quote");
const TEMPLATE_RICH_TEXT: &str = include_str!("../templates/rich_text");
const TEMPLATE_RICH_TEXT_QUOTE: &str = include_str!("../templates/rich_text_quote");

pub struct Copier<S>
where
    S: State,
{
    state: S,
}

impl<S> Deref for Copier<S>
where
    S: State,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl Copier<Uninitialized> {
    pub async fn new() -> Result<Copier<Initialized>> {
        let Uninitialized { token, quote, timezone, templates } = CliArgs::parse();

        Ok(Copier {
            state: Initialized {
                token,
                quote,
                timezone,
                tera: Self::setup_tera(&templates).await?,
            },
        })
    }

    #[rustfmt::skip]
    async fn setup_tera(arg: &Templates) -> Result<Tera> {
        let mut tera = Tera::default();

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

impl Copier<Initialized> {
    pub async fn resolve(&self, url: &url::Url) -> Result<Copier<Resolved>> {
        let message: SlackMessage<slack_client::message::Initialized> =
            SlackMessage::try_from(url)?;
        let message: SlackMessage<slack_client::message::Resolved> =
            message.resolve(&self.token).await?;

        Ok(Copier {
            state: Resolved {
                quote: self.quote,
                tera: self.tera.clone(),
                context: Self::setup_context(&message, &self.timezone).await?,
            },
        })
    }

    async fn setup_context(
        message: &SlackMessage<slack_client::message::Resolved<'_>>,
        timezone: &str,
    ) -> Result<Context> {
        let mut context = Context::new();
        let datetime = jiff::Timestamp::from_microsecond(message.ts)?.intz(timezone)?;

        context.insert(ChannelName.as_ref(), &message.channel_name);
        context.insert(UserName.as_ref(), &message.user_name);
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

impl Copier<Resolved> {
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
