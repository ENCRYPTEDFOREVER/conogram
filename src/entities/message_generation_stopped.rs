use serde::{Deserialize, Serialize};

use crate::entities::chat::Chat;

/// This object describes an update about a user stopping message generation.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#messagegenerationstopped)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGenerationStopped {
    /// Chat in which the message is generated
    pub chat: Box<Chat>,

    /// *Optional*. Unique identifier of the message thread in which the message is generated
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// Unique identifier of the message draft which was stopped
    pub draft_id: i64,
}

// Divider: all content below this line will be preserved after code regen
