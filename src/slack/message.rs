use std::{num::ParseFloatError, ops::Deref};

use anyhow::{anyhow, bail, Error, Result};
use serde::Deserialize;
use url::Url;

use crate::slack::{
    query::{
        conversations::{History, Info as ConversationsInfo, Replies},
        users::Info as UsersInfo,
    },
    response::conversations::Message,
    Client, Emojify,
};

#[derive(Deserialize, Debug, Clone, Copy)]
struct QueryParams {
    thread_ts: Option<f64>,
}

pub trait State {}
#[derive(Debug)]
pub struct Initialized<'a> {
    pub url: &'a Url,
    pub channel_id: &'a str,
    pub ts: &'a str,
    pub ts64: f64,
    pub thread_ts64: Option<f64>,
}

impl<'a> State for Initialized<'a> {}

#[derive(Debug)]
pub struct Resolved<'a> {
    pub url: &'a Url,
    pub channel_name: String,
    pub user_name: String,
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

impl<'a> TryFrom<&'a Url> for SlackMessage<Initialized<'a>> {
    type Error = Error;

    fn try_from(url: &'a Url) -> Result<SlackMessage<Initialized<'a>>> {
        let (channel_id, ts, ts64, thread_ts64) = Self::parse(url)?;
        Ok(SlackMessage {
            state: Initialized { url, channel_id, ts, ts64, thread_ts64 },
        })
    }
}

impl SlackMessage<Initialized<'_>> {
    pub async fn resolve(&self, token: &str) -> Result<SlackMessage<Resolved>> {
        let client = Client::new(token)?;

        let channel_name = client
            .conversations::<_>(&ConversationsInfo { channel: self.channel_id })
            .await?
            .channel
            .name_normalized;

        let history = client
            .conversations(&History {
                channel: self.channel_id,
                latest: self.ts64,
                oldest: self.ts64,
                limit: 1,
                inclusive: true,
            })
            .await?
            .messages;

        let get_id_and_body = |messages: Vec<Message>| {
            let user = messages.last().unwrap().user.clone();
            let messages = messages.into_iter().filter_map(|m| m.text).collect::<Vec<String>>();
            (user, messages)
        };

        let (id, body) = match history {
            Some(messages) if !messages.is_empty() => get_id_and_body(messages),
            Some(_) => {
                // If the message didn't send to the main channel, the response of the
                // conversation.history will be blank. I'm not sure why. Try to
                // fetch using conversation.replies
                let messages = client
                    .conversations(&Replies {
                        channel: self.channel_id,
                        ts: self.thread_ts64.unwrap_or(self.ts64),
                        latest: self.ts64,
                        oldest: self.ts64,
                        limit: 1,
                        inclusive: true,
                    })
                    .await?
                    .messages;
                match messages {
                    Some(messages) => get_id_and_body(messages),
                    None => bail!("No messages found"),
                }
            }
            None => {
                bail!("No messages found")
            }
        };

        let user = client.users(&UsersInfo { id: &id }).await?.user;

        Ok(SlackMessage {
            state: Resolved {
                url: self.url,
                channel_name,
                user_name: if user.profile.display_name.is_empty() {
                    user.name
                } else {
                    user.profile.display_name
                },
                body: body.into_iter().last().unwrap_or("".to_string()).emojify(),
                ts: self.ts.parse::<i64>()?,
            },
        })
    }

    fn parse(url: &Url) -> Result<(&str, &str, f64, Option<f64>)> {
        let channel_id = url
            .path_segments()
            .ok_or(anyhow!("Failed to get path segments"))?
            .nth(1)
            .ok_or(anyhow!("Failed to get the last path segment"))?;
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

    fn convert_to_ts(input: &str) -> Result<(&str, f64), ParseFloatError> {
        let num = input.trim_start_matches(|c: char| !c.is_numeric());
        let (int_part, decimal_part) = num.split_at(num.len() - 6);
        let ts64 = format!("{int_part}.{decimal_part}").parse::<f64>()?;
        Ok((num, ts64))
    }
}
