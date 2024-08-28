use serde::Deserialize;

use crate::slack::response::Response;

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
    pub user: String,
    pub text: Option<String>,
}
