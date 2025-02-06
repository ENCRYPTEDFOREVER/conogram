use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::misc::chat_id::ChatId, utils::deserialize_utils::is_false};

/// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#pinchatmessage)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct PinChatMessageParams {
    /// Unique identifier of the business connection on behalf of which the message will be pinned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Identifier of a message to pin
    pub message_id: i64,

    /// Pass *True* if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    #[serde(skip_serializing_if = "is_false")]
    pub disable_notification: bool,
}

// Divider: all content below this line will be preserved after code regen
