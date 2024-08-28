use serde::Serialize;

use crate::slack::{query::Query, response::users::UsersInfo};

/// A marker trait which denotes a query for the `users` API.
pub trait UsersQuery: Query {}

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
impl<'a> Query for Info<'a> {
    type Response = UsersInfo;

    fn path(&self) -> &'static str {
        "users.info"
    }
}
