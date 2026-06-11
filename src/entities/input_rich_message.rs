use serde::Serialize;

use crate::utils::deserialize_utils::is_false;

/// Describes a rich message to be sent. Exactly **one** of the fields *html* or *markdown* must be used.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichmessage)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct InputRichMessage {
    /// *Optional*. Content of the rich message to send described using HTML formatting. See [rich message formatting options](https://core.telegram.org/bots/api/#rich-message-formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,

    /// *Optional*. Content of the rich message to send described using Markdown formatting. See [rich message formatting options](https://core.telegram.org/bots/api/#rich-message-formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,

    /// *Optional*. Pass *True* if the rich message must be shown right-to-left
    #[serde(skip_serializing_if = "is_false")]
    pub is_rtl: bool,

    /// *Optional*. Pass *True* to skip automatic detection of entities (e.g., URLs, email addresses, username mentions, hashtags, cashtags, bot commands, or phone numbers) in the text
    #[serde(skip_serializing_if = "is_false")]
    pub skip_entity_detection: bool,
}

// Divider: all content below this line will be preserved after code regen
