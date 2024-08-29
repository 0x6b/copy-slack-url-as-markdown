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
    pub name_normalized: String,
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
    pub user: String,
    /// The text of the message.
    pub text: Option<String>,
}
