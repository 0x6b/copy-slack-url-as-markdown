use anyhow::{bail, Result};

use crate::copier::Copier;

mod copier;
mod state;
mod template;

#[tokio::main]
async fn main() -> Result<()> {
    let copier = Copier::new().await?;

    let mut clipboard = arboard::Clipboard::new()?;

    let text = match clipboard.get_text() {
        Ok(text) => text,
        Err(_) => bail!("failed to access system clipboard"),
    };

    let url = match url::Url::parse(text.trim()) {
        Ok(url) => url,
        Err(_) => bail!("invalid URL provided: {text}"),
    };

    let message = copier.resolve(&url).await?;

    let (rich_text, text) = message.render()?;
    match clipboard.set_html(rich_text.trim(), Some(text.trim())) {
        Ok(_) => println!("{text}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}
