use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to remove a message from the list of pinned messages in a chat. In private chats and channel direct messages chats, all messages can be unpinned. Conversely, the bot must be an administrator with the 'can\_pin\_messages' right or the 'can\_edit\_messages' right to unpin messages in groups and channels respectively. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#unpinchatmessage)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct UnpinChatMessageParams {
    /// Unique identifier of the business connection on behalf of which the message will be unpinned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Unique identifier for the target chat or username of the target channel in the format `@username`
    pub chat_id: ChatId,

    /// Identifier of the message to unpin. Required if *business\_connection\_id* is specified. If not specified, the most recent pinned message (by sending date) will be unpinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
