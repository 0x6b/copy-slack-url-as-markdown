use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Emoji {
    pub name: String,
    pub unified: String,
    pub short_name: String,
    pub short_names: Vec<String>,
}
