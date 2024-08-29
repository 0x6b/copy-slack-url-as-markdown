use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::from_str;
use serde_qs::to_string;

use crate::request::{
    conversations::ConversationsQuery, usergroups::UsergroupsQuery, users::UsersQuery, Request,
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
    pub async fn users<T>(&self, query: &T) -> Result<T::Response>
    where
        T: UsersQuery,
    {
        self.request(query).await
    }

    /// https://api.slack.com/methods/conversations.* API
    pub async fn conversations<T>(&self, query: &T) -> Result<T::Response>
    where
        T: ConversationsQuery,
    {
        self.request(query).await
    }

    /// https://api.slack.com/methods/usergroups.* API
    pub async fn usergroups<T>(&self, query: &T) -> Result<T::Response>
    where
        T: UsergroupsQuery,
    {
        self.request(query).await
    }

    // Helper method to make a request with query `T`, and deserialize the response into
    // `T::Response`
    async fn request<T>(&self, query: &T) -> Result<T::Response>
    where
        T: Request,
    {
        let response = self
            .client
            .get(&format!("{}/{}?{}", self.endpoint, query.path(), to_string(query)?))
            .send()
            .await?
            .text()
            .await?;
        // println!("Response: {:?}", text);

        Ok(from_str::<T::Response>(&response)?)
    }
}
