use serde::{Deserialize, Serialize};

use crate::entities::chat::Chat;

/// The message was originally sent to a channel chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#messageoriginchannel)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageOriginChannel {
    /// Date the message was sent originally in Unix time
    pub date: i64,

    /// Channel chat to which the message was originally sent
    pub chat: Box<Chat>,

    /// Unique message identifier inside the chat
    pub message_id: i64,

    /// *Optional*. Signature of the original post author
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
