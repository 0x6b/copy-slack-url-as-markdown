use url::Url;

use crate::response::usergroups::Usergroup;

pub trait State {}
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
    /// Cache the usergroups to avoid fetching it multiple times, as there is no API to fetch a
    /// single usergroup.
    pub(crate) usergroups: Option<Vec<Usergroup>>,
}

impl<'a> State for Initialized<'a> {}

#[derive(Debug)]
pub struct Resolved<'a> {
    pub url: &'a Url,
    pub channel_name: String,
    pub user_name: String,
    pub body: String,
    pub ts: i64,
}

impl<'a> State for Resolved<'a> {}
