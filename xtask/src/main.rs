use std::{collections::BTreeMap, env::var, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use reqwest::get;
use serde_json::from_slice;
use tokio::fs::{create_dir_all, write};

use crate::emoji::Emoji;
mod emoji;

#[derive(Parser)]
#[clap(about, version)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Download the latest data and build an emoji table
    BuildEmojiTable,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::BuildEmojiTable => {
            let url = "https://raw.githubusercontent.com/iamcal/emoji-data/master/emoji.json";
            let blob = get(url).await?.bytes().await?;

            let emojis = from_slice::<Vec<Emoji>>(&blob)?
                .into_iter()
                .flat_map(|emoji| {
                    emoji
                        .short_names
                        .into_iter()
                        .map(|name| (format!(":{name}:"), to_emoji(&emoji.unified)))
                        .collect::<Vec<(String, String)>>()
                })
                .collect::<BTreeMap<String, String>>();

            let output_dir = PathBuf::from(var("CARGO_WORKSPACE_DIR")?).join("assets");
            create_dir_all(&output_dir).await?;

            let output_file = output_dir.join("emoji.json");
            write(&output_file, serde_json::to_string_pretty(&emojis)?).await?;
        }
    }

    Ok(())
}

#[inline(always)]
fn to_emoji(s: &str) -> String {
    s.split('-')
        .take(1)
        .map(|c| char::from_u32(u32::from_str_radix(c, 16).unwrap()).unwrap())
        .collect::<String>()
}
