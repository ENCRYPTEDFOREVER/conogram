use serde::{Deserialize, Serialize};

/// This object represents a forum topic.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#forumtopic)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    pub message_thread_id: i64,

    /// Name of the topic
    pub name: String,

    /// Color of the topic icon in RGB format
    pub icon_color: i64,

    /// *Optional*. Unique identifier of the custom emoji shown as the topic icon
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
