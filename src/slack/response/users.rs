use serde::Deserialize;

use crate::slack::response::Response;

#[derive(Deserialize, Debug)]
pub struct UsersInfo {
    pub user: User,
}
impl Response for UsersInfo {}

#[derive(Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub profile: Profile,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub display_name: String,
}
