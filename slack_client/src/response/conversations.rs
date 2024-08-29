use serde::Deserialize;

use crate::response::Response;

#[derive(Deserialize)]
pub struct ConversationsInfo {
    pub channel: Channel,
}
impl Response for ConversationsInfo {}

#[derive(Deserialize)]
pub struct Channel {
    pub name_normalized: String,
}

#[derive(Deserialize)]
pub struct Conversations {
    pub messages: Option<Vec<Message>>,
}
impl Response for Conversations {}

#[derive(Deserialize, Debug)]
pub struct Message {
    /// User ID of the author.
    pub user: String,
    /// The text of the message.
    pub text: Option<String>,
}
