use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{de::DeserializeOwned, Deserialize};

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

    pub async fn conversations_info(&self, channel_id: &str) -> Result<ConversationsInfoResponse> {
        Ok(self
            .request::<ConversationsInfoResponse>(&format!(
                "/conversations.info?channel={channel_id}"
            ))
            .await?)
    }

    pub async fn conversations_history(
        &self,
        channel_id: &str,
        latest: f64,
        oldest: f64,
        limit: u64,
    ) -> Result<Option<Vec<Message>>> {
        Ok(self
            .request::<ConversationsResponse>(&format!(
                "/conversations.history?channel={channel_id}&limit={limit}&latest={latest}&oldest={oldest}&inclusive=true"
            ))
            .await?
            .messages)
    }

    pub async fn conversations_replies(
        &self,
        channel_id: &str,
        ts: f64,
        latest: f64,
        oldest: f64,
        limit: u64,
    ) -> Result<Option<Vec<Message>>> {
        println!("channel_id: {}, ts: {}, latest: {}, oldest: {}, limit: {}", channel_id, ts, latest, oldest, limit);
        Ok(self
            .request::<ConversationsResponse>(&format!(
                "/conversations.replies?channel={channel_id}&ts={ts}&limit={limit}&latest={latest}&oldest={oldest}&inclusive=true"
            ))
            .await?
            .messages)
    }

    // Helper method to make a request and deserialize the response into `T`
    async fn request<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(&format!("{}{}", self.endpoint, url))
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(response)
    }
}

#[derive(Deserialize)]
pub struct ConversationsInfoResponse {
    pub ok: bool,
    pub channel: Channel,
}

#[derive(Deserialize)]
pub struct Channel {
    pub id: String,
    pub name_normalized: String,
    pub is_channel: bool,
    pub is_group: bool,
    pub is_im: bool,
}

#[derive(Deserialize)]
pub struct ConversationsResponse {
    pub ok: bool,
    pub messages: Option<Vec<Message>>,
}

#[derive(Deserialize, Debug )]
pub struct Message {
    pub ts: String,
    pub user: Option<String>,
    pub text: Option<String>,
}
