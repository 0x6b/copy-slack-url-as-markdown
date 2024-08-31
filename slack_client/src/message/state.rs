use url::Url;

use crate::{response::usergroups::Usergroup, Client};

/// A marker trait for the state of a Slack message.
///
/// This trait is used to ensure that the message is in a valid state before performing any
/// operations.
///
/// Possible states are:
///
/// - `Uninitialized`: Just the URL. No content.
/// - `Initialized`: The message has been initialized with the URL, channel ID, and timestamp. No
///   content, and the API client is ready.
/// - `Resolved`: The message has been retrieved and resolved with the channel name, user name, and
///   message body.
pub trait State {}
impl<'a> State for Uninitialized<'a> {}
impl<'a> State for Initialized<'a> {}
impl<'a> State for Resolved<'a> {}

#[derive(Debug)]
pub struct Uninitialized<'a> {
    /// The plain URL.
    pub url: &'a Url,
}

#[derive(Debug)]
pub struct Initialized<'a> {
    /// The plain URL.
    pub url: &'a Url,
    /// The channel ID.
    pub channel_id: &'a str,
    /// The timestamp as &str.
    pub ts: &'a str,
    /// The timestamp as f64.
    pub ts64: f64,
    /// The thread timestamp as f64.
    pub thread_ts64: Option<f64>,
    /// The Slack API client.
    pub client: Client,
    /// Cache the usergroups to avoid fetching it multiple times, as there is no API to fetch a
    /// single usergroup.
    pub(crate) usergroups: Option<Vec<Usergroup>>,
}

#[derive(Debug)]
pub struct Resolved<'a> {
    /// The plain URL.
    pub url: &'a Url,
    /// The channel name.
    pub channel_name: String,
    /// The user name.
    pub user_name: String,
    /// The message body.
    pub body: String,
    /// The timestamp as f64.
    pub ts: i64,
}
