use serde::Deserialize;

use crate::response::Response;

#[derive(Deserialize, Debug)]
pub struct UsergroupsList {
    pub usergroups: Vec<Usergroup>,
}
impl Response for UsergroupsList {}

#[derive(Deserialize, Debug)]
pub struct Usergroup {
    pub handle: String,
}
