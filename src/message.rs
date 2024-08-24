use std::{borrow::Cow, num::ParseFloatError, ops::Deref};

use anyhow::{anyhow, bail, Error, Result};
use serde::Deserialize;
use slack_api::sync as slack;
use url::Url;

#[derive(Deserialize, Debug, Clone, Copy)]
struct QueryParams {
    thread_ts: Option<f64>,
}

pub trait State {}
#[derive(Debug)]
pub struct Initialized<'a> {
    pub url: &'a str,
    pub channel_id: String,
    pub ts: f64,
    pub thread_ts: Option<f64>,
}

impl<'a> State for Initialized<'a> {}

#[derive(Debug)]
pub struct Resolved<'a> {
    pub url: &'a str,
    pub channel_name: String,
    pub body: String,
}

impl<'a> State for Resolved<'a> {}

#[derive(Debug)]
pub struct SlackMessage<S>
where
    S: State,
{
    state: S,
}

impl<S> Deref for SlackMessage<S>
where
    S: State,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<'a> TryFrom<&'a str> for SlackMessage<Initialized<'a>> {
    type Error = Error;

    fn try_from(text: &'a str) -> Result<SlackMessage<Initialized<'a>>> {
        let (channel_id, ts, thread_ts) = Self::parse(text.trim())?;
        Ok(SlackMessage { state:  Initialized { url: text, channel_id, ts, thread_ts } })
    }
}

impl SlackMessage<Initialized<'_>> {
    pub fn resolve(&self, token: &str) -> Result<SlackMessage<Resolved>> {
        let client = slack::default_client()?;
        let channel = Cow::from(&self.channel_id);

        let channel_name = slack::conversations::info(
            &client,
            token,
            &slack::conversations::InfoRequest {
                channel: channel.clone(),
                include_locale: Some(false),
                include_num_members: Some(false),
            },
        )?
        .channel
        .name_normalized;

        let history = slack::conversations::history(
            &client,
            token,
            &slack_api::conversations::HistoryRequest {
                channel: channel.clone(),
                latest: Some(self.ts),
                oldest: Some(self.ts),
                inclusive: Some(true),
                limit: Some(1),
                cursor: None,
            },
        )?;

        let mut body = match history.messages {
            Some(messages) => messages.into_iter().map(|m| m.text).collect::<Vec<String>>(),
            None => {
                bail!("No messages found")
            }
        };

        // If the message didn't send to the main channel, the response of the conversation.history
        // will be blank. I'm not sure why. Try to fetch using conversation.replies
        if body.join("").is_empty() {
            let replies = slack::conversations::replies(
                &client,
                Some(token),
                &slack_api::conversations::RepliesRequest {
                    channel: Some(channel),
                    latest: Some(self.ts),
                    oldest: Some(self.ts),
                    inclusive: Some(true),
                    limit: Some(1),
                    cursor: None,
                    ts: Some(self.thread_ts.unwrap_or(self.ts)),
                },
            )?;

            body = match replies.messages {
                Some(replies) => replies.into_iter().map(|m| m.text).collect::<Vec<String>>(),
                None => bail!("No messages found"),
            }
        }

        Ok(SlackMessage {
            state: Resolved {
                url: self.url,
                channel_name,
                body: body.into_iter().last().unwrap_or("".to_string()),
            },
        })
    }

    fn parse(text: &str) -> Result<(String, f64, Option<f64>)> {
        let url = match Url::parse(text) {
            Ok(u) => u,
            Err(e) => bail!("Failed to parse the clipboard content: {e}\nProvided:\n{text}"),
        };

        let channel_id = url
            .path_segments()
            .ok_or(anyhow!("Failed to get path segments"))?
            .nth(1)
            .ok_or(anyhow!("Failed to get the last path segment"))?
            .to_string();

        let ts = url
            .path_segments()
            .ok_or(anyhow!("Failed to get path segments"))?
            .last()
            .ok_or(anyhow!("Failed to get the last path segment"))?;

        let ts = Self::convert_to_f64(ts)?;

        let params: QueryParams =
            serde_qs::Config::new(5, false).deserialize_str(url.query().unwrap_or(""))?;

        Ok((channel_id, ts, params.thread_ts))
    }

    fn convert_to_f64(input: &str) -> Result<f64, ParseFloatError> {
        let numeric_part = input.trim_start_matches(|c: char| !c.is_numeric());
        let (int_part, decimal_part) = numeric_part.split_at(numeric_part.len() - 6);
        format!("{int_part}.{decimal_part}").parse::<f64>()
    }
}
