use serde::{Deserialize, Serialize};

use crate::entities::chat::Chat;

/// This object is received when messages are deleted from a connected business account.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#businessmessagesdeleted)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection
    pub business_connection_id: String,

    /// Information about a chat in the business account. The bot may not have access to the chat or the corresponding user.
    pub chat: Box<Chat>,

    /// The list of identifiers of deleted messages in the chat of the business account
    pub message_ids: Vec<i64>,
}

// Divider: all content below this line will be preserved after code regen
