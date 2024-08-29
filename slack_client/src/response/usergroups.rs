use serde::Deserialize;

use crate::response::Response;

#[derive(Deserialize, Debug)]
pub struct UsergroupsList {
    pub ok: bool,
    pub usergroups: Option<Vec<Usergroup>>,
}
impl Response for UsergroupsList {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Deserialize, Debug)]
pub struct Usergroup {
    /// The ID of the usergroup.
    pub id: String,
    /// The name of the usergroup.
    pub handle: String,
}
