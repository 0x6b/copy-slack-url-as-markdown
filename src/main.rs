use anyhow::{bail, Result};
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
    let client = match Client::from((&args).into()).await {
        Ok(c) => c,
        Err(why) => bail!("failed to initialize client: {why}"),
    };

    let mut clipboard = match Clipboard::new() {
        Ok(c) => c,
        Err(why) => bail!("failed to access system clipboard: {why}"),
    };

    let text = clipboard.get_text();
    let text = match args.url {
        Some(ref url) => url,
        None => match text {
            Ok(ref t) => t,
            Err(why) => bail!("failed to get text from clipboard: {why}"),
        },
    };

    let url = Url::parse(text).map_err(|why| {
        anyhow!(
            "The provided text '{}...' is not a valid URL: {why}",
            text.chars().take(40).collect::<String>().trim()
        )
    })?;

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
