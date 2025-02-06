use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{misc::chat_id::ChatId, user_chat_boosts::UserChatBoosts};

/// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a [UserChatBoosts](https://core.telegram.org/bots/api/#userchatboosts) object.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getuserchatboosts)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = UserChatBoosts)]
pub struct GetUserChatBoostsParams {
    /// Unique identifier for the chat or username of the channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier of the target user
    pub user_id: i64,
}

// Divider: all content below this line will be preserved after code regen
