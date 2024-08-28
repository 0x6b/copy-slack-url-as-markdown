use serde::Serialize;

pub trait Query: Serialize {
    type Response: crate::slack::response::Response;

    fn path(&self) -> &'static str;
}

pub trait ConversationsQuery: Query {}
pub trait UsersQuery: Query {}

// https://api.slack.com/methods/conversations.info
#[derive(Serialize)]
pub struct ConversationsInfo<'a> {
    pub channel: &'a str,
}

impl<'a> Query for ConversationsInfo<'a> {
    type Response = crate::slack::response::ConversationsInfo;

    fn path(&self) -> &'static str {
        "conversations.info"
    }
}

impl<'a> ConversationsQuery for ConversationsInfo<'a> {}

// https://api.slack.com/methods/conversations.history
#[derive(Serialize)]
pub struct ConversationsHistory<'a> {
    pub channel: &'a str,
    pub latest: f64,
    pub oldest: f64,
    pub limit: u64,
    pub inclusive: bool,
}

impl<'a> Query for ConversationsHistory<'a> {
    type Response = crate::slack::response::Conversations;

    fn path(&self) -> &'static str {
        "conversations.history"
    }
}
impl<'a> ConversationsQuery for ConversationsHistory<'a> {}

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

impl<'a> Query for ConversationsReplies<'a> {
    type Response = crate::slack::response::Conversations;

    fn path(&self) -> &'static str {
        "conversations.replies"
    }
}
impl<'a> ConversationsQuery for ConversationsReplies<'a> {}

// https://api.slack.com/methods/users.info
#[derive(Serialize)]
pub struct UsersInfo<'a> {
    #[serde(rename = "user")]
    pub id: &'a str,
}

impl<'a> Query for UsersInfo<'a> {
    type Response = crate::slack::response::UsersInfo;

    fn path(&self) -> &'static str {
        "users.info"
    }
}

impl<'a> UsersQuery for UsersInfo<'a> {}
