use serde::{de::DeserializeOwned, Deserialize};

// Just an alias for `serde::de::DeserializeOwned`
#[allow(dead_code)]
pub trait Response: DeserializeOwned {}

#[derive(Deserialize)]
pub struct Info {
    pub channel: Channel,
}

impl Response for Info {}

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
    pub text: Option<String>,
}
