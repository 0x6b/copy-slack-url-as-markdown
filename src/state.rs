use clap::Parser;
use tera::{Context, Tera};

/// A marker trait for the state of the client.
///
/// This trait is used to ensure that the client is in a valid state before performing any
/// operations.
///
/// Possible states are:
///
/// - `Uninitialized`: The client has not been initialized yet.
/// - `Initialized`: The client has been initialized with the CLI arguments. "Initialized" means
///   that Tera has been set up with the templates.
/// - `Retrieved`: The client has retrieved a message from Slack. "Retrieved" means that the Slack
///   message data has been successfully retrieved and processed and is ready to be rendered.
///
/// The client transitions through these states in the following order:
///
/// `Client<Uninitialized>` → `new()` → `Client<Initialized>` → `retrieve()` →
/// `Client<Retrieved>` → `render()` → Your text
pub trait State {}
impl State for Uninitialized {}
impl State for Initialized {}
impl State for Retrieved {}

/// Uninitialized state of the client, or the CLI arguments.
pub struct Uninitialized {
    /// Slack API token.
    pub token: String,

    /// Include the message body as a quote.
    pub quote: bool,

    /// The IANA time zone database identifiers to use for the timestamp.
    pub timezone: String,

    pub templates: Templates,
}

/// Templates for the client.
#[derive(Parser, Clone)]
pub struct Templates {
    /// Path to the template file or a string for plain text (without quote). Leave empty to use
    /// the default.
    #[arg(long, env = "TEMPLATE_TEXT")]
    pub text: Option<String>,

    /// Path to the template file or a string for plain text (with quote). Leave empty to use the
    /// default.
    #[arg(long, env = "TEMPLATE_TEXT_QUOTE")]
    pub text_quote: Option<String>,

    /// Path to the template file or a string for rich text (without quote). Leave empty to use the
    /// default.
    #[arg(long, env = "TEMPLATE_RICH_TEXT")]
    pub rich_text: Option<String>,

    /// Path to the template file or a string for rich text (with quote). Leave empty to use the
    /// template.
    #[arg(long, env = "TEMPLATE_RICH_TEXT_QUOTE")]
    pub rich_text_quote: Option<String>,
}

/// Initialized state of the client.
pub struct Initialized {
    /// Slack API token.
    pub token: String,
    /// Include the message body as a quote.
    pub quote: bool,
    /// The IANA time zone database identifiers to use for the timestamp.
    pub timezone: String,
    /// The Tera template engine with the templates set up.
    pub tera: Tera,
}

/// Retrieved state of the client.
pub struct Retrieved {
    /// Include the message body as a quote.
    pub quote: bool,
    /// The tera template engine.
    pub tera: Tera,
    /// The Slack message data as a template context.
    pub context: Context,
}
