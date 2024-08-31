mod slack_message;
pub mod state;

use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;
pub use slack_message::SlackMessage;

static RE_USER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<@([UW][A-Z0-9]+)>").unwrap());
static RE_CHANNEL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<#([CG][A-Z0-9]+)(\|.*)?>").unwrap());
static RE_USERGROUP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<!subteam\^([A-Z0-9]+)>").unwrap());
static RE_LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<([^|]+)\|([^>]+)?>").unwrap());

#[derive(Deserialize, Debug, Clone, Copy)]
struct QueryParams {
    thread_ts: Option<f64>,
}