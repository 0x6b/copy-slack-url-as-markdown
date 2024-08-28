use serde::{de::DeserializeOwned, Deserialize};

// Just an alias for `serde::de::DeserializeOwned`
#[allow(dead_code)]
pub trait Response: DeserializeOwned {}

#[derive(Deserialize)]
pub struct ConversationsInfo {
    pub channel: Channel,
}

impl Response for ConversationsInfo {}

#[derive(Deserialize)]
pub struct Channel {
    pub name_normalized: String,
}

#[derive(Deserialize)]
pub struct Conversations {
    pub messages: Option<Vec<Message>>,
}

impl Response for Conversations {}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub user: String,
    pub text: Option<String>,
}

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
