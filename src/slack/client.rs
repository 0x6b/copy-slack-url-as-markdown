use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_qs::to_string;

use crate::slack::{
    query::{History, Info, Query, Replies},
    response::{Channel, Message},
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

    // https://api.slack.com/methods/conversations.info
    pub async fn conversations_info(&self, query: &Info<'_>) -> Result<Channel> {
        Ok(self.request::<Info>(query).await?.channel)
    }

    // https://api.slack.com/methods/conversations.history
    pub async fn conversations_history(&self, query: &History<'_>) -> Result<Option<Vec<Message>>> {
        Ok(self.request::<History>(query).await?.messages)
    }

    // https://api.slack.com/methods/conversations.replies
    pub async fn conversations_replies(&self, query: &Replies<'_>) -> Result<Option<Vec<Message>>> {
        Ok(self.request::<Replies>(query).await?.messages)
    }

    // Helper method to make a request and deserialize the response into `T`
    async fn request<T>(&self, query: &T) -> Result<T::Response>
    where
        T: Query,
    {
        let response = self
            .client
            .get(&format!("{}/{}?{}", self.endpoint, query.path(), to_string(query)?))
            .send()
            .await?
            .json::<T::Response>()
            .await?;

        Ok(response)
    }
}
