use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::misc::chat_id::ChatId, utils::deserialize_utils::is_false};

/// Use this method to add a message to the list of pinned messages in a chat. In private chats and channel direct messages chats, all non-service messages can be pinned. Conversely, the bot must be an administrator with the 'can\_pin\_messages' right or the 'can\_edit\_messages' right to pin messages in groups and channels respectively. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#pinchatmessage)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct PinChatMessageParams {
    /// Unique identifier of the business connection on behalf of which the message will be pinned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Unique identifier for the target chat or username of the target channel in the format `@username`
    pub chat_id: ChatId,

    /// Identifier of a message to pin
    pub message_id: i64,

    /// Pass *True* if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    #[serde(skip_serializing_if = "is_false")]
    pub disable_notification: bool,
}

// Divider: all content below this line will be preserved after code regen
