use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to clear the list of pinned messages in a forum topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the *can\_pin\_messages* administrator right in the supergroup. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#unpinallforumtopicmessages)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct UnpinAllForumTopicMessagesParams {
    /// Unique identifier for the target chat or username of the target supergroup in the format `@username`
    pub chat_id: ChatId,

    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

// Divider: all content below this line will be preserved after code regen
