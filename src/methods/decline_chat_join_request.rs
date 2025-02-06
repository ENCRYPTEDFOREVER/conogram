use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\_invite\_users* administrator right. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#declinechatjoinrequest)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeclineChatJoinRequestParams {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier of the target user
    pub user_id: i64,
}

// Divider: all content below this line will be preserved after code regen
