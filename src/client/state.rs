use tera::{Context, Tera};

use crate::template::Templates;

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
impl State for Uninitialized<'_> {}
impl State for Initialized<'_> {}
impl State for Retrieved {}

/// Uninitialized state of the client, or the CLI arguments.
pub struct Uninitialized<'state> {
    /// Slack API token.
    pub token: &'state str,
    /// Include the message body as a quote.
    pub quote: bool,
    /// The IANA time zone database identifiers to use for the timestamp.
    pub timezone: &'state str,
    pub templates: Templates,
}

/// Initialized state of the client.
pub struct Initialized<'state> {
    /// Slack API token.
    pub token: &'state str,

    /// Include the message body as a quote.
    pub quote: bool,

    /// The IANA time zone database identifiers to use for the timestamp.
    pub timezone: &'state str,

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
