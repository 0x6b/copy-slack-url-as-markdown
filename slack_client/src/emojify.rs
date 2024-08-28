use std::{collections::BTreeMap, sync::LazyLock};

use regex::Regex;
use serde_json::from_slice;

static DATA: &[u8] = include_bytes!("../../assets/emoji.json");
static TABLE: LazyLock<BTreeMap<&str, &str>> =
    LazyLock::new(|| from_slice::<BTreeMap<&str, &str>>(DATA).unwrap());
static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(:[a-zA-Z0-9\-_+]+:)").unwrap());

pub trait Emojify {
    #[allow(dead_code)]
    fn emojify(&self) -> String;
}

impl<T> Emojify for T
where
    T: AsRef<str>,
{
    fn emojify(&self) -> String {
        let s = self.as_ref();
        let mut new_text = String::with_capacity(s.len());
        let mut last = 0;

        for cap in RE.captures_iter(s) {
            if let Some(m) = cap.get(0) {
                if let Some(emoji) = TABLE.get(m.as_str()) {
                    new_text.push_str(&s[last..m.start()]);
                    new_text.push_str(emoji);
                    last = m.end();
                }
            }
        }

        new_text.push_str(&s[last..]);
        new_text
    }
}

#[cfg(test)]
mod tests {
    use crate::slack::Emojify;

    #[test]
    fn test_emojify() {
        assert_eq!(
            ":omochi: :hiking_boot: :anger::canned_food: :wavy_dash: :motorway: I kicked the can down the road on my other two in-progress tasks.".emojify(),
            ":omochi: 🥾 💢🥫 〰 🛣 I kicked the can down the road on my other two in-progress tasks."
        );
    }
}
