use serde::{Deserialize, Serialize};

use crate::utils::deserialize_utils::is_false;

/// This object represents a forum topic.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#forumtopic)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
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

    /// *Optional*. *True*, if the name of the topic wasn't specified explicitly by its creator and likely needs to be changed by the bot
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_name_implicit: bool,
}

// Divider: all content below this line will be preserved after code regen
