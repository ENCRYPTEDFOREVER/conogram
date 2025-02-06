use conogram_derives::Request;
use serde::Serialize;

use crate::entities::misc::chat_id::ChatId;

/// Use this method to get the number of members in a chat. Returns *Int* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getchatmembercount)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = i64)]
pub struct GetChatMemberCountParams {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

// Divider: all content below this line will be preserved after code regen
