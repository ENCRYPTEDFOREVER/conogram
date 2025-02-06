use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the *can\_pin\_messages* administrator right in the supergroup. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#unpinallgeneralforumtopicmessages)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct UnpinAllGeneralForumTopicMessagesParams {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatId,
}

// Divider: all content below this line will be preserved after code regen
