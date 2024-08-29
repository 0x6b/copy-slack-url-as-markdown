use std::{
    ops::{Deref, DerefMut},
    sync::LazyLock,
};

use anyhow::{anyhow, bail, Error, Result};
use regex::Regex;
use serde::Deserialize;
use url::Url;

use crate::{
    request::{
        conversations::{History, Info as ConversationsInfo, Replies},
        usergroups::List,
        users::Info as UsersInfo,
    },
    response::{conversations::Message, usergroups::Usergroup, users::User},
    Client, Emojify,
};

static RE_USER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<@([A-Z0-9]+)>").unwrap());
static RE_USERGROUP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<!subteam\^([A-Z0-9]+)>").unwrap());
static RE_LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<([^|]+)\|([^>]+)?>").unwrap());

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
    // Cache the usergroups to avoid fetching it multiple times, as there is no API to fetch a
    // single usergroup
    usergroups: Option<Vec<Usergroup>>,
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

impl<S> DerefMut for SlackMessage<S>
where
    S: State,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl<'a> TryFrom<&'a Url> for SlackMessage<Initialized<'a>> {
    type Error = Error;

    fn try_from(url: &'a Url) -> Result<SlackMessage<Initialized<'a>>> {
        let (channel_id, ts, ts64, thread_ts64) = Self::parse(url)?;
        Ok(SlackMessage {
            state: Initialized {
                url,
                channel_id,
                ts,
                ts64,
                thread_ts64,
                usergroups: None,
            },
        })
    }
}

impl SlackMessage<Initialized<'_>> {
    /// Resolve the channel name, user name, and the body of the message.
    ///
    /// # Arguments
    ///
    /// - `token` - The Slack API token.
    pub async fn resolve(&mut self, token: &str) -> Result<SlackMessage<Resolved>> {
        let client = Client::new(token)?;

        let channel_name = match client
            .conversations(&ConversationsInfo { channel: self.channel_id })
            .await?
            .channel
        {
            Some(channel) => channel.name_normalized,
            None => bail!("Channel not found: {}", self.channel_id),
        };

        let (user_name, body) = self.get_user_name_and_body(&client).await?;
        let body = self.replace_user_ids(&client, &body).await?;
        let body = self.replace_usergroups_ids(&client, &body).await?;
        let body = self.replace_links(&body)?;

        Ok(SlackMessage {
            state: Resolved {
                url: self.url,
                channel_name,
                user_name,
                body,
                ts: self.ts.parse::<i64>()?,
            },
        })
    }

    /// Get the body of the message and the user name who posted the message.
    async fn get_user_name_and_body(&self, client: &Client) -> Result<(String, String)> {
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
            let id = messages.last().unwrap().user.clone();
            let body = messages.into_iter().filter_map(|m| m.text).collect::<Vec<String>>();
            (id, body)
        };

        let (user_id, body) = match history {
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

        let user = match client.users(&UsersInfo { id: &user_id }).await?.user {
            Some(user) => user,
            None => bail!("User not found: {}", user_id),
        };
        let user_name = self.get_user_name(user);

        Ok((user_name, body.into_iter().last().unwrap_or("".to_string()).emojify()))
    }

    /// Replace the user mentions (`<@ID>`) to the actual user name.
    async fn replace_user_ids(&self, client: &Client, body: &str) -> Result<String> {
        let mut new_text = String::with_capacity(body.len());
        let mut last = 0;

        for cap in RE_USER.captures_iter(body) {
            if let Some(m) = cap.get(1) {
                if let Ok(response) = client.users(&UsersInfo { id: m.as_str() }).await {
                    if let Some(user) = response.user {
                        new_text.push_str(&body[last..m.start().saturating_sub(2)]); // remove the `<@`
                        new_text.push('@');
                        new_text.push_str(&self.get_user_name(user));
                        last = m.end().saturating_add(1); // remove the `>`
                    }
                }
            }
        }
        new_text.push_str(&body[last..]);
        Ok(new_text)
    }

    /// Replace the usergroup mentions (`<!subteam^ID>`) to the actual usergroup handle.
    async fn replace_usergroups_ids(&mut self, client: &Client, body: &str) -> Result<String> {
        let mut new_text = String::with_capacity(body.len());
        let mut last = 0;

        for cap in RE_USERGROUP.captures_iter(body) {
            if self.usergroups.as_ref().is_none() {
                self.usergroups = Some(match client.usergroups(&List {}).await?.usergroups {
                    Some(list) => list,
                    None => bail!("Failed to get usergroups"),
                });
            }

            if let Some(m) = cap.get(1) {
                if let Some(list) = self.usergroups.as_ref() {
                    let group_handle = list.iter().find(|g| g.id == m.as_str());
                    if let Some(handle) = group_handle {
                        new_text.push_str(&body[last..m.start().saturating_sub(10)]); // remove the `<subteam^`
                        new_text.push('@');
                        new_text.push_str(&handle.handle);
                        last = m.end().saturating_add(1); // remove the `>`
                    }
                }
            }
        }
        new_text.push_str(&body[last..]);
        Ok(new_text)
    }

    /// Replace the mrkdown format of the links (`<url|title>`) to more readable format (`"title"
    /// <url>`) for both plain text and the HTML.
    fn replace_links(&self, body: &str) -> Result<String> {
        let mut new_text = String::with_capacity(body.len());
        let mut last = 0;

        for cap in RE_LINK.captures_iter(body) {
            if let (Some(url), Some(title)) = (cap.get(1), cap.get(2)) {
                new_text.push_str(&body[last..url.start().saturating_sub(1)]); // remove the `<`
                new_text.push('"');
                new_text.push_str(title.as_str());
                new_text.push_str(r#"" <"#);
                new_text.push_str(url.as_str());
                new_text.push('>');
                last = title.end().saturating_add(1); // remove the `>`
            }
        }
        new_text.push_str(&body[last..]);
        Ok(new_text)
    }

    /// Naive implementation to get the username. If the user is a bot, return the real name, else
    /// return the display name if it's not empty, otherwise return the name.
    fn get_user_name(&self, user: User) -> String {
        if user.is_bot {
            user.real_name
        } else if user.profile.display_name.is_empty() {
            user.name
        } else {
            user.profile.display_name
        }
    }

    /// Parse the given URL and return the channel ID, timestamp, and thread timestamp.
    ///
    /// # Arguments
    ///
    /// - `url` - The URL to parse.
    ///
    /// # Returns
    ///
    /// A tuple containing the channel ID (from path segments), timestamp as &str (from another path
    /// segment), timestamp in f64 (parsed the timestamp as f64), and thread timestamp (from query
    /// parameters).
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
        let num = ts.trim_start_matches(|c: char| !c.is_numeric());
        let (int_part, decimal_part) = num.split_at(num.len() - 6);
        let ts64 = format!("{int_part}.{decimal_part}").parse::<f64>()?;

        let params: QueryParams =
            serde_qs::Config::new(5, false).deserialize_str(url.query().unwrap_or(""))?;

        Ok((channel_id, num, ts64, params.thread_ts))
    }
}
