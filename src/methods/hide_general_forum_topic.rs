use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. The topic will be automatically closed if it was open. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#hidegeneralforumtopic)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct HideGeneralForumTopicParams {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatId,
}

// Divider: all content below this line will be preserved after code regen
