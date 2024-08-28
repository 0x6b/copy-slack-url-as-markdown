use crate::cli::Cli;

mod cli;
mod slack;
mod template;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::new().await?;

    let mut clipboard = arboard::Clipboard::new()?;
    let resolved = cli.resolve(&url::Url::parse(clipboard.get_text()?.trim())?).await?;

    let (rich_text, text) = resolved.render()?;
    match clipboard.set_html(rich_text.trim(), Some(text.trim())) {
        Ok(_) => println!("{text}"),
        Err(why) => println!("{why}"),
    }

    Ok(())
}
