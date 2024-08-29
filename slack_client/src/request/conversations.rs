use serde::Serialize;

use crate::{
    request::Request,
    response::conversations::{Conversations, ConversationsInfo},
};

/// A marker trait which denotes a request for the `conversations` API.
pub trait ConversationsQuery: Request {}

/// A request for `conversations.info` API.
///
/// See: https://api.slack.com/methods/conversations.info
#[derive(Serialize)]
pub struct Info<'a> {
    /// Conversation ID to learn more about.
    pub channel: &'a str,
}
impl<'a> ConversationsQuery for Info<'a> {}
impl<'a> Request for Info<'a> {
    type Response = ConversationsInfo;

    fn path(&self) -> &'static str {
        "conversations.info"
    }
}

/// A request for `conversations.history` API.
///
/// See: https://api.slack.com/methods/conversations.history
#[derive(Serialize)]
pub struct History<'a> {
    /// Conversation ID to fetch history for.
    pub channel: &'a str,
    /// Only messages before this Unix timestamp will be included in results.
    pub latest: f64,
    /// Only messages after this Unix timestamp will be included in results.
    pub oldest: f64,
    /// The maximum number of items to return. Fewer than the requested number of items may be
    /// returned, even if the end of the conversation history hasn't been reached. Maximum of 999.
    pub limit: u64,
    /// Include messages with `oldest` or `latest` timestamps in results. Ignored unless either
    /// timestamp is specified.
    pub inclusive: bool,
}
impl<'a> ConversationsQuery for History<'a> {}
impl<'a> Request for History<'a> {
    type Response = Conversations;

    fn path(&self) -> &'static str {
        "conversations.history"
    }
}

/// A request for `conversations.replies` API.
///
/// See: https://api.slack.com/methods/conversations.replies
#[derive(Serialize)]
pub struct Replies<'a> {
    /// Conversation ID to fetch thread from.
    pub channel: &'a str,
    /// Unique identifier of either a thread’s parent message or a message in the thread. `ts` must
    /// be the timestamp of an existing message with 0 or more replies. If there are no replies
    /// then just the single message referenced by `ts` will return - it is just an ordinary,
    /// unthreaded message.
    pub ts: f64,
    /// Only messages before this Unix timestamp will be included in results.
    pub latest: f64,
    /// Only messages after this Unix timestamp will be included in results.
    pub oldest: f64,
    /// The maximum number of items to return. Fewer than the requested number of items may be
    /// returned, even if the end of the users list hasn't been reached.
    pub limit: u64,
    /// Include messages with `oldest` or `latest` timestamps in results. Ignored unless either
    /// timestamp is specified.
    pub inclusive: bool,
}
impl<'a> ConversationsQuery for Replies<'a> {}
impl<'a> Request for Replies<'a> {
    type Response = Conversations;

    fn path(&self) -> &'static str {
        "conversations.replies"
    }
}
