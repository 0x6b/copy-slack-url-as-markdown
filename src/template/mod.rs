mod context_key;
mod template_type;

use clap::Parser;
pub use context_key::ContextKey;
pub use template_type::TemplateType;

/// Templates for the client.
#[derive(Parser, Clone)]
pub struct Templates {
    /// Path to the template file or a string for plain text (without quote). Leave empty to use
    /// the default.
    #[arg(long, env = "TEMPLATE_PLAIN_TEXT")]
    pub plain_text: Option<String>,

    /// Path to the template file or a string for plain text (with quote). Leave empty to use the
    /// default.
    #[arg(long, env = "TEMPLATE_PLAIN_TEXT_QUOTE")]
    pub plain_text_quote: Option<String>,

    /// Path to the template file or a string for rich text (without quote). Leave empty to use the
    /// default.
    #[arg(long, env = "TEMPLATE_RICH_TEXT")]
    pub rich_text: Option<String>,

    /// Path to the template file or a string for rich text (with quote). Leave empty to use the
    /// template.
    #[arg(long, env = "TEMPLATE_RICH_TEXT_QUOTE")]
    pub rich_text_quote: Option<String>,
}
