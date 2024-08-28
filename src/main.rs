use crate::copier::Copier;

mod copier;
mod state;
mod template;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let copier = Copier::new().await?;

    let mut clipboard = arboard::Clipboard::new()?;
    let copier = copier
        .resolve(&url::Url::parse(clipboard.get_text()?.trim())?)
        .await?;

    let (rich_text, text) = copier.render()?;
    match clipboard.set_html(rich_text.trim(), Some(text.trim())) {
        Ok(_) => println!("{text}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}
