use anyhow::{anyhow, Result};
use arboard::Clipboard;
use clap::Parser;
use url::Url;

use crate::{args::Args, client::Client};

mod args;
mod client;
mod template;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::from((&args).into())
        .await
        .map_err(|why| anyhow!("failed to initialize client: {why}"))?;

    let mut clipboard =
        Clipboard::new().map_err(|why| anyhow!("failed to access system clipboard: {why}"))?;

    let text = match args.url {
        Some(ref url) => url,
        None => &clipboard
            .get_text()
            .map_err(|why| anyhow!("failed to get text from clipboard: {why}"))?,
    };

    let url = Url::parse(text).map_err(|why| {
        anyhow!(
            "The provided text '{}...' is not a valid URL: {why}",
            text.chars().take(40).collect::<String>().trim()
        )
    })?;

    let message = client
        .retrieve(&url)
        .await
        .map_err(|why| anyhow!("failed to retrieve message from Slack: {why}"))?;

    let (rich_text, text) = message
        .render()
        .map_err(|why| anyhow!("failed to render message: {why}"))?;

    match clipboard.set_html(rich_text.trim(), Some(text.trim())) {
        Ok(_) => println!("{text}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}
