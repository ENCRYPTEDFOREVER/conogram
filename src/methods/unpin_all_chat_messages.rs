use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to clear the list of pinned messages in a chat. In private chats and channel direct messages chats, no additional rights are required to unpin all pinned messages. Conversely, the bot must be an administrator with the 'can\_pin\_messages' right or the 'can\_edit\_messages' right to unpin all pinned messages in groups and channels respectively. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#unpinallchatmessages)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct UnpinAllChatMessagesParams {
    /// Unique identifier for the target chat or username of the target channel in the format `@username`
    pub chat_id: ChatId,
}

// Divider: all content below this line will be preserved after code regen
