use serde::Deserialize;

use crate::response::Response;

#[derive(Deserialize, Debug)]
pub struct BotsInfo {
    pub ok: bool,
    /// The bot object.
    pub bot: Option<Bot>,
}
impl Response for BotsInfo {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Deserialize, Debug)]
pub struct Bot {
    /// The name of the bot.
    pub name: String,
}
