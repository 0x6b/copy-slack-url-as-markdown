use serde::Deserialize;

use crate::response::Response;

#[derive(Deserialize, Debug)]
pub struct UsersInfo {
    pub ok: bool,
    /// The user object.
    pub user: Option<User>,
}
impl Response for UsersInfo {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Deserialize, Debug)]
pub struct User {
    /// The name of the user.
    pub name: String,
    /// The real name of the user.
    pub real_name: String,
    /// The profile object.
    pub profile: Profile,
    /// Whether the user is a bot.
    pub is_bot: bool,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    /// The display name, which you can see at the Slack client app.
    pub display_name: String,
}
