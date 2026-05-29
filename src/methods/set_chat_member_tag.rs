use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to set a tag for a regular member in a group or a supergroup. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_tags* administrator right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setchatmembertag)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetChatMemberTagParams {
    /// Unique identifier for the target chat or username of the target supergroup in the format `@username`
    pub chat_id: ChatId,

    /// Unique identifier of the target user
    pub user_id: i64,

    /// New tag for the member; 0-16 characters, emoji are not allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

// Divider: all content below this line will be preserved after code regen
