pub mod conversations;
pub mod usergroups;
pub mod users;

use serde::Serialize;

use crate::response::Response;

/// A trait for a query to the Slack API, which defines the path to the endpoint and the response
/// type as its associated type.
pub trait Query: Serialize {
    type Response: Response;

    fn path(&self) -> &'static str;
}
