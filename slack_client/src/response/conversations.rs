use mrkdwn2markdown::Block;
use serde::Deserialize;

use crate::response::Response;

#[derive(Deserialize, Debug)]
pub struct ConversationsInfo {
    pub ok: bool,
    pub channel: Option<Channel>,
}
impl Response for ConversationsInfo {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Deserialize, Debug)]
pub struct Channel {
    pub is_im: Option<bool>,
    pub is_mpim: Option<bool>,
    pub name_normalized: Option<String>,
    pub purpose: Option<Purpose>,
    pub user: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Conversations {
    pub ok: bool,
    pub messages: Option<Vec<Message>>,
}
impl Response for Conversations {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Deserialize, Debug)]
pub struct Message {
    /// User ID of the author.
    pub user: Option<String>,
    /// bot ID of the author.
    pub bot_id: Option<String>,
    /// The text of the message.
    pub text: Option<String>,
    /// The Slack block kit blocks of the message.
    pub blocks: Option<Vec<Block>>,
}

#[derive(Deserialize, Debug)]
pub struct Purpose {
    pub value: String,
}
