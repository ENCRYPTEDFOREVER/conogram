use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::misc::chat_id::ChatId, utils::deserialize_utils::is_false};

/// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [unbanned](https://core.telegram.org/bots/api/#unbanchatmember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#banchatmember)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct BanChatMemberParams {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier of the target user
    pub user_id: i64,

    /// Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,

    /// Pass *True* to delete all messages from the chat for the user that is being removed. If *False*, the user will be able to see messages in the group that were sent before the user was removed. Always *True* for supergroups and channels.
    #[serde(skip_serializing_if = "is_false")]
    pub revoke_messages: bool,
}

// Divider: all content below this line will be preserved after code regen
