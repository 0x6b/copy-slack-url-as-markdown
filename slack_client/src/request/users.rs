use serde::Serialize;

use crate::{request::Request, response::users::UsersInfo};

/// A marker trait which denotes a query for the `users` API.
pub trait UsersQuery: Request {}

/// A query for `users.info` API.
///
/// See: https://api.slack.com/methods/users.info
#[derive(Serialize)]
pub struct Info<'a> {
    /// User ID to get info on
    #[serde(rename = "user")]
    pub id: &'a str,
}
impl<'a> UsersQuery for Info<'a> {}
impl<'a> Request for Info<'a> {
    type Response = UsersInfo;

    fn path(&self) -> &'static str {
        "users.info"
    }
}
