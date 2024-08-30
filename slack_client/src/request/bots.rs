use serde::Serialize;

use crate::{request::Request, response::bots::BotsInfo};

/// A marker trait which denotes a request for the `users` API.
pub trait BotsQuery: Request {}

/// A request for `users.info` API.
///
/// See: https://api.slack.com/methods/users.info
#[derive(Serialize, Debug)]
pub struct Info<'a> {
    /// User ID to get info on
    #[serde(rename = "bot")]
    pub id: &'a str,
}
impl<'a> BotsQuery for Info<'a> {}
impl<'a> Request for Info<'a> {
    type Response = BotsInfo;

    fn path(&self) -> &'static str {
        "bots.info"
    }
}
