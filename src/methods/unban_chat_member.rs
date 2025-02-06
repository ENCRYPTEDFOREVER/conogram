use conogram_derives::Request;
use serde::Serialize;

use crate::{entities::misc::chat_id::ChatId, utils::deserialize_utils::is_false};

/// Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter *only\_if\_banned*. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#unbanchatmember)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct UnbanChatMemberParams {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier of the target user
    pub user_id: i64,

    /// Do nothing if the user is not banned
    #[serde(skip_serializing_if = "is_false")]
    pub only_if_banned: bool,
}

// Divider: all content below this line will be preserved after code regen
