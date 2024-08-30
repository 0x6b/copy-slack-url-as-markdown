use anyhow::{bail, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::from_str;
use serde_qs::to_string;

use crate::{
    request::{
        bots::BotsQuery, conversations::ConversationsQuery, usergroups::UsergroupsQuery,
        users::UsersQuery, Request,
    },
    response::Response,
};

pub struct Client {
    endpoint: String,
    client: reqwest::Client,
}

impl Client {
    /// Create a new Slack API client.
    pub fn new(token: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .default_headers(HeaderMap::from_iter([
                (CONTENT_TYPE, HeaderValue::from_static("application/json")),
                (AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {token}"))?),
            ]))
            .build()?;
        Ok(Self { endpoint: "https://slack.com/api".into(), client })
    }

    /// https://api.slack.com/methods/users.* API
    pub async fn users<T>(&self, request: &T) -> Result<T::Response>
    where
        T: UsersQuery,
    {
        self.request(request).await
    }

    /// https://api.slack.com/methods/bots.* API
    pub async fn bots<T>(&self, request: &T) -> Result<T::Response>
    where
        T: BotsQuery,
    {
        self.request(request).await
    }

    /// https://api.slack.com/methods/conversations.* API
    pub async fn conversations<T>(&self, request: &T) -> Result<T::Response>
    where
        T: ConversationsQuery,
    {
        self.request(request).await
    }

    /// https://api.slack.com/methods/usergroups.* API
    pub async fn usergroups<T>(&self, request: &T) -> Result<T::Response>
    where
        T: UsergroupsQuery,
    {
        self.request(request).await
    }

    // Helper method to make a request with query `T`, and deserialize the response into
    // `T::Response`
    async fn request<T>(&self, request: &T) -> Result<T::Response>
    where
        T: Request,
    {
        let url = format!("{}/{}?{}", self.endpoint, request.path(), to_string(request)?);
        let response = self
            .client
            .request(request.method().into(), &url)
            .send()
            .await?
            .text()
            .await?;

        // println!("Request: {} {}", request.method(), url);
        // println!("Response: {}", response);

        let result = from_str::<T::Response>(&response)?;

        if result.is_ok() {
            Ok(result)
        } else {
            bail!("Request failed: {}", response);
        }
    }
}
