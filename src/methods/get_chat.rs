use conogram_derives::Request;
use serde::Serialize;

use crate::entities::{chat_full_info::ChatFullInfo, misc::chat_id::ChatId};

/// Use this method to get up-to-date information about the chat. Returns a [ChatFullInfo](https://core.telegram.org/bots/api/#chatfullinfo) object on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getchat)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = ChatFullInfo)]
pub struct GetChatParams {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatId,
}

// Divider: all content below this line will be preserved after code regen
