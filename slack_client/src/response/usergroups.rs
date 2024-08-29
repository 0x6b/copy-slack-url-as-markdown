use serde::Deserialize;

use crate::response::Response;

#[derive(Deserialize, Debug)]
pub struct UsergroupsList {
    pub usergroups: Vec<Usergroup>,
}
impl Response for UsergroupsList {}

#[derive(Deserialize, Debug)]
pub struct Usergroup {
    /// The ID of the usergroup.
    pub id: String,
    /// The name of the usergroup.
    pub handle: String,
}
