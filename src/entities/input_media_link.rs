use serde::Serialize;

/// Represents an HTTP link to be sent.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputmedialink)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename = "link", tag = "type")]
pub struct InputMediaLink {
    /// HTTP URL of the link
    pub url: String,
}

// Divider: all content below this line will be preserved after code regen
