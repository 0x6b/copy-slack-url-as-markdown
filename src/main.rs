use anyhow::{bail, Result};

use crate::client::Client;

mod client;
mod state;
mod template;

#[tokio::main]
async fn main() -> Result<()> {
    let client = match Client::new().await {
        Ok(c) => c,
        Err(why) => bail!("failed to initialize client: {why}"),
    };

    let mut clipboard = match arboard::Clipboard::new() {
        Ok(c) => c,
        Err(why) => bail!("failed to access system clipboard: {why}"),
    };

    let text = match clipboard.get_text() {
        Ok(t) => t,
        Err(why) => bail!("failed to get text from clipboard: {why}"),
    };

    let url = match url::Url::parse(text.trim()) {
        Ok(u) => u,
        Err(why) => bail!("The provided text '{text}' is not a valid URL: {why}"),
    };

    let message = match client.retrieve(&url).await {
        Ok(m) => m,
        Err(why) => bail!("failed to retrieve message from Slack: {why}"),
    };

    let (rich_text, text) = match message.render() {
        Ok((r, t)) => (r, t),
        Err(why) => bail!("failed to render message: {why}"),
    };

    match clipboard.set_html(rich_text.trim(), Some(text.trim())) {
        Ok(_) => println!("{text}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}
