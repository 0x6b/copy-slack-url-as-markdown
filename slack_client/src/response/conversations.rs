use std::fmt::Display;

use serde::Deserialize;

use crate::response::Response;

#[derive(Deserialize, Debug)]
pub struct ConversationsInfo {
    pub ok: bool,
    pub channel: Option<Channel>,
}
impl Response for ConversationsInfo {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Deserialize, Debug)]
pub struct Channel {
    pub is_im: Option<bool>,
    pub is_mpim: Option<bool>,
    pub name_normalized: Option<String>,
    pub purpose: Option<Purpose>,
    pub user: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Conversations {
    pub ok: bool,
    pub messages: Option<Vec<Message>>,
}
impl Response for Conversations {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Deserialize, Debug)]
pub struct Message {
    /// User ID of the author.
    pub user: Option<String>,
    /// bot ID of the author.
    pub bot_id: Option<String>,
    /// The text of the message.
    pub text: Option<String>,
    /// The Slack block kit blocks of the message.
    pub blocks: Option<Vec<Block>>,
}

#[derive(Deserialize, Debug)]
pub struct Purpose {
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Block {
    #[serde(rename = "type")]
    pub block_type: BlockType,
    pub elements: Option<Vec<RichTextElement>>,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.block_type {
            BlockType::RichText => match &self.elements {
                Some(elements) => {
                    for element in elements {
                        write!(f, "{}", element)?;
                    }
                }
                None => {}
            },
            BlockType::Header => {}
            BlockType::Divider => {}
            BlockType::Actions => {}
            BlockType::Context => {}
            BlockType::File => {}
            BlockType::Image => {}
            BlockType::Input => {}
            BlockType::Section => {}
            BlockType::Video => {}
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    RichText,
    Header,
    Divider,

    Actions, // not supported
    Context, // not supported
    File,    // not supported
    Image,   // not supported
    Input,   // not supported
    Section, // not supported
    Video,   // not supported}
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichTextElement {
    RichTextSection { elements: Vec<RichTextElement> },
    RichTextQuote { elements: Vec<RichTextElement> },
    RichTextList { style: ListStyle, indent: i64, elements: Vec<RichTextElement> },
    RichTextPreformatted { elements: Vec<RichTextElement> },
    Emoji { name: String, style: Option<Style> },
    Text { text: String, style: Option<Style> },
    Mrkdwn { text: String, style: Option<Style> },
    Link { url: String, text: Option<String> },
    User { user_id: String },
    Usergroup { usergroup_id: String },
    Broadcast { range: String },
    Channel { channel_id: String, style: Option<Style> },
}

impl Display for RichTextElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RichTextElement::RichTextSection { elements } => {
                let mut result = String::new();
                for element in elements {
                    result.push_str(&element.to_string());
                }
                result
            }
            RichTextElement::RichTextQuote { elements } => {
                let mut result = String::new();
                for element in elements {
                    result.push_str(&format!("> {}", element));
                }
                result
            }
            RichTextElement::RichTextList { style, indent, elements } => {
                let mut result = String::new();
                for (n, element) in elements.iter().enumerate() {
                    match style {
                        ListStyle::Bullet => {
                            result.push_str(&"  ".repeat(*indent as usize));
                            result.push_str("- ");
                        }
                        ListStyle::Ordered => {
                            result.push_str(&"   ".repeat(*indent as usize));
                            result.push_str(&format!("{}. ", n + 1));
                        }
                    }
                    result.push_str(&element.to_string());
                    result.push('\n');
                }
                result
            }
            RichTextElement::RichTextPreformatted { elements } => {
                let mut result = String::new();
                for element in elements {
                    result.push_str("```\n");
                    result.push_str(&element.to_string());
                    result.push_str("\n```\n");
                }
                result
            }
            RichTextElement::Emoji { name, style: _style } => {
                let mut result = String::new();
                result.push_str(name);
                result
            }
            RichTextElement::Text { text, style } => {
                let mut result = String::new();
                match style {
                    Some(Style { code, bold, italic, strike }) => {
                        let (code, bold, italic, strike) = (
                            code.unwrap_or_default(),
                            bold.unwrap_or_default(),
                            italic.unwrap_or_default(),
                            strike.unwrap_or_default(),
                        );
                        result.push_str(
                            &Some(text.to_string())
                                .map(|t| if code { format!("`{}`", t) } else { t })
                                .map(|t| if bold { format!("**{}**", t) } else { t })
                                .map(|t| if italic { format!("_{}_", t) } else { t })
                                .map(|t| if strike { format!("~~{}~~", t) } else { t })
                                .unwrap(),
                        );
                    }
                    None => {
                        result.push_str(text);
                    }
                }
                result
            }
            RichTextElement::Mrkdwn { text, style } => {
                let mut result = String::new();
                match style {
                    Some(Style { code, bold, italic, strike }) => {
                        let (code, bold, italic, strike) = (
                            code.unwrap_or_default(),
                            bold.unwrap_or_default(),
                            italic.unwrap_or_default(),
                            strike.unwrap_or_default(),
                        );
                        result.push_str(
                            &Some(text.to_string())
                                .map(|t| if code { format!("`{}`", t) } else { t })
                                .map(|t| if bold { format!("**{}**", t) } else { t })
                                .map(|t| if italic { format!("_{}_", t) } else { t })
                                .map(|t| if strike { format!("~~{}~~", t) } else { t })
                                .unwrap(),
                        );
                    }
                    None => {
                        result.push_str(text);
                    }
                }
                result
            }
            RichTextElement::Link { url, text } => {
                let mut result = String::new();
                match text {
                    Some(t) => {
                        result.push('[');
                        result.push_str(t);
                        result.push_str("](");
                        result.push_str(url);
                        result.push(')');
                    }
                    None => {
                        result.push_str(url);
                    }
                }
                result
            }
            RichTextElement::User { user_id } => {
                let mut result = String::new();
                result.push_str("<@");
                result.push_str(user_id);
                result.push('>');
                result
            }
            RichTextElement::Usergroup { usergroup_id } => {
                let mut result = String::new();
                result.push_str("<!subteam^");
                result.push_str(usergroup_id);
                result.push('>');
                result
            }
            RichTextElement::Broadcast { range } => {
                let mut result = String::new();
                result.push('@');
                result.push_str(range);
                result
            }
            RichTextElement::Channel { channel_id, style: _style } => {
                let mut result = String::new();
                result.push_str("<#");
                result.push_str(channel_id);
                result.push('>');
                result
            }
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Style {
    code: Option<bool>,
    bold: Option<bool>,
    italic: Option<bool>,
    strike: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ListStyle {
    Bullet,
    Ordered,
}
