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
        Ok(self
            .request::<InfoQuery, InfoResponse>("conversations.info", query)
            .await?
            .channel)
    }

    pub async fn conversations_history(
        &self,
        query: &HistoryQuery<'_>,
    ) -> Result<Option<Vec<Message>>> {
        Ok(self
            .request::<HistoryQuery, ConversationsResponse>("conversations.history", query)
            .await?
            .messages)
    }

    pub async fn conversations_replies(
        &self,
        query: &RepliesQuery<'_>,
    ) -> Result<Option<Vec<Message>>> {
        Ok(self
            .request::<RepliesQuery, ConversationsResponse>("conversations.replies", query)
            .await?
            .messages)
    }

    // Helper method to make a request and deserialize the response into `T`
    async fn request<T, U>(&self, path: &str, query: &T) -> Result<U>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let response = self
            .client
            .get(&format!("{}/{}?{}", self.endpoint, path, to_string(query)?))
            .send()
            .await?
            .json::<U>()
            .await?;

        Ok(response)
    }
}

#[derive(Serialize)]
pub struct InfoQuery<'a> {
    pub channel: &'a str,
}

#[derive(Deserialize)]
pub struct InfoResponse {
    pub channel: Channel,
}

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

#[derive(Serialize)]
pub struct RepliesQuery<'a> {
    pub channel: &'a str,
    pub ts: f64,
    pub latest: f64,
    pub oldest: f64,
    pub limit: u64,
    pub inclusive: bool,
}

#[derive(Deserialize)]
pub struct ConversationsResponse {
    pub messages: Option<Vec<Message>>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub text: Option<String>,
}
