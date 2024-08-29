use serde::Serialize;

use crate::{query::Query, response::usergroups::UsergroupsList};

/// A marker trait which denotes a query for the `usergroups` API.
pub trait UsergroupsQuery: Query {}

/// A query for `usergroups.list` API. No parameters.
///
/// See: https://api.slack.com/methods/usergroups.list
#[derive(Serialize)]
pub struct List {}
impl UsergroupsQuery for List {}
impl Query for List {
    type Response = UsergroupsList;

    fn path(&self) -> &'static str {
        "usergroups.list"
    }
}
