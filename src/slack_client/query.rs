use serde::Serialize;

pub trait Query: Serialize {
    type Response: crate::slack_client::response::Response;
    fn path(&self) -> &'static str;
}

#[derive(Serialize)]
pub struct Info<'a> {
    pub channel: &'a str,
}

impl<'a> Query for Info<'a> {
    type Response = crate::slack_client::response::Info;

    fn path(&self) -> &'static str {
        "conversations.info"
    }
}

#[derive(Serialize)]
pub struct History<'a> {
    pub channel: &'a str,
    pub latest: f64,
    pub oldest: f64,
    pub limit: u64,
    pub inclusive: bool,
}

impl<'a> Query for History<'a> {
    type Response = crate::slack_client::response::Conversations;

    fn path(&self) -> &'static str {
        "conversations.history"
    }
}

#[derive(Serialize)]
pub struct Replies<'a> {
    pub channel: &'a str,
    pub ts: f64,
    pub latest: f64,
    pub oldest: f64,
    pub limit: u64,
    pub inclusive: bool,
}

impl<'a> Query for Replies<'a> {
    type Response = crate::slack_client::response::Conversations;

    fn path(&self) -> &'static str {
        "conversations.replies"
    }
}
