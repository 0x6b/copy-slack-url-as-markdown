use serde::Serialize;

use crate::{request::Request, response::usergroups::UsergroupsList};

/// A marker trait which denotes a request for the `usergroups` API.
pub trait UsergroupsQuery: Request {}

/// A request for `usergroups.list` API. No parameters.
///
/// See: https://api.slack.com/methods/usergroups.list
#[derive(Serialize, Debug)]
pub struct List {}
impl UsergroupsQuery for List {}
impl Request for List {
    type Response = UsergroupsList;

    fn path(&self) -> &'static str {
        "usergroups.list"
    }
}
