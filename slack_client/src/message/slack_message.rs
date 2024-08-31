use std::ops::{Deref, DerefMut};

use anyhow::{anyhow, bail, Result};
use url::Url;

use crate::{
    message::{
        state::{Initialized, Resolved, State, Uninitialized},
        QueryParams, RE_CHANNEL, RE_LINK, RE_SPECIAL_MENTION, RE_USER, RE_USERGROUP,
    },
    request::{bots, conversations, usergroups, users},
    response::{conversations::Message, users::User},
    Client, Emojify,
};

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

impl<'a> SlackMessage<Uninitialized<'_>> {
    /// Create a new Slack message with the given URL and token.
    ///
    /// # Arguments
    ///
    /// - `url` - The URL of the message.
    /// - `token` - The Slack API token.
    pub fn try_new(url: &'a Url, token: &'a str) -> Result<SlackMessage<Initialized<'a>>> {
        if !url.domain().unwrap_or_default().ends_with("slack.com") {
            bail!("No Slack URL: {url}");
        }

        let (channel_id, ts, ts64, thread_ts64) = Self::parse(url)?;
        Ok(SlackMessage {
            state: Initialized {
                url,
                channel_id,
                ts,
                ts64,
                thread_ts64,
                client: Client::new(token)?,
                usergroups: None,
            },
        })
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

impl SlackMessage<Initialized<'_>> {
    /// Resolve the channel name, user name, and the body of the message with given Slack API token.
    ///
    /// # Arguments
    ///
    /// - `token` - The Slack API token.
    ///
    /// # Reference
    ///
    /// [Formatting text for app surfaces | Slack](https://api.slack.com/reference/surfaces/formatting)
    ///
    /// ## Notes on retrieving formatted messages
    ///
    /// If you're [retrieving messages](https://api.slack.com/messaging/retrieving), we've included some extra details in the sections above to help you parse the formatting syntax. This will allow you to properly format it for display on a different service, or to help your app fully understand the intent of a message. Here are the general steps involved in detecting advanced formatting syntax:
    ///
    /// 1. Detect all sub-strings matching `<(.*?)>`.
    /// 2. Within those sub-strings, format content starting with `#C` as a [channel link](https://api.slack.com/reference/surfaces/formatting#linking-channels).
    /// 3. Format content starting with `@U` or `@W` as a [user mention](https://api.slack.com/reference/surfaces/formatting#mentioning-users).
    /// 4. Format content starting with `!subteam` as a [user group mention](https://api.slack.com/reference/surfaces/formatting#mentioning-groups).
    /// 5. Format content starting with `!` according to the rules for [special mentions](https://api.slack.com/reference/surfaces/formatting#special-mentions).
    /// 6. For any other content within those sub-strings, format as a [URL link](https://api.slack.com/reference/surfaces/formatting#linking-urls).
    /// 7. Once the format has been determined, check for a pipe (`|`) \- if present, use the text
    ///    following the pipe as the label for the link or mention.
    pub async fn resolve(&mut self) -> Result<SlackMessage<Resolved>> {
        let channel_name = self.get_channel_name().await?;
        let messages = self.get_messages().await?;
        let user_name = self.determine_user_name(&messages).await?;
        let body = self.messages_to_body(&messages);
        let body = self.replace_channel_ids(&body).await?; // 2
        let body = self.replace_user_ids(&body).await?; // 3
        let body = self.replace_usergroups_ids(&body).await?; // 4
        let body = self.replace_special_mentions(&body)?; // 5
        let body = self.replace_links(&body)?; // 6 and 7

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

    /// Get the channel name from the Slack API. The name will be different based on the
    /// conversation type.
    ///
    /// - If the conversation is a direct message, then the name will be the display name of the
    ///   user.
    /// - If the conversation is a multi-party direct message, then the name will be the purpose of
    ///   the conversation.
    /// - The name will be the normalized name of the channel otherwise.
    async fn get_channel_name(&self) -> Result<String> {
        let channel = match self
            .client
            .conversations(&conversations::Info { channel: self.channel_id })
            .await?
            .channel
        {
            Some(channel) => channel,
            None => bail!("Channel not found: {}", self.channel_id),
        };

        if channel.is_im.unwrap_or_default() {
            let user = match self
                .client
                .users(&users::Info { id: &channel.user.unwrap_or_default() })
                .await?
                .user
            {
                Some(user) => user.profile.display_name,
                None => "UNKNOWN".to_string(),
            };
            return Ok(format!("DM with {user}"));
        }

        if channel.is_mpim.unwrap_or_default() {
            return match channel.purpose {
                Some(purpose) => Ok(purpose.value),
                None => Ok("UNKNOWN".to_string()),
            };
        }

        Ok(channel.name_normalized.unwrap_or_else(|| "UNKNOWN".to_string()))
    }

    /// Get the messages from the Slack API. If the message didn't send to the main channel, the
    /// response of the `conversation.history` will be blank. I'm not sure why. Try to fetch using
    /// `conversation.replies` API instead.
    async fn get_messages(&self) -> Result<Vec<Message>> {
        let history = self
            .client
            .conversations(&conversations::History {
                channel: self.channel_id,
                latest: self.ts64,
                oldest: self.ts64,
                limit: 1,
                inclusive: true,
            })
            .await?
            .messages;

        if let Some(messages) = history {
            if !messages.is_empty() {
                return Ok(messages);
            }

            let messages = self
                .client
                .conversations(&conversations::Replies {
                    channel: self.channel_id,
                    ts: self.thread_ts64.unwrap_or(self.ts64),
                    latest: self.ts64,
                    oldest: self.ts64,
                    limit: 1,
                    inclusive: true,
                })
                .await?
                .messages;

            if let Some(messages) = messages {
                if !messages.is_empty() {
                    return Ok(messages);
                }
            }
        }

        bail!("No messages found")
    }

    /// Determine the user name from the messages. If the message is from a user, then get the user
    /// name from the user ID. If the message is from a bot, then get the bot name from the bot ID.
    async fn determine_user_name(&self, messages: &[Message]) -> Result<String> {
        let user_id = &messages.last().unwrap().user;
        let bot_id = &messages.last().unwrap().bot_id;

        // If user ID is there, use it or die.
        if let Some(id) = user_id {
            match self.client.users(&users::Info { id }).await?.user {
                Some(user) => return Ok(self.get_user_name(user)),
                None => bail!("User not found: {id:?}"),
            }
        }

        // If bot ID is there, use it or die.
        if let Some(id) = bot_id {
            match self.client.bots(&bots::Info { id }).await?.bot {
                Some(bot) => return Ok(bot.name),
                None => bail!("Bot not found: {id:?}"),
            }
        }

        bail!("No user or bot found");
    }

    /// Convert the messages to the body of the message. If the message contains blocks, then
    /// convert the blocks to the string. Otherwise, return the text of the message.
    fn messages_to_body(&self, messages: &[Message]) -> String {
        messages
            .iter()
            .flat_map(|m| match &m.blocks {
                Some(blocks) => blocks.iter().map(|b| b.to_string()).collect::<Vec<String>>(),
                None => vec![m.text.clone().unwrap_or_default()],
            })
            .last()
            .unwrap_or("".to_string())
            .emojify()
    }

    /// Replace the channel (`<#CID>`) to the actual channel name.
    async fn replace_channel_ids(&self, body: &str) -> Result<String> {
        let mut new_text = String::with_capacity(body.len());
        let mut last = 0;

        for cap in RE_CHANNEL.captures_iter(body) {
            if let Some(m) = cap.get(1) {
                if let Ok(response) = self
                    .client
                    .conversations(&conversations::Info { channel: m.as_str() })
                    .await
                {
                    if let Some(channel) = response.channel {
                        new_text.push_str(&body[last..m.start().saturating_sub(2)]); // remove the `<#`
                        new_text.push_str("**#");
                        new_text.push_str(
                            &channel.name_normalized.unwrap_or_else(|| "Unknown".to_string()),
                        );
                        new_text.push_str("**");
                        last = m.end().saturating_add(match cap.get(2) {
                            Some(s) => s.as_str().len() + 1,
                            None => 1,
                        }); // remove the `(|.*)?>`
                    }
                } else {
                    println!("Failed to get channel: {}", m.as_str());
                    new_text.push_str(&body[last..m.start().saturating_sub(2)]); // remove the `<#`
                    new_text.push_str("**#private channel**");
                    last = m.end().saturating_add(match cap.get(2) {
                        Some(s) => s.as_str().len() + 1,
                        None => 1,
                    });
                }
            }
        }
        new_text.push_str(&body[last..]);
        Ok(new_text)
    }

    /// Replace the user mentions (`<@ID>`) to the actual user name.
    async fn replace_user_ids(&self, body: &str) -> Result<String> {
        let mut new_text = String::with_capacity(body.len());
        let mut last = 0;

        for cap in RE_USER.captures_iter(body) {
            if let Some(m) = cap.get(1) {
                if let Ok(response) = self.client.users(&users::Info { id: m.as_str() }).await {
                    if let Some(user) = response.user {
                        new_text.push_str(&body[last..m.start().saturating_sub(2)]); // remove the `<@`
                        new_text.push_str("**@");
                        new_text.push_str(&self.get_user_name(user));
                        new_text.push_str("**");
                        last = m.end().saturating_add(1); // remove the `>`
                    }
                }
            }
        }
        new_text.push_str(&body[last..]);
        Ok(new_text)
    }

    /// Replace the usergroup mentions (`<!subteam^ID>`) to the actual usergroup handle.
    async fn replace_usergroups_ids(&mut self, body: &str) -> Result<String> {
        let mut new_text = String::with_capacity(body.len());
        let mut last = 0;

        for cap in RE_USERGROUP.captures_iter(body) {
            if self.usergroups.as_ref().is_none() {
                self.usergroups =
                    Some(match self.client.usergroups(&usergroups::List {}).await?.usergroups {
                        Some(list) => list,
                        None => bail!("Failed to get usergroups"),
                    });
            }

            if let Some(m) = cap.get(1) {
                if let Some(list) = self.usergroups.as_ref() {
                    let group_handle = list.iter().find(|g| g.id == m.as_str());
                    if let Some(handle) = group_handle {
                        new_text.push_str(&body[last..m.start().saturating_sub(10)]); // remove the `<subteam^`
                        new_text.push_str("**@");
                        new_text.push_str(&handle.handle);
                        new_text.push_str("**");
                        last = m.end().saturating_add(1); // remove the `>`
                    }
                }
            }
        }
        new_text.push_str(&body[last..]);
        Ok(new_text)
    }

    /// Replace special mentions.
    fn replace_special_mentions(&self, body: &str) -> Result<String> {
        let mut new_text = String::with_capacity(body.len());
        let mut last = 0;

        for cap in RE_SPECIAL_MENTION.captures_iter(body) {
            if let Some(m) = cap.get(1) {
                new_text.push_str(&body[last..m.start().saturating_sub(2)]); // remove the `<@`
                new_text.push_str("**@");
                new_text.push_str(m.as_str());
                new_text.push_str("**");
                last = m.end().saturating_add(1); // remove the `>`
            }
        }
        new_text.push_str(&body[last..]);
        Ok(new_text)
    }

    /// Replace the mrkdwn format of the links (`<url|title>`) to the markdown format
    /// (`[title](url)`). Actually, this is not necessary because the
    /// `response.conversations#to_string()` will convert the links to the markdown format.
    fn replace_links(&self, body: &str) -> Result<String> {
        let mut new_text = String::with_capacity(body.len());
        let mut last = 0;

        for cap in RE_LINK.captures_iter(body) {
            if let (Some(url), Some(title)) = (cap.get(1), cap.get(2)) {
                new_text.push_str(&body[last..url.start().saturating_sub(1)]); // remove the `<`
                new_text.push('[');
                new_text.push_str(title.as_str());
                new_text.push_str(r#"]("#);
                new_text.push_str(url.as_str());
                new_text.push(')');
                last = title.end().saturating_add(1); // remove the `>`
            }
        }
        new_text.push_str(&body[last..]);
        Ok(new_text)
    }

    /// Naive implementation to get the username.
    fn get_user_name(&self, user: User) -> String {
        if user.is_bot {
            user.real_name.unwrap_or(user.name)
        } else if user.profile.display_name.is_empty() {
            user.name
        } else {
            user.profile.display_name
        }
    }
}
