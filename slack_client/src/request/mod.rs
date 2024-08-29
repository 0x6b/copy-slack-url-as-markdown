pub mod conversations;
pub mod usergroups;
pub mod users;

use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
};

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

impl Display for RequestMethod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RequestMethod::Get => write!(f, "GET"),
            RequestMethod::Post => write!(f, "POST"),
        }
    }
}

impl Debug for RequestMethod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// A trait for a request to the Slack API, which defines the path to the endpoint and the response
/// type as its associated type.
pub trait Request: Serialize + Debug {
    type Response: Response;

    /// Returns the path to the endpoint.
    fn path(&self) -> &'static str;

    /// Returns the HTTP request method.
    fn method(&self) -> RequestMethod {
        RequestMethod::Get
    }
}
