use serde::{Deserialize, Serialize};

/// This object represents a service message about a new forum topic created in the chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#forumtopiccreated)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForumTopicCreated {
    /// Name of the topic
    pub name: String,

    /// Color of the topic icon in RGB format
    pub icon_color: i64,

    /// *Optional*. Unique identifier of the custom emoji shown as the topic icon
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
