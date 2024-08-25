use std::{num::ParseFloatError, ops::Deref};

use anyhow::{anyhow, bail, Error, Result};
use serde::Deserialize;
use url::Url;

use crate::{
    slack_client,
    slack_client::{HistoryQuery, InfoQuery, RepliesQuery},
};

#[derive(Deserialize, Debug, Clone, Copy)]
struct QueryParams {
    thread_ts: Option<f64>,
}

pub trait State {}
#[derive(Debug)]
pub struct Initialized<'a> {
    pub url: &'a str,
    pub channel_id: String,
    pub ts: String,
    pub ts64: f64,
    pub thread_ts64: Option<f64>,
}

impl<'a> State for Initialized<'a> {}

#[derive(Debug)]
pub struct Resolved<'a> {
    pub url: &'a str,
    pub channel_name: String,
    pub body: String,
    pub ts: i64,
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
        let (channel_id, ts, ts64, thread_ts64) = Self::parse(text.trim())?;
        Ok(SlackMessage {
            state: Initialized { url: text, channel_id, ts, ts64, thread_ts64 },
        })
    }
}

impl SlackMessage<Initialized<'_>> {
    pub async fn resolve(&self, token: &str) -> Result<SlackMessage<Resolved>> {
        let client = slack_client::Client::new(token)?;
        let channel_name = client
            .conversations_info(&InfoQuery { channel: self.channel_id.clone() })
            .await?
            .name_normalized;
        let history = client
            .conversations_history(&HistoryQuery {
                channel: self.channel_id.clone(),
                latest: self.ts64,
                oldest: self.ts64,
                limit: 1,
                inclusive: true,
            })
            .await?;
        let mut body = match history {
            Some(messages) => messages.into_iter().filter_map(|m| m.text).collect::<Vec<String>>(),
            None => {
                bail!("No messages found")
            }
        };

        if body.join("").is_empty() {
            let replies = client
                .conversations_replies(&RepliesQuery {
                    channel: self.channel_id.clone(),
                    ts: self.thread_ts64.unwrap_or(self.ts64),
                    latest: self.ts64,
                    oldest: self.ts64,
                    limit: 1,
                    inclusive: true,
                })
                .await?;
            body = match replies {
                Some(replies) => {
                    replies.into_iter().filter_map(|m| m.text).collect::<Vec<String>>()
                }
                None => bail!("No messages found"),
            }
        }

        Ok(SlackMessage {
            state: Resolved {
                url: self.url,
                channel_name,
                body: body.into_iter().last().unwrap_or("".to_string()),
                ts: self.ts.parse::<i64>()?,
            },
        })
    }

    fn parse(text: &str) -> Result<(String, String, f64, Option<f64>)> {
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

        let (ts, ts64) = Self::convert_to_ts(ts)?;

        let params: QueryParams =
            serde_qs::Config::new(5, false).deserialize_str(url.query().unwrap_or(""))?;

        Ok((channel_id, ts, ts64, params.thread_ts))
    }

    fn convert_to_ts(input: &str) -> Result<(String, f64), ParseFloatError> {
        let numeric_part = input.trim_start_matches(|c: char| !c.is_numeric());
        let (int_part, decimal_part) = numeric_part.split_at(numeric_part.len() - 6);
        let ts64 = format!("{int_part}.{decimal_part}").parse::<f64>()?;
        Ok((numeric_part.to_string(), ts64))
    }
}
