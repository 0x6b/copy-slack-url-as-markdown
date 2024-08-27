use anyhow::Result;
use arboard::Clipboard;
use url::Url;

use crate::cli::Cli;

mod cli;
mod context_key;
mod message;
mod slack_client;
mod template_type;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::new().await?;

    let mut clipboard = Clipboard::new()?;
    let content = clipboard.get_text()?;
    let url = Url::parse(content.trim())?;

    let resolved = cli.resolve(&url).await?;
    let (rich_text, text) = resolved.render()?;

    match clipboard.set_html(rich_text.trim(), Some(text.trim())) {
        Ok(_) => println!("{text}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}
