use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_qs::to_string;

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

    pub async fn conversations_info(&self, query: &InfoQuery<'_>) -> Result<Channel> {
        Ok(self.request::<InfoQuery>(query).await?.channel)
    }

    pub async fn conversations_history(
        &self,
        query: &HistoryQuery<'_>,
    ) -> Result<Option<Vec<Message>>> {
        Ok(self.request::<HistoryQuery>(query).await?.messages)
    }

    pub async fn conversations_replies(
        &self,
        query: &RepliesQuery<'_>,
    ) -> Result<Option<Vec<Message>>> {
        Ok(self.request::<RepliesQuery>(query).await?.messages)
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

pub trait Query: Serialize {
    type Response: Response;
    fn path(&self) -> &'static str;
}

// Just an alias for `serde::de::DeserializeOwned`
#[allow(dead_code)]
pub trait Response: DeserializeOwned {}

#[derive(Serialize)]
pub struct InfoQuery<'a> {
    pub channel: &'a str,
}

impl<'a> Query for InfoQuery<'a> {
    type Response = InfoResponse;

    fn path(&self) -> &'static str {
        "conversations.info"
    }
}

#[derive(Deserialize)]
pub struct InfoResponse {
    pub channel: Channel,
}

impl Response for InfoResponse {}

#[derive(Deserialize)]
pub struct Channel {
    pub name_normalized: String,
}

#[derive(Serialize)]
pub struct HistoryQuery<'a> {
    pub channel: &'a str,
    pub latest: f64,
    pub oldest: f64,
    pub limit: u64,
    pub inclusive: bool,
}

impl<'a> Query for HistoryQuery<'a> {
    type Response = ConversationsResponse;

    fn path(&self) -> &'static str {
        "conversations.history"
    }
}

#[derive(Serialize)]
pub struct RepliesQuery<'a> {
    pub channel: &'a str,
    pub ts: f64,
    pub latest: f64,
    pub oldest: f64,
    pub limit: u64,
    pub inclusive: bool,
}

impl<'a> Query for RepliesQuery<'a> {
    type Response = ConversationsResponse;

    fn path(&self) -> &'static str {
        "conversations.replies"
    }
}

#[derive(Deserialize)]
pub struct ConversationsResponse {
    pub messages: Option<Vec<Message>>,
}

impl Response for ConversationsResponse {}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub text: Option<String>,
}
