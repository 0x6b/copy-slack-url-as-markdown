use strum_macros::AsRefStr;

#[derive(Debug, AsRefStr)]
pub enum TemplateType {
    #[strum(serialize = "text")]
    Text,

    #[strum(serialize = "text_quote")]
    TextQuote,

    #[strum(serialize = "rich_text")]
    RichText,

    #[strum(serialize = "rich_text_quote")]
    RichTextQuote,
}
