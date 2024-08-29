pub mod conversations;
pub mod usergroups;
pub mod users;

/// A marker trait for a response from the Slack API, which is just an alias for
/// `serde::de::DeserializeOwned`. Other restrictions may be added in the future.
#[allow(dead_code)]
pub trait Response: serde::de::DeserializeOwned {}
