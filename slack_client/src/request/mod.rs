pub mod conversations;
pub mod usergroups;
pub mod users;

use reqwest::Method;
use serde::Serialize;

use crate::response::Response;

/// An enum representing the HTTP request method.
pub enum RequestMethod {
    Get,
    Post,
}

impl From<RequestMethod> for Method {
    fn from(method: RequestMethod) -> Self {
        match method {
            RequestMethod::Get => Method::GET,
            RequestMethod::Post => Method::POST,
        }
    }
}

/// A trait for a request to the Slack API, which defines the path to the endpoint and the response
/// type as its associated type.
pub trait Request: Serialize {
    type Response: Response;

    /// Returns the path to the endpoint.
    fn path(&self) -> &'static str;

    /// Returns the HTTP request method.
    fn method(&self) -> RequestMethod {
        RequestMethod::Get
    }
}
