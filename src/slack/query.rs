use serde::Serialize;

pub trait ConversationsQuery: Serialize {
    type Response: crate::slack::response::Response;
    fn path(&self) -> &'static str;
}

// https://api.slack.com/methods/conversations.info
#[derive(Serialize)]
pub struct ConversationsInfo<'a> {
    pub channel: &'a str,
}

impl<'a> ConversationsQuery for ConversationsInfo<'a> {
    type Response = crate::slack::response::Info;

    fn path(&self) -> &'static str {
        "conversations.info"
    }
}

// https://api.slack.com/methods/conversations.history
#[derive(Serialize)]
pub struct ConversationsHistory<'a> {
    pub channel: &'a str,
    pub latest: f64,
    pub oldest: f64,
    pub limit: u64,
    pub inclusive: bool,
}

impl<'a> ConversationsQuery for ConversationsHistory<'a> {
    type Response = crate::slack::response::Conversations;

    fn path(&self) -> &'static str {
        "conversations.history"
    }
}

// https://api.slack.com/methods/conversations.replies
#[derive(Serialize)]
pub struct ConversationsReplies<'a> {
    pub channel: &'a str,
    pub ts: f64,
    pub latest: f64,
    pub oldest: f64,
    pub limit: u64,
    pub inclusive: bool,
}

impl<'a> ConversationsQuery for ConversationsReplies<'a> {
    type Response = crate::slack::response::Conversations;

    fn path(&self) -> &'static str {
        "conversations.replies"
    }
}
