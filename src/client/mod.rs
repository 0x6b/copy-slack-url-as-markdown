use std::{ops::Deref, path::PathBuf};

use anyhow::Result;
use comrak::{markdown_to_html, ComrakOptions, RenderOptions};
use slack_client::message_retriever::{state::Resolved, MessageRetriever};
use state::{Initialized, Retrieved, State, Uninitialized};
use strum::EnumProperty;
use tera::{Context, Tera};
use tokio::fs::read_to_string;

use crate::template::{ContextKey::*, TemplateType::*, Templates};

pub mod state;

const TEMPLATE_PLAIN_TEXT: &str = include_str!("../../templates/plain_text");
const TEMPLATE_PLAIN_TEXT_QUOTE: &str = include_str!("../../templates/plain_text_quote");
const TEMPLATE_RICH_TEXT: &str = include_str!("../../templates/rich_text");
const TEMPLATE_RICH_TEXT_QUOTE: &str = include_str!("../../templates/rich_text_quote");

pub struct Client<S>
where
    S: State,
{
    state: S,
}

impl<S> Deref for Client<S>
where
    S: State,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<'template> Client<Uninitialized<'_>> {
    /// Create a new Copier client with the given Slack API token, quote flag, timezone, and
    /// templates.
    pub async fn from(state: Uninitialized<'_>) -> Result<Client<Initialized>> {
        Ok(Client {
            state: Initialized {
                token: state.token,
                quote: state.quote,
                timezone: state.timezone,
                tera: Self::setup_tera(&state.templates).await?,
            },
        })
    }

    // Set up the Tera template engine with the given [`Templates`], which might contain paths to
    // the template file, or just the template string.
    #[rustfmt::skip]
    async fn setup_tera(arg: &'template Templates) -> Result<Tera> {
        let mut tera = Tera::default();

        for (name, pathlike, default) in [
            (PlainText,      &arg.plain_text,       TEMPLATE_PLAIN_TEXT),
            (PlainTextQuote, &arg.plain_text_quote, TEMPLATE_PLAIN_TEXT_QUOTE),
            (RichText,       &arg.rich_text,        TEMPLATE_RICH_TEXT),
            (RichTextQuote,  &arg.rich_text_quote,  TEMPLATE_RICH_TEXT_QUOTE),
        ] {
            tera.add_raw_template(name.as_ref(), Self::get_template(pathlike, default).await)?;
        }

        Ok(tera)
    }

    // Resolve the template content from the given pathlike. If the pathlike is not a valid path,
    // then return content as is assuming it's a template string. If no pathlike is given, then
    // return the default template string.
    async fn get_template(
        input: &'template Option<String>,
        default: &'template str,
    ) -> &'template str {
        match input {
            Some(pathlike) => {
                if PathBuf::from(&pathlike).exists() {
                    let content = read_to_string(&pathlike).await.unwrap_or_default();
                    // Leak the content to make it have a static lifetime.
                    Box::leak(content.into_boxed_str())
                } else {
                    // Leak the content to make it have a static lifetime.
                    Box::leak(pathlike.clone().into_boxed_str())
                }
            }
            None => default,
        }
    }
}

impl<'state> Client<Initialized<'state>> {
    /// Retrieve a Slack message from the given URL.
    ///
    /// # Arguments
    ///
    /// - `url`: The [`url::URL`] of the Slack message.
    pub async fn retrieve(&self, url: &url::Url) -> Result<Client<Retrieved>> {
        let mut retriever = MessageRetriever::try_new(url, self.token)?;
        let message = retriever.resolve(self.quote).await?;

        Ok(Client {
            state: Retrieved {
                quote: self.quote,
                tera: self.tera.clone(),
                context: self.setup_context(&message, self.timezone).await?,
            },
        })
    }

    // Set up the Tera template context from the Slack message just retrieved.
    async fn setup_context(
        &self,
        message: &MessageRetriever<Resolved<'state>>,
        timezone: &str,
    ) -> Result<Context> {
        let mut context = Context::new();
        let datetime = jiff::Timestamp::from_microsecond(message.ts)?.in_tz(timezone)?;

        context.insert(ChannelName.as_ref(), &message.channel_name);
        context.insert(IsPrivateChannel.as_ref(), &message.is_private_channel);
        context.insert(UserName.as_ref(), &message.user_name);
        context.insert(Url.as_ref(), &message.url.as_str());

        if self.quote {
            context.insert(Text.as_ref(), &message.body.lines().collect::<Vec<_>>());

            let mut comrak_options = ComrakOptions {
                render: RenderOptions::builder().unsafe_(true).escape(false).build(),
                ..ComrakOptions::default()
            };
            comrak_options.extension.autolink = true;
            comrak_options.extension.strikethrough = true;
            comrak_options.extension.table = true;
            comrak_options.extension.tasklist = true;
            comrak_options.extension.tagfilter = true;
            context.insert(Html.as_ref(), &markdown_to_html(&message.body, &comrak_options));
        }

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

impl Client<Retrieved> {
    /// Render the Slack message into a rich text and a plain text.
    ///
    /// # Returns
    ///
    /// A tuple of the rich text and the plain text [`String`].
    pub fn render(&self) -> Result<(String, String)> {
        let (rich_text, text) = if self.quote {
            (
                self.tera.render(RichTextQuote.as_ref(), &self.context)?,
                self.tera.render(PlainTextQuote.as_ref(), &self.context)?,
            )
        } else {
            (
                self.tera.render(RichText.as_ref(), &self.context)?,
                self.tera.render(PlainText.as_ref(), &self.context)?,
            )
        };

        Ok((rich_text, text))
    }
}
