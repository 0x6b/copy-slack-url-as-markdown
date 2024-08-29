use serde::Deserialize;

use crate::response::Response;

#[derive(Deserialize, Debug)]
pub struct UsersInfo {
    pub user: User,
}
impl Response for UsersInfo {}

#[derive(Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub real_name: String,
    pub profile: Profile,
    pub is_bot: bool,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub display_name: String,
}
