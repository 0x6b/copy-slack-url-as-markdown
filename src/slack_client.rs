use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;

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

    pub async fn conversations_info(&self, id: &str) -> Result<ConversationsInfoResponse> {
        Ok(self
            .request::<ConversationsInfoResponse>(&format!("/conversations.info?channel={id}"))
            .await?)
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

#[derive(serde::Deserialize)]
pub struct ConversationsInfoResponse {
    pub ok: bool,
    pub channel: Channel,
}

#[derive(serde::Deserialize)]
pub struct Channel {
    pub id: String,
    pub name_normalized: String,
    pub is_channel: bool,
    pub is_group: bool,
    pub is_im: bool,
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_conversations_info() {
        let client = super::Client::new(env!("SLACK_TOKEN")).unwrap();
        let response = client.conversations_info("C02HVJKFGD8").await.unwrap();
        assert_eq!(response.ok, true);
        assert_eq!(response.channel.id, "C02HVJKFGD8");
        assert_eq!(response.channel.name_normalized, "_kaoru-scratchpad");
        assert_eq!(response.channel.is_channel, true);
        assert_eq!(response.channel.is_group, false);
        assert_eq!(response.channel.is_im, false);
    }
}
