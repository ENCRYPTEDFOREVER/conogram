use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{chat_member::ChatMember, misc::chat_id::ChatId};

/// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a [ChatMember](https://core.telegram.org/bots/api/#chatmember) object on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getchatmember)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = ChatMember)]
pub struct GetChatMemberParams {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier of the target user
    pub user_id: i64,
}

// Divider: all content below this line will be preserved after code regen
