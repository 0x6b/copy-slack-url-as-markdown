use strum_macros::AsRefStr;

/// The type of the template.
#[derive(Debug, AsRefStr)]
pub enum TemplateType {
    /// For plain text, without a quote.
    #[strum(serialize = "text")]
    Text,
    /// For plain text, with a quote.
    #[strum(serialize = "text_quote")]
    TextQuote,
    /// For rich text, without a quote.
    #[strum(serialize = "rich_text")]
    RichText,
    /// For rich text, with a quote.
    #[strum(serialize = "rich_text_quote")]
    RichTextQuote,
}
