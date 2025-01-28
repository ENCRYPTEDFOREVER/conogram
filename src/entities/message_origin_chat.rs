use serde::{Deserialize, Serialize};

use crate::entities::chat::Chat;

/// The message was originally sent on behalf of a chat to a group chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#messageoriginchat)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageOriginChat {
    /// Date the message was sent originally in Unix time
    pub date: i64,

    /// Chat that sent the message originally
    pub sender_chat: Box<Chat>,

    /// *Optional*. For messages originally sent by an anonymous chat administrator, original message author signature
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
